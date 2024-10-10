//! This module contains all the traits used for building the sequencer types.
//! It also includes some trait implementations that cannot be implemented in an external crate.
use std::{cmp::max, collections::BTreeMap, fmt::Debug, ops::Range, sync::Arc};

use anyhow::{bail, ensure, Context};
use async_trait::async_trait;
use committable::Commitment;
use dyn_clone::DynClone;
use futures::{FutureExt, TryFutureExt};
use hotshot::{types::EventType, HotShotInitializer};
use hotshot_types::{
    consensus::CommitmentMap,
    data::{DaProposal, QuorumProposal, VidDisperseShare, ViewNumber},
    event::{HotShotAction, LeafInfo},
    message::Proposal,
    simple_certificate::{QuorumCertificate, UpgradeCertificate},
    traits::{
        node_implementation::{ConsensusTime, Versions},
        storage::Storage,
        ValidatedState as HotShotState,
    },
    utils::View,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    v0::impls::ValidatedState, v0_3::ChainConfig, BackoffParams, BlockMerkleTree, Event,
    FeeAccount, FeeAccountProof, FeeMerkleCommitment, FeeMerkleTree, Leaf, NetworkConfig, SeqTypes,
};

use super::impls::NodeState;

#[async_trait]
pub trait StateCatchup: Send + Sync + std::fmt::Debug {
    /// Try to fetch the given accounts state, failing without retrying if unable.
    async fn try_fetch_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        account: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree>;

    /// Fetch the given list of accounts, retrying on transient errors.
    async fn fetch_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<FeeAccountProof>> {
        Ok(self
            .backoff()
            .retry(self, |provider| {
                async {
                    let tree = provider
                        .try_fetch_accounts(instance, height, view, fee_merkle_tree_root, &accounts)
                        .await
                        .map_err(|err| {
                            err.context(format!(
                                "fetching accounts {accounts:?}, height {height}, view {view:?}"
                            ))
                        })?;
                    accounts
                        .iter()
                        .map(|account| {
                            FeeAccountProof::prove(&tree, (*account).into())
                                .context(format!("missing account {account}"))
                                .map(|(proof, _)| proof)
                        })
                        .collect::<anyhow::Result<Vec<FeeAccountProof>>>()
                }
                .boxed()
            })
            .await)
    }

    /// Try to fetch and remember the blocks frontier, failing without retrying if unable.
    async fn try_remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()>;

    /// Fetch and remember the blocks frontier, retrying on transient errors.
    async fn remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        self.backoff()
            .retry(mt, |mt| {
                self.try_remember_blocks_merkle_tree(height, view, mt)
                    .map_err(|err| err.context("fetching frontier"))
                    .boxed()
            })
            .await;
        Ok(())
    }

    async fn try_fetch_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig>;

    async fn fetch_chain_config(&self, commitment: Commitment<ChainConfig>) -> ChainConfig {
        self.backoff()
            .retry(self, |provider| {
                provider
                    .try_fetch_chain_config(commitment)
                    .map_err(|err| err.context("fetching chain config"))
                    .boxed()
            })
            .await
    }

    fn backoff(&self) -> &BackoffParams;
}

#[async_trait]
impl<T: StateCatchup + ?Sized> StateCatchup for Box<T> {
    async fn try_fetch_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        (**self)
            .try_fetch_accounts(instance, height, view, fee_merkle_tree_root, accounts)
            .await
    }

    async fn fetch_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<FeeAccountProof>> {
        (**self)
            .fetch_accounts(instance, height, view, fee_merkle_tree_root, accounts)
            .await
    }

    async fn try_remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        (**self)
            .try_remember_blocks_merkle_tree(height, view, mt)
            .await
    }

    async fn remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        (**self).remember_blocks_merkle_tree(height, view, mt).await
    }

    async fn try_fetch_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        (**self).try_fetch_chain_config(commitment).await
    }

    async fn fetch_chain_config(&self, commitment: Commitment<ChainConfig>) -> ChainConfig {
        (**self).fetch_chain_config(commitment).await
    }

    fn backoff(&self) -> &BackoffParams {
        (**self).backoff()
    }
}

#[async_trait]
impl<T: StateCatchup + ?Sized> StateCatchup for Arc<T> {
    async fn try_fetch_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        (**self)
            .try_fetch_accounts(instance, height, view, fee_merkle_tree_root, accounts)
            .await
    }

    async fn fetch_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<FeeAccountProof>> {
        (**self)
            .fetch_accounts(instance, height, view, fee_merkle_tree_root, accounts)
            .await
    }

    async fn try_remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        (**self)
            .try_remember_blocks_merkle_tree(height, view, mt)
            .await
    }

    async fn remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        (**self).remember_blocks_merkle_tree(height, view, mt).await
    }

    async fn try_fetch_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        (**self).try_fetch_chain_config(commitment).await
    }

    async fn fetch_chain_config(&self, commitment: Commitment<ChainConfig>) -> ChainConfig {
        (**self).fetch_chain_config(commitment).await
    }

    fn backoff(&self) -> &BackoffParams {
        (**self).backoff()
    }
}

/// Catchup from multiple providers tries each provider in a round robin fashion until it succeeds.
#[async_trait]
impl<T: StateCatchup> StateCatchup for Vec<T> {
    #[tracing::instrument(skip(self, instance))]
    async fn try_fetch_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        for provider in self {
            match provider
                .try_fetch_accounts(instance, height, view, fee_merkle_tree_root, accounts)
                .await
            {
                Ok(tree) => return Ok(tree),
                Err(err) => {
                    tracing::warn!(?accounts, ?provider, "failed to fetch accounts: {err:#}");
                }
            }
        }

        bail!("could not fetch account from any provider");
    }

    #[tracing::instrument(skip(self, mt))]
    async fn try_remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        for provider in self {
            match provider
                .try_remember_blocks_merkle_tree(height, view, mt)
                .await
            {
                Ok(()) => return Ok(()),
                Err(err) => {
                    tracing::warn!(?provider, "failed to fetch frontier: {err:#}");
                }
            }
        }

        bail!("could not fetch account from any provider");
    }

    async fn try_fetch_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        for provider in self {
            match provider.try_fetch_chain_config(commitment).await {
                Ok(cf) => return Ok(cf),
                Err(err) => {
                    tracing::warn!(?provider, "failed to fetch chain config: {err:#}");
                }
            }
        }

        bail!("could not fetch chain config from any provider");
    }

    fn backoff(&self) -> &BackoffParams {
        // Use whichever provider's backoff is most conservative.
        self.iter()
            .map(|p| p.backoff())
            .max()
            .expect("provider list not empty")
    }
}

#[async_trait]
pub trait PersistenceOptions: Clone + Send + Sync + 'static {
    type Persistence: SequencerPersistence;

    async fn create(self) -> anyhow::Result<Self::Persistence>;
    async fn reset(self) -> anyhow::Result<()>;

    async fn create_catchup_provider(
        self,
        backoff: BackoffParams,
    ) -> anyhow::Result<Arc<dyn StateCatchup>> {
        self.create().await?.into_catchup_provider(backoff)
    }
}

#[async_trait]
pub trait SequencerPersistence: Sized + Send + Sync + 'static {
    /// Use this storage as a state catchup backend, if supported.
    fn into_catchup_provider(
        self,
        _backoff: BackoffParams,
    ) -> anyhow::Result<Arc<dyn StateCatchup>> {
        bail!("state catchup is not implemented for this persistence type");
    }

    /// Load the orchestrator config from storage.
    ///
    /// Returns `None` if no config exists (we are joining a network for the first time). Fails with
    /// `Err` if it could not be determined whether a config exists or not.
    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>>;

    /// Save the orchestrator config to storage.
    async fn save_config(&self, cfg: &NetworkConfig) -> anyhow::Result<()>;

    /// Load the highest view saved with [`save_voted_view`](Self::save_voted_view).
    async fn load_latest_acted_view(&self) -> anyhow::Result<Option<ViewNumber>>;

    /// Load undecided state saved by consensus before we shut down.
    async fn load_undecided_state(
        &self,
    ) -> anyhow::Result<Option<(CommitmentMap<Leaf>, BTreeMap<ViewNumber, View<SeqTypes>>)>>;

    /// Load the proposals saved by consensus
    async fn load_quorum_proposals(
        &self,
    ) -> anyhow::Result<BTreeMap<ViewNumber, Proposal<SeqTypes, QuorumProposal<SeqTypes>>>>;

    async fn load_vid_share(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>>;
    async fn load_da_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, DaProposal<SeqTypes>>>>;
    async fn load_upgrade_certificate(
        &self,
    ) -> anyhow::Result<Option<UpgradeCertificate<SeqTypes>>>;

    /// Load the latest known consensus state.
    ///
    /// Returns an initializer to resume HotShot from the latest saved state (or start from genesis,
    /// if there is no saved state). Also returns the anchor view number, which can be used as a
    /// reference point to process any events which were not processed before a previous shutdown,
    /// if applicable,.
    async fn load_consensus_state<V: Versions>(
        &self,
        state: NodeState,
    ) -> anyhow::Result<(HotShotInitializer<SeqTypes>, Option<ViewNumber>)> {
        let genesis_validated_state = ValidatedState::genesis(&state).0;
        let highest_voted_view = match self
            .load_latest_acted_view()
            .await
            .context("loading last voted view")?
        {
            Some(view) => {
                tracing::info!(?view, "starting from saved view");
                view
            }
            None => {
                tracing::info!("no saved view, starting from genesis");
                ViewNumber::genesis()
            }
        };
        let (leaf, high_qc, anchor_view) = match self
            .load_anchor_leaf()
            .await
            .context("loading anchor leaf")?
        {
            Some((leaf, high_qc)) => {
                tracing::info!(?leaf, ?high_qc, "starting from saved leaf");
                ensure!(
                    leaf.view_number() == high_qc.view_number,
                    format!(
                        "loaded anchor leaf from view {:?}, but high QC is from view {:?}",
                        leaf.view_number(),
                        high_qc.view_number
                    )
                );

                let anchor_view = leaf.view_number();
                (leaf, high_qc, Some(anchor_view))
            }
            None => {
                tracing::info!("no saved leaf, starting from genesis leaf");
                (
                    Leaf::genesis(&genesis_validated_state, &state).await,
                    QuorumCertificate::genesis::<V>(&genesis_validated_state, &state).await,
                    None,
                )
            }
        };
        let validated_state = if leaf.block_header().height() == 0 {
            // If we are starting from genesis, we can provide the full state.
            Some(Arc::new(genesis_validated_state))
        } else {
            // Otherwise, we will have to construct a sparse state and fetch missing data during
            // catchup.
            None
        };

        // If we are not starting from genesis, we start from the view following the maximum view
        // between `highest_voted_view` and `leaf.view_number`. This prevents double votes from
        // starting in a view in which we had already voted before the restart, and prevents
        // unnecessary catchup from starting in a view earlier than the anchor leaf.
        let view = max(highest_voted_view, leaf.view_number());

        let (undecided_leaves, undecided_state) = self
            .load_undecided_state()
            .await
            .context("loading undecided state")?
            .unwrap_or_default();

        let saved_proposals = self
            .load_quorum_proposals()
            .await
            .context("loading saved proposals")?;

        let upgrade_certificate = self
            .load_upgrade_certificate()
            .await
            .context("loading upgrade certificate")?;

        tracing::info!(
            ?leaf,
            ?view,
            ?high_qc,
            ?validated_state,
            ?undecided_leaves,
            ?undecided_state,
            ?saved_proposals,
            ?upgrade_certificate,
            "loaded consensus state"
        );

        Ok((
            HotShotInitializer::from_reload(
                leaf,
                state,
                validated_state,
                view,
                highest_voted_view,
                saved_proposals,
                high_qc,
                upgrade_certificate,
                undecided_leaves.into_values().collect(),
                undecided_state,
            ),
            anchor_view,
        ))
    }

    /// Update storage based on an event from consensus.
    async fn handle_event(&self, event: &Event, consumer: &(impl EventConsumer + 'static)) {
        if let EventType::Decide { leaf_chain, qc, .. } = &event.event {
            let Some(LeafInfo { leaf, .. }) = leaf_chain.first() else {
                // No new leaves.
                return;
            };

            // Associate each decided leaf with a QC.
            let chain = leaf_chain.iter().zip(
                // The first (most recent) leaf corresponds to the QC triggering the decide event.
                std::iter::once((**qc).clone())
                    // Moving backwards in the chain, each leaf corresponds with the subsequent
                    // leaf's justify QC.
                    .chain(leaf_chain.iter().map(|leaf| leaf.leaf.justify_qc())),
            );

            if let Err(err) = self
                .append_decided_leaves(leaf.view_number(), chain, consumer)
                .await
            {
                tracing::error!(
                    "failed to save decided leaves, chain may not be up to date: {err:#}"
                );
                return;
            }
        }
    }

    /// Append decided leaves to persistent storage and emit a corresponding event.
    ///
    /// `consumer` will be sent a `Decide` event containing all decided leaves in persistent storage
    /// up to and including `view`. If available in persistent storage, full block payloads and VID
    /// info will also be included for each leaf.
    ///
    /// Once the new decided leaves have been processed, old data up to `view` will be garbage
    /// collected The consumer's handling of this event is a prerequisite for the completion of
    /// garbage collection: if the consumer fails to process the event, no data is deleted. This
    /// ensures that, if called repeatedly, all decided leaves ever recorded in consensus storage
    /// will eventually be passed to the consumer.
    ///
    /// Note that the converse is not true: if garbage collection fails, it is not guaranteed that
    /// the consumer hasn't processed the decide event. Thus, in rare cases, some events may be
    /// processed twice, or the consumer may get two events which share a subset of their data.
    /// Thus, it is the consumer's responsibility to make sure its handling of each leaf is
    /// idempotent.
    ///
    /// If the consumer fails to handle the new decide event, it may be retried, or simply postponed
    /// until the next decide, at which point all persisted leaves from the failed GC run will be
    /// included in the event along with subsequently decided leaves.
    ///
    /// This functionality is useful for keeping a separate view of the blockchain in sync with the
    /// consensus storage. For example, the `consumer` could be used for moving data from consensus
    /// storage to long-term archival storage.
    async fn append_decided_leaves(
        &self,
        decided_view: ViewNumber,
        leaf_chain: impl IntoIterator<Item = (&LeafInfo<SeqTypes>, QuorumCertificate<SeqTypes>)> + Send,
        consumer: &(impl EventConsumer + 'static),
    ) -> anyhow::Result<()>;

    async fn load_anchor_leaf(&self)
        -> anyhow::Result<Option<(Leaf, QuorumCertificate<SeqTypes>)>>;
    async fn append_vid(
        &self,
        proposal: &Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()>;
    async fn append_da(
        &self,
        proposal: &Proposal<SeqTypes, DaProposal<SeqTypes>>,
    ) -> anyhow::Result<()>;
    async fn record_action(&self, view: ViewNumber, action: HotShotAction) -> anyhow::Result<()>;
    async fn update_undecided_state(
        &self,
        leaves: CommitmentMap<Leaf>,
        state: BTreeMap<ViewNumber, View<SeqTypes>>,
    ) -> anyhow::Result<()>;
    async fn append_quorum_proposal(
        &self,
        proposal: &Proposal<SeqTypes, QuorumProposal<SeqTypes>>,
    ) -> anyhow::Result<()>;
    async fn store_upgrade_certificate(
        &self,
        decided_upgrade_certificate: Option<UpgradeCertificate<SeqTypes>>,
    ) -> anyhow::Result<()>;
}

#[async_trait]
pub trait EventConsumer: Debug + DynClone + Send + Sync {
    async fn handle_event(&self, event: &Event) -> anyhow::Result<()>;
}

dyn_clone::clone_trait_object!(EventConsumer);

#[async_trait]
impl<T> EventConsumer for Box<T>
where
    Self: Clone,
    T: EventConsumer + ?Sized,
{
    async fn handle_event(&self, event: &Event) -> anyhow::Result<()> {
        (**self).handle_event(event).await
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NullEventConsumer;

#[async_trait]
impl EventConsumer for NullEventConsumer {
    async fn handle_event(&self, _event: &Event) -> anyhow::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl<P: SequencerPersistence> Storage<SeqTypes> for Arc<P> {
    async fn append_vid(
        &self,
        proposal: &Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()> {
        (**self).append_vid(proposal).await
    }

    async fn append_da(
        &self,
        proposal: &Proposal<SeqTypes, DaProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        (**self).append_da(proposal).await
    }
    async fn record_action(&self, view: ViewNumber, action: HotShotAction) -> anyhow::Result<()> {
        (**self).record_action(view, action).await
    }
    async fn update_high_qc(&self, _high_qc: QuorumCertificate<SeqTypes>) -> anyhow::Result<()> {
        Ok(())
    }

    async fn update_undecided_state(
        &self,
        leaves: CommitmentMap<Leaf>,
        state: BTreeMap<ViewNumber, View<SeqTypes>>,
    ) -> anyhow::Result<()> {
        (**self).update_undecided_state(leaves, state).await
    }

    async fn append_proposal(
        &self,
        proposal: &Proposal<SeqTypes, QuorumProposal<SeqTypes>>,
    ) -> anyhow::Result<()> {
        (**self).append_quorum_proposal(proposal).await
    }

    async fn update_decided_upgrade_certificate(
        &self,
        decided_upgrade_certificate: Option<UpgradeCertificate<SeqTypes>>,
    ) -> anyhow::Result<()> {
        (**self)
            .store_upgrade_certificate(decided_upgrade_certificate)
            .await
    }
}

/// Data that can be deserialized from a subslice of namespace payload bytes.
///
/// Companion trait for [`NsPayloadBytesRange`], which specifies the subslice of
/// namespace payload bytes to read.
pub trait FromNsPayloadBytes<'a> {
    /// Deserialize `Self` from namespace payload bytes.
    fn from_payload_bytes(bytes: &'a [u8]) -> Self;
}

/// Specifies a subslice of namespace payload bytes to read.
///
/// Companion trait for [`FromNsPayloadBytes`], which holds data that can be
/// deserialized from that subslice of bytes.
pub trait NsPayloadBytesRange<'a> {
    type Output: FromNsPayloadBytes<'a>;

    /// Range relative to this ns payload
    fn ns_payload_range(&self) -> Range<usize>;
}

/// Types which can be deserialized from either integers or strings.
///
/// Some types can be represented as an integer or a string in human-readable formats like JSON or
/// TOML. For example, 1 GWEI might be represented by the integer `1000000000` or the string `"1
/// gwei"`. Such types can implement `FromStringOrInteger` and then use [`impl_string_or_integer`]
/// to derive this user-friendly serialization.
///
/// These types are assumed to have an efficient representation as an integral type in Rust --
/// [`Self::Binary`] -- and will be serialized to and from this type when using a non-human-readable
/// encoding. With human readable encodings, serialization is always to a string.
pub trait FromStringOrInteger: Sized {
    type Binary: Serialize + DeserializeOwned;
    type Integer: Serialize + DeserializeOwned;

    fn from_binary(b: Self::Binary) -> anyhow::Result<Self>;
    fn from_string(s: String) -> anyhow::Result<Self>;
    fn from_integer(i: Self::Integer) -> anyhow::Result<Self>;

    fn to_binary(&self) -> anyhow::Result<Self::Binary>;
    fn to_string(&self) -> anyhow::Result<String>;
}
