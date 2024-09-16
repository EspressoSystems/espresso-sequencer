use hotshot::{
    traits::{election::static_committee::GeneralStaticCommittee, NodeImplementation},
    types::{Event, SystemContextHandle},
};
use hotshot_builder_api::v0_1::{
    block_info::{AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo},
    builder::BuildError,
    data_source::{AcceptsTxnSubmits, BuilderDataSource},
};
use hotshot_types::{
    data::{DaProposal, Leaf, QuorumProposal},
    event::EventType,
    message::Proposal,
    traits::{
        block_contents::BlockPayload,
        consensus_api::ConsensusApi,
        election::Membership,
        network::Topic,
        node_implementation::{ConsensusTime, NodeType, Versions},
        signature_key::{BuilderSignatureKey, SignatureKey},
    },
    utils::BuilderCommitment,
    vid::{VidCommitment, VidPrecomputeData},
};
use lru::LruCache;

use crate::{
    builder_state::{
        BuildBlockInfo, DaProposalMessage, DecideMessage, QCMessage, TransactionSource,
        TriggerStatus,
    },
    BlockId,
};
use crate::{
    builder_state::{MessageType, RequestMessage, ResponseMessage},
    BuilderStateId,
};
use crate::{WaitAndKeep, WaitAndKeepGetError};
use anyhow::anyhow;
pub use async_broadcast::{broadcast, RecvError, TryRecvError};
use async_broadcast::{Sender as BroadcastSender, TrySendError};
use async_compatibility_layer::{
    art::async_sleep,
    art::async_timeout,
    channel::{unbounded, OneShotSender},
};
use async_lock::RwLock;
use async_trait::async_trait;
use committable::{Commitment, Committable};
use derivative::Derivative;
use futures::stream::StreamExt;
use futures::{future::BoxFuture, Stream};
use hotshot_events_service::{events::Error as EventStreamError, events_source::StartupInfo};
use sha2::{Digest, Sha256};
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashMap, marker::PhantomData};
use std::{fmt::Display, time::Instant};
use tagged_base64::TaggedBase64;
use tide_disco::{method::ReadState, Url};

// Start assuming we're fine calculatig VID for 5 megabyte blocks
const INITIAL_MAX_BLOCK_SIZE: u64 = 5_000_000;
// Never go lower than 10 kilobytes
const MAX_BLOCK_SIZE_FLOOR: u64 = 10_000;
// When adjusting max block size, we it will be decremented or incremented
// by current value / [`MAX_BLOCK_SIZE_CHANGE_DIVISOR`]
const MAX_BLOCK_SIZE_CHANGE_DIVISOR: u64 = 10;
// We will not increment max block value if we aren't able to serve a response
// with a margin below [`ProxyGlobalState::max_api_waiting_time`]
// more than [`ProxyGlobalState::max_api_waiting_time`] / `VID_RESPONSE_TARGET_MARGIN_DIVISOR`
const VID_RESPONSE_TARGET_MARGIN_DIVISOR: u32 = 10;

// It holds all the necessary information for a block
#[derive(Debug)]
pub struct BlockInfo<Types: NodeType> {
    pub block_payload: Types::BlockPayload,
    pub metadata: <<Types as NodeType>::BlockPayload as BlockPayload<Types>>::Metadata,
    pub vid_trigger: Arc<RwLock<Option<OneShotSender<TriggerStatus>>>>,
    pub vid_receiver: Arc<RwLock<WaitAndKeep<(VidCommitment, VidPrecomputeData)>>>,
    pub offered_fee: u64,
    // Could we have included more transactions with this block, but chose not to?
    pub truncated: bool,
}

/// [ReceivedTransaction] represents receipt information concerning a received
/// [NodeType::Transaction].
#[derive(Debug)]
pub struct ReceivedTransaction<Types: NodeType> {
    // the transaction
    pub tx: Types::Transaction,
    // transaction's hash
    pub commit: Commitment<Types::Transaction>,
    // transaction's esitmated length
    pub len: u64,
    // transaction's source
    pub source: TransactionSource,
    // received time
    pub time_in: Instant,
}

/// [GlobalState] represents the internalized state of the Builder service as
/// represented from its public facing API.
#[allow(clippy::type_complexity)]
#[derive(Debug)]
pub struct GlobalState<Types: NodeType> {
    // data store for the blocks
    pub blocks: lru::LruCache<BlockId<Types>, BlockInfo<Types>>,

    // registered builder states
    pub spawned_builder_states: HashMap<BuilderStateId<Types>, BroadcastSender<MessageType<Types>>>,

    // builder state -> last built block , it is used to respond the client
    // if the req channel times out during get_available_blocks
    pub builder_state_to_last_built_block: HashMap<BuilderStateId<Types>, ResponseMessage>,

    // sending a transaction from the hotshot/private mempool to the builder states
    // NOTE: Currently, we don't differentiate between the transactions from the hotshot and the private mempool
    pub tx_sender: BroadcastSender<Arc<ReceivedTransaction<Types>>>,

    // last garbage collected view number
    pub last_garbage_collected_view_num: Types::Time,

    // highest view running builder task
    pub highest_view_num_builder_id: BuilderStateId<Types>,

    // estimated maximum block size we can build in time
    pub max_block_size: u64,
}

/// GetChannelForMatchingBuilderError is an error enum that represents the
/// class of possible errors that can be returned when calling
/// `get_channel_for_matching_builder_or_highest_view_builder` on a
/// `GlobalState`.  These errors are used for internal representations for
/// consistency and testing, and do not leak beyond the `GlobalState` API.
/// As such, they intentionally do not implement traits for serialization.
#[derive(Debug)]
pub(crate) enum GetChannelForMatchingBuilderError {
    NoBuilderStateFound,
}

impl From<GetChannelForMatchingBuilderError> for BuildError {
    fn from(_error: GetChannelForMatchingBuilderError) -> Self {
        BuildError::Error {
            message: "No builder state found".to_string(),
        }
    }
}

impl<Types: NodeType> GlobalState<Types> {
    /// [new] creates a new [GlobalState] with the given parameters.
    /// The resulting [GlobalState] will have the given
    /// `last_garbage_collected_view_num` as passed.  Additionally, the
    /// `highest_view_num_builder_id` will be set to a [BuilderStateId]
    /// comprised of the given `bootstrapped_builder_state_id` and
    /// `bootstrapped_view_num`.  The `spawned_builder_states` will be created
    /// with a single entry of the same [BuilderStateId] and the given
    /// `bootstrap_sender`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bootstrap_sender: BroadcastSender<MessageType<Types>>,
        tx_sender: BroadcastSender<Arc<ReceivedTransaction<Types>>>,
        bootstrapped_builder_state_id: VidCommitment,
        bootstrapped_view_num: Types::Time,
        last_garbage_collected_view_num: Types::Time,
        _buffer_view_num_count: u64,
    ) -> Self {
        let mut spawned_builder_states = HashMap::new();
        let bootstrap_id = BuilderStateId {
            parent_commitment: bootstrapped_builder_state_id,
            view: bootstrapped_view_num,
        };
        spawned_builder_states.insert(bootstrap_id.clone(), bootstrap_sender.clone());
        GlobalState {
            blocks: LruCache::new(NonZeroUsize::new(256).unwrap()),
            spawned_builder_states,
            tx_sender,
            last_garbage_collected_view_num,
            builder_state_to_last_built_block: Default::default(),
            highest_view_num_builder_id: bootstrap_id,
            max_block_size: INITIAL_MAX_BLOCK_SIZE,
        }
    }

    /// [register_builder_state] associates the given [BuilderStateId] with
    /// the given [BroadcastSender] in the [GlobalState].
    ///
    /// Additionally, if the view of the [BuilderStateId] is greater than the
    /// current highest view number, the [BuilderStateId] is set as the new
    /// highest view number.
    ///
    /// There is potential here for data loss.  Since we just blindly insert
    /// the [BuilderStateId] and [BroadcastSender] into the hashmap, we could
    /// potentially be overwriting an existing entry.  This would result in
    /// the loss of access to a [BroadcastSender], and could potentially
    /// result in unexpected behavior.
    pub fn register_builder_state(
        &mut self,
        parent_id: BuilderStateId<Types>,
        request_sender: BroadcastSender<MessageType<Types>>,
    ) {
        // register the builder state
        let previous_value = self
            .spawned_builder_states
            .insert(parent_id.clone(), request_sender);

        if let Some(previous_value) = previous_value {
            tracing::warn!(
                "builder {parent_id} overwrote previous spawned_builder_state entry: {:?}",
                previous_value
            );
        }

        // keep track of the max view number
        if parent_id.view > self.highest_view_num_builder_id.view {
            tracing::info!("registering builder {parent_id} as highest",);
            self.highest_view_num_builder_id = parent_id;
        } else {
            tracing::warn!(
                "builder {parent_id} created; highest registered is {}",
                self.highest_view_num_builder_id,
            );
        }
    }

    /// [update_global_state] Ensures that the given [BuildBlockInfo]'d id
    /// is within the [GlobalState]'s [blocks] LRU Cache.  The cache stores the
    /// [BlockInfo] associated with the given [BuildBlockInfo]'s id.  However
    /// if it already exists within the LRU cache, then the `BlockInfo` is not
    /// updated.
    ///
    /// Additionally, the [BuilderStateId] is associated with the given
    /// [ResponseMessage] in the [builder_state_to_last_built_block] hashmap.
    ///
    /// No care or consideration is given to anything that may have been
    /// stored with the same key in the [builder_state_to_last_built_block].
    pub fn update_global_state(
        &mut self,
        state_id: BuilderStateId<Types>,
        build_block_info: BuildBlockInfo<Types>,
        response_msg: ResponseMessage,
    ) {
        let BuildBlockInfo {
            id,
            block_payload,
            metadata,
            vid_trigger,
            vid_receiver,
            offered_fee,
            truncated,
            ..
        } = build_block_info;

        let previous_cache_entry = self.blocks.put(
            id.clone(),
            BlockInfo {
                block_payload,
                metadata,
                vid_trigger: Arc::new(RwLock::new(Some(vid_trigger))),
                vid_receiver: Arc::new(RwLock::new(WaitAndKeep::Wait(vid_receiver))),
                offered_fee,
                truncated,
            },
        );

        // update the builder state to last built block
        let previous_builder_state_entry = self
            .builder_state_to_last_built_block
            .insert(state_id, response_msg);

        if let Some(previous_builder_state_entry) = previous_builder_state_entry {
            tracing::warn!(
                "block {id} overwrote previous block: {:?}.  previous cache entry: {:?}",
                previous_builder_state_entry,
                previous_cache_entry
            );
        }
    }

    /// [remove_handles] cleans up the [GlobalState] by removing all
    /// `spawned_builder_states` that have been stored, up to a derived
    /// reference view.  This cutoff point can be up to the given
    /// `on_decide_view` so long as the provided value is less than or equal
    /// to the `highest_view_num_builder_id`'s view stored on the state.
    /// Beyond that, the state prefers to drop all `spawned_builder_states`
    /// preceding the derived cutoff view.
    ///
    /// In addition the `last_garbage_collected_view_num` is updated to the
    /// target cutoff view number for tracking purposes.  The value returned
    /// is the cutoff view number such that the returned value indicates the
    /// point before which everything was cleaned up.
    pub fn remove_handles(&mut self, on_decide_view: Types::Time) -> Types::Time {
        // remove everything from the spawned builder states when view_num <= on_decide_view;
        // if we don't have a highest view > decide, use highest view as cutoff.
        let cutoff = std::cmp::min(self.highest_view_num_builder_id.view, on_decide_view);
        self.spawned_builder_states
            .retain(|id, _| id.view >= cutoff);

        let cutoff_u64 = cutoff.u64();
        let gc_view = if cutoff_u64 > 0 { cutoff_u64 - 1 } else { 0 };

        self.last_garbage_collected_view_num = Types::Time::new(gc_view);

        cutoff
    }

    // private mempool submit txn
    // Currently, we don't differentiate between the transactions from the hotshot and the private mempool
    pub async fn submit_client_txns(
        &self,
        txns: Vec<<Types as NodeType>::Transaction>,
    ) -> Vec<Result<Commitment<<Types as NodeType>::Transaction>, BuildError>> {
        handle_received_txns(
            &self.tx_sender,
            txns,
            TransactionSource::External,
            self.max_block_size,
        )
        .await
    }

    /// [get_channel_for_matching_builder_or_highest_view_builder] is a helper
    /// function that attempts to retrieve the broadcast sender for the given
    /// [BuilderStateId]. If the sender does not exist, it will return the
    /// broadcast sender for the for the hightest view number [BuilderStateId]
    /// instead.
    pub(crate) fn get_channel_for_matching_builder_or_highest_view_builder(
        &self,
        key: &BuilderStateId<Types>,
    ) -> Result<&BroadcastSender<MessageType<Types>>, GetChannelForMatchingBuilderError> {
        if let Some(channel) = self.spawned_builder_states.get(key) {
            tracing::info!("Got matching builder for parent {}", key);
            Ok(channel)
        } else {
            tracing::warn!(
                "failed to recover builder for parent {}, using highest view num builder with {}",
                key,
                self.highest_view_num_builder_id,
            );
            // get the sender for the highest view number builder
            self.spawned_builder_states
                .get(&self.highest_view_num_builder_id)
                .ok_or(GetChannelForMatchingBuilderError::NoBuilderStateFound)
        }
    }

    // check for the existence of the builder state for a view
    pub fn check_builder_state_existence_for_a_view(&self, key: &Types::Time) -> bool {
        // iterate over the spawned builder states and check if the view number exists
        self.spawned_builder_states
            .iter()
            .any(|(id, _)| id.view == *key)
    }

    pub fn should_view_handle_other_proposals(
        &self,
        builder_view: &Types::Time,
        proposal_view: &Types::Time,
    ) -> bool {
        *builder_view == self.highest_view_num_builder_id.view
            && !self.check_builder_state_existence_for_a_view(proposal_view)
    }
}

pub struct ProxyGlobalState<Types: NodeType> {
    // global state
    global_state: Arc<RwLock<GlobalState<Types>>>,

    // identity keys for the builder
    // May be ideal place as GlobalState interacts with hotshot apis
    // and then can sign on responders as desired
    builder_keys: (
        Types::BuilderSignatureKey, // pub key
        <<Types as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::BuilderPrivateKey, // private key
    ),

    // max waiting time to serve first api request
    max_api_waiting_time: Duration,
}

impl<Types: NodeType> ProxyGlobalState<Types> {
    pub fn new(
        global_state: Arc<RwLock<GlobalState<Types>>>,
        builder_keys: (
            Types::BuilderSignatureKey,
            <<Types as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::BuilderPrivateKey,
        ),
        max_api_waiting_time: Duration,
    ) -> Self {
        ProxyGlobalState {
            global_state,
            builder_keys,
            max_api_waiting_time,
        }
    }
}

/// AvailableBlocksError is an error enum that represents the class of possible
/// errors  that can be returned when calling `available_blocks` on a
/// `ProxyGlobalState`.  These errors are used for internal representations
/// for consistency and testing, and do not leak beyond the `ProxyGlobalState`
/// API.  As such, they intentionally do not implement traits for serialization.
#[derive(Debug)]
enum AvailableBlocksError<Types: NodeType> {
    SignatureValidationFailed,
    RequestForAvailableViewThatHasAlreadyBeenDecided,
    SigningBlockFailed(
        <<Types as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::SignError,
    ),
    GetChannelForMatchingBuilderError(GetChannelForMatchingBuilderError),
    NoBlocksAvailable,
    ChannelUnexpectedlyClosed,
}

impl<Types: NodeType> From<GetChannelForMatchingBuilderError> for AvailableBlocksError<Types> {
    fn from(error: GetChannelForMatchingBuilderError) -> Self {
        AvailableBlocksError::GetChannelForMatchingBuilderError(error)
    }
}

impl<Types: NodeType> From<AvailableBlocksError<Types>> for BuildError {
    fn from(error: AvailableBlocksError<Types>) -> Self {
        match error {
            AvailableBlocksError::SignatureValidationFailed => BuildError::Error {
                message: "Signature validation failed in get_available_blocks".to_string(),
            },
            AvailableBlocksError::RequestForAvailableViewThatHasAlreadyBeenDecided => {
                BuildError::Error {
                    message:
                        "Request for available blocks for a view that has already been decided."
                            .to_string(),
                }
            }
            AvailableBlocksError::SigningBlockFailed(e) => BuildError::Error {
                message: format!("Signing over block info failed: {:?}", e),
            },
            AvailableBlocksError::GetChannelForMatchingBuilderError(e) => e.into(),
            AvailableBlocksError::NoBlocksAvailable => BuildError::Error {
                message: "No blocks available".to_string(),
            },
            AvailableBlocksError::ChannelUnexpectedlyClosed => BuildError::Error {
                message: "Channel unexpectedly closed".to_string(),
            },
        }
    }
}

/// ClaimBlockError is an error enum that represents the class of possible
/// errors that can be returned when calling `claim_block` on a
/// `ProxyGlobalState`.  These errors are used for internal representations
/// for consistency and testing, and do not leak beyond the `ProxyGlobalState`
/// API.  As such, they intentionally do not implement traits for serialization.
#[derive(Debug)]
enum ClaimBlockError<Types: NodeType> {
    SignatureValidationFailed,
    SigningCommitmentFailed(
        <<Types as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::SignError,
    ),
    BlockDataNotFound,
}

impl<Types: NodeType> From<ClaimBlockError<Types>> for BuildError {
    fn from(error: ClaimBlockError<Types>) -> Self {
        match error {
            ClaimBlockError::SignatureValidationFailed => BuildError::Error {
                message: "Signature validation failed in claim block".to_string(),
            },
            ClaimBlockError::SigningCommitmentFailed(e) => BuildError::Error {
                message: format!("Signing over builder commitment failed: {:?}", e),
            },
            ClaimBlockError::BlockDataNotFound => BuildError::Error {
                message: "Block data not found".to_string(),
            },
        }
    }
}

#[derive(Debug)]
enum ClaimBlockHeaderInputError<Types: NodeType> {
    SignatureValidationFailed,
    BlockHeaderNotFound,
    CouldNotGetVidInTime,
    WaitAndKeepGetError(WaitAndKeepGetError),
    FailedToSignVidCommitment(
        <<Types as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::SignError,
    ),
    FailedToSignFeeInfo(
        <<Types as NodeType>::BuilderSignatureKey as BuilderSignatureKey>::SignError,
    ),
}

impl<Types: NodeType> From<ClaimBlockHeaderInputError<Types>> for BuildError {
    fn from(error: ClaimBlockHeaderInputError<Types>) -> Self {
        match error {
            ClaimBlockHeaderInputError::SignatureValidationFailed => BuildError::Error {
                message: "Signature validation failed in claim block header input".to_string(),
            },
            ClaimBlockHeaderInputError::BlockHeaderNotFound => BuildError::Error {
                message: "Block header not found".to_string(),
            },
            ClaimBlockHeaderInputError::CouldNotGetVidInTime => BuildError::Error {
                message: "Couldn't get vid in time".to_string(),
            },
            ClaimBlockHeaderInputError::WaitAndKeepGetError(e) => e.into(),
            ClaimBlockHeaderInputError::FailedToSignVidCommitment(e) => BuildError::Error {
                message: format!("Failed to sign VID commitment: {:?}", e),
            },
            ClaimBlockHeaderInputError::FailedToSignFeeInfo(e) => BuildError::Error {
                message: format!("Failed to sign fee info: {:?}", e),
            },
        }
    }
}

impl<Types: NodeType> ProxyGlobalState<Types> {
    async fn available_blocks_implementation(
        &self,
        for_parent: &VidCommitment,
        view_number: u64,
        sender: Types::SignatureKey,
        signature: &<Types::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<Vec<AvailableBlockInfo<Types>>, AvailableBlocksError<Types>> {
        let starting_time = Instant::now();

        let state_id = BuilderStateId {
            parent_commitment: *for_parent,
            view: Types::Time::new(view_number),
        };

        // verify the signature
        if !sender.validate(signature, state_id.parent_commitment.as_ref()) {
            tracing::error!("Signature validation failed in get_available_blocks");
            return Err(AvailableBlocksError::SignatureValidationFailed);
        }

        tracing::info!("Requesting available blocks for {state_id}",);

        let view_num = state_id.view;
        // check in the local spawned builder states
        // if it doesn't exist; there are three cases
        // 1) it has already been garbage collected (view < decide) and we should return an error
        // 2) it has not yet been created, and we should try to wait
        // 3) we missed the triggering event, and should use the BuilderState with the highest available view

        {
            // 1st case: Decide event received, and not bootstrapping.
            // If this `BlockBuilder` hasn't been reaped, it should have been.
            let global_state = self.global_state.read_arc().await;
            if view_num < global_state.last_garbage_collected_view_num
                && global_state.highest_view_num_builder_id.view
                    != global_state.last_garbage_collected_view_num
            {
                tracing::warn!(
                    "Requesting for view {:?}, last decide-triggered cleanup on view {:?}, highest view num is {:?}",
                    view_num,
                    global_state.last_garbage_collected_view_num,
                    global_state.highest_view_num_builder_id.view
                );
                return Err(AvailableBlocksError::RequestForAvailableViewThatHasAlreadyBeenDecided);
            }
        }

        let (response_sender, response_receiver) = unbounded();
        let req_msg = RequestMessage {
            state_id: state_id.clone(),
            response_channel: response_sender,
        };
        let timeout_after = starting_time + self.max_api_waiting_time;
        let check_duration = self.max_api_waiting_time / 10;

        let time_to_wait_for_matching_builder = starting_time + self.max_api_waiting_time / 2;

        let mut sent = false;
        while Instant::now() < time_to_wait_for_matching_builder {
            // try to broadcast the request to the correct builder state
            let found_builder_state = {
                let global_state_read_lock_guard = self.global_state.read_arc().await;

                global_state_read_lock_guard
                    .spawned_builder_states
                    .get(&state_id)
                    .cloned()
            };

            if let Some(builder) = found_builder_state {
                tracing::info!(
                    "Got matching BlockBuilder for {state_id}, sending get_available_blocks request",
                );

                if let Err(e) = builder
                    .broadcast(MessageType::RequestMessage(req_msg.clone()))
                    .await
                {
                    tracing::warn!("Error {e} sending get_available_blocks request for {state_id}",);
                }
                sent = true;
                break;
            }

            tracing::info!("Failed to get matching BlockBuilder for {state_id}, will try again",);
            async_sleep(check_duration).await;
        }

        if !sent {
            // broadcast the request to the best fallback builder state
            if let Err(e) = self
                .global_state
                .read_arc()
                .await
                .get_channel_for_matching_builder_or_highest_view_builder(&state_id)?
                .broadcast(MessageType::RequestMessage(req_msg.clone()))
                .await
            {
                tracing::warn!(
                    "Error {e} sending get_available_blocks request for parent {state_id}",
                );
            }
        }

        tracing::debug!("Waiting for response for get_available_blocks with parent {state_id}",);

        let response_received = loop {
            match async_timeout(check_duration, response_receiver.recv()).await {
                Err(toe) => {
                    if Instant::now() >= timeout_after {
                        tracing::debug!(%toe, "Couldn't get available blocks in time for parent {state_id}");
                        // lookup into the builder_state_to_last_built_block, if it contains the result, return that otherwise return error
                        if let Some(last_built_block) = self
                            .global_state
                            .read_arc()
                            .await
                            .builder_state_to_last_built_block
                            .get(&state_id)
                        {
                            tracing::info!("Returning last built block for parent {state_id}",);
                            break Ok(last_built_block.clone());
                        }
                        break Err(AvailableBlocksError::NoBlocksAvailable);
                    }
                    continue;
                }
                Ok(recv_attempt) => {
                    if let Err(ref e) = recv_attempt {
                        tracing::error!(%e, "Channel closed while getting available blocks for parent {state_id}");
                    }
                    break recv_attempt
                        .map_err(|_| AvailableBlocksError::ChannelUnexpectedlyClosed);
                }
            }
        };

        match response_received {
            Ok(response) => {
                let (pub_key, sign_key) = self.builder_keys.clone();
                // sign over the block info
                let signature_over_block_info =
                    <Types as NodeType>::BuilderSignatureKey::sign_block_info(
                        &sign_key,
                        response.block_size,
                        response.offered_fee,
                        &response.builder_hash,
                    )
                    .map_err(AvailableBlocksError::SigningBlockFailed)?;

                // insert the block info into local hashmap
                let initial_block_info = AvailableBlockInfo::<Types> {
                    block_hash: response.builder_hash.clone(),
                    block_size: response.block_size,
                    offered_fee: response.offered_fee,
                    signature: signature_over_block_info,
                    sender: pub_key.clone(),
                    _phantom: Default::default(),
                };
                tracing::info!(
                    "Sending available Block info response for {state_id} with block hash: {:?}",
                    response.builder_hash
                );
                Ok(vec![initial_block_info])
            }

            // We failed to get available blocks
            Err(e) => {
                tracing::debug!("Failed to get available blocks for parent {state_id}",);
                Err(e)
            }
        }
    }

    async fn claim_block_implementation(
        &self,
        block_hash: &BuilderCommitment,
        view_number: u64,
        sender: Types::SignatureKey,
        signature: &<<Types as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockData<Types>, ClaimBlockError<Types>> {
        let block_id = BlockId {
            hash: block_hash.clone(),
            view: Types::Time::new(view_number),
        };

        tracing::info!("Received request for claiming block {block_id}",);
        // verify the signature
        if !sender.validate(signature, block_id.hash.as_ref()) {
            tracing::error!("Signature validation failed in claim block");
            return Err(ClaimBlockError::SignatureValidationFailed);
        }
        let (pub_key, sign_key) = self.builder_keys.clone();

        let extracted_block_info_option = {
            // We store this write lock guard separately to make it explicit
            // that this will end up holding a lock for the duration of this
            // closure.
            //
            // Additionally, we clone the properties from the block_info that
            // end up being cloned if found anyway.  Since we know this already
            // we can perform the clone here to avoid holding the lock for
            // longer than needed.
            let mut global_state_write_lock_guard = self.global_state.write_arc().await;
            let block_info_some = global_state_write_lock_guard.blocks.get(&block_id);

            block_info_some.map(|block_info| {
                (
                    block_info.vid_trigger.clone(),
                    block_info.block_payload.clone(),
                    block_info.metadata.clone(),
                )
            })
        };

        if let Some((vid_trigger, block_payload, metadata)) = extracted_block_info_option {
            tracing::info!("Trying sending vid trigger info for {block_id}",);

            if let Some(trigger_writer) = vid_trigger.write().await.take() {
                tracing::info!("Sending vid trigger for {block_id}");
                trigger_writer.send(TriggerStatus::Start);
                tracing::info!("Sent vid trigger for {block_id}");
            }
            tracing::info!("Done Trying sending vid trigger info for {block_id}",);

            // sign over the builder commitment, as the proposer can computer it based on provide block_payload
            // and the metadata
            let response_block_hash = block_payload.builder_commitment(&metadata);
            let signature_over_builder_commitment =
                <Types as NodeType>::BuilderSignatureKey::sign_builder_message(
                    &sign_key,
                    response_block_hash.as_ref(),
                )
                .map_err(ClaimBlockError::SigningCommitmentFailed)?;

            let block_data = AvailableBlockData::<Types> {
                block_payload: block_payload.clone(),
                metadata: metadata.clone(),
                signature: signature_over_builder_commitment,
                sender: pub_key.clone(),
            };
            tracing::info!("Sending Claim Block data for {block_id}",);
            Ok(block_data)
        } else {
            tracing::warn!("Claim Block not found");
            Err(ClaimBlockError::BlockDataNotFound)
        }
    }

    async fn claim_block_header_input_implementation(
        &self,
        block_hash: &BuilderCommitment,
        view_number: u64,
        sender: Types::SignatureKey,
        signature: &<<Types as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockHeaderInput<Types>, ClaimBlockHeaderInputError<Types>> {
        let id = BlockId {
            hash: block_hash.clone(),
            view: Types::Time::new(view_number),
        };

        tracing::info!("Received request for claiming block header input for block {id}");
        // verify the signature
        if !sender.validate(signature, id.hash.as_ref()) {
            tracing::error!("Signature validation failed in claim block header input");
            return Err(ClaimBlockHeaderInputError::SignatureValidationFailed);
        }
        let (pub_key, sign_key) = self.builder_keys.clone();

        let extracted_block_info_option = {
            // We store this write lock guard separately to make it explicit
            // that this will end up holding a lock for the duration of this
            // closure.
            //
            // Additionally, we clone the properties from the block_info that
            // end up being cloned if found anyway.  Since we know this already
            // we can perform the clone here to avoid holding the lock for
            // longer than needed.
            let mut global_state_write_lock_guard = self.global_state.write_arc().await;
            let block_info_some = global_state_write_lock_guard.blocks.get(&id);

            block_info_some.map(|block_info| {
                (
                    block_info.vid_receiver.clone(),
                    block_info.metadata.clone(),
                    block_info.offered_fee,
                    block_info.truncated,
                )
            })
        };

        if let Some((vid_receiver, metadata, offered_fee, truncated)) = extracted_block_info_option
        {
            tracing::info!("Waiting for vid commitment for block {id}");

            let timeout_after = Instant::now() + self.max_api_waiting_time;
            let check_duration = self.max_api_waiting_time / 10;

            let response_received = loop {
                match async_timeout(check_duration, vid_receiver.write().await.get()).await {
                    Err(_toe) => {
                        if Instant::now() >= timeout_after {
                            tracing::warn!("Couldn't get vid commitment in time for block {id}",);
                            {
                                // we can't keep up with this block size, reduce max block size
                                let mut global_state_write_lock_guard =
                                    self.global_state.write_arc().await;
                                global_state_write_lock_guard.max_block_size = std::cmp::min(
                                    global_state_write_lock_guard.max_block_size
                                        - global_state_write_lock_guard
                                            .max_block_size
                                            .div_ceil(MAX_BLOCK_SIZE_CHANGE_DIVISOR),
                                    MAX_BLOCK_SIZE_FLOOR,
                                );
                            }
                            break Err(ClaimBlockHeaderInputError::CouldNotGetVidInTime);
                        }
                        continue;
                    }
                    Ok(recv_attempt) => {
                        if recv_attempt.is_err() {
                            tracing::error!(
                                "Channel closed while getting vid commitment for block {id}",
                            );
                        }
                        break recv_attempt
                            .map_err(ClaimBlockHeaderInputError::WaitAndKeepGetError);
                    }
                }
            };

            tracing::info!("Got vid commitment for block {id}",);

            // This block was truncated, but we got VID in time with margin left.
            // Maybe we can handle bigger blocks?
            if truncated
                && timeout_after.duration_since(Instant::now())
                    > self.max_api_waiting_time / VID_RESPONSE_TARGET_MARGIN_DIVISOR
            {
                // Increase max block size
                let mut global_state_write_lock_guard = self.global_state.write_arc().await;
                global_state_write_lock_guard.max_block_size = global_state_write_lock_guard
                    .max_block_size
                    + global_state_write_lock_guard
                        .max_block_size
                        .div_ceil(MAX_BLOCK_SIZE_CHANGE_DIVISOR);
            }

            match response_received {
                Ok((vid_commitment, vid_precompute_data)) => {
                    // sign over the vid commitment
                    let signature_over_vid_commitment =
                        <Types as NodeType>::BuilderSignatureKey::sign_builder_message(
                            &sign_key,
                            vid_commitment.as_ref(),
                        )
                        .map_err(ClaimBlockHeaderInputError::FailedToSignVidCommitment)?;

                    let signature_over_fee_info = Types::BuilderSignatureKey::sign_fee(
                        &sign_key,
                        offered_fee,
                        &metadata,
                        &vid_commitment,
                    )
                    .map_err(ClaimBlockHeaderInputError::FailedToSignFeeInfo)?;

                    let response = AvailableBlockHeaderInput::<Types> {
                        vid_commitment,
                        vid_precompute_data,
                        fee_signature: signature_over_fee_info,
                        message_signature: signature_over_vid_commitment,
                        sender: pub_key.clone(),
                    };
                    tracing::info!("Sending Claim Block Header Input response for {id}",);
                    Ok(response)
                }
                Err(err) => {
                    tracing::warn!("Claim Block Header Input not found");
                    Err(err)
                }
            }
        } else {
            tracing::warn!("Claim Block Header Input not found");
            Err(ClaimBlockHeaderInputError::BlockHeaderNotFound)
        }
    }
}

/*
Handling Builder API responses
*/
#[async_trait]
impl<Types: NodeType> BuilderDataSource<Types> for ProxyGlobalState<Types>
where
    for<'a> <<Types::SignatureKey as SignatureKey>::PureAssembledSignatureType as TryFrom<
        &'a TaggedBase64,
    >>::Error: Display,
    for<'a> <Types::SignatureKey as TryFrom<&'a TaggedBase64>>::Error: Display,
{
    async fn available_blocks(
        &self,
        for_parent: &VidCommitment,
        view_number: u64,
        sender: Types::SignatureKey,
        signature: &<Types::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<Vec<AvailableBlockInfo<Types>>, BuildError> {
        Ok(self
            .available_blocks_implementation(for_parent, view_number, sender, signature)
            .await?)
    }

    async fn claim_block(
        &self,
        block_hash: &BuilderCommitment,
        view_number: u64,
        sender: Types::SignatureKey,
        signature: &<<Types as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockData<Types>, BuildError> {
        Ok(self
            .claim_block_implementation(block_hash, view_number, sender, signature)
            .await?)
    }

    async fn claim_block_header_input(
        &self,
        block_hash: &BuilderCommitment,
        view_number: u64,
        sender: Types::SignatureKey,
        signature: &<<Types as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockHeaderInput<Types>, BuildError> {
        Ok(self
            .claim_block_header_input_implementation(block_hash, view_number, sender, signature)
            .await?)
    }

    /// Returns the public key of the builder
    async fn builder_address(
        &self,
    ) -> Result<<Types as NodeType>::BuilderSignatureKey, BuildError> {
        Ok(self.builder_keys.0.clone())
    }
}

#[async_trait]
impl<Types: NodeType> AcceptsTxnSubmits<Types> for ProxyGlobalState<Types> {
    async fn submit_txns(
        &self,
        txns: Vec<<Types as NodeType>::Transaction>,
    ) -> Result<Vec<Commitment<<Types as NodeType>::Transaction>>, BuildError> {
        tracing::debug!(
            "Submitting {:?} transactions to the builder states{:?}",
            txns.len(),
            txns.iter().map(|txn| txn.commit()).collect::<Vec<_>>()
        );
        let response = self
            .global_state
            .read_arc()
            .await
            .submit_client_txns(txns)
            .await;

        tracing::debug!(
            "Transaction submitted to the builder states, sending response: {:?}",
            response
        );

        // NOTE: ideally we want to respond with original Vec<Result>
        // instead of Result<Vec> not to loose any information,
        //  but this requires changes to builder API
        response.into_iter().collect()
    }
}
#[async_trait]
impl<Types: NodeType> ReadState for ProxyGlobalState<Types> {
    type State = ProxyGlobalState<Types>;

    async fn read<T>(
        &self,
        op: impl Send + for<'a> FnOnce(&'a Self::State) -> BoxFuture<'a, T> + 'async_trait,
    ) -> T {
        op(self).await
    }
}

/// [HotShotEventsService] is a trait that defines the interface for the
/// hotshot events service. The service is expected to provide a stream of
/// events and a startup info.
#[async_trait]
trait HotShotEventsService<Types: NodeType> {
    type EventsStream: Stream<Item = Result<Event<Types>, EventStreamError>>;
    type EventsError;

    type StartUpInfo;
    type StartUpInfoError;

    /// [check_connection] is a function that will check that the service is
    /// connected, or can be connected to the hotshot events service.
    /// It's use is optional, and it has been included strictly for current
    /// logic compatibility.
    async fn check_connection(&self, timeout: Option<Duration>) -> bool;

    /// [events] is a function that will return a stream of events from the
    /// hotshot events service. The stream itself is expected to contain
    /// items that are a [Result] of [Event] or an [EventsError].
    /// If [check_connection] has not been called before this function, the
    /// connection should be established.
    async fn events(&self) -> Result<Self::EventsStream, Self::EventsError>;

    /// [startup_info] is a function that will return the startup info from
    /// the hotshot events service. The response is expected to be a
    /// [Result] of [StartupInfo] or a [StartupInfoError].
    /// If [check_connection] has not been called before this function, the
    /// connection should be established.
    async fn startup_info(&self) -> Result<Self::StartUpInfo, Self::StartUpInfoError>;
}

struct HotShotEventsServiceTideDiscoClient<Types: NodeType, V: Versions> {
    client: surf_disco::Client<hotshot_events_service::events::Error, V::Base>,
    _pd: PhantomData<Types>,
}

impl<Types: NodeType, V: Versions> HotShotEventsServiceTideDiscoClient<Types, V> {
    fn new(client: surf_disco::Client<hotshot_events_service::events::Error, V::Base>) -> Self {
        HotShotEventsServiceTideDiscoClient {
            client,
            _pd: Default::default(),
        }
    }

    fn from_url(url: Url) -> Self {
        let client = surf_disco::Client::<hotshot_events_service::events::Error, V::Base>::new(url);
        Self::new(client)
    }
}

#[async_trait]
impl<Types: NodeType, V: Versions> HotShotEventsService<Types>
    for HotShotEventsServiceTideDiscoClient<Types, V>
{
    type EventsStream = surf_disco::socket::Connection<
        Event<Types>,
        surf_disco::socket::Unsupported,
        EventStreamError,
        V::Base,
    >;
    type EventsError = hotshot_events_service::events::Error;

    type StartUpInfo = StartupInfo<Types>;
    type StartUpInfoError = hotshot_events_service::events::Error;

    async fn check_connection(&self, timeout: Option<Duration>) -> bool {
        self.client.connect(timeout).await
    }

    async fn events(&self) -> Result<Self::EventsStream, Self::EventsError> {
        self.client
            .socket("hotshot-events/events")
            .subscribe::<Event<Types>>()
            .await
    }

    async fn startup_info(&self) -> Result<Self::StartUpInfo, Self::StartUpInfoError> {
        self.client
            .get::<StartupInfo<Types>>("hotshot-events/startup_info")
            .send()
            .await
    }
}

/// [ConnectToEventsServiceError] is an error enum that represents the class
/// of possible errors that can be returned when calling
/// `connect_to_events_service`.
#[derive(Debug)]
enum ConnectToEventsServiceError {
    /// [Connection] is an error variant that represents a failure to
    /// connect to the hotshot events service.
    Connection,

    /// [Subscription] is an error variant that represents a failure to
    /// subscribe to the events stream.
    Subscription(hotshot_events_service::events::Error),

    /// [StartupInfo] is an error variant that represents a failure to
    /// retrieve the startup info from the hotshot events service.
    StartupInfo(hotshot_events_service::events::Error),
}

impl Display for ConnectToEventsServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectToEventsServiceError::Connection => {
                write!(f, "failed to connect to the hotshot events service")
            }
            ConnectToEventsServiceError::Subscription(e) => {
                write!(f, "failed to subscribe to the events stream: {:?}", e)
            }
            ConnectToEventsServiceError::StartupInfo(e) => {
                write!(f, "failed to retrieve the startup info: {:?}", e)
            }
        }
    }
}

/// [connect_to_events_service] is a function that will attempt to connect to
/// the hotshot events service, setup a subscription to the events stream,
/// and retrieve the startup info.  If all is successful, it will return the
/// stream, and membership information derived from the startup info.
async fn connect_to_events_service<Types: NodeType, C, S>(
    client: &C,
) -> Result<
    (
        C::EventsStream,
        GeneralStaticCommittee<Types, <Types as NodeType>::SignatureKey>,
    ),
    ConnectToEventsServiceError,
>
where
    C: HotShotEventsService<
        Types,
        EventsStream = S,
        EventsError = hotshot_events_service::events::Error,
        StartUpInfo = StartupInfo<Types>,
        StartUpInfoError = hotshot_events_service::events::Error,
    >,
    S: Stream<Item = Result<Event<Types>, EventStreamError>>,
{
    if !(client.check_connection(None).await) {
        return Err(ConnectToEventsServiceError::Connection);
    }

    tracing::info!("Builder client connected to the hotshot events api");

    // client subscribe to hotshot events
    let subscribed_events = client
        .events()
        .await
        .map_err(ConnectToEventsServiceError::Subscription)?;

    // handle the startup event at the start
    let StartupInfo {
        known_node_with_stake,
        non_staked_node_count,
    } = client
        .startup_info()
        .await
        .map_err(ConnectToEventsServiceError::StartupInfo)?;

    let membership: GeneralStaticCommittee<Types, <Types as NodeType>::SignatureKey> =
        GeneralStaticCommittee::<Types, <Types as NodeType>::SignatureKey>::create_election(
            known_node_with_stake.clone(),
            known_node_with_stake.clone(),
            Topic::Global,
            0,
        );

    tracing::info!(
        "Startup info: Known nodes with stake: {:?}, Non-staked node count: {:?}",
        known_node_with_stake,
        non_staked_node_count
    );

    Ok((subscribed_events, membership))
}

/*
Running Non-Permissioned Builder Service
*/
pub async fn run_non_permissioned_standalone_builder_service<Types: NodeType, V: Versions>(
    // sending a DA proposal from the hotshot to the builder states
    da_sender: BroadcastSender<MessageType<Types>>,

    // sending a QC proposal from the hotshot to the builder states
    qc_sender: BroadcastSender<MessageType<Types>>,

    // sending a Decide event from the hotshot to the builder states
    decide_sender: BroadcastSender<MessageType<Types>>,

    // Url to (re)connect to for the events stream
    hotshot_events_api_url: Url,

    // Global state
    global_state: Arc<RwLock<GlobalState<Types>>>,
) -> Result<(), anyhow::Error> {
    // connection to the events stream
    let events_service_client =
        HotShotEventsServiceTideDiscoClient::<Types, V>::from_url(hotshot_events_api_url.clone());

    let connect_to_events_service_result = connect_to_events_service(&events_service_client).await;
    let (mut subscribed_events, mut membership) = match connect_to_events_service_result {
        Ok((subscribed_events, membership)) => (subscribed_events, membership),
        Err(err) => {
            return Err(anyhow!(
                "failed to connect to API at {hotshot_events_api_url}: {err}"
            ));
        }
    };

    let tx_sender = {
        // This closure is likely unnecessary, but we want to play it safe
        // with our RWLocks.
        let global_state_read_lock_guard = global_state.read_arc().await;
        global_state_read_lock_guard.tx_sender.clone()
    };

    loop {
        let event = subscribed_events.next().await;
        //tracing::debug!("Builder Event received from HotShot: {:?}", event);
        match event {
            Some(Ok(event)) => {
                match event.event {
                    EventType::Error { error } => {
                        tracing::error!("Error event in HotShot: {:?}", error);
                    }
                    // tx event
                    EventType::Transactions { transactions } => {
                        let max_block_size = {
                            // This closure is likely unnecessary, but we want
                            // to play it safe with our RWLocks.
                            let global_state_read_lock_guard = global_state.read_arc().await;
                            global_state_read_lock_guard.max_block_size
                        };

                        handle_received_txns(
                            &tx_sender,
                            transactions,
                            TransactionSource::HotShot,
                            max_block_size,
                        )
                        .await;
                    }
                    // decide event
                    EventType::Decide {
                        block_size: _,
                        leaf_chain,
                        qc: _,
                    } => {
                        let latest_decide_view_num = leaf_chain[0].leaf.view_number();
                        handle_decide_event(&decide_sender, latest_decide_view_num).await;
                    }
                    // DA proposal event
                    EventType::DaProposal { proposal, sender } => {
                        // get the leader for current view
                        let leader = membership.leader(proposal.data.view_number);
                        // get the committee mstatked node count
                        let total_nodes = membership.total_nodes();

                        handle_da_event(
                            &da_sender,
                            Arc::new(proposal),
                            sender,
                            leader,
                            NonZeroUsize::new(total_nodes).unwrap_or(NonZeroUsize::MIN),
                        )
                        .await;
                    }
                    // QC proposal event
                    EventType::QuorumProposal { proposal, sender } => {
                        // get the leader for current view
                        let leader = membership.leader(proposal.data.view_number);
                        handle_qc_event(&qc_sender, Arc::new(proposal), sender, leader).await;
                    }
                    _ => {
                        tracing::debug!("Unhandled event from Builder");
                    }
                }
            }
            Some(Err(e)) => {
                tracing::error!("Error in the event stream: {:?}", e);
            }
            None => {
                tracing::error!("Event stream ended");
                let connected = connect_to_events_service(&events_service_client).await;

                (subscribed_events, membership) = match connected {
                    Ok((subscribed_events, membership)) => (subscribed_events, membership),
                    Err(err) => {
                        return Err(anyhow!(
                            "failed to reconnect to API at {hotshot_events_api_url}: {err}"
                        ));
                    }
                };
            }
        }
    }
}

/*
Running Permissioned Builder Service
*/
pub async fn run_permissioned_standalone_builder_service<
    Types: NodeType,
    I: NodeImplementation<Types>,
    V: Versions,
>(
    // sending a DA proposal from the hotshot to the builder states
    da_sender: BroadcastSender<MessageType<Types>>,

    // sending a QC proposal from the hotshot to the builder states
    qc_sender: BroadcastSender<MessageType<Types>>,

    // sending a Decide event from the hotshot to the builder states
    decide_sender: BroadcastSender<MessageType<Types>>,

    // hotshot context handle
    hotshot_handle: Arc<SystemContextHandle<Types, I, V>>,

    // Global state
    global_state: Arc<RwLock<GlobalState<Types>>>,
) {
    let mut event_stream = hotshot_handle.event_stream();
    let tx_sender = {
        // This closure is likely unnecessary, but we want to play it safe
        // with our RWLocks.
        let global_state_read_lock_guard = global_state.read_arc().await;
        global_state_read_lock_guard.tx_sender.clone()
    };

    loop {
        tracing::debug!("Waiting for events from HotShot");
        match event_stream.next().await {
            None => {
                tracing::error!("Didn't receive any event from the HotShot event stream");
            }
            Some(event) => {
                match event.event {
                    // error event
                    EventType::Error { error } => {
                        tracing::error!("Error event in HotShot: {:?}", error);
                    }
                    // tx event
                    EventType::Transactions { transactions } => {
                        let max_block_size = {
                            // This closure is likely unnecessary, but we want
                            // to play it safe with our RWLocks.
                            let global_state_read_lock_guard = global_state.read_arc().await;
                            global_state_read_lock_guard.max_block_size
                        };

                        handle_received_txns(
                            &tx_sender,
                            transactions,
                            TransactionSource::HotShot,
                            max_block_size,
                        )
                        .await;
                    }
                    // decide event
                    EventType::Decide { leaf_chain, .. } => {
                        let latest_decide_view_number = leaf_chain[0].leaf.view_number();

                        handle_decide_event(&decide_sender, latest_decide_view_number).await;
                    }
                    // DA proposal event
                    EventType::DaProposal { proposal, sender } => {
                        // get the leader for current view
                        let leader = hotshot_handle.leader(proposal.data.view_number).await;
                        // get the committee staked node count
                        let total_nodes = hotshot_handle.total_nodes();

                        handle_da_event(
                            &da_sender,
                            Arc::new(proposal),
                            sender,
                            leader,
                            total_nodes,
                        )
                        .await;
                    }
                    // QC proposal event
                    EventType::QuorumProposal { proposal, sender } => {
                        // get the leader for current view
                        let leader = hotshot_handle.leader(proposal.data.view_number).await;
                        handle_qc_event(&qc_sender, Arc::new(proposal), sender, leader).await;
                    }
                    _ => {
                        tracing::error!("Unhandled event from Builder: {:?}", event.event);
                    }
                }
            }
        }
    }
}

/// [HandleDaEventError] represents the internal class of errors that can
/// occur when attempting to process an incoming da proposal event.  More
/// specifically these are the class of error that can be returned from
/// [handle_da_event_implementation].
#[derive(Debug)]
enum HandleDaEventError<Types: NodeType> {
    SenderIsNotLeader,
    SignatureValidationFailed,
    BroadcastFailed(async_broadcast::SendError<MessageType<Types>>),
}

/// [handle_da_event] is a utility function that will attempt to broadcast the
/// given `da_proposal` to the given `da_channel_sender` if the given details
/// pass validation checks, and the [BroadcastSender] `da_channel_sender` is
/// still open.
async fn handle_da_event<Types: NodeType>(
    da_channel_sender: &BroadcastSender<MessageType<Types>>,
    da_proposal: Arc<Proposal<Types, DaProposal<Types>>>,
    sender: <Types as NodeType>::SignatureKey,
    leader: <Types as NodeType>::SignatureKey,
    total_nodes: NonZeroUsize,
) {
    // We're explicitly not inspecting this error, as this function is not
    // expected to return an error or any indication of an error.
    let _ =
        handle_da_event_implementation(da_channel_sender, da_proposal, sender, leader, total_nodes)
            .await;
}

/// [handle_da_event_implementation] is a utility function that will attempt
/// to broadcast the given da_proposal to the given [da_channel_sender] if the
/// given details pass all relevant checks.
///
/// There are only three conditions under which this will fail to send the
/// message via the given `da_channel_sender`, and they are all represented
/// via [HandleDaEventError]. They are as follows:
/// - [HandleDaEventError::SenderIsNotLeader]: The sender is not the leader
/// - [HandleDaEventError::SignatureValidationFailed]: The signature validation
///    failed
/// - [HandleDaEventError::BroadcastFailed]: The broadcast failed as no receiver
///    is in place to receive the message
///
/// This function is the implementation for [handle_da_event].
async fn handle_da_event_implementation<Types: NodeType>(
    da_channel_sender: &BroadcastSender<MessageType<Types>>,
    da_proposal: Arc<Proposal<Types, DaProposal<Types>>>,
    sender: <Types as NodeType>::SignatureKey,
    leader: <Types as NodeType>::SignatureKey,
    total_nodes: NonZeroUsize,
) -> Result<(), HandleDaEventError<Types>> {
    tracing::debug!(
        "DaProposal: Leader: {:?} for the view: {:?}",
        leader,
        da_proposal.data.view_number
    );

    // get the encoded transactions hash
    let encoded_txns_hash = Sha256::digest(&da_proposal.data.encoded_transactions);
    // check if the sender is the leader and the signature is valid; if yes, broadcast the DA proposal

    // We check the failure condition first to prevent unnecessary conditional
    // blocks
    if leader != sender {
        tracing::error!("Sender is not Leader on DaProposal for view {:?}: Leader for the current view: {:?} and sender: {:?}", da_proposal.data.view_number, leader, sender);
        return Err(HandleDaEventError::SenderIsNotLeader);
    }

    if !sender.validate(&da_proposal.signature, &encoded_txns_hash) {
        tracing::error!("Validation Failure on DaProposal for view {:?}: Leader for the current view: {:?} and sender: {:?}", da_proposal.data.view_number, leader, sender);
        return Err(HandleDaEventError::SignatureValidationFailed);
    }

    let da_msg = DaProposalMessage::<Types> {
        proposal: da_proposal,
        sender: leader,
        total_nodes: total_nodes.into(),
    };

    let view_number = da_msg.proposal.data.view_number;
    tracing::debug!(
        "Sending DA proposal to the builder states for view {:?}",
        view_number
    );

    if let Err(e) = da_channel_sender
        .broadcast(MessageType::DaProposalMessage(da_msg))
        .await
    {
        tracing::warn!(
            "Error {e}, failed to send DA proposal to builder states for view {:?}",
            view_number
        );

        return Err(HandleDaEventError::BroadcastFailed(e));
    }

    Ok(())
}

/// [HandleQcEventError] represents the internal class of errors that can
/// occur when attempting to process an incoming qc proposal event.  More
/// specifically these are the class of error that can be returned from
/// [handle_qc_event_implementation].
#[derive(Debug)]
enum HandleQcEventError<Types: NodeType> {
    SenderIsNotLeader,
    SignatureValidationFailed,
    BroadcastFailed(async_broadcast::SendError<MessageType<Types>>),
}

/// [handle_qc_event] is a utility function that will attempt to broadcast the
/// given `qc_proposal` to the given `qc_channel_sender` if the given details
/// pass validation checks, and the [BroadcastSender] `qc_channel_sender` is
/// still open.
async fn handle_qc_event<Types: NodeType>(
    qc_channel_sender: &BroadcastSender<MessageType<Types>>,
    qc_proposal: Arc<Proposal<Types, QuorumProposal<Types>>>,
    sender: <Types as NodeType>::SignatureKey,
    leader: <Types as NodeType>::SignatureKey,
) {
    // We're explicitly not inspecting this error, as this function is not
    // expected to return an error or any indication of an error.
    let _ = handle_qc_event_implementation(qc_channel_sender, qc_proposal, sender, leader).await;
}

/// [handle_qc_event_implementation] is a utility function that will attempt
/// to broadcast the given qc_proposal to the given [qc_channel_sender] if the
/// given details pass all relevant checks.
///
/// There are only three conditions under which this will fail to send the
/// message via the given `qc_channel_sender`, and they are all represented
/// via [HandleQcEventError]. They are as follows:
/// - [HandleQcEventError::SenderIsNotLeader]: The sender is not the leader
/// - [HandleQcEventError::SignatureValidationFailed]: The signature validation
///   failed
/// - [HandleQcEventError::BroadcastFailed]: The broadcast failed as no receiver
///   is in place to receive the message
///
/// This function is the implementation for [handle_qc_event].
async fn handle_qc_event_implementation<Types: NodeType>(
    qc_channel_sender: &BroadcastSender<MessageType<Types>>,
    qc_proposal: Arc<Proposal<Types, QuorumProposal<Types>>>,
    sender: <Types as NodeType>::SignatureKey,
    leader: <Types as NodeType>::SignatureKey,
) -> Result<(), HandleQcEventError<Types>> {
    tracing::debug!(
        "QCProposal: Leader: {:?} for the view: {:?}",
        leader,
        qc_proposal.data.view_number
    );

    let leaf = Leaf::from_quorum_proposal(&qc_proposal.data);

    // check if the sender is the leader and the signature is valid; if yes, broadcast the QC proposal
    if sender != leader {
        tracing::error!("Sender is not Leader on QCProposal for view {:?}: Leader for the current view: {:?} and sender: {:?}", qc_proposal.data.view_number, leader, sender);
        return Err(HandleQcEventError::SenderIsNotLeader);
    }

    if !sender.validate(&qc_proposal.signature, leaf.commit().as_ref()) {
        tracing::error!("Validation Failure on QCProposal for view {:?}: Leader for the current view: {:?} and sender: {:?}", qc_proposal.data.view_number, leader, sender);
        return Err(HandleQcEventError::SignatureValidationFailed);
    }

    let qc_msg = QCMessage::<Types> {
        proposal: qc_proposal,
        sender: leader,
    };
    let view_number = qc_msg.proposal.data.view_number;
    tracing::debug!(
        "Sending QC proposal to the builder states for view {:?}",
        view_number
    );

    if let Err(e) = qc_channel_sender
        .broadcast(MessageType::QCMessage(qc_msg))
        .await
    {
        tracing::warn!(
            "Error {e}, failed to send QC proposal to builder states for view {:?}",
            view_number
        );
        return Err(HandleQcEventError::BroadcastFailed(e));
    }

    Ok(())
}

async fn handle_decide_event<Types: NodeType>(
    decide_channel_sender: &BroadcastSender<MessageType<Types>>,
    latest_decide_view_number: Types::Time,
) {
    let decide_msg: DecideMessage<Types> = DecideMessage::<Types> {
        latest_decide_view_number,
    };
    tracing::debug!(
        "Sending Decide event to builder states for view {:?}",
        latest_decide_view_number
    );
    if let Err(e) = decide_channel_sender
        .broadcast(MessageType::DecideMessage(decide_msg))
        .await
    {
        tracing::warn!(
            "Error {e}, failed to send Decide event to builder states for view {:?}",
            latest_decide_view_number
        );
    }
}

#[derive(Debug)]
enum HandleReceivedTxnsError<Types: NodeType> {
    TransactionTooBig {
        estimated_length: u64,
        max_txn_len: u64,
    },

    TooManyTransactions,

    Internal(TrySendError<Arc<ReceivedTransaction<Types>>>),
}

impl<Types: NodeType> From<HandleReceivedTxnsError<Types>> for BuildError {
    fn from(error: HandleReceivedTxnsError<Types>) -> Self {
        match error {
            HandleReceivedTxnsError::TransactionTooBig {
                estimated_length,
                max_txn_len,
            } => BuildError::Error {
                message: format!("Transaction too big (estimated length {estimated_length}, currently accepting <= {max_txn_len})"),
            },
            HandleReceivedTxnsError::TooManyTransactions => BuildError::Error {
                message: "Too many transactions".to_owned(),
            },
            HandleReceivedTxnsError::Internal(err) => BuildError::Error {
                message: format!("Internal error when submitting transaction: {}", err),
            },
        }
    }
}

impl<Types: NodeType> From<TrySendError<Arc<ReceivedTransaction<Types>>>>
    for HandleReceivedTxnsError<Types>
{
    fn from(err: TrySendError<Arc<ReceivedTransaction<Types>>>) -> Self {
        match err {
            TrySendError::Full(_) => HandleReceivedTxnsError::TooManyTransactions,
            err => HandleReceivedTxnsError::Internal(err),
        }
    }
}

/// [handle_received_txns] is a utility function that will take the given list
/// of transactions, [txns], wraps them in a [ReceivedTransaction] struct
/// and attempt to broadcast them to the given transaction [BroadcastSender]
/// [tx_sender]. The broadcast itself it a non-blocking operation, and any
/// failures of the broadcast are collected into the returned vector
/// of [Result]s.
///
/// There is also a [max_txn_len] parameter that is used to check to ensure
/// that transactions that exceed this threshold will also not be broadcasted.
pub(crate) async fn handle_received_txns<Types: NodeType>(
    tx_sender: &BroadcastSender<Arc<ReceivedTransaction<Types>>>,
    txns: Vec<Types::Transaction>,
    source: TransactionSource,
    max_txn_len: u64,
) -> Vec<Result<Commitment<<Types as NodeType>::Transaction>, BuildError>> {
    HandleReceivedTxns::new(tx_sender.clone(), txns, source, max_txn_len)
        .map(|res| res.map_err(Into::into))
        .collect()
}

/// HandleReceivedTxns is a struct that is used to handle the processing of
/// the function [handle_received_txns].  In order to avoid the need to
/// double allocate a [Vec] from processing these entries, this struct exists
/// to be processed as an [Iterator] instead.
struct HandleReceivedTxns<Types: NodeType> {
    tx_sender: BroadcastSender<Arc<ReceivedTransaction<Types>>>,
    txns: Vec<Types::Transaction>,
    source: TransactionSource,
    max_txn_len: u64,
    offset: usize,
    txns_length: usize,
    time_in: Instant,
}

impl<Types: NodeType> HandleReceivedTxns<Types> {
    fn new(
        tx_sender: BroadcastSender<Arc<ReceivedTransaction<Types>>>,
        txns: Vec<Types::Transaction>,
        source: TransactionSource,
        max_txn_len: u64,
    ) -> Self {
        let txns_length = txns.len();

        Self {
            tx_sender,
            txns,
            source,
            max_txn_len,
            offset: 0,
            txns_length,
            time_in: Instant::now(),
        }
    }
}

impl<Types: NodeType> Iterator for HandleReceivedTxns<Types> {
    type Item =
        Result<Commitment<<Types as NodeType>::Transaction>, HandleReceivedTxnsError<Types>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.txns.is_empty() {
            return None;
        }

        if self.offset >= self.txns_length {
            return None;
        }

        let offset = self.offset;
        // increment the offset so we can ensure we're making progress;
        self.offset += 1;

        let tx = self.txns[offset].clone();
        let commit = tx.commit();
        // This is a rough estimate, but we don't have any other way to get real
        // encoded transaction length. Luckily, this being roughly proportional
        // to encoded length is enough, because we only use this value to estimate
        // our limitations on computing the VID in time.
        let len = bincode::serialized_size(&tx).unwrap_or_default();
        if len > self.max_txn_len {
            tracing::warn!(%commit, %len, %max_txn_len, "Transaction too big");
            return Some(Err(HandleReceivedTxnsError::TransactionTooBig {
                estimated_length: len,
                max_txn_len: self.max_txn_len,
            }));
        }

        let res = self
            .tx_sender
            .try_broadcast(Arc::new(ReceivedTransaction {
                tx,
                source: self.source.clone(),
                commit,
                time_in: self.time_in,
                len,
            }))
            .inspect(|val| {
                if let Some(evicted_txn) = val {
                    tracing::warn!(
                        "Overflow mode enabled, transaction {} evicted",
                        evicted_txn.commit
                    );
                }
            })
            .map(|_| commit)
            .inspect_err(|err| {
                tracing::warn!("Failed to broadcast txn with commit {:?}: {}", commit, err);
            })
            .map_err(HandleReceivedTxnsError::from);

        Some(res)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.txns_length - self.offset,
            Some(self.txns.capacity() - self.offset),
        )
    }
}

#[cfg(test)]
mod test {
    use std::{num::NonZeroUsize, sync::Arc, time::Duration};

    use async_compatibility_layer::channel::unbounded;
    use async_lock::RwLock;
    use committable::Committable;
    use futures::{
        channel::mpsc::{channel, Receiver},
        StreamExt,
    };
    use hotshot::{
        traits::BlockPayload,
        types::{BLSPubKey, Event, SignatureKey},
    };
    use hotshot_builder_api::v0_2::block_info::AvailableBlockInfo;
    use hotshot_events_service::events_source::StartupInfo;
    use hotshot_example_types::{
        block_types::{TestBlockPayload, TestMetadata, TestTransaction},
        node_types::TestTypes,
        state_types::{TestInstanceState, TestValidatedState},
    };
    use hotshot_types::{
        data::{DaProposal, Leaf, QuorumProposal, ViewNumber},
        message::Proposal,
        simple_certificate::QuorumCertificate,
        traits::{
            block_contents::{precompute_vid_commitment, vid_commitment},
            node_implementation::ConsensusTime,
            signature_key::BuilderSignatureKey,
        },
        utils::BuilderCommitment,
    };
    use sha2::{Digest, Sha256};
    use tide_disco::StatusCode;

    use crate::{
        builder_state::{
            BuildBlockInfo, MessageType, RequestMessage, ResponseMessage, TransactionSource,
            TriggerStatus,
        },
        service::{
            connect_to_events_service, ConnectToEventsServiceError, HandleReceivedTxnsError,
            INITIAL_MAX_BLOCK_SIZE,
        },
        BlockId, BuilderStateId,
    };

    use super::{
        handle_da_event_implementation, handle_qc_event_implementation, AvailableBlocksError,
        BlockInfo, ClaimBlockError, ClaimBlockHeaderInputError, GlobalState, HandleDaEventError,
        HandleQcEventError, HandleReceivedTxns, HotShotEventsService, ProxyGlobalState,
    };

    // GlobalState Tests

    // GlobalState::new Tests

    /// This test checks a [GlobalState] created from [GlobalState::new] has
    /// the appropriate values stored within it.
    #[async_std::test]
    async fn test_global_state_new() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let parent_commit = vid_commitment(&[], 8);
        let state = GlobalState::<TestTypes>::new(
            bootstrap_sender,
            tx_sender,
            parent_commit,
            ViewNumber::new(1),
            ViewNumber::new(2),
            10,
        );

        assert_eq!(state.blocks.len(), 0, "The blocks LRU should be empty");

        let builder_state_id = BuilderStateId {
            parent_commitment: parent_commit,
            view: ViewNumber::new(1),
        };

        // There should be a single entry within the spawned_builder_states,
        // and it should be the one that was just created.
        assert_eq!(
            state.spawned_builder_states.len(),
            1,
            "There should be a single entry in the spawned builder states hashmap"
        );

        assert!(state.spawned_builder_states.contains_key(&builder_state_id), "The spawned builder states should contain an entry with the bootstrapped parameters passed into new");

        assert!(!state.spawned_builder_states.contains_key(&BuilderStateId { parent_commitment: parent_commit, view: ViewNumber::new(0) }), "The spawned builder states should not contain any other entry, as such it should not contain any entry with a higher view number, but the same parent commit");

        // We can't compare the Senders directly

        assert_eq!(
            state.last_garbage_collected_view_num,
            ViewNumber::new(2),
            "The last garbage collected view number should be the one passed into new"
        );

        assert_eq!(
            state.builder_state_to_last_built_block.len(),
            0,
            "The builder state to last built block should be empty"
        );

        assert_eq!(
            state.highest_view_num_builder_id, builder_state_id,
            "The highest view number builder id should be the bootstrapped build state id"
        );

        assert_eq!(
            state.max_block_size, INITIAL_MAX_BLOCK_SIZE,
            "The max block size should be the expected default value"
        );
    }

    // GlobalState::register_builder_state Tests

    /// This test checks that the [GlobalState::register_builder_state] function
    /// will correctly register a new builder state, and that the highest view
    /// number builder id will be updated to the new builder state id.
    /// Additionally, it will check that the spawned builder states hashmap
    /// will contain the new builder state id.
    #[async_std::test]
    async fn test_global_state_register_builder_state_different_states() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let parent_commit = vid_commitment(&[], 8);
        let mut state = GlobalState::<TestTypes>::new(
            bootstrap_sender,
            tx_sender,
            parent_commit,
            ViewNumber::new(0),
            ViewNumber::new(0),
            10,
        );

        {
            let (req_sender, _) = async_broadcast::broadcast(10);
            let builder_state_id = BuilderStateId {
                parent_commitment: parent_commit,
                view: ViewNumber::new(5),
            };

            state.register_builder_state(builder_state_id.clone(), req_sender.clone());

            assert_eq!(
                state.spawned_builder_states.len(),
                2,
                "The spawned_builder_states should now have 2 elements in it"
            );
            assert_eq!(
                state.highest_view_num_builder_id, builder_state_id,
                "The highest view number builder id should now be the one that was just registered"
            );
            assert!(
                state.spawned_builder_states.contains_key(&builder_state_id),
                "The spawned builder states should contain the new builder state id"
            );
        };

        {
            let (req_sender, _) = async_broadcast::broadcast(10);
            let builder_state_id = BuilderStateId {
                parent_commitment: parent_commit,
                view: ViewNumber::new(6),
            };

            state.register_builder_state(builder_state_id.clone(), req_sender.clone());

            assert_eq!(
                state.spawned_builder_states.len(),
                3,
                "The spawned_builder_states should now have 3 elements in it"
            );
            assert_eq!(
                state.highest_view_num_builder_id, builder_state_id,
                "The highest view number builder id should now be the one that was just registered"
            );
            assert!(
                state.spawned_builder_states.contains_key(&builder_state_id),
                "The spawned builder states should contain the new builder state id"
            );
        };
    }

    /// This test checks that the register_builder_state method will overwrite
    /// the previous sender in the `spawned_builder_states` hashmap if the same
    /// `BuilderStateId` is used to register a new sender.
    ///
    /// It also demonstrates that doing this will drop the previous sender,
    /// effectively closing it if it is the only reference to it.
    #[async_std::test]
    async fn test_global_state_register_builder_state_same_builder_state_id() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let parent_commit = vid_commitment(&[], 8);
        let mut state = GlobalState::<TestTypes>::new(
            bootstrap_sender,
            tx_sender,
            parent_commit,
            ViewNumber::new(0),
            ViewNumber::new(0),
            10,
        );

        let mut req_receiver_1 = {
            let (req_sender, req_receiver) = async_broadcast::broadcast(10);
            let builder_state_id = BuilderStateId {
                parent_commitment: parent_commit,
                view: ViewNumber::new(5),
            };

            state.register_builder_state(builder_state_id.clone(), req_sender.clone());

            assert_eq!(
                state.spawned_builder_states.len(),
                2,
                "The spawned_builder_states should now have 2 elements in it"
            );
            assert_eq!(
                state.highest_view_num_builder_id, builder_state_id,
                "The highest view number builder id should now be the one that was just registered"
            );

            req_receiver
        };

        let mut req_receiver_2 = {
            let (req_sender, req_receiver) = async_broadcast::broadcast(10);
            let builder_state_id = BuilderStateId {
                parent_commitment: parent_commit,
                view: ViewNumber::new(5),
            };

            // This is the same BuilderStateId as the previous one, so it should
            // replace the previous one.  Which means that the previous one
            // may no longer be published to.
            state.register_builder_state(builder_state_id.clone(), req_sender.clone());

            assert_eq!(
                state.spawned_builder_states.len(),
                2,
                "The spawned_builder_states should still have 2 elements in it"
            );
            assert_eq!(state.highest_view_num_builder_id, builder_state_id, "The highest view number builder id should still be the one that was just registered");

            req_receiver
        };

        {
            let builder_state_id = BuilderStateId {
                parent_commitment: parent_commit,
                view: ViewNumber::new(5),
            };

            let req_sender = state.spawned_builder_states.get(&builder_state_id).unwrap();
            let (response_sender, _) = unbounded();

            assert!(
                req_sender
                    .broadcast(MessageType::RequestMessage(RequestMessage {
                        state_id: builder_state_id,
                        response_channel: response_sender,
                    }))
                    .await
                    .is_ok(),
                "This should be able to send a Message through the sender"
            );
        }

        // The first receiver should have been replaced, so we won't get any
        // results from it.

        assert!(
            req_receiver_1.recv().await.is_err(),
            "This first receiver should be closed"
        );
        assert!(
            req_receiver_2.recv().await.is_ok(),
            "The second receiver should receive a message"
        );
    }

    /// This test checks that the register_builder_state method will only
    /// update the highest_view_num_builder_id if the new [BuilderStateId] has
    /// a higher view number than the current highest_view_num_builder_id.
    #[async_std::test]
    async fn test_global_state_register_builder_state_decrementing_builder_state_ids() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let parent_commit = vid_commitment(&[], 8);
        let mut state = GlobalState::<TestTypes>::new(
            bootstrap_sender,
            tx_sender,
            parent_commit,
            ViewNumber::new(0),
            ViewNumber::new(0),
            10,
        );

        {
            let (req_sender, _) = async_broadcast::broadcast(10);
            let builder_state_id = BuilderStateId {
                parent_commitment: parent_commit,
                view: ViewNumber::new(6),
            };

            state.register_builder_state(builder_state_id.clone(), req_sender.clone());

            assert_eq!(
                state.spawned_builder_states.len(),
                2,
                "The spawned_builder_states should now have 2 elements in it"
            );
            assert_eq!(
                state.highest_view_num_builder_id, builder_state_id,
                "The highest view number builder id should now be the one that was just registered"
            );
            assert!(
                state.spawned_builder_states.contains_key(&builder_state_id),
                "The spawned builder states should contain the new builder state id"
            );
        };

        {
            let (req_sender, _) = async_broadcast::broadcast(10);
            let builder_state_id = BuilderStateId {
                parent_commitment: parent_commit,
                view: ViewNumber::new(5),
            };

            state.register_builder_state(builder_state_id.clone(), req_sender.clone());

            assert_eq!(
                state.spawned_builder_states.len(),
                3,
                "The spawned_builder_states should now have 3 elements in it"
            );
            assert_eq!(
                state.highest_view_num_builder_id,
                BuilderStateId {
                    parent_commitment: parent_commit,
                    view: ViewNumber::new(6)
                },
                "The highest view number builder id should now be the one that was just registered"
            );
            assert!(
                state.spawned_builder_states.contains_key(&builder_state_id),
                "The spawned builder states should contain the new builder state id"
            );
        };
    }

    // GlobalState::update_global_state Tests

    /// This test checks that the update_global_state method will correctly
    /// update the LRU blocks cache and the builder_state_to_last_built_block
    /// hashmap with values derived from the parameters passed into the method.
    ///
    /// The assumption behind this test is that the values being stored were
    /// not being stored previously.
    #[async_std::test]
    async fn test_global_state_update_global_state_success() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let parent_commit = vid_commitment(&[], 8);
        let mut state = GlobalState::<TestTypes>::new(
            bootstrap_sender,
            tx_sender,
            parent_commit,
            ViewNumber::new(0),
            ViewNumber::new(0),
            10,
        );

        let new_parent_commit = vid_commitment(&[], 9);
        let new_view_num = ViewNumber::new(1);
        let builder_state_id = BuilderStateId {
            parent_commitment: new_parent_commit,
            view: new_view_num,
        };

        let builder_hash_1 = BuilderCommitment::from_bytes([1, 2, 3, 4]);
        let block_id = BlockId {
            hash: builder_hash_1,
            view: new_view_num,
        };

        let (vid_trigger_sender, vid_trigger_receiver) =
            async_compatibility_layer::channel::oneshot();
        let (vid_sender, vid_receiver) = unbounded();
        let (block_payload, metadata) =
            <TestBlockPayload as BlockPayload<TestTypes>>::from_transactions(
                vec![TestTransaction::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])],
                &TestValidatedState::default(),
                &TestInstanceState::default(),
            )
            .await
            .unwrap();
        let offered_fee = 64u64;
        let block_size = 64u64;
        let truncated = false;

        let build_block_info = BuildBlockInfo {
            id: block_id.clone(),
            block_size,
            offered_fee,
            block_payload: block_payload.clone(),
            metadata,
            vid_trigger: vid_trigger_sender,
            vid_receiver,
            truncated,
        };

        let builder_hash_2 = BuilderCommitment::from_bytes([2, 3, 4, 5]);
        let response_msg = ResponseMessage {
            builder_hash: builder_hash_2.clone(),
            block_size: 32,
            offered_fee: 128,
        };

        // Now that every object is prepared and setup for storage, we can
        // test the `update_global_state` method.

        // `update_global_state` has not return value from its method, so can
        // only inspect its "success" based on the mutation of the state object.
        state.update_global_state(builder_state_id.clone(), build_block_info, response_msg);

        // two things should be adjusted by `update_global_state`:
        // - state.blocks
        // - state.builder_state_to_last_built_block

        // start with blocks

        assert_eq!(
            state.blocks.len(),
            1,
            "The blocks LRU should have a single entry"
        );

        let retrieved_block_info = state.blocks.get(&block_id);
        assert!(
            retrieved_block_info.is_some(),
            "Retrieval of the block id should result is a valid block info data"
        );

        let retrieved_block_info = retrieved_block_info.unwrap();

        assert_eq!(
            retrieved_block_info.block_payload, block_payload,
            "The block payloads should match"
        );
        assert_eq!(
            retrieved_block_info.metadata, metadata,
            "The metadata should match"
        );
        assert_eq!(
            retrieved_block_info.offered_fee, offered_fee,
            "The offered fee should match"
        );
        assert_eq!(
            retrieved_block_info.truncated, truncated,
            "The truncated flag should match"
        );

        {
            // This ensures that the vid_trigger that is stored is still the
            // same, or links to the vid_trigger_receiver that we submitted.
            let mut vid_trigger_write_lock_guard =
                retrieved_block_info.vid_trigger.write_arc().await;
            if let Some(vid_trigger_sender) = vid_trigger_write_lock_guard.take() {
                vid_trigger_sender.send(TriggerStatus::Start);
            }

            match vid_trigger_receiver.recv().await {
                Ok(TriggerStatus::Start) => {
                    // This is expected
                }
                _ => {
                    panic!("did not receive TriggerStatus::Start from vid_trigger_receiver as expected");
                }
            }
        }

        {
            // This ensures that the vid_sender that is stored is still the
            // same, or links to the vid_receiver that we submitted.
            let (vid_commitment, vid_precompute) = precompute_vid_commitment(&[1, 2, 3, 4, 5], 4);
            assert_eq!(
                vid_sender
                    .send((vid_commitment, vid_precompute.clone()))
                    .await,
                Ok(()),
                "The vid_sender should be able to send the vid commitment and precompute"
            );

            let mut vid_receiver_write_lock_guard =
                retrieved_block_info.vid_receiver.write_arc().await;

            // Get and Keep object

            match vid_receiver_write_lock_guard.get().await {
                Ok((received_vid_commitment, received_vid_precompute)) => {
                    assert_eq!(
                        received_vid_commitment, vid_commitment,
                        "The received vid commitment should match the expected vid commitment"
                    );
                    assert_eq!(
                        received_vid_precompute, vid_precompute,
                        "The received vid precompute should match the expected vid precompute"
                    );
                }
                _ => {
                    panic!("did not receive the expected vid commitment and precompute from vid_receiver_write_lock_guard");
                }
            }
        }

        // finish with builder_state_to_last_built_block

        assert_eq!(
            state.builder_state_to_last_built_block.len(),
            1,
            "The builder state to last built block should have a single entry"
        );

        let last_built_block = state
            .builder_state_to_last_built_block
            .get(&builder_state_id);

        assert!(
            last_built_block.is_some(),
            "The last built block should be retrievable"
        );

        let last_built_block = last_built_block.unwrap();

        assert_eq!(
            last_built_block.builder_hash, builder_hash_2,
            "The last built block id should match the block id"
        );

        assert_eq!(
            last_built_block.block_size, 32,
            "The last built block size should match the response message"
        );

        assert_eq!(
            last_built_block.offered_fee, 128,
            "The last built block offered fee should match the response message"
        );
    }

    /// This test demonstrates the replacement behavior of the the
    /// `update_global_state` method.
    ///
    /// When given a `BuilderStateId` that already exists in the `blocks` LRU,
    /// and the `builder_state_to_last_built_block` hashmap, the method will
    /// replace the values in the `builder_state_to_last_built_block` hashmap,
    /// but will not replace the values in the `blocks` LRU.
    #[async_std::test]
    async fn test_global_state_update_global_state_replacement() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let parent_commit = vid_commitment(&[], 8);
        let mut state = GlobalState::<TestTypes>::new(
            bootstrap_sender,
            tx_sender,
            parent_commit,
            ViewNumber::new(0),
            ViewNumber::new(0),
            10,
        );

        let new_parent_commit = vid_commitment(&[], 9);
        let new_view_num = ViewNumber::new(1);
        let builder_state_id = BuilderStateId {
            parent_commitment: new_parent_commit,
            view: new_view_num,
        };

        let builder_hash = BuilderCommitment::from_bytes([1, 2, 3, 4]);
        let block_id_1 = BlockId {
            hash: builder_hash.clone(),
            view: new_view_num,
        };
        let (vid_trigger_sender_1, vid_trigger_receiver_1) =
            async_compatibility_layer::channel::oneshot();
        let (vid_sender_1, vid_receiver_1) = unbounded();
        let (block_payload_1, metadata_1) =
            <TestBlockPayload as BlockPayload<TestTypes>>::from_transactions(
                vec![TestTransaction::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])],
                &TestValidatedState::default(),
                &TestInstanceState::default(),
            )
            .await
            .unwrap();
        let offered_fee_1 = 64u64;
        let block_size_1 = 64u64;
        let truncated_1 = false;
        let build_block_info_1 = BuildBlockInfo {
            id: block_id_1.clone(),
            block_size: block_size_1,
            offered_fee: offered_fee_1,
            block_payload: block_payload_1.clone(),
            metadata: metadata_1,
            vid_trigger: vid_trigger_sender_1,
            vid_receiver: vid_receiver_1,
            truncated: truncated_1,
        };
        let response_msg_1 = ResponseMessage {
            builder_hash: builder_hash.clone(),
            block_size: block_size_1,
            offered_fee: offered_fee_1,
        };

        // Now that every object is prepared and setup for storage, we can
        // test the `update_global_state` method.

        // `update_global_state` has not return value from its method, so can
        // only inspect its "success" based on the mutation of the state object.
        state.update_global_state(builder_state_id.clone(), build_block_info_1, response_msg_1);

        // We're going to enter another update_global_state_entry with the same
        // builder_state_id, but with different values for the block info and
        // response message.  This should highlight what values get replaced
        // in this update, and which stay the same.

        let block_id_2 = BlockId {
            hash: builder_hash.clone(),
            view: new_view_num,
        };
        let (vid_trigger_sender_2, vid_trigger_receiver_2) =
            async_compatibility_layer::channel::oneshot();
        let (vid_sender_2, vid_receiver_2) = unbounded();
        let (block_payload_2, metadata_2) =
            <TestBlockPayload as BlockPayload<TestTypes>>::from_transactions(
                vec![TestTransaction::new(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11])],
                &TestValidatedState::default(),
                &TestInstanceState::default(),
            )
            .await
            .unwrap();
        let offered_fee_2 = 16u64;
        let block_size_2 = 32u64;
        let truncated_2 = true;
        let build_block_info_2 = BuildBlockInfo {
            id: block_id_2.clone(),
            block_size: block_size_2,
            offered_fee: offered_fee_2,
            block_payload: block_payload_2.clone(),
            metadata: metadata_2,
            vid_trigger: vid_trigger_sender_2,
            vid_receiver: vid_receiver_2,
            truncated: truncated_2,
        };
        let response_msg_2: ResponseMessage = ResponseMessage {
            builder_hash: builder_hash.clone(),
            block_size: block_size_2,
            offered_fee: offered_fee_2,
        };

        // two things should be adjusted by `update_global_state`:
        // When given the same build_state_ids, the current logic indicates
        // that the LRU value does **NOT** get replaced, but the response
        // message does.

        state.update_global_state(builder_state_id.clone(), build_block_info_2, response_msg_2);

        // start with blocks

        assert_eq!(
            state.blocks.len(),
            1,
            "The blocks LRU should have a single entry"
        );

        let retrieved_block_info = state.blocks.get(&block_id_1);
        assert!(
            retrieved_block_info.is_some(),
            "Retrieval of the block id should result is a valid block info data"
        );

        let retrieved_block_info = retrieved_block_info.unwrap();

        assert_eq!(
            retrieved_block_info.block_payload, block_payload_1,
            "The block payloads should match"
        );
        assert_ne!(
            retrieved_block_info.block_payload, block_payload_2,
            "The block payloads should not match"
        );
        assert_eq!(
            retrieved_block_info.metadata, metadata_2,
            "The metadata should match"
        );
        assert_eq!(
            retrieved_block_info.metadata, metadata_1,
            "The metadata should match"
        );
        // TestMetadata will always match

        assert_eq!(
            retrieved_block_info.offered_fee, offered_fee_1,
            "The offered fee should match"
        );
        assert_ne!(
            retrieved_block_info.offered_fee, offered_fee_2,
            "The offered fee should not match"
        );
        assert_eq!(
            retrieved_block_info.truncated, truncated_1,
            "The truncated flag should match"
        );
        assert_ne!(
            retrieved_block_info.truncated, truncated_2,
            "The truncated flag should not match"
        );

        {
            // This ensures that the vid_trigger that is stored is still the
            // same, or links to the vid_trigger_receiver that we submitted.
            let mut vid_trigger_write_lock_guard =
                retrieved_block_info.vid_trigger.write_arc().await;
            if let Some(vid_trigger_sender) = vid_trigger_write_lock_guard.take() {
                vid_trigger_sender.send(TriggerStatus::Start);
            }

            match vid_trigger_receiver_1.recv().await {
                Ok(TriggerStatus::Start) => {
                    // This is expected
                }
                _ => {
                    panic!("did not receive TriggerStatus::Start from vid_trigger_receiver as expected");
                }
            }

            assert!(
                vid_trigger_receiver_2.recv().await.is_err(),
                "This should not receive anything from vid_trigger_receiver_2"
            );
        }

        {
            // This ensures that the vid_sender that is stored is still the
            // same, or links to the vid_receiver that we submitted.
            let (vid_commitment, vid_precompute) = precompute_vid_commitment(&[1, 2, 3, 4, 5], 4);
            assert_eq!(
                vid_sender_1
                    .send((vid_commitment, vid_precompute.clone()))
                    .await,
                Ok(()),
                "The vid_sender should be able to send the vid commitment and precompute"
            );

            assert!(
                vid_sender_2
                    .send((vid_commitment, vid_precompute.clone()))
                    .await
                    .is_err(),
                "The vid_sender should not be able to send the vid commitment and precompute"
            );

            let mut vid_receiver_write_lock_guard =
                retrieved_block_info.vid_receiver.write_arc().await;

            // Get and Keep object

            match vid_receiver_write_lock_guard.get().await {
                Ok((received_vid_commitment, received_vid_precompute)) => {
                    assert_eq!(
                        received_vid_commitment, vid_commitment,
                        "The received vid commitment should match the expected vid commitment"
                    );
                    assert_eq!(
                        received_vid_precompute, vid_precompute,
                        "The received vid precompute should match the expected vid precompute"
                    );
                }
                _ => {
                    panic!("did not receive the expected vid commitment and precompute from vid_receiver_write_lock_guard");
                }
            }
        }

        // finish with builder_state_to_last_built_block

        assert_eq!(
            state.builder_state_to_last_built_block.len(),
            1,
            "The builder state to last built block should have a single entry"
        );

        let last_built_block = state
            .builder_state_to_last_built_block
            .get(&builder_state_id);

        assert!(
            last_built_block.is_some(),
            "The last built block should be retrievable"
        );

        let last_built_block = last_built_block.unwrap();

        assert_eq!(
            last_built_block.builder_hash, builder_hash,
            "The last built block id should match the block id"
        );

        assert_eq!(
            last_built_block.block_size, block_size_2,
            "The last built block size should match the response message"
        );
        assert_ne!(
            last_built_block.block_size, block_size_1,
            "The last built block size should not match the previous block size"
        );

        assert_eq!(
            last_built_block.offered_fee, offered_fee_2,
            "The last built block offered fee should match the response message"
        );
        assert_ne!(
            last_built_block.offered_fee, offered_fee_1,
            "The last built block offered fee should not match the previous block offered fee"
        );
    }

    // GlobalState::remove_handles Tests

    /// This test checks to ensure that remove_handles will only consider
    /// views up to what is known to have been stored. As a result it will
    /// indicate that is has only targeted to the highest view number that it
    /// is aware of.
    #[async_std::test]
    async fn test_global_state_remove_handles_prune_up_to_latest() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let parent_commit = vid_commitment(&[0], 4);
        let mut state = GlobalState::<TestTypes>::new(
            bootstrap_sender,
            tx_sender,
            parent_commit,
            ViewNumber::new(0),
            ViewNumber::new(0),
            10,
        );

        // We register a few builder states.
        for i in 1..=10 {
            state.register_builder_state(
                BuilderStateId {
                    parent_commitment: vid_commitment(&[i], 4),
                    view: ViewNumber::new(i as u64),
                },
                async_broadcast::broadcast(10).0,
            );
        }

        assert_eq!(
            state.remove_handles(ViewNumber::new(100)),
            ViewNumber::new(10),
            "It should only be able to prune up to what has been stored"
        );

        assert_eq!(
            state.spawned_builder_states.len(),
            1,
            "The spawned_builder_states should only have a single entry in it"
        );

        for i in 0..10 {
            let builder_state_id = BuilderStateId {
                parent_commitment: vid_commitment(&[i], 4),
                view: ViewNumber::new(i as u64),
            };
            assert!(
                !state.spawned_builder_states.contains_key(&builder_state_id),
                "the spawned builder states should not contain the builder state id, {builder_state_id}"
            );
        }

        let builder_state_id = BuilderStateId {
            parent_commitment: vid_commitment(&[10], 4),
            view: ViewNumber::new(10),
        };
        assert_eq!(
            state.highest_view_num_builder_id, builder_state_id,
            "The highest view number builder id should be the one that was just registered"
        );

        assert_eq!(
            state.last_garbage_collected_view_num,
            ViewNumber::new(9),
            "The last garbage collected view number should match expected value"
        );

        assert!(
            state.spawned_builder_states.contains_key(&BuilderStateId {
                parent_commitment: vid_commitment(&[10], 4),
                view: ViewNumber::new(10),
            }),
            "The spawned builder states should contain the builder state id: {builder_state_id}"
        );
    }

    /// This test checks that the remove_handles doesn't ensure that the
    /// `last_garbage_collected_view_num` is strictly increasing. By first
    /// removing a higher view  number, followed by a smaller view number
    /// (with the highest_view_num_builder_id having a view greater than or
    /// equal to both targets) we can demonstrate this property.
    ///
    /// Furthermore this demonstrates that by supplying any view number to
    /// remove_handles that is less than `last_garbage_collected_view_num` will
    /// result in `last_garbage_collected_view_num` being updated to the given
    /// value minus 1, without regard for it actually removing / cleaning
    /// anything, or whether it is moving backwards in view numbers.
    ///
    /// If we were to account for the view numbers actually being cleaned up,
    /// we could still trigger this behavior be re-adding the builder states
    /// with a view number that precedes the last garbage collected view number,
    /// then removing them would trigger the same behavior.
    #[async_std::test]
    async fn test_global_state_remove_handles_can_reduce_last_garbage_collected_view_num_simple() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let parent_commit = vid_commitment(&[0], 4);
        let mut state = GlobalState::<TestTypes>::new(
            bootstrap_sender,
            tx_sender,
            parent_commit,
            ViewNumber::new(0),
            ViewNumber::new(0),
            10,
        );

        // We register a few builder states.
        for i in 1..=10 {
            state.register_builder_state(
                BuilderStateId {
                    parent_commitment: vid_commitment(&[i], 4),
                    view: ViewNumber::new(i as u64),
                },
                async_broadcast::broadcast(10).0,
            );
        }

        assert_eq!(
            state.highest_view_num_builder_id,
            BuilderStateId {
                parent_commitment: vid_commitment(&[10], 4),
                view: ViewNumber::new(10),
            },
            "The highest view number builder id should be the one that was just registered"
        );

        assert_eq!(
            state.remove_handles(ViewNumber::new(10)),
            ViewNumber::new(10),
            "It should remove what has been stored"
        );

        assert_eq!(
            state.last_garbage_collected_view_num,
            ViewNumber::new(9),
            "The last garbage collected view number should match expected value"
        );

        assert_eq!(
            state.remove_handles(ViewNumber::new(5)),
            ViewNumber::new(5),
            "If we only remove up to view 5, then only entries preceding view 5 should be removed"
        );

        // The last garbage collected view has gone down as a result of our
        // new remove_handles target, demonstrating that this number isn't
        // strictly increasing in value.
        assert_eq!(
            state.last_garbage_collected_view_num,
            ViewNumber::new(4),
            "The last garbage collected view number should match expected value",
        );
    }

    /// This test checks that the remove_handles doesn't ensure that the
    /// `last_garbage_collected_view_num` is strictly increasing. It is very
    /// similar to `test_global_state_remove_handles_can_reduce_last_garbage_collected_view_num_simple`
    /// but differs in that it re-adds the removed builder states, just in case
    /// the previous test's behavior is erroneous and fixed by ensuring that we
    /// only consider removed view numbers.
    #[async_std::test]
    async fn test_global_state_remove_handles_can_reduce_last_garbage_collected_view_num_strict() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let parent_commit = vid_commitment(&[0], 4);
        let mut state = GlobalState::<TestTypes>::new(
            bootstrap_sender,
            tx_sender,
            parent_commit,
            ViewNumber::new(0),
            ViewNumber::new(0),
            10,
        );

        // We register a few builder states.
        for i in 1..=10 {
            state.register_builder_state(
                BuilderStateId {
                    parent_commitment: vid_commitment(&[i], 4),
                    view: ViewNumber::new(i as u64),
                },
                async_broadcast::broadcast(10).0,
            );
        }

        assert_eq!(
            state.highest_view_num_builder_id,
            BuilderStateId {
                parent_commitment: vid_commitment(&[10], 4),
                view: ViewNumber::new(10),
            },
            "The highest view number builder id should be the one that was just registered"
        );

        assert_eq!(
            state.remove_handles(ViewNumber::new(10)),
            ViewNumber::new(10),
            "It should remove what has been stored"
        );

        assert_eq!(
            state.last_garbage_collected_view_num,
            ViewNumber::new(9),
            "The last garbage collected view number should match expected value"
        );

        // We re-add these removed builder_state_ids
        for i in 1..10 {
            state.register_builder_state(
                BuilderStateId {
                    parent_commitment: vid_commitment(&[i], 4),
                    view: ViewNumber::new(i as u64),
                },
                async_broadcast::broadcast(10).0,
            );
        }

        assert_eq!(
            state.remove_handles(ViewNumber::new(5)),
            ViewNumber::new(5),
            "If we only remove up to view 5, then only entries preceding view 5 should be removed"
        );

        // The last garbage collected view has gone down as a result of our
        // new remove_handles target, demonstrating that this number isn't
        // strictly increasing in value.
        assert_eq!(
            state.last_garbage_collected_view_num,
            ViewNumber::new(4),
            "The last garbage collected view number should match expected value",
        );
    }

    /// This test checks that the remove_handles methods will correctly remove
    /// The expected number of builder states from the spawned_builder_states
    /// hashmap.  It does this by specifically controlling the number of builder
    /// states that are registered, and then removing a subset of them. It
    /// verifies the absence of the entries that should have been removed, and
    /// the presence of the entries that should have been kept.
    #[async_std::test]
    async fn test_global_state_remove_handles_expected() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let parent_commit = vid_commitment(&[0], 4);
        let mut state = GlobalState::<TestTypes>::new(
            bootstrap_sender,
            tx_sender,
            parent_commit,
            ViewNumber::new(0),
            ViewNumber::new(0),
            10,
        );

        // We register a few builder states.
        for i in 1..=10 {
            state.register_builder_state(
                BuilderStateId {
                    parent_commitment: vid_commitment(&[i], 4),
                    view: ViewNumber::new(i as u64),
                },
                async_broadcast::broadcast(10).0,
            );
        }

        assert_eq!(
            state.spawned_builder_states.len(),
            11,
            "The spawned_builder_states should have 11 elements in it"
        );

        assert_eq!(
            state.highest_view_num_builder_id,
            BuilderStateId {
                parent_commitment: vid_commitment(&[10], 4),
                view: ViewNumber::new(10),
            },
            "The highest view number builder id should be the one that was just registered"
        );

        assert_eq!(
            state.last_garbage_collected_view_num,
            ViewNumber::new(0),
            "The last garbage collected view number should be hat was passed in"
        );

        // Now we want to clean up some previous builder states to ensure that we
        // remove the appropriate targets.

        // This should remove the view builder states preceding the view number 5
        assert_eq!(
            state.remove_handles(ViewNumber::new(5)),
            ViewNumber::new(5),
            "The last garbage collected view number should match expected value"
        );

        // There should be 11 - 5 entries remaining
        assert_eq!(
            state.spawned_builder_states.len(),
            6,
            "The spawned_builder_states should have 6 elements in it"
        );

        for i in 0..5 {
            let builder_state_id = BuilderStateId {
                parent_commitment: vid_commitment(&[i], 4),
                view: ViewNumber::new(i as u64),
            };
            assert!(
                !state.spawned_builder_states.contains_key(&builder_state_id),
                "the spawned builder states should contain the builder state id, {builder_state_id}"
            );
        }

        for i in 5..=10 {
            let builder_state_id = BuilderStateId {
                parent_commitment: vid_commitment(&[i], 4),
                view: ViewNumber::new(i as u64),
            };
            assert!(
                state.spawned_builder_states.contains_key(&builder_state_id),
                "The spawned builder states should contain the builder state id: {builder_state_id}"
            );
        }
    }

    // Get Available Blocks Tests

    /// This test checks that the error `AvailableBlocksError::NoBlocksAvailable`
    /// is returned when no blocks are available.
    ///
    /// To trigger this condition, we simply submit a request to the
    /// implementation of get_available_blocks, and we do not provide any
    /// information for the block view number requested.  As a result, the
    /// implementation will ultimately timeout, and return an error that
    /// indicates that no blocks were available.
    #[async_std::test]
    async fn test_get_available_blocks_error_no_blocks_available() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(0),
                10,
            ))),
            (builder_public_key, builder_private_key),
            Duration::from_millis(100),
        );

        // leader_private_key
        let signature = BLSPubKey::sign(&leader_private_key, parent_commit.as_ref()).unwrap();

        // This *should* just time out
        let result = state
            .available_blocks_implementation(
                &vid_commitment(&[], 8),
                1,
                leader_public_key,
                &signature,
            )
            .await;

        match result {
            Err(AvailableBlocksError::NoBlocksAvailable) => {
                // This is what we expect.
                // This message *should* indicate that no blocks were available.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
        }
    }

    /// This test checks that the error `AvailableBlocksError::SignatureValidationFailed`
    /// is returned when the signature is invalid.
    ///
    /// To trigger this condition, we simply submit a request to the
    /// implementation of get_available_blocks, but we sign the request with
    /// the builder's private key instead of the leader's private key.  Since
    /// these keys do not match, this will result in a signature verification
    /// error.
    #[async_std::test]
    async fn test_get_available_blocks_error_invalid_signature() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, _leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(0),
                10,
            ))),
            (builder_public_key, builder_private_key.clone()),
            Duration::from_millis(100),
        );

        // leader_private_key
        let signature = BLSPubKey::sign(&builder_private_key, parent_commit.as_ref()).unwrap();

        // This *should* just time out
        let result = state
            .available_blocks_implementation(
                &vid_commitment(&[], 8),
                1,
                leader_public_key,
                &signature,
            )
            .await;

        match result {
            Err(AvailableBlocksError::SignatureValidationFailed) => {
                // This is what we expect.
                // This message *should* indicate that the signature passed
                // did not match the given public key.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
        }
    }

    /// This test checks that the error `AvailableBlocksError::RequestForAvailableViewThatHasAlreadyBeenDecided`
    /// is returned when the requested view number has already been garbage
    /// collected.
    ///
    /// To trigger this condition, we initialize the GlobalState with a
    /// garbage collected view number that is higher than the view that will
    /// be requested.
    #[async_std::test]
    async fn test_get_available_blocks_error_requesting_previous_view_number() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(2),
                10,
            ))),
            (builder_public_key, builder_private_key),
            Duration::from_millis(100),
        );

        // leader_private_key
        let signature = BLSPubKey::sign(&leader_private_key, parent_commit.as_ref()).unwrap();

        // This *should* just time out
        let result = state
            .available_blocks_implementation(
                &vid_commitment(&[], 8),
                1,
                leader_public_key,
                &signature,
            )
            .await;

        match result {
            Err(AvailableBlocksError::RequestForAvailableViewThatHasAlreadyBeenDecided) => {
                // This is what we expect.
                // This message *should* indicate that the signature passed
                // did not match the given public key.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
        }
    }

    /// This test checks that the error `AvailableBlocksError::GetChannelForMatchingBuilderError`
    /// is returned when attempting to retrieve a view that is not stored within the state, and
    /// the highest view is also no longer stored within the state.
    ///
    /// To trigger this condition, we initialize the GlobalState with an initial
    /// state, and then we mutate the state to record the wrong latest state id.
    /// When interacted with `GlobalState` via `register_builder_state`, and
    /// `remove_handles`, this error doesn't seem possible immediately possible.
    #[async_std::test]
    async fn test_get_available_blocks_error_get_channel_for_matching_builder() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = Arc::new(ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(4),
                ViewNumber::new(4),
                10,
            ))),
            (builder_public_key, builder_private_key.clone()),
            Duration::from_secs(1),
        ));

        {
            let mut write_locked_global_state = state.global_state.write_arc().await;
            write_locked_global_state.highest_view_num_builder_id = BuilderStateId {
                parent_commitment: parent_commit,
                view: ViewNumber::new(5),
            };
        }

        // As a result, we **should** be receiving a request for the available
        // blocks with our expected state id on the receiver, along with a channel
        // to send the response back to the caller.

        let signature = BLSPubKey::sign(&leader_private_key, parent_commit.as_ref()).unwrap();
        let result = state
            .available_blocks_implementation(&parent_commit, 6, leader_public_key, &signature)
            .await;
        match result {
            Err(AvailableBlocksError::GetChannelForMatchingBuilderError(_)) => {
                // This is what we expect.
                // This message *should* indicate that the response channel was closed.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
        }
    }

    // We have two error cases for `available_blocks_implementation` that we
    // cannot seem trigger directly due to the nature of how the implementation
    // performs.
    //
    // The first is ChannelUnexpectedlyClosed, which doesn't seem to be
    // producible as the unbounded channel doesn't seem to be able to be
    // closed.
    //
    // The second is SigningBlockFailed, which doesn't seem to be producible
    // with a valid private key, and it's not clear how to create an invalid
    // private key.

    /// This test checks that call to `available_blocks_implementation` returns
    /// a successful response when the function is called before blocks are
    /// made available.
    #[async_std::test]
    async fn test_get_available_blocks_requested_before_blocks_available() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = Arc::new(ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(0),
                10,
            ))),
            (builder_public_key, builder_private_key.clone()),
            Duration::from_secs(1),
        ));

        let cloned_parent_commit = parent_commit;
        let cloned_state = state.clone();
        let cloned_leader_private_key = leader_private_key.clone();

        // We want to trigger a request for the available blocks, before we make the available block available
        let get_available_blocks_handle = async_std::task::spawn(async move {
            // leader_private_key
            let signature =
                BLSPubKey::sign(&cloned_leader_private_key, cloned_parent_commit.as_ref()).unwrap();
            cloned_state
                .available_blocks_implementation(
                    &cloned_parent_commit,
                    1,
                    leader_public_key,
                    &signature,
                )
                .await
        });

        // Now we want to make the block data available to the state.
        let expected_builder_state_id = BuilderStateId {
            parent_commitment: parent_commit,
            view: ViewNumber::new(1),
        };

        let mut response_receiver = {
            // We only want to keep this write lock for the time needed, and
            // no more.
            let mut write_locked_global_state = state.global_state.write_arc().await;

            // We insert a sender so that the next time this stateId is requested,
            // it will be available to send data back.
            let (response_sender, response_receiver) = async_broadcast::broadcast(10);
            write_locked_global_state
                .register_builder_state(expected_builder_state_id.clone(), response_sender);

            response_receiver
        };

        // As a result, we **should** be receiving a request for the available
        // blocks with our expected state id on the receiver, along with a channel
        // to send the response back to the caller.

        let response_channel = match response_receiver.next().await {
            None => {
                panic!("Expected a request for available blocks, but didn't get one");
            }
            Some(MessageType::RequestMessage(req_msg)) => {
                assert_eq!(req_msg.state_id, expected_builder_state_id);
                req_msg.response_channel
            }
            Some(message) => {
                panic!(
                    "Expected a request for available blocks, but got a different message: {:?}",
                    message
                );
            }
        };

        // We want to send a ResponseMessage to the channel
        let expected_response = ResponseMessage {
            block_size: 9,
            offered_fee: 7,
            builder_hash: BuilderCommitment::from_bytes([1, 2, 3, 4, 5]),
        };

        assert!(
            response_channel
                .send(expected_response.clone())
                .await
                .is_ok(),
            "failed to send ResponseMessage"
        );

        let result = get_available_blocks_handle.await;
        match result {
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(result) => {
                assert_eq!(
                    result,
                    vec![AvailableBlockInfo {
                        block_hash: expected_response.builder_hash.clone(),
                        block_size: expected_response.block_size,
                        offered_fee: expected_response.offered_fee,
                        signature: <BLSPubKey as BuilderSignatureKey>::sign_block_info(
                            &builder_private_key,
                            expected_response.block_size,
                            expected_response.offered_fee,
                            &expected_response.builder_hash,
                        )
                        .unwrap(),
                        sender: builder_public_key,
                        _phantom: Default::default(),
                    }],
                    "get_available_blocks response matches expectation"
                );
            }
        }
    }

    /// This test checks that call to `available_blocks_implementation` returns
    /// a successful response when the function is called after blocks are
    /// made available.
    #[async_std::test]
    async fn test_get_available_blocks_requested_after_blocks_available() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = Arc::new(ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(0),
                10,
            ))),
            (builder_public_key, builder_private_key.clone()),
            Duration::from_secs(1),
        ));

        let cloned_parent_commit = parent_commit;
        let cloned_state = state.clone();
        let cloned_leader_private_key = leader_private_key.clone();

        // Now we want to make the block data available to the state.
        let expected_builder_state_id = BuilderStateId {
            parent_commitment: parent_commit,
            view: ViewNumber::new(1),
        };

        let mut response_receiver = {
            // We only want to keep this write lock for the time needed, and
            // no more.
            let mut write_locked_global_state = state.global_state.write_arc().await;

            // We insert a sender so that the next time this stateId is requested,
            // it will be available to send data back.
            let (response_sender, response_receiver) = async_broadcast::broadcast(10);
            write_locked_global_state
                .register_builder_state(expected_builder_state_id.clone(), response_sender);

            response_receiver
        };

        // We want to trigger a request for the available blocks, before we make the available block available
        let get_available_blocks_handle = async_std::task::spawn(async move {
            // leader_private_key
            let signature =
                BLSPubKey::sign(&cloned_leader_private_key, cloned_parent_commit.as_ref()).unwrap();
            cloned_state
                .available_blocks_implementation(
                    &cloned_parent_commit,
                    1,
                    leader_public_key,
                    &signature,
                )
                .await
        });

        // As a result, we **should** be receiving a request for the available
        // blocks with our expected state id on the receiver, along with a channel
        // to send the response back to the caller.

        let response_channel = match response_receiver.next().await {
            None => {
                panic!("Expected a request for available blocks, but didn't get one");
            }
            Some(MessageType::RequestMessage(req_msg)) => {
                assert_eq!(req_msg.state_id, expected_builder_state_id);
                req_msg.response_channel
            }
            Some(message) => {
                panic!(
                    "Expected a request for available blocks, but got a different message: {:?}",
                    message
                );
            }
        };

        // We want to send a ResponseMessage to the channel
        let expected_response = ResponseMessage {
            block_size: 9,
            offered_fee: 7,
            builder_hash: BuilderCommitment::from_bytes([1, 2, 3, 4, 5]),
        };

        assert!(
            response_channel
                .send(expected_response.clone())
                .await
                .is_ok(),
            "failed to send ResponseMessage"
        );

        let result = get_available_blocks_handle.await;
        match result {
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(result) => {
                assert_eq!(
                    result,
                    vec![AvailableBlockInfo {
                        block_hash: expected_response.builder_hash.clone(),
                        block_size: expected_response.block_size,
                        offered_fee: expected_response.offered_fee,
                        signature: <BLSPubKey as BuilderSignatureKey>::sign_block_info(
                            &builder_private_key,
                            expected_response.block_size,
                            expected_response.offered_fee,
                            &expected_response.builder_hash,
                        )
                        .unwrap(),
                        sender: builder_public_key,
                        _phantom: Default::default(),
                    }],
                    "get_available_blocks response matches expectation"
                );
            }
        }
    }

    // Claim Block Tests

    /// This test checks that the error `ClaimBlockError::SignatureValidationFailed`
    /// is returned when the signature is invalid.
    ///
    /// To trigger this condition, we simply submit a request to the
    /// implementation of claim_block, but we sign the request with
    /// the builder's private key instead of the leader's private key.  Since
    /// these keys do not match, this will result in a signature verification
    /// error.
    #[async_std::test]
    async fn test_claim_block_error_signature_validation_failed() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, _leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = Arc::new(ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(0),
                10,
            ))),
            (builder_public_key, builder_private_key.clone()),
            Duration::from_secs(1),
        ));

        let commitment = BuilderCommitment::from_bytes([0; 256]);

        let signature = BLSPubKey::sign(&builder_private_key, commitment.as_ref()).unwrap();
        let result = state
            .claim_block_implementation(&commitment, 1, leader_public_key, &signature)
            .await;

        match result {
            Err(ClaimBlockError::SignatureValidationFailed) => {
                // This is what we expect.
                // This message *should* indicate that the signature passed
                // did not match the given public key.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
        }
    }

    /// This test checks that the error `ClaimBlockError::BlockDataNotFound`
    /// is returned when the block data is not found.
    ///
    /// To trigger this condition, we simply submit a request to the
    /// implementation of claim_block, but we do not provide any information
    /// for the block data requested.  As a result, the implementation will
    /// ultimately timeout, and return an error that indicates that the block
    /// data was not found.
    #[async_std::test]
    async fn test_claim_block_error_block_data_not_found() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = Arc::new(ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(0),
                10,
            ))),
            (builder_public_key, builder_private_key.clone()),
            Duration::from_secs(1),
        ));

        let commitment = BuilderCommitment::from_bytes([0; 256]);

        let signature = BLSPubKey::sign(&leader_private_key, commitment.as_ref()).unwrap();
        let result = state
            .claim_block_implementation(&commitment, 1, leader_public_key, &signature)
            .await;

        match result {
            Err(ClaimBlockError::BlockDataNotFound) => {
                // This is what we expect.
                // This message *should* indicate that the signature passed
                // did not match the given public key.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
        }
    }

    /// This test checks that the function completes successfully.
    #[async_std::test]
    async fn test_claim_block_success() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = Arc::new(ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(0),
                10,
            ))),
            (builder_public_key, builder_private_key.clone()),
            Duration::from_secs(1),
        ));

        let commitment = BuilderCommitment::from_bytes([0; 256]);
        let cloned_commitment = commitment.clone();
        let cloned_state = state.clone();

        let vid_trigger_receiver = {
            let mut global_state_write_lock = state.global_state.write_arc().await;
            let block_id = BlockId {
                hash: commitment,
                view: ViewNumber::new(1),
            };

            let payload = TestBlockPayload {
                transactions: vec![TestTransaction::new(vec![1, 2, 3, 4])],
            };

            let (vid_trigger_sender, vid_trigger_receiver) =
                async_compatibility_layer::channel::oneshot();
            let (_, vid_receiver) = unbounded();

            global_state_write_lock.blocks.put(
                block_id,
                BlockInfo {
                    block_payload: payload,
                    metadata: TestMetadata,
                    vid_trigger: Arc::new(async_lock::RwLock::new(Some(vid_trigger_sender))),
                    vid_receiver: Arc::new(async_lock::RwLock::new(crate::WaitAndKeep::Wait(
                        vid_receiver,
                    ))),
                    offered_fee: 100,
                    truncated: false,
                },
            );

            vid_trigger_receiver
        };

        let claim_block_join_handle = async_std::task::spawn(async move {
            let signature =
                BLSPubKey::sign(&leader_private_key, cloned_commitment.as_ref()).unwrap();
            cloned_state
                .claim_block_implementation(&cloned_commitment, 1, leader_public_key, &signature)
                .await
        });

        // This should be the started event
        match vid_trigger_receiver.recv().await {
            Ok(TriggerStatus::Start) => {
                // This is what we expect.
            }
            _ => {
                panic!("Expected a TriggerStatus::Start event");
            }
        }

        let result = claim_block_join_handle.await;

        match result {
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(_) => {
                // This is expected
            }
        }
    }

    // Claim Block Header Input Tests

    /// This test checks that the error `ClaimBlockHeaderInputError::SignatureValidationFailed`
    /// is returned when the signature is invalid.
    ///
    /// To trigger this condition, we simply submit a request to the
    /// implementation of claim_block, but we sign the request with
    /// the builder's private key instead of the leader's private key.  Since
    /// these keys do not match, this will result in a signature verification
    /// error.
    #[async_std::test]
    async fn test_claim_block_header_input_error_signature_verification_failed() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, _leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = Arc::new(ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(0),
                10,
            ))),
            (builder_public_key, builder_private_key.clone()),
            Duration::from_secs(1),
        ));

        let commitment = BuilderCommitment::from_bytes([0; 256]);

        let signature = BLSPubKey::sign(&builder_private_key, commitment.as_ref()).unwrap();

        let result = state
            .claim_block_header_input_implementation(&commitment, 1, leader_public_key, &signature)
            .await;

        match result {
            Err(ClaimBlockHeaderInputError::SignatureValidationFailed) => {
                // This is what we expect.
                // This message *should* indicate that the signature passed
                // did not match the given public key.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
        }
    }

    /// This test checks that the error `ClaimBlockHeaderInputError::BlockHeaderNotFound`
    /// is returned when the block header is not found.
    ///
    /// To trigger this condition, we simply submit a request to the
    /// implementation of claim_block, but we do not provide any information
    /// for the block header requested.  As a result, the implementation will
    /// ultimately timeout, and return an error that indicates that the block
    /// header was not found.
    #[async_std::test]
    async fn test_claim_block_header_input_error_block_header_not_found() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = Arc::new(ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(0),
                10,
            ))),
            (builder_public_key, builder_private_key.clone()),
            Duration::from_secs(1),
        ));

        let commitment = BuilderCommitment::from_bytes([0; 256]);

        let signature = BLSPubKey::sign(&leader_private_key, commitment.as_ref()).unwrap();

        let result = state
            .claim_block_header_input_implementation(&commitment, 1, leader_public_key, &signature)
            .await;

        match result {
            Err(ClaimBlockHeaderInputError::BlockHeaderNotFound) => {
                // This is what we expect.
                // This message *should* indicate that the signature passed
                // did not match the given public key.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
        }
    }

    /// This test checks that the error `ClaimBlockHeaderInputError::CouldNotGetVidInTime`
    /// is returned when the VID is not received in time.
    ///
    /// To trigger this condition, we simply submit a request to the
    /// implementation of claim_block, but we do not provide a VID. As a result,
    /// the implementation will ultimately timeout, and return an error that
    /// indicates that the VID was not received in time.
    ///
    /// At least that's what it should do.  At the moment, this results in a
    /// deadlock due to attempting to acquire the `write_arc` twice.
    #[async_std::test]
    async fn test_claim_block_header_input_error_could_not_get_vid_in_time() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = Arc::new(ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(0),
                10,
            ))),
            (builder_public_key, builder_private_key.clone()),
            Duration::from_secs(1),
        ));

        let commitment = BuilderCommitment::from_bytes([0; 256]);
        let cloned_commitment = commitment.clone();
        let cloned_state = state.clone();

        let _vid_sender = {
            let mut global_state_write_lock = state.global_state.write_arc().await;
            let block_id = BlockId {
                hash: commitment,
                view: ViewNumber::new(1),
            };

            let payload = TestBlockPayload {
                transactions: vec![TestTransaction::new(vec![1, 2, 3, 4])],
            };

            let (vid_trigger_sender, _) = async_compatibility_layer::channel::oneshot();
            let (vid_sender, vid_receiver) = unbounded();

            global_state_write_lock.blocks.put(
                block_id,
                BlockInfo {
                    block_payload: payload,
                    metadata: TestMetadata,
                    vid_trigger: Arc::new(async_lock::RwLock::new(Some(vid_trigger_sender))),
                    vid_receiver: Arc::new(async_lock::RwLock::new(crate::WaitAndKeep::Wait(
                        vid_receiver,
                    ))),
                    offered_fee: 100,
                    truncated: false,
                },
            );

            vid_sender
        };

        let claim_block_header_input_join_handle = async_std::task::spawn(async move {
            let signature =
                BLSPubKey::sign(&leader_private_key, cloned_commitment.as_ref()).unwrap();
            cloned_state
                .claim_block_header_input_implementation(
                    &cloned_commitment,
                    1,
                    leader_public_key,
                    &signature,
                )
                .await
        });

        let result = claim_block_header_input_join_handle.await;

        match result {
            Err(ClaimBlockHeaderInputError::CouldNotGetVidInTime) => {
                // This is what we expect.
                // This message *should* indicate that the signature passed
                // did not match the given public key.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
        }
    }

    /// This test checks that the error `ClaimBlockHeaderInputError::WaitAndKeepGetError`
    /// is returned when the VID is not received in time.
    ///
    /// To trigger this condition, we simply submit a request to the
    /// implementation of claim_block, but we close the VID receiver channel's
    /// sender.
    #[async_std::test]
    async fn test_claim_block_header_input_error_keep_and_wait_get_error() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = Arc::new(ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(0),
                10,
            ))),
            (builder_public_key, builder_private_key.clone()),
            Duration::from_secs(1),
        ));

        let commitment = BuilderCommitment::from_bytes([0; 256]);
        let cloned_commitment = commitment.clone();
        let cloned_state = state.clone();

        {
            let mut global_state_write_lock = state.global_state.write_arc().await;
            let block_id = BlockId {
                hash: commitment,
                view: ViewNumber::new(1),
            };

            let payload = TestBlockPayload {
                transactions: vec![TestTransaction::new(vec![1, 2, 3, 4])],
            };

            let (vid_trigger_sender, _) = async_compatibility_layer::channel::oneshot();
            let (_, vid_receiver) = unbounded();

            global_state_write_lock.blocks.put(
                block_id,
                BlockInfo {
                    block_payload: payload,
                    metadata: TestMetadata,
                    vid_trigger: Arc::new(async_lock::RwLock::new(Some(vid_trigger_sender))),
                    vid_receiver: Arc::new(async_lock::RwLock::new(crate::WaitAndKeep::Wait(
                        vid_receiver,
                    ))),
                    offered_fee: 100,
                    truncated: false,
                },
            );
        };

        let claim_block_header_input_join_handle = async_std::task::spawn(async move {
            let signature =
                BLSPubKey::sign(&leader_private_key, cloned_commitment.as_ref()).unwrap();
            cloned_state
                .claim_block_header_input_implementation(
                    &cloned_commitment,
                    1,
                    leader_public_key,
                    &signature,
                )
                .await
        });

        let result = claim_block_header_input_join_handle.await;

        match result {
            Err(ClaimBlockHeaderInputError::WaitAndKeepGetError(_)) => {
                // This is what we expect.
                // This message *should* indicate that the signature passed
                // did not match the given public key.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
        }
    }

    /// This test checks that successful response is returned when the VID is
    /// received in time.
    #[async_std::test]
    async fn test_claim_block_header_input_success() {
        let (bootstrap_sender, _) = async_broadcast::broadcast(10);
        let (tx_sender, _) = async_broadcast::broadcast(10);
        let (builder_public_key, builder_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, leader_private_key) =
            <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let parent_commit = vid_commitment(&[], 8);

        let state = Arc::new(ProxyGlobalState::<TestTypes>::new(
            Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
                bootstrap_sender,
                tx_sender,
                parent_commit,
                ViewNumber::new(0),
                ViewNumber::new(0),
                10,
            ))),
            (builder_public_key, builder_private_key.clone()),
            Duration::from_secs(1),
        ));

        let commitment = BuilderCommitment::from_bytes([0; 256]);
        let cloned_commitment = commitment.clone();
        let cloned_state = state.clone();

        let vid_sender = {
            let mut global_state_write_lock = state.global_state.write_arc().await;
            let block_id = BlockId {
                hash: commitment,
                view: ViewNumber::new(1),
            };

            let payload = TestBlockPayload {
                transactions: vec![TestTransaction::new(vec![1, 2, 3, 4])],
            };

            let (vid_trigger_sender, _) = async_compatibility_layer::channel::oneshot();
            let (vid_sender, vid_receiver) = unbounded();

            global_state_write_lock.blocks.put(
                block_id,
                BlockInfo {
                    block_payload: payload,
                    metadata: TestMetadata,
                    vid_trigger: Arc::new(async_lock::RwLock::new(Some(vid_trigger_sender))),
                    vid_receiver: Arc::new(async_lock::RwLock::new(crate::WaitAndKeep::Wait(
                        vid_receiver,
                    ))),
                    offered_fee: 100,
                    truncated: false,
                },
            );

            vid_sender
        };

        let claim_block_header_input_join_handle = async_std::task::spawn(async move {
            let signature =
                BLSPubKey::sign(&leader_private_key, cloned_commitment.as_ref()).unwrap();
            cloned_state
                .claim_block_header_input_implementation(
                    &cloned_commitment,
                    1,
                    leader_public_key,
                    &signature,
                )
                .await
        });

        vid_sender
            .send(precompute_vid_commitment(&[1, 2, 3, 4], 2))
            .await
            .unwrap();

        let result = claim_block_header_input_join_handle.await;

        match result {
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
            Ok(_) => {
                // This is expected.
            }
        }
    }

    // handle_da_event Tests

    /// This test checks that the error [HandleDaEventError::SignatureValidationFailed]
    /// is returned under the right conditions of invoking
    /// [handle_da_event_implementation].
    ///
    /// To trigger this error, we simply need to ensure that the public keys
    /// provided for the leader and the sender do not match.
    #[async_std::test]
    async fn test_handle_da_event_implementation_error_sender_is_not_leader() {
        let (sender_public_key, sender_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, _) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let (da_channel_sender, _) = async_broadcast::broadcast(10);
        let total_nodes = NonZeroUsize::new(10).unwrap();
        let view_number = ViewNumber::new(10);

        let da_proposal = DaProposal::<TestTypes> {
            encoded_transactions: Arc::new([1, 2, 3, 4, 5, 6]),
            metadata: TestMetadata,
            view_number,
        };

        let encoded_txns_hash = Sha256::digest(&da_proposal.encoded_transactions);
        let signature =
            <BLSPubKey as SignatureKey>::sign(&sender_private_key, &encoded_txns_hash).unwrap();

        let signed_da_proposal = Arc::new(Proposal {
            data: da_proposal,
            signature,
            _pd: Default::default(),
        });

        let result = handle_da_event_implementation(
            &da_channel_sender,
            signed_da_proposal.clone(),
            sender_public_key,
            leader_public_key,
            total_nodes,
        )
        .await;

        match result {
            Err(HandleDaEventError::SenderIsNotLeader) => {
                // This is expected.
            }
            Ok(_) => {
                panic!("expected an error, but received a successful attempt instead")
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    /// This test checks that the error [HandleDaEventError::SignatureValidationFailed]
    /// is returned under the right conditions of invoking
    /// [handle_da_event_implementation].
    ///
    /// To trigger this error, we simply need to ensure that signature provided
    /// to the [Proposal] does not match the public key of the sender.
    /// Additionally, the public keys passed for both the leader and the sender
    /// need to match each other.
    #[async_std::test]
    async fn test_handle_da_event_implementation_error_signature_validation_failed() {
        let (sender_public_key, _) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (_, leader_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let (da_channel_sender, _) = async_broadcast::broadcast(10);
        let total_nodes = NonZeroUsize::new(10).unwrap();
        let view_number = ViewNumber::new(10);

        let da_proposal = DaProposal::<TestTypes> {
            encoded_transactions: Arc::new([1, 2, 3, 4, 5, 6]),
            metadata: TestMetadata,
            view_number,
        };

        let encoded_txns_hash = Sha256::digest(&da_proposal.encoded_transactions);
        let signature =
            <BLSPubKey as SignatureKey>::sign(&leader_private_key, &encoded_txns_hash).unwrap();

        let signed_da_proposal = Arc::new(Proposal {
            data: da_proposal,
            signature,
            _pd: Default::default(),
        });

        let result = handle_da_event_implementation(
            &da_channel_sender,
            signed_da_proposal.clone(),
            sender_public_key,
            sender_public_key,
            total_nodes,
        )
        .await;

        match result {
            Err(HandleDaEventError::SignatureValidationFailed) => {
                // This is expected.
            }
            Ok(_) => {
                panic!("expected an error, but received a successful attempt instead")
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    /// This test checks that the error [HandleDaEventError::BroadcastFailed]
    /// is returned under the right conditions of invoking
    /// [handle_da_event_implementation].
    ///
    /// To trigger this error, we simply need to ensure that the broadcast
    /// channel receiver has been closed / dropped before the attempt to
    /// send on the broadcast sender is performed.
    #[async_std::test]
    async fn test_handle_da_event_implementation_error_broadcast_failed() {
        let (sender_public_key, sender_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, _) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let da_channel_sender = {
            let (da_channel_sender, _) = async_broadcast::broadcast(10);
            da_channel_sender
        };

        let total_nodes = NonZeroUsize::new(10).unwrap();
        let view_number = ViewNumber::new(10);

        let da_proposal = DaProposal::<TestTypes> {
            encoded_transactions: Arc::new([1, 2, 3, 4, 5, 6]),
            metadata: TestMetadata,
            view_number,
        };

        let encoded_txns_hash = Sha256::digest(&da_proposal.encoded_transactions);
        let signature =
            <BLSPubKey as SignatureKey>::sign(&sender_private_key, &encoded_txns_hash).unwrap();

        let signed_da_proposal = Arc::new(Proposal {
            data: da_proposal,
            signature,
            _pd: Default::default(),
        });

        let result = handle_da_event_implementation(
            &da_channel_sender,
            signed_da_proposal.clone(),
            sender_public_key,
            leader_public_key,
            total_nodes,
        )
        .await;

        match result {
            Err(HandleDaEventError::BroadcastFailed(_)) => {
                // This error is expected
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    /// This test checks the expected successful behavior of the
    /// [handle_da_event_implementation] function.
    #[async_std::test]
    async fn test_handle_da_event_implementation_success() {
        let (sender_public_key, sender_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, _) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (da_channel_sender, da_channel_receiver) = async_broadcast::broadcast(10);
        let total_nodes = NonZeroUsize::new(10).unwrap();
        let view_number = ViewNumber::new(10);

        let da_proposal = DaProposal::<TestTypes> {
            encoded_transactions: Arc::new([1, 2, 3, 4, 5, 6]),
            metadata: TestMetadata,
            view_number,
        };

        let encoded_txns_hash = Sha256::digest(&da_proposal.encoded_transactions);
        let signature =
            <BLSPubKey as SignatureKey>::sign(&sender_private_key, &encoded_txns_hash).unwrap();

        let signed_da_proposal = Arc::new(Proposal {
            data: da_proposal,
            signature,
            _pd: Default::default(),
        });

        let result = handle_da_event_implementation(
            &da_channel_sender,
            signed_da_proposal.clone(),
            sender_public_key,
            leader_public_key,
            total_nodes,
        )
        .await;

        match result {
            Ok(_) => {
                // This is expected.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }

        let mut da_channel_receiver = da_channel_receiver;
        match da_channel_receiver.next().await {
            Some(MessageType::DaProposalMessage(da_proposal_message)) => {
                assert_eq!(da_proposal_message.proposal, signed_da_proposal);
            }
            _ => {
                panic!("Expected a DaProposalMessage, but got something else");
            }
        }
    }

    // handle_qc_event Tests

    /// This test checks that the error [HandleQcEventError::SenderIsNotLeader]
    /// is returned under the right conditions of invoking
    /// [handle_qc_event_implementation].
    ///
    /// To trigger this error, we simply need to ensure that the public keys
    /// provided for the leader and the sender do not match.
    #[async_std::test]
    async fn test_handle_qc_event_error_sender_is_not_leader() {
        let (sender_public_key, sender_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, _) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let (qc_channel_sender, _) = async_broadcast::broadcast(10);
        let view_number = ViewNumber::new(10);

        let qc_proposal = {
            let leaf = Leaf::<TestTypes>::genesis(
                &TestValidatedState::default(),
                &TestInstanceState::default(),
            )
            .await;

            QuorumProposal::<TestTypes> {
                block_header: leaf.block_header().clone(),
                view_number,
                justify_qc: QuorumCertificate::genesis(
                    &TestValidatedState::default(),
                    &TestInstanceState::default(),
                )
                .await,
                upgrade_certificate: None,
                proposal_certificate: None,
            }
        };

        let leaf = Leaf::from_quorum_proposal(&qc_proposal);

        let signature =
            <BLSPubKey as SignatureKey>::sign(&sender_private_key, leaf.commit().as_ref()).unwrap();

        let signed_qc_proposal = Arc::new(Proposal {
            data: qc_proposal,
            signature,
            _pd: Default::default(),
        });

        let result = handle_qc_event_implementation(
            &qc_channel_sender,
            signed_qc_proposal.clone(),
            sender_public_key,
            leader_public_key,
        )
        .await;

        match result {
            Err(HandleQcEventError::SenderIsNotLeader) => {
                // This is expected.
            }
            Ok(_) => {
                panic!("expected an error, but received a successful attempt instead");
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    /// This test checks that the error [HandleQcEventError::SignatureValidationFailed]
    /// is returned under the right conditions of invoking
    /// [handle_qc_event_implementation].
    ///
    /// To trigger this error, we simply need to ensure that the signature
    /// provided to the [Proposal] does not match the public key of the sender.
    ///
    /// Additionally, the public keys passed for both the leader and the sender
    /// need to match each other.
    #[async_std::test]
    async fn test_handle_qc_event_error_signature_validation_failed() {
        let (sender_public_key, _) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (_, leader_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 1);
        let (qc_channel_sender, _) = async_broadcast::broadcast(10);
        let view_number = ViewNumber::new(10);

        let qc_proposal = {
            let leaf = Leaf::<TestTypes>::genesis(
                &TestValidatedState::default(),
                &TestInstanceState::default(),
            )
            .await;

            QuorumProposal::<TestTypes> {
                block_header: leaf.block_header().clone(),
                view_number,
                justify_qc: QuorumCertificate::genesis(
                    &TestValidatedState::default(),
                    &TestInstanceState::default(),
                )
                .await,
                upgrade_certificate: None,
                proposal_certificate: None,
            }
        };

        let leaf = Leaf::from_quorum_proposal(&qc_proposal);

        let signature =
            <BLSPubKey as SignatureKey>::sign(&leader_private_key, leaf.commit().as_ref()).unwrap();

        let signed_qc_proposal = Arc::new(Proposal {
            data: qc_proposal,
            signature,
            _pd: Default::default(),
        });

        let result = handle_qc_event_implementation(
            &qc_channel_sender,
            signed_qc_proposal.clone(),
            sender_public_key,
            sender_public_key,
        )
        .await;

        match result {
            Err(HandleQcEventError::SignatureValidationFailed) => {
                // This is expected.
            }
            Ok(_) => {
                panic!("expected an error, but received a successful attempt instead");
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    /// This test checks that the error [HandleQcEventError::BroadcastFailed]
    /// is returned under the right conditions of invoking
    /// [handle_qc_event_implementation].
    ///
    /// To trigger this error, we simply need to ensure that the broadcast
    /// channel receiver has been closed / dropped before the attempt to
    /// send on the broadcast sender is performed.
    #[async_std::test]
    async fn test_handle_qc_event_error_broadcast_failed() {
        let (sender_public_key, sender_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, _) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let qc_channel_sender = {
            let (qc_channel_sender, _) = async_broadcast::broadcast(10);
            qc_channel_sender
        };

        let view_number = ViewNumber::new(10);

        let qc_proposal = {
            let leaf = Leaf::<TestTypes>::genesis(
                &TestValidatedState::default(),
                &TestInstanceState::default(),
            )
            .await;

            QuorumProposal::<TestTypes> {
                block_header: leaf.block_header().clone(),
                view_number,
                justify_qc: QuorumCertificate::genesis(
                    &TestValidatedState::default(),
                    &TestInstanceState::default(),
                )
                .await,
                upgrade_certificate: None,
                proposal_certificate: None,
            }
        };

        let leaf = Leaf::from_quorum_proposal(&qc_proposal);

        let signature =
            <BLSPubKey as SignatureKey>::sign(&sender_private_key, leaf.commit().as_ref()).unwrap();

        let signed_qc_proposal = Arc::new(Proposal {
            data: qc_proposal,
            signature,
            _pd: Default::default(),
        });

        let result = handle_qc_event_implementation(
            &qc_channel_sender,
            signed_qc_proposal.clone(),
            sender_public_key,
            leader_public_key,
        )
        .await;

        match result {
            Err(HandleQcEventError::BroadcastFailed(_)) => {
                // This is expected.
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    /// This test checks to ensure that [handle_qc_event_implementation]
    /// completes successfully as expected when the correct conditions are met.
    #[async_std::test]
    async fn test_handle_qc_event_success() {
        let (sender_public_key, sender_private_key) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (leader_public_key, _) =
            <BLSPubKey as BuilderSignatureKey>::generated_from_seed_indexed([0; 32], 0);
        let (qc_channel_sender, qc_channel_receiver) = async_broadcast::broadcast(10);
        let view_number = ViewNumber::new(10);

        let qc_proposal = {
            let leaf = Leaf::<TestTypes>::genesis(
                &TestValidatedState::default(),
                &TestInstanceState::default(),
            )
            .await;

            QuorumProposal::<TestTypes> {
                block_header: leaf.block_header().clone(),
                view_number,
                justify_qc: QuorumCertificate::genesis(
                    &TestValidatedState::default(),
                    &TestInstanceState::default(),
                )
                .await,
                upgrade_certificate: None,
                proposal_certificate: None,
            }
        };

        let leaf = Leaf::from_quorum_proposal(&qc_proposal);

        let signature =
            <BLSPubKey as SignatureKey>::sign(&sender_private_key, leaf.commit().as_ref()).unwrap();

        let signed_qc_proposal = Arc::new(Proposal {
            data: qc_proposal,
            signature,
            _pd: Default::default(),
        });

        let result = handle_qc_event_implementation(
            &qc_channel_sender,
            signed_qc_proposal.clone(),
            sender_public_key,
            leader_public_key,
        )
        .await;

        match result {
            Ok(_) => {
                // This is expected.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }

        let mut qc_channel_receiver = qc_channel_receiver;
        match qc_channel_receiver.next().await {
            Some(MessageType::QCMessage(da_proposal_message)) => {
                assert_eq!(da_proposal_message.proposal, signed_qc_proposal);
            }
            _ => {
                panic!("Expected a QCMessage, but got something else");
            }
        }
    }

    // HandleReceivedTxns Tests

    /// This test checks that the error [HandleReceivedTxnsError::TooManyTransactions]
    /// is returned when the conditions are met.
    ///
    /// To trigger this error we simply provide a broadcast channel with a
    /// buffer smaller than the number of transactions we are attempting to
    /// send through it.
    #[async_std::test]
    async fn test_handle_received_txns_error_too_many_transactions() {
        let (tx_sender, tx_receiver) = async_broadcast::broadcast(2);
        let num_transactions = 5;
        let mut txns = Vec::with_capacity(num_transactions);
        for index in 0..num_transactions {
            txns.push(TestTransaction::new(vec![index as u8]));
        }
        let txns = txns;

        {
            let mut handle_received_txns_iter = HandleReceivedTxns::<TestTypes>::new(
                tx_sender,
                txns.clone(),
                TransactionSource::HotShot,
                10,
            );

            assert!(handle_received_txns_iter.next().is_some());
            assert!(handle_received_txns_iter.next().is_some());
            match handle_received_txns_iter.next() {
                Some(Err(HandleReceivedTxnsError::TooManyTransactions)) => {
                    // This is expected,
                }
                Some(Err(err)) => {
                    panic!("Unexpected error: {:?}", err);
                }
                Some(Ok(_)) => {
                    panic!("Expected an error, but got a result");
                }
                None => {
                    panic!("Expected an error, but got a result");
                }
            }
        }

        let mut tx_receiver = tx_receiver;
        assert!(tx_receiver.next().await.is_some());
        assert!(tx_receiver.next().await.is_some());
        assert!(tx_receiver.next().await.is_none());
    }

    /// This test checks that the error [HandleReceivedTxnsError::TransactionTooBig]
    /// when the conditions are met.
    ///
    /// To trigger this error we simply provide a [TestTransaction] whose size
    /// exceeds the maximum transaction length. we pass to [HandleReceivedTxns].
    #[async_std::test]
    async fn test_handle_received_txns_error_transaction_too_big() {
        let (tx_sender, tx_receiver) = async_broadcast::broadcast(10);
        let num_transactions = 2;
        let mut txns = Vec::with_capacity(num_transactions + 1);
        for index in 0..num_transactions {
            txns.push(TestTransaction::new(vec![index as u8]));
        }
        txns.push(TestTransaction::new(vec![0; 256]));
        let txns = txns;

        {
            let mut handle_received_txns_iter = HandleReceivedTxns::<TestTypes>::new(
                tx_sender,
                txns.clone(),
                TransactionSource::HotShot,
                10,
            );

            assert!(handle_received_txns_iter.next().is_some());
            assert!(handle_received_txns_iter.next().is_some());
            match handle_received_txns_iter.next() {
                Some(Err(HandleReceivedTxnsError::TransactionTooBig {
                    estimated_length,
                    max_txn_len,
                })) => {
                    // This is expected,
                    assert!(estimated_length >= 256);
                    assert_eq!(max_txn_len, 10);
                }
                Some(Err(err)) => {
                    panic!("Unexpected error: {:?}", err);
                }
                Some(Ok(_)) => {
                    panic!("Expected an error, but got a result");
                }
                None => {
                    panic!("Expected an error, but got a result");
                }
            }
        }

        let mut tx_receiver = tx_receiver;
        assert!(tx_receiver.next().await.is_some());
        assert!(tx_receiver.next().await.is_some());
        assert!(tx_receiver.next().await.is_none());
    }

    /// This test checks that the error [HandleReceivedTxnsError::Internal]
    /// is returned when the broadcast channel is closed.
    ///
    /// To trigger this error we simply close the broadcast channel receiver
    /// before attempting to send any transactions through the broadcast channel
    /// sender.
    #[async_std::test]
    async fn test_handle_received_txns_error_internal() {
        let tx_sender = {
            let (tx_sender, _) = async_broadcast::broadcast(10);
            tx_sender
        };

        let num_transactions = 10;
        let mut txns = Vec::with_capacity(num_transactions);
        for index in 0..num_transactions {
            txns.push(TestTransaction::new(vec![index as u8]));
        }
        txns.push(TestTransaction::new(vec![0; 256]));
        let txns = txns;

        {
            let mut handle_received_txns_iter = HandleReceivedTxns::<TestTypes>::new(
                tx_sender,
                txns.clone(),
                TransactionSource::HotShot,
                10,
            );

            match handle_received_txns_iter.next() {
                Some(Err(HandleReceivedTxnsError::Internal(err))) => {
                    // This is expected,

                    match err {
                        async_broadcast::TrySendError::Closed(_) => {
                            // This is expected.
                        }
                        _ => {
                            panic!("Unexpected error: {:?}", err);
                        }
                    }
                }
                Some(Err(err)) => {
                    panic!("Unexpected error: {:?}", err);
                }
                Some(Ok(_)) => {
                    panic!("Expected an error, but got a result");
                }
                None => {
                    panic!("Expected an error, but got a result");
                }
            }
        }
    }

    /// This test checks that [HandleReceivedTxns] processes completely without
    /// issue when the conditions are correct for it to do so.
    #[async_std::test]
    async fn test_handle_received_txns_success() {
        let (tx_sender, tx_receiver) = async_broadcast::broadcast(10);
        let num_transactions = 10;
        let mut txns = Vec::with_capacity(num_transactions);
        for index in 0..num_transactions {
            txns.push(TestTransaction::new(vec![index as u8]));
        }
        let txns = txns;

        let handle_received_txns_iter = HandleReceivedTxns::<TestTypes>::new(
            tx_sender,
            txns.clone(),
            TransactionSource::HotShot,
            10,
        );

        for iteration in handle_received_txns_iter {
            match iteration {
                Ok(_) => {
                    // This is expected.
                }
                Err(err) => {
                    panic!("Unexpected error: {:?}", err);
                }
            }
        }

        let mut tx_receiver = tx_receiver;
        for tx in txns {
            match tx_receiver.next().await {
                Some(received_txn) => {
                    assert_eq!(received_txn.tx, tx);
                }
                _ => {
                    panic!("Expected a TransactionMessage, but got something else");
                }
            }
        }
    }

    /// This test checks that the error [ConnectToEventsServiceError::Connection]
    /// is returned when [connect_to_events_service] is invoked.
    ///
    /// This error may be difficult to control in all environments. In order
    /// to control this specific case, the underlying client being passed to
    /// [connect_to_events_service] is a mock that always returns false when
    /// [check_connection] is invoked.
    #[async_std::test]
    async fn test_connect_to_events_service_mock_error_connection() {
        struct ConnectionFailure;

        #[async_trait::async_trait]
        impl HotShotEventsService<TestTypes> for ConnectionFailure {
            type EventsStream =
                Receiver<Result<Event<TestTypes>, hotshot_events_service::events::Error>>;
            type EventsError = hotshot_events_service::events::Error;
            type StartUpInfo = StartupInfo<TestTypes>;
            type StartUpInfoError = hotshot_events_service::events::Error;

            async fn check_connection(&self, _timeout: Option<Duration>) -> bool {
                false
            }

            async fn events(&self) -> Result<Self::EventsStream, Self::EventsError> {
                todo!()
            }

            async fn startup_info(&self) -> Result<Self::StartUpInfo, Self::StartUpInfoError> {
                todo!()
            }
        }

        let client = ConnectionFailure;

        match connect_to_events_service(&client).await {
            Err(ConnectToEventsServiceError::Connection) => {
                // This is expected.
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    /// This test checks that the error [ConnectToEventsServiceError::Subscription]
    /// is returned when [connect_to_events_service] is invoked.
    ///
    /// This error may be difficult to control in all environments. In order
    /// to control this specific case, the underlying client being passed to
    /// [connect_to_events_service] is a mock that always returns an error when
    /// [events] is invoked.
    #[async_std::test]
    async fn test_connect_to_events_service_mock_error_events() {
        struct EventsFailure;

        #[async_trait::async_trait]
        impl HotShotEventsService<TestTypes> for EventsFailure {
            type EventsStream =
                Receiver<Result<Event<TestTypes>, hotshot_events_service::events::Error>>;
            type EventsError = hotshot_events_service::events::Error;
            type StartUpInfo = StartupInfo<TestTypes>;
            type StartUpInfoError = hotshot_events_service::events::Error;

            async fn check_connection(&self, _timeout: Option<Duration>) -> bool {
                true
            }

            async fn events(&self) -> Result<Self::EventsStream, Self::EventsError> {
                Err(hotshot_events_service::events::Error::Custom {
                    message: "This is a custom error".to_string(),
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                })
            }

            async fn startup_info(&self) -> Result<Self::StartUpInfo, Self::StartUpInfoError> {
                todo!()
            }
        }

        let client = EventsFailure;

        match connect_to_events_service(&client).await {
            Err(ConnectToEventsServiceError::Subscription(
                hotshot_events_service::events::Error::Custom { .. },
            )) => {
                // This is expected.
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    /// This test checks that the error [ConnectToEventsServiceError::StartupInfo]
    /// is returned when [connect_to_events_service] is invoked.
    ///
    /// This error may be difficult to control in all environments. In order
    /// to control this specific case, the underlying client being passed to
    /// [connect_to_events_service] is a mock that always returns an error when
    /// [startup_info] is invoked.
    #[async_std::test]
    async fn test_connect_to_service_mock_error_startup_info() {
        type EventsStream =
            Receiver<Result<Event<TestTypes>, hotshot_events_service::events::Error>>;
        struct StartupInfoFailure(Arc<RwLock<Option<EventsStream>>>);

        #[async_trait::async_trait]
        impl HotShotEventsService<TestTypes> for StartupInfoFailure {
            type EventsStream = EventsStream;
            type EventsError = hotshot_events_service::events::Error;
            type StartUpInfo = StartupInfo<TestTypes>;
            type StartUpInfoError = hotshot_events_service::events::Error;

            async fn check_connection(&self, _timeout: Option<Duration>) -> bool {
                true
            }

            async fn events(&self) -> Result<Self::EventsStream, Self::EventsError> {
                let mut write_lock_guard = self.0.write_arc().await;

                write_lock_guard
                    .take()
                    .ok_or(hotshot_events_service::events::Error::Custom {
                        message: "This is a custom error".to_string(),
                        status: StatusCode::INTERNAL_SERVER_ERROR,
                    })
            }

            async fn startup_info(&self) -> Result<Self::StartUpInfo, Self::StartUpInfoError> {
                Err(hotshot_events_service::events::Error::Custom {
                    message: "This is a custom error".to_string(),
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                })
            }
        }

        let client = StartupInfoFailure(Arc::new(RwLock::new(Some(channel(10).1))));

        match connect_to_events_service(&client).await {
            Err(ConnectToEventsServiceError::StartupInfo(
                hotshot_events_service::events::Error::Custom { .. },
            )) => {
                // This is expected.
            }
            Ok(_) => {
                panic!("Expected an error, but got a result");
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    /// This test checks that [connect_to_events_service] completes
    /// successfully.
    #[async_std::test]
    async fn test_connect_to_service_mock_success() {
        type EventsStream =
            Receiver<Result<Event<TestTypes>, hotshot_events_service::events::Error>>;
        struct Success(Arc<RwLock<Option<EventsStream>>>);

        #[async_trait::async_trait]
        impl HotShotEventsService<TestTypes> for Success {
            type EventsStream = EventsStream;
            type EventsError = hotshot_events_service::events::Error;
            type StartUpInfo = StartupInfo<TestTypes>;
            type StartUpInfoError = hotshot_events_service::events::Error;

            async fn check_connection(&self, _timeout: Option<Duration>) -> bool {
                true
            }

            async fn events(&self) -> Result<Self::EventsStream, Self::EventsError> {
                let mut write_lock_guard = self.0.write_arc().await;

                write_lock_guard
                    .take()
                    .ok_or(hotshot_events_service::events::Error::Custom {
                        message: "This is a custom error".to_string(),
                        status: StatusCode::INTERNAL_SERVER_ERROR,
                    })
            }

            async fn startup_info(&self) -> Result<Self::StartUpInfo, Self::StartUpInfoError> {
                Ok(StartupInfo {
                    known_node_with_stake: vec![],
                    non_staked_node_count: 0,
                })
            }
        }

        let client = Success(Arc::new(RwLock::new(Some(channel(10).1))));

        match connect_to_events_service(&client).await {
            Ok(_) => {
                // This is expected.
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }
}
