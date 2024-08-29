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
use anyhow::{anyhow, Context};
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
use futures::future::BoxFuture;
use futures::stream::StreamExt;
use hotshot_events_service::{events::Error as EventStreamError, events_source::StartupInfo};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;
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

// It holds the information for the proposed block
#[derive(Debug)]
pub struct ProposedBlockId<Types: NodeType> {
    pub parent_commitment: VidCommitment,
    pub payload_commitment: BuilderCommitment,
    pub parent_view: Types::Time,
}

impl<Types: NodeType> ProposedBlockId<Types> {
    pub fn new(
        parent_commitment: VidCommitment,
        payload_commitment: BuilderCommitment,
        parent_view: Types::Time,
    ) -> Self {
        ProposedBlockId {
            parent_commitment,
            payload_commitment,
            parent_view,
        }
    }
}

#[derive(Debug, Derivative)]
#[derivative(Default(bound = ""))]
pub struct BuilderStatesInfo<Types: NodeType> {
    // list of all the builder states spawned for a view
    pub vid_commitments: Vec<VidCommitment>,
    // list of all the proposed blocks for a view
    pub block_ids: Vec<ProposedBlockId<Types>>,
}

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

    pub fn register_builder_state(
        &mut self,
        parent_id: BuilderStateId<Types>,
        request_sender: BroadcastSender<MessageType<Types>>,
    ) {
        // register the builder state
        self.spawned_builder_states
            .insert(parent_id.clone(), request_sender);

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

    pub fn update_global_state(
        &mut self,
        state_id: BuilderStateId<Types>,
        build_block_info: BuildBlockInfo<Types>,
        response_msg: ResponseMessage,
    ) {
        if self.blocks.contains(&build_block_info.id) {
            self.blocks.promote(&build_block_info.id)
        } else {
            self.blocks.push(
                build_block_info.id,
                BlockInfo {
                    block_payload: build_block_info.block_payload,
                    metadata: build_block_info.metadata,
                    vid_trigger: Arc::new(RwLock::new(Some(build_block_info.vid_trigger))),
                    vid_receiver: Arc::new(RwLock::new(WaitAndKeep::Wait(
                        build_block_info.vid_receiver,
                    ))),
                    offered_fee: build_block_info.offered_fee,
                    truncated: build_block_info.truncated,
                },
            );
        }

        // update the builder state to last built block
        self.builder_state_to_last_built_block
            .insert(state_id, response_msg);
    }

    // remove the builder state handles based on the decide event
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

    /// get_channel_for_matching_builder_or_highest_view_builder is a helper
    /// function that attempts to retrieve the broadcast sender for the given
    /// `BuilderStateId`. If the sender does not exist, it will return the
    /// broadcast sender for the for the hightest view number `BuilderStateId``
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
        // };

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
            // and the met data data
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

            if response_received.is_ok() {
                let (vid_commitment, vid_precompute_data) = response_received?;

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
            } else {
                tracing::warn!("Claim Block Header Input not found");
                Err(ClaimBlockHeaderInputError::BlockHeaderNotFound)
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

async fn connect_to_events_service<Types: NodeType, V: Versions>(
    hotshot_events_api_url: Url,
) -> Option<(
    surf_disco::socket::Connection<
        Event<Types>,
        surf_disco::socket::Unsupported,
        EventStreamError,
        V::Base,
    >,
    GeneralStaticCommittee<Types, <Types as NodeType>::SignatureKey>,
)> {
    let client = surf_disco::Client::<hotshot_events_service::events::Error, V::Base>::new(
        hotshot_events_api_url.clone(),
    );

    if !(client.connect(None).await) {
        return None;
    }

    tracing::info!("Builder client connected to the hotshot events api");

    // client subscrive to hotshot events
    let subscribed_events = client
        .socket("hotshot-events/events")
        .subscribe::<Event<Types>>()
        .await
        .ok()?;

    // handle the startup event at the start
    let membership = if let Ok(response) = client
        .get::<StartupInfo<Types>>("hotshot-events/startup_info")
        .send()
        .await
    {
        let StartupInfo {
            known_node_with_stake,
            non_staked_node_count,
        } = response;
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
        Some(membership)
    } else {
        None
    };
    membership.map(|membership| (subscribed_events, membership))
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
    let connected = connect_to_events_service::<_, V>(hotshot_events_api_url.clone()).await;
    if connected.is_none() {
        return Err(anyhow!(
            "failed to connect to API at {hotshot_events_api_url}"
        ));
    }
    let (mut subscribed_events, mut membership) =
        connected.context("Failed to connect to events service")?;

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
                let connected =
                    connect_to_events_service::<_, V>(hotshot_events_api_url.clone()).await;
                if connected.is_none() {
                    return Err(anyhow!(
                        "failed to reconnect to API at {hotshot_events_api_url}"
                    ));
                }
                (subscribed_events, membership) =
                    connected.context("Failed to reconnect to events service")?;
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

/*
Utility functions to handle the hotshot events
*/
async fn handle_da_event<Types: NodeType>(
    da_channel_sender: &BroadcastSender<MessageType<Types>>,
    da_proposal: Arc<Proposal<Types, DaProposal<Types>>>,
    sender: <Types as NodeType>::SignatureKey,
    leader: <Types as NodeType>::SignatureKey,
    total_nodes: NonZeroUsize,
) {
    tracing::debug!(
        "DaProposal: Leader: {:?} for the view: {:?}",
        leader,
        da_proposal.data.view_number
    );

    // get the encoded transactions hash
    let encoded_txns_hash = Sha256::digest(&da_proposal.data.encoded_transactions);
    // check if the sender is the leader and the signature is valid; if yes, broadcast the DA proposal
    if leader == sender && sender.validate(&da_proposal.signature, &encoded_txns_hash) {
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
        }
    } else {
        tracing::error!("Validation Failure on DaProposal for view {:?}: Leader for the current view: {:?} and sender: {:?}", da_proposal.data.view_number, leader, sender);
    }
}

async fn handle_qc_event<Types: NodeType>(
    qc_channel_sender: &BroadcastSender<MessageType<Types>>,
    qc_proposal: Arc<Proposal<Types, QuorumProposal<Types>>>,
    sender: <Types as NodeType>::SignatureKey,
    leader: <Types as NodeType>::SignatureKey,
) {
    tracing::debug!(
        "QCProposal: Leader: {:?} for the view: {:?}",
        leader,
        qc_proposal.data.view_number
    );

    let leaf = Leaf::from_quorum_proposal(&qc_proposal.data);

    // check if the sender is the leader and the signature is valid; if yes, broadcast the QC proposal
    if sender == leader && sender.validate(&qc_proposal.signature, leaf.commit().as_ref()) {
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
        }
    } else {
        tracing::error!("Validation Failure on QCProposal for view {:?}: Leader for the current view: {:?} and sender: {:?}", qc_proposal.data.view_number, leader, sender);
    }
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

pub(crate) async fn handle_received_txns<Types: NodeType>(
    tx_sender: &BroadcastSender<Arc<ReceivedTransaction<Types>>>,
    txns: Vec<Types::Transaction>,
    source: TransactionSource,
    max_txn_len: u64,
) -> Vec<Result<Commitment<<Types as NodeType>::Transaction>, BuildError>> {
    let mut results = Vec::with_capacity(txns.len());
    let time_in = Instant::now();
    for tx in txns.into_iter() {
        let commit = tx.commit();
        // This is a rough estimate, but we don't have any other way to get real
        // encoded transaction length. Luckily, this being roughly proportional
        // to encoded length is enough, because we only use this value to estimate
        // our limitations on computing the VID in time.
        let len = bincode::serialized_size(&tx).unwrap_or_default();
        if len > max_txn_len {
            tracing::warn!(%commit, %len, %max_txn_len, "Transaction too big");
            results.push(Err(BuildError::Error {
                message: format!("Transaction {commit} too big (estimated length {len}, currently accepting <= {max_txn_len})"),
            }));
            continue;
        }
        let res = tx_sender
            .try_broadcast(Arc::new(ReceivedTransaction {
                tx,
                source: source.clone(),
                commit,
                time_in,
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
            .map_err(|err| match err {
                TrySendError::Full(_) => BuildError::Error {
                    message: "Too many transactions".to_owned(),
                },
                e => BuildError::Error {
                    message: format!("Internal error when submitting transaction: {}", e),
                },
            });
        results.push(res);
    }
    results
}

#[cfg(test)]
mod test {
    use std::{sync::Arc, time::Duration};

    use async_lock::RwLock;
    use futures::StreamExt;
    use hotshot::types::{BLSPubKey, SignatureKey};
    use hotshot_builder_api::v0_2::block_info::AvailableBlockInfo;
    use hotshot_example_types::node_types::TestTypes;
    use hotshot_types::{
        data::ViewNumber,
        traits::{
            block_contents::vid_commitment, node_implementation::ConsensusTime,
            signature_key::BuilderSignatureKey,
        },
        utils::BuilderCommitment,
    };

    use crate::{
        builder_state::{MessageType, ResponseMessage},
        BuilderStateId,
    };

    use super::{AvailableBlocksError, GlobalState, ProxyGlobalState};

    // Get Available Blocks Tests

    /// This test checks that the error `AvailableBlocksError::NoBlocksAvailable`
    /// is returned when no blocks are available.
    ///
    /// To Trigger this condition, we simply submit a request to the
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
    async fn test_get_available_blocks_error_() {
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
    /// a successful response when the function is called before after are
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
}
