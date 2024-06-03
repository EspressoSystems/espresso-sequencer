use crate::{
    api::data_source::CatchupDataSource, catchup::SqlStateCatchup,
    chain_config::ResolvableChainConfig, eth_signature_key::EthKeyPair, ChainConfig, Header, Leaf,
    NodeState, SeqTypes,
};
use anyhow::{anyhow, bail, ensure, Context};
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, Read, SerializationError, Valid, Validate,
};
use async_std::stream::StreamExt;
use async_std::sync::RwLock;
use committable::{Commitment, Committable, RawCommitmentBuilder};
use contract_bindings::fee_contract::DepositFilter;
use core::fmt::Debug;
use derive_more::{Add, Display, From, Into, Mul, Sub};
use ethers::{
    abi::Address,
    types::U256,
    utils::{parse_units, ParseUnits},
};
use futures::future::Future;
use hotshot::traits::ValidatedState as HotShotState;
use hotshot_query_service::{
    availability::{AvailabilityDataSource, LeafQueryData},
    data_source::VersionedDataSource,
    explorer::MonetaryValue,
    merklized_state::{MerklizedState, MerklizedStateHeightPersistence, UpdateStateData},
    types::HeightIndexed,
};
use hotshot_types::{
    data::{BlockError, ViewNumber},
    traits::{
        block_contents::{BlockHeader, BuilderFee},
        node_implementation::ConsensusTime,
        signature_key::BuilderSignatureKey,
        states::StateDelta,
    },
    vid::{VidCommon, VidSchemeType},
};
use itertools::Itertools;
use jf_merkle_tree::{
    prelude::{LightWeightSHA3MerkleTree, MerkleProof, Sha3Digest, Sha3Node},
    universal_merkle_tree::UniversalMerkleTree,
    AppendableMerkleTreeScheme, ForgetableMerkleTreeScheme, ForgetableUniversalMerkleTreeScheme,
    LookupResult, MerkleCommitment, MerkleTreeScheme, PersistentUniversalMerkleTreeScheme,
    ToTraversalPath, UniversalMerkleTreeScheme,
};
use jf_vid::VidScheme;
use num_traits::CheckedSub;
use sequencer_utils::{
    impl_serde_from_string_or_integer, impl_to_fixed_bytes, ser::FromStringOrInteger,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashSet, ops::Add, str::FromStr};
use thiserror::Error;
use vbs::version::Version;

const BLOCK_MERKLE_TREE_HEIGHT: usize = 32;
const FEE_MERKLE_TREE_HEIGHT: usize = 20;

#[derive(Hash, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ValidatedState {
    /// Frontier of Block Merkle Tree
    pub block_merkle_tree: BlockMerkleTree,
    /// Fee Merkle Tree
    pub fee_merkle_tree: FeeMerkleTree,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Delta {
    pub fees_delta: HashSet<FeeAccount>,
}

impl StateDelta for Delta {}

impl Default for ValidatedState {
    fn default() -> Self {
        let block_merkle_tree = BlockMerkleTree::from_elems(
            Some(BLOCK_MERKLE_TREE_HEIGHT),
            Vec::<Commitment<Header>>::new(),
        )
        .unwrap();

        // Words of wisdom from @mrain: "capacity = arity^height"
        // "For index space 2^160, arity 256 (2^8),
        // you should set the height as 160/8=20"
        let fee_merkle_tree = FeeMerkleTree::from_kv_set(
            FEE_MERKLE_TREE_HEIGHT,
            Vec::<(FeeAccount, FeeAmount)>::new(),
        )
        .unwrap();
        Self {
            block_merkle_tree,
            fee_merkle_tree,
        }
    }
}

impl ValidatedState {
    /// Prefund an account with a given amount. Only for demo purposes.
    pub fn prefund_account(&mut self, account: FeeAccount, amount: FeeAmount) {
        self.fee_merkle_tree.update(account, amount).unwrap();
    }

    pub fn balance(&mut self, account: FeeAccount) -> Option<FeeAmount> {
        match self.fee_merkle_tree.lookup(account) {
            LookupResult::Ok(balance, _) => Some(*balance),
            LookupResult::NotFound(_) => Some(0.into()),
            LookupResult::NotInMemory => None,
        }
    }

    /// Find accounts that are not in memory.
    ///
    /// As an optimization we could try to apply updates and return the
    /// forgotten accounts to be fetched from peers and update them later.
    pub fn forgotten_accounts(
        &self,
        accounts: impl IntoIterator<Item = FeeAccount>,
    ) -> Vec<FeeAccount> {
        accounts
            .into_iter()
            .unique()
            .filter(|account| {
                self.fee_merkle_tree
                    .lookup(*account)
                    .expect_not_in_memory()
                    .is_ok()
            })
            .collect()
    }

    /// Check if the merkle tree is available
    pub fn need_to_fetch_blocks_mt_frontier(&self) -> bool {
        let num_leaves = self.block_merkle_tree.num_leaves();
        if num_leaves == 0 {
            false
        } else {
            self.block_merkle_tree
                .lookup(num_leaves - 1)
                .expect_ok()
                .is_err()
        }
    }

    /// Insert a fee deposit receipt
    pub fn insert_fee_deposit(
        &mut self,
        fee_info: FeeInfo,
    ) -> anyhow::Result<LookupResult<FeeAmount, (), ()>> {
        Ok(self
            .fee_merkle_tree
            .update_with(fee_info.account, |balance| {
                Some(balance.cloned().unwrap_or_default().add(fee_info.amount))
            })?)
    }

    /// Charge a fee to an account, transferring the funds to the fee recipient account.
    pub fn charge_fee(&mut self, fee_info: FeeInfo, recipient: FeeAccount) -> anyhow::Result<()> {
        let fee_state = self.fee_merkle_tree.clone();

        // Deduct the fee from the paying account.
        let FeeInfo { account, amount } = fee_info;
        let mut err = None;
        let fee_state = fee_state.persistent_update_with(account, |balance| {
            let balance = balance.copied();
            let Some(updated) = balance.unwrap_or_default().checked_sub(&amount) else {
                // Return an error without updating the account.
                err = Some(anyhow!(
                    "insufficient funds (have {balance:?}, required {amount:?})"
                ));
                return balance;
            };
            if updated == FeeAmount::default() {
                // Delete the account from the tree if its balance ended up at 0; this saves some
                // space since the account is no longer carrying any information.
                None
            } else {
                // Otherwise store the updated balance.
                Some(updated)
            }
        })?;

        // Fail if there was an error during `persistent_update_with` (e.g. insufficient balance).
        if let Some(err) = err {
            return Err(err);
        }

        // If we successfully deducted the fee from the source account, increment the balance of the
        // recipient account.
        let fee_state = fee_state.persistent_update_with(recipient, |balance| {
            Some(balance.copied().unwrap_or_default() + amount)
        })?;

        // If the whole update was successful, update the original state.
        self.fee_merkle_tree = fee_state;
        Ok(())
    }
}

#[cfg(any(test, feature = "testing"))]
impl ValidatedState {
    pub fn forget(&self) -> Self {
        Self {
            fee_merkle_tree: FeeMerkleTree::from_commitment(self.fee_merkle_tree.commitment()),
            block_merkle_tree: BlockMerkleTree::from_commitment(
                self.block_merkle_tree.commitment(),
            ),
        }
    }
}

#[derive(Error, Debug)]
pub enum ProposalValidationError {
    #[error("Invalid ChainConfig: (expected={expected:?}, proposal={proposal:?})")]
    InvalidChainConfig {
        expected: ChainConfig,
        proposal: ResolvableChainConfig,
    },

    #[error("Invalid Payload Size: (expected={expected:?}, proposal={proposal:?})")]
    MaxBlockSizeExceeded {
        expected: ChainConfig,
        proposal: ResolvableChainConfig,
    },
    #[error("Insufficient Fee: (block_size={block_size}, base_fee={base_fee:?}, proposed_fee={proposed_fee:?})")]
    InsufficientFee {
        block_size: u64,
        base_fee: FeeAmount,
        proposed_fee: FeeAmount,
    },
    #[error("Invalid Height: parent_height={parent_height}, proposal_height={proposal_height}")]
    InvalidHeight {
        parent_height: u64,
        proposal_height: u64,
    },
    #[error("Invalid Block Root Error: expected={expected_root}, proposal={proposal_root}")]
    InvalidBlockRoot {
        expected_root: BlockMerkleCommitment,
        proposal_root: BlockMerkleCommitment,
    },
    #[error("Invalid Fee Root Error: expected={expected_root}, proposal={proposal_root}")]
    InvalidFeeRoot {
        expected_root: FeeMerkleCommitment,
        proposal_root: FeeMerkleCommitment,
    },
    #[error("Unknown validation error")]
    Unknown,
}

pub fn validate_proposal(
    state: &ValidatedState,
    expected_chain_config: ChainConfig,
    parent_leaf: &Leaf,
    proposal: &Header,
    vid_common: &VidCommon,
) -> Result<(), ProposalValidationError> {
    let parent_header = parent_leaf.block_header();

    // validate `ChainConfig`
    if proposal.chain_config.commit() != expected_chain_config.commit() {
        return Err(ProposalValidationError::InvalidChainConfig {
            expected: expected_chain_config,
            proposal: proposal.chain_config,
        });
    }

    // validate block size and fee
    let block_size = VidSchemeType::get_payload_byte_len(vid_common) as u64;
    if block_size >= *expected_chain_config.max_block_size {
        return Err(ProposalValidationError::MaxBlockSizeExceeded {
            expected: expected_chain_config,
            proposal: proposal.chain_config,
        });
    }

    if proposal.fee_info.amount() < expected_chain_config.base_fee * block_size {
        return Err(ProposalValidationError::InsufficientFee {
            block_size,
            base_fee: expected_chain_config.base_fee,
            proposed_fee: proposal.fee_info.amount(),
        });
    }

    // validate height
    if proposal.height != parent_header.height + 1 {
        return Err(ProposalValidationError::InvalidHeight {
            parent_height: parent_header.height,
            proposal_height: proposal.height,
        });
    }

    let ValidatedState {
        block_merkle_tree,
        fee_merkle_tree,
    } = state;

    let block_merkle_tree_root = block_merkle_tree.commitment();
    if proposal.block_merkle_tree_root != block_merkle_tree_root {
        return Err(ProposalValidationError::InvalidBlockRoot {
            expected_root: block_merkle_tree_root,
            proposal_root: proposal.block_merkle_tree_root,
        });
    }

    let fee_merkle_tree_root = fee_merkle_tree.commitment();
    if proposal.fee_merkle_tree_root == fee_merkle_tree_root {
        return Err(ProposalValidationError::InvalidFeeRoot {
            expected_root: fee_merkle_tree_root,
            proposal_root: proposal.fee_merkle_tree_root,
        });
    }

    Ok(())
}

fn charge_fee(
    state: &mut ValidatedState,
    delta: &mut Delta,
    fee_info: FeeInfo,
    recipient: FeeAccount,
) -> anyhow::Result<()> {
    state.charge_fee(fee_info, recipient)?;
    delta.fees_delta.extend([fee_info.account, recipient]);
    Ok(())
}

/// Validate builder account by verifying signature
fn validate_builder_fee(proposed_header: &Header) -> anyhow::Result<()> {
    // Beware of Malice!
    let signature = proposed_header
        .builder_signature
        .ok_or_else(|| anyhow::anyhow!("Builder signature not found"))?;
    let fee_amount = proposed_header.fee_info.amount().as_u64().context(format!(
        "fee amount out of range: {:?}",
        proposed_header.fee_info.amount()
    ))?;
    // verify signature
    anyhow::ensure!(
        proposed_header.fee_info.account.validate_fee_signature(
            &signature,
            fee_amount,
            proposed_header.metadata(),
            &proposed_header.payload_commitment()
        ),
        "Invalid Builder Signature"
    );

    Ok(())
}

async fn compute_state_update(
    state: &ValidatedState,
    instance: &NodeState,
    parent_leaf: &LeafQueryData<SeqTypes>,
    proposed_leaf: &LeafQueryData<SeqTypes>,
) -> anyhow::Result<(ValidatedState, Delta)> {
    let proposed_leaf = proposed_leaf.leaf();
    let parent_leaf = parent_leaf.leaf();
    let header = proposed_leaf.block_header();

    // Check internal consistency.
    let parent_header = parent_leaf.block_header();
    ensure!(
        state.block_merkle_tree.commitment() == parent_header.block_merkle_tree_root,
        "internal error! in-memory block tree {:?} does not match parent header {:?}",
        state.block_merkle_tree.commitment(),
        parent_header.block_merkle_tree_root
    );
    ensure!(
        state.fee_merkle_tree.commitment() == parent_header.fee_merkle_tree_root,
        "internal error! in-memory fee tree {:?} does not match parent header {:?}",
        state.fee_merkle_tree.commitment(),
        parent_header.fee_merkle_tree_root
    );

    state.apply_header(instance, parent_leaf, header).await
}

async fn store_state_update(
    storage: &mut impl SequencerStateDataSource,
    block_number: u64,
    state: &ValidatedState,
    delta: Delta,
) -> anyhow::Result<()> {
    let ValidatedState {
        fee_merkle_tree,
        block_merkle_tree,
    } = state;
    let Delta { fees_delta } = delta;

    // Insert fee merkle tree nodes
    for delta in fees_delta {
        let proof = match fee_merkle_tree.universal_lookup(delta) {
            LookupResult::Ok(_, proof) => proof,
            LookupResult::NotFound(proof) => proof,
            LookupResult::NotInMemory => bail!("missing merkle path for fee account {delta}"),
        };
        let path: Vec<usize> =
            <FeeAccount as ToTraversalPath<{ FeeMerkleTree::ARITY }>>::to_traversal_path(
                &delta,
                fee_merkle_tree.height(),
            );

        UpdateStateData::<SeqTypes, _, { FeeMerkleTree::ARITY }>::insert_merkle_nodes(
            storage,
            proof,
            path,
            block_number,
        )
        .await
        .context("failed to store fee merkle nodes")?;
    }

    // Insert block merkle tree nodes
    let (_, proof) = block_merkle_tree
        .lookup(block_number - 1)
        .expect_ok()
        .context("getting blocks frontier")?;
    let path = <u64 as ToTraversalPath<{ BlockMerkleTree::ARITY }>>::to_traversal_path(
        &(block_number - 1),
        block_merkle_tree.height(),
    );

    {
        UpdateStateData::<SeqTypes, _, { BlockMerkleTree::ARITY }>::insert_merkle_nodes(
            storage,
            proof,
            path,
            block_number,
        )
        .await
        .context("failed to store block merkle nodes")?;
    }

    storage
        .set_last_state_height(block_number as usize)
        .await
        .context("setting state height")?;
    storage.commit().await.context("committing state update")?;
    Ok(())
}

#[tracing::instrument(
    skip_all,
    fields(
        node_id = instance.node_id,
        view = ?parent_leaf.leaf().view_number(),
        height = parent_leaf.height(),
    ),
)]
async fn update_state_storage(
    parent_state: &ValidatedState,
    storage: &Arc<RwLock<impl SequencerStateDataSource>>,
    instance: &NodeState,
    parent_leaf: &LeafQueryData<SeqTypes>,
    proposed_leaf: &LeafQueryData<SeqTypes>,
) -> anyhow::Result<ValidatedState> {
    let (state, delta) = compute_state_update(parent_state, instance, parent_leaf, proposed_leaf)
        .await
        .context("computing state update")?;

    let mut storage = storage.write().await;
    if let Err(err) = store_state_update(&mut *storage, proposed_leaf.height(), &state, delta).await
    {
        storage.revert().await;
        return Err(err);
    }

    Ok(state)
}

async fn store_genesis_state(
    storage: &mut impl SequencerStateDataSource,
    state: &ValidatedState,
) -> anyhow::Result<()> {
    ensure!(
        state.block_merkle_tree.num_leaves() == 0,
        "genesis state with non-empty block tree is unsupported"
    );

    // Insert fee merkle tree nodes
    for (account, _) in state.fee_merkle_tree.iter() {
        let proof = match state.fee_merkle_tree.universal_lookup(account) {
            LookupResult::Ok(_, proof) => proof,
            LookupResult::NotFound(proof) => proof,
            LookupResult::NotInMemory => bail!("missing merkle path for fee account {account}"),
        };
        let path: Vec<usize> =
            <FeeAccount as ToTraversalPath<{ FeeMerkleTree::ARITY }>>::to_traversal_path(
                account,
                state.fee_merkle_tree.height(),
            );

        UpdateStateData::<SeqTypes, _, { FeeMerkleTree::ARITY }>::insert_merkle_nodes(
            storage, proof, path, 0,
        )
        .await
        .context("failed to store fee merkle nodes")?;
    }

    storage.commit().await?;
    Ok(())
}

pub(crate) async fn update_state_storage_loop(
    storage: Arc<RwLock<impl SequencerStateDataSource>>,
    instance: impl Future<Output = NodeState>,
) -> anyhow::Result<()> {
    let mut instance = instance.await;
    instance.peers = Arc::new(SqlStateCatchup::from(storage.clone()));

    // get last saved merklized state
    let (last_height, parent_leaf, mut leaves) = {
        let state = storage.upgradable_read().await;

        let last_height = state.get_last_state_height().await?;
        tracing::info!(last_height, "updating state storage");

        let parent_leaf = state.get_leaf(last_height).await;
        let leaves = state.subscribe_leaves(last_height + 1).await;
        (last_height, parent_leaf, leaves)
    };
    // resolve the parent leaf future _after_ dropping our lock on the state, in case it is not
    // ready yet and another task needs a mutable lock on the state to produce the parent leaf.
    let mut parent_leaf = parent_leaf.await;
    let mut parent_state = ValidatedState::from_header(parent_leaf.header());

    if last_height == 0 {
        // If the last height is 0, we need to insert the genesis state, since this state is
        // never the result of a state update and thus is not inserted in the loop below.
        tracing::info!("storing genesis merklized state");
        let mut storage = storage.write().await;
        if let Err(err) = store_genesis_state(&mut *storage, &instance.genesis_state).await {
            tracing::error!("failed to store genesis state: {err:#}");
            storage.revert().await;
            return Err(err);
        }
    }

    while let Some(leaf) = leaves.next().await {
        loop {
            match update_state_storage(&parent_state, &storage, &instance, &parent_leaf, &leaf)
                .await
            {
                Ok(state) => {
                    parent_leaf = leaf;
                    parent_state = state;
                    break;
                }
                Err(err) => {
                    tracing::error!(height = leaf.height(), "failed to updated state: {err:#}");
                    // If we fail, delay for a second and retry.
                    async_std::task::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    Ok(())
}

pub(crate) trait SequencerStateDataSource:
    'static
    + Debug
    + AvailabilityDataSource<SeqTypes>
    + VersionedDataSource
    + CatchupDataSource
    + UpdateStateData<SeqTypes, FeeMerkleTree, { FeeMerkleTree::ARITY }>
    + UpdateStateData<SeqTypes, BlockMerkleTree, { BlockMerkleTree::ARITY }>
    + MerklizedStateHeightPersistence
{
}

impl<T> SequencerStateDataSource for T where
    T: 'static
        + Debug
        + AvailabilityDataSource<SeqTypes>
        + VersionedDataSource
        + CatchupDataSource
        + UpdateStateData<SeqTypes, FeeMerkleTree, { FeeMerkleTree::ARITY }>
        + UpdateStateData<SeqTypes, BlockMerkleTree, { BlockMerkleTree::ARITY }>
        + MerklizedStateHeightPersistence
{
}

impl ValidatedState {
    pub(crate) async fn apply_header(
        &self,
        instance: &NodeState,
        parent_leaf: &Leaf,
        proposed_header: &Header,
    ) -> anyhow::Result<(Self, Delta)> {
        // Clone state to avoid mutation. Consumer can take update
        // through returned value.

        let l1_deposits = get_l1_deposits(instance, proposed_header, parent_leaf).await;

        let mut validated_state = self.clone();

        // Find missing fee state entries. We will need to use the builder account which is paying a
        // fee and the recipient account which is receiving it, plus any counts receiving deposits
        // in this block.
        let missing_accounts = self.forgotten_accounts(
            [
                proposed_header.fee_info.account,
                instance.chain_config.fee_recipient,
            ]
            .into_iter()
            .chain(l1_deposits.iter().map(|fee_info| fee_info.account)),
        );

        let parent_height = parent_leaf.height();
        let parent_view = parent_leaf.view_number();

        // Ensure merkle tree has frontier
        if self.need_to_fetch_blocks_mt_frontier() {
            tracing::info!(
                parent_height,
                ?parent_view,
                "fetching block frontier from peers"
            );
            instance
                .peers
                .as_ref()
                .remember_blocks_merkle_tree(
                    parent_height,
                    parent_view,
                    &mut validated_state.block_merkle_tree,
                )
                .await?;
        }

        // Fetch missing fee state entries
        if !missing_accounts.is_empty() {
            tracing::info!(
                parent_height,
                ?parent_view,
                ?missing_accounts,
                "fetching missing accounts from peers"
            );

            let missing_account_proofs = instance
                .peers
                .as_ref()
                .fetch_accounts(
                    parent_height,
                    parent_view,
                    validated_state.fee_merkle_tree.commitment(),
                    missing_accounts,
                )
                .await?;

            // Remember the fee state entries
            for account in missing_account_proofs.iter() {
                account
                    .proof
                    .remember(&mut validated_state.fee_merkle_tree)
                    .expect("proof previously verified");
            }
        }

        let mut delta = Delta::default();

        let mut validated_state =
            apply_proposal(&validated_state, &mut delta, parent_leaf, l1_deposits);

        charge_fee(
            &mut validated_state,
            &mut delta,
            proposed_header.fee_info,
            instance.chain_config.fee_recipient,
        )?;

        Ok((validated_state, delta))
    }
}

pub async fn get_l1_deposits(
    instance: &NodeState,
    header: &Header,
    parent_leaf: &Leaf,
) -> Vec<FeeInfo> {
    if let (Some(addr), Some(block_info)) =
        (instance.chain_config.fee_contract, header.l1_finalized)
    {
        instance
            .l1_client
            .get_finalized_deposits(
                addr,
                parent_leaf
                    .block_header()
                    .l1_finalized
                    .map(|block_info| block_info.number),
                block_info.number,
            )
            .await
    } else {
        vec![]
    }
}

#[must_use]
fn apply_proposal(
    validated_state: &ValidatedState,
    delta: &mut Delta,
    parent_leaf: &Leaf,
    l1_deposits: Vec<FeeInfo>,
) -> ValidatedState {
    let mut validated_state = validated_state.clone();
    // pushing a block into merkle tree shouldn't fail
    validated_state
        .block_merkle_tree
        .push(parent_leaf.block_header().commit())
        .unwrap();

    for FeeInfo { account, amount } in l1_deposits.iter() {
        validated_state
            .fee_merkle_tree
            .update_with(account, |balance| {
                Some(balance.cloned().unwrap_or_default().add(*amount))
            })
            .expect("update_with succeeds");
        delta.fees_delta.insert(*account);
    }

    validated_state
}

impl HotShotState<SeqTypes> for ValidatedState {
    type Error = BlockError;
    type Instance = NodeState;

    type Time = ViewNumber;

    type Delta = Delta;
    fn on_commit(&self) {}
    /// Validate parent against known values (from state) and validate
    /// proposal descends from parent. Returns updated `ValidatedState`.
    #[tracing::instrument(
        skip_all,
        fields(
            node_id = instance.node_id,
            view = ?parent_leaf.view_number(),
            height = parent_leaf.height(),
        ),
    )]
    async fn validate_and_apply_header(
        &self,
        instance: &Self::Instance,
        parent_leaf: &Leaf,
        proposed_header: &Header,
        vid_common: VidCommon,
        _version: Version,
    ) -> Result<(Self, Self::Delta), Self::Error> {
        //validate builder fee
        if let Err(err) = validate_builder_fee(proposed_header) {
            tracing::error!("invalid builder fee: {err:#}");
            return Err(BlockError::InvalidBlockHeader);
        }

        // Unwrapping here is okay as we retry in a loop
        //so we should either get a validated state or until hotshot cancels the task
        let (validated_state, delta) = self
            .apply_header(instance, parent_leaf, proposed_header)
            .await
            .unwrap();

        // validate the proposal
        if let Err(err) = validate_proposal(
            &validated_state,
            instance.chain_config,
            parent_leaf,
            proposed_header,
            &vid_common,
        ) {
            tracing::error!("invalid proposal: {err:#}");
            return Err(BlockError::InvalidBlockHeader);
        }

        // log successful progress about once in 10 - 20 seconds,
        // TODO: we may want to make this configurable
        if parent_leaf.view_number().u64() % 10 == 0 {
            tracing::info!("validated and applied new header");
        }
        Ok((validated_state, delta))
    }
    /// Construct the state with the given block header.
    ///
    /// This can also be used to rebuild the state for catchup.
    fn from_header(block_header: &Header) -> Self {
        let fee_merkle_tree = if block_header.fee_merkle_tree_root.size() == 0 {
            // If the commitment tells us that the tree is supposed to be empty, it is convenient to
            // just create an empty tree, rather than a commitment-only tree.
            FeeMerkleTree::new(FEE_MERKLE_TREE_HEIGHT)
        } else {
            FeeMerkleTree::from_commitment(block_header.fee_merkle_tree_root)
        };
        let block_merkle_tree = if block_header.block_merkle_tree_root.size() == 0 {
            // If the commitment tells us that the tree is supposed to be empty, it is convenient to
            // just create an empty tree, rather than a commitment-only tree.
            BlockMerkleTree::new(BLOCK_MERKLE_TREE_HEIGHT)
        } else {
            BlockMerkleTree::from_commitment(block_header.block_merkle_tree_root)
        };
        Self {
            fee_merkle_tree,
            block_merkle_tree,
        }
    }
    /// Construct a genesis validated state.
    #[must_use]
    fn genesis(instance: &Self::Instance) -> (Self, Self::Delta) {
        (instance.genesis_state.clone(), Delta::default())
    }
}

// Required for TestableState
#[cfg(any(test, feature = "testing"))]
impl std::fmt::Display for ValidatedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[cfg(any(test, feature = "testing"))]
impl hotshot_types::traits::states::TestableState<SeqTypes> for ValidatedState {
    fn create_random_transaction(
        _state: Option<&Self>,
        rng: &mut dyn rand::RngCore,
        _padding: u64,
    ) -> crate::Transaction {
        crate::Transaction::random(rng)
    }
}

pub type BlockMerkleTree = LightWeightSHA3MerkleTree<Commitment<Header>>;
pub type BlockMerkleCommitment = <BlockMerkleTree as MerkleTreeScheme>::Commitment;

impl MerklizedState<SeqTypes, { Self::ARITY }> for BlockMerkleTree {
    type Key = Self::Index;
    type Entry = Commitment<Header>;
    type T = Sha3Node;
    type Commit = Self::Commitment;
    type Digest = Sha3Digest;

    fn state_type() -> &'static str {
        "block_merkle_tree"
    }

    fn header_state_commitment_field() -> &'static str {
        "block_merkle_tree_root"
    }

    fn tree_height() -> usize {
        BLOCK_MERKLE_TREE_HEIGHT
    }

    fn insert_path(
        &mut self,
        key: Self::Key,
        proof: &MerkleProof<Self::Entry, Self::Key, Self::T, { Self::ARITY }>,
    ) -> anyhow::Result<()> {
        let Some(elem) = proof.elem() else {
            bail!("BlockMerkleTree does not support non-membership proofs");
        };
        self.remember(key, elem, proof)?;
        Ok(())
    }
}

#[derive(
    Hash,
    Copy,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    CanonicalSerialize,
    CanonicalDeserialize,
)]
/// `FeeInfo` holds data related to builder fees.
pub struct FeeInfo {
    account: FeeAccount,
    amount: FeeAmount,
}
impl FeeInfo {
    pub fn new(account: impl Into<FeeAccount>, amount: impl Into<FeeAmount>) -> Self {
        Self {
            account: account.into(),
            amount: amount.into(),
        }
    }
    /// The minimum fee paid by the given builder account for a proposed block.
    // TODO this function should take the block size as an input, we need to get this information
    // from HotShot.
    pub fn base_fee(account: FeeAccount) -> Self {
        Self {
            account,
            amount: FeeAmount::default(),
        }
    }

    pub fn genesis() -> Self {
        Self {
            account: Default::default(),
            amount: Default::default(),
        }
    }

    pub fn account(&self) -> FeeAccount {
        self.account
    }

    pub fn amount(&self) -> FeeAmount {
        self.amount
    }
}

impl From<BuilderFee<SeqTypes>> for FeeInfo {
    fn from(fee: BuilderFee<SeqTypes>) -> Self {
        Self {
            amount: fee.fee_amount.into(),
            account: fee.fee_account,
        }
    }
}

impl From<DepositFilter> for FeeInfo {
    fn from(item: DepositFilter) -> Self {
        Self {
            amount: item.amount.into(),
            account: item.user.into(),
        }
    }
}

impl Committable for FeeInfo {
    fn commit(&self) -> Commitment<Self> {
        RawCommitmentBuilder::new(&Self::tag())
            .fixed_size_field("account", &self.account.to_fixed_bytes())
            .fixed_size_field("amount", &self.amount.to_fixed_bytes())
            .finalize()
    }
    fn tag() -> String {
        "FEE_INFO".into()
    }
}

// New Type for `U256` in order to implement `CanonicalSerialize` and
// `CanonicalDeserialize`
#[derive(
    Default,
    Hash,
    Copy,
    Clone,
    Debug,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Add,
    Sub,
    Mul,
    From,
    Into,
)]
#[display(fmt = "{_0}")]
pub struct FeeAmount(U256);

impl_serde_from_string_or_integer!(FeeAmount);
impl_to_fixed_bytes!(FeeAmount, U256);

impl From<u64> for FeeAmount {
    fn from(amt: u64) -> Self {
        Self(amt.into())
    }
}

impl From<FeeAmount> for MonetaryValue {
    fn from(value: FeeAmount) -> Self {
        MonetaryValue::eth(value.0.as_u128() as i128)
    }
}

impl CheckedSub for FeeAmount {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        self.0.checked_sub(v.0).map(FeeAmount)
    }
}

impl FromStr for FeeAmount {
    type Err = <U256 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl FromStringOrInteger for FeeAmount {
    type Binary = U256;
    type Integer = u64;

    fn from_binary(b: Self::Binary) -> anyhow::Result<Self> {
        Ok(Self(b))
    }

    fn from_integer(i: Self::Integer) -> anyhow::Result<Self> {
        Ok(i.into())
    }

    fn from_string(s: String) -> anyhow::Result<Self> {
        // For backwards compatibility, we have an ad hoc parser for WEI amounts represented as hex
        // strings.
        if let Some(s) = s.strip_prefix("0x") {
            return Ok(Self(s.parse()?));
        }

        // Strip an optional non-numeric suffix, which will be interpreted as a unit.
        let (base, unit) = s
            .split_once(char::is_whitespace)
            .unwrap_or((s.as_str(), "wei"));
        match parse_units(base, unit)? {
            ParseUnits::U256(n) => Ok(Self(n)),
            ParseUnits::I256(_) => bail!("amount cannot be negative"),
        }
    }

    fn to_binary(&self) -> anyhow::Result<Self::Binary> {
        Ok(self.0)
    }

    fn to_string(&self) -> anyhow::Result<String> {
        Ok(format!("{self}"))
    }
}

impl FeeAmount {
    pub fn as_u64(&self) -> Option<u64> {
        if self.0 <= u64::MAX.into() {
            Some(self.0.as_u64())
        } else {
            None
        }
    }
}

// New Type for `Address` in order to implement `CanonicalSerialize` and
// `CanonicalDeserialize`
#[derive(
    Default,
    Hash,
    Copy,
    Clone,
    Debug,
    Display,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    From,
    Into,
)]
#[display(fmt = "{_0:x}")]
pub struct FeeAccount(Address);
impl FeeAccount {
    /// Return inner `Address`
    pub fn address(&self) -> Address {
        self.0
    }
    /// Return byte slice representation of inner `Address` type
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
    /// Return array containing underlying bytes of inner `Address` type
    pub fn to_fixed_bytes(self) -> [u8; 20] {
        self.0.to_fixed_bytes()
    }
    pub fn test_key_pair() -> EthKeyPair {
        EthKeyPair::from_mnemonic(
            "test test test test test test test test test test test junk",
            0u32,
        )
        .unwrap()
    }
}

impl FromStr for FeeAccount {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl Valid for FeeAmount {
    fn check(&self) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl Valid for FeeAccount {
    fn check(&self) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl CanonicalSerialize for FeeAmount {
    fn serialize_with_mode<W: std::io::prelude::Write>(
        &self,
        mut writer: W,
        _compress: Compress,
    ) -> Result<(), SerializationError> {
        Ok(writer.write_all(&self.to_fixed_bytes())?)
    }

    fn serialized_size(&self, _compress: Compress) -> usize {
        core::mem::size_of::<U256>()
    }
}
impl CanonicalDeserialize for FeeAmount {
    fn deserialize_with_mode<R: Read>(
        mut reader: R,
        _compress: Compress,
        _validate: Validate,
    ) -> Result<Self, SerializationError> {
        let mut bytes = [0u8; core::mem::size_of::<U256>()];
        reader.read_exact(&mut bytes)?;
        let value = U256::from_little_endian(&bytes);
        Ok(Self(value))
    }
}
impl CanonicalSerialize for FeeAccount {
    fn serialize_with_mode<W: std::io::prelude::Write>(
        &self,
        mut writer: W,
        _compress: Compress,
    ) -> Result<(), SerializationError> {
        Ok(writer.write_all(&self.0.to_fixed_bytes())?)
    }

    fn serialized_size(&self, _compress: Compress) -> usize {
        core::mem::size_of::<Address>()
    }
}
impl CanonicalDeserialize for FeeAccount {
    fn deserialize_with_mode<R: Read>(
        mut reader: R,
        _compress: Compress,
        _validate: Validate,
    ) -> Result<Self, SerializationError> {
        let mut bytes = [0u8; core::mem::size_of::<Address>()];
        reader.read_exact(&mut bytes)?;
        let value = Address::from_slice(&bytes);
        Ok(Self(value))
    }
}

impl ToTraversalPath<256> for FeeAccount {
    fn to_traversal_path(&self, height: usize) -> Vec<usize> {
        self.0
            .to_fixed_bytes()
            .into_iter()
            .take(height)
            .map(|i| i as usize)
            .collect()
    }
}

pub type FeeMerkleTree = UniversalMerkleTree<FeeAmount, Sha3Digest, FeeAccount, 256, Sha3Node>;
pub type FeeMerkleCommitment = <FeeMerkleTree as MerkleTreeScheme>::Commitment;

impl MerklizedState<SeqTypes, { Self::ARITY }> for FeeMerkleTree {
    type Key = Self::Index;
    type Entry = Self::Element;
    type T = Sha3Node;
    type Commit = Self::Commitment;
    type Digest = Sha3Digest;

    fn state_type() -> &'static str {
        "fee_merkle_tree"
    }

    fn header_state_commitment_field() -> &'static str {
        "fee_merkle_tree_root"
    }

    fn tree_height() -> usize {
        FEE_MERKLE_TREE_HEIGHT
    }

    fn insert_path(
        &mut self,
        key: Self::Key,
        proof: &MerkleProof<Self::Entry, Self::Key, Self::T, { Self::ARITY }>,
    ) -> anyhow::Result<()> {
        match proof.elem() {
            Some(elem) => self.remember(key, elem, proof)?,
            None => self.non_membership_remember(key, proof)?,
        }
        Ok(())
    }
}

/// A proof of the balance of an account in the fee ledger.
///
/// If the account of interest does not exist in the fee state, this is a Merkle non-membership
/// proof, and the balance is implicitly zero. Otherwise, this is a normal Merkle membership proof.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FeeAccountProof {
    account: Address,
    proof: FeeMerkleProof,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum FeeMerkleProof {
    Presence(<FeeMerkleTree as MerkleTreeScheme>::MembershipProof),
    Absence(<FeeMerkleTree as UniversalMerkleTreeScheme>::NonMembershipProof),
}

impl FeeAccountProof {
    pub(crate) fn presence(
        pos: FeeAccount,
        proof: <FeeMerkleTree as MerkleTreeScheme>::MembershipProof,
    ) -> Self {
        Self {
            account: pos.into(),
            proof: FeeMerkleProof::Presence(proof),
        }
    }

    pub(crate) fn absence(
        pos: FeeAccount,
        proof: <FeeMerkleTree as UniversalMerkleTreeScheme>::NonMembershipProof,
    ) -> Self {
        Self {
            account: pos.into(),
            proof: FeeMerkleProof::Absence(proof),
        }
    }

    pub fn prove(tree: &FeeMerkleTree, account: Address) -> Option<(Self, U256)> {
        match tree.universal_lookup(FeeAccount(account)) {
            LookupResult::Ok(balance, proof) => Some((
                Self {
                    account,
                    proof: FeeMerkleProof::Presence(proof),
                },
                balance.0,
            )),
            LookupResult::NotFound(proof) => Some((
                Self {
                    account,
                    proof: FeeMerkleProof::Absence(proof),
                },
                0.into(),
            )),
            LookupResult::NotInMemory => None,
        }
    }

    pub fn verify(&self, comm: &FeeMerkleCommitment) -> anyhow::Result<U256> {
        match &self.proof {
            FeeMerkleProof::Presence(proof) => {
                ensure!(
                    FeeMerkleTree::verify(comm.digest(), FeeAccount(self.account), proof)?.is_ok(),
                    "invalid proof"
                );
                Ok(proof
                    .elem()
                    .context("presence proof is missing account balance")?
                    .0)
            }
            FeeMerkleProof::Absence(proof) => {
                let tree = FeeMerkleTree::from_commitment(comm);
                ensure!(
                    tree.non_membership_verify(FeeAccount(self.account), proof)?,
                    "invalid proof"
                );
                Ok(0.into())
            }
        }
    }

    pub fn remember(&self, tree: &mut FeeMerkleTree) -> anyhow::Result<()> {
        match &self.proof {
            FeeMerkleProof::Presence(proof) => {
                tree.remember(
                    FeeAccount(self.account),
                    proof
                        .elem()
                        .context("presence proof is missing account balance")?,
                    proof,
                )?;
                Ok(())
            }
            FeeMerkleProof::Absence(proof) => {
                tree.non_membership_remember(FeeAccount(self.account), proof)?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use hotshot_types::vid::vid_scheme;
    use jf_vid::VidScheme;

    #[test]
    fn test_fee_proofs() {
        setup_logging();
        setup_backtrace();

        let mut tree = ValidatedState::default().fee_merkle_tree;
        let account1 = Address::random();
        let account2 = Address::default();
        tracing::info!(%account1, %account2);

        let balance1 = U256::from(100);
        tree.update(FeeAccount(account1), FeeAmount(balance1))
            .unwrap();

        // Membership proof.
        let (proof1, balance) = FeeAccountProof::prove(&tree, account1).unwrap();
        tracing::info!(?proof1, %balance);
        assert_eq!(balance, balance1);
        assert!(matches!(proof1.proof, FeeMerkleProof::Presence(_)));
        assert_eq!(proof1.verify(&tree.commitment()).unwrap(), balance1);

        // Non-membership proof.
        let (proof2, balance) = FeeAccountProof::prove(&tree, account2).unwrap();
        tracing::info!(?proof2, %balance);
        assert_eq!(balance, 0.into());
        assert!(matches!(proof2.proof, FeeMerkleProof::Absence(_)));
        assert_eq!(proof2.verify(&tree.commitment()).unwrap(), 0.into());

        // Test forget/remember. We cannot generate proofs in a completely sparse tree:
        let mut tree = FeeMerkleTree::from_commitment(tree.commitment());
        assert!(FeeAccountProof::prove(&tree, account1).is_none());
        assert!(FeeAccountProof::prove(&tree, account2).is_none());
        // After remembering the proofs, we can generate proofs again:
        proof1.remember(&mut tree).unwrap();
        proof2.remember(&mut tree).unwrap();
        FeeAccountProof::prove(&tree, account1).unwrap();
        FeeAccountProof::prove(&tree, account2).unwrap();
    }

    #[async_std::test]
    async fn test_validation_max_block_size() {
        setup_logging();
        setup_backtrace();

        const MAX_BLOCK_SIZE: usize = 10;
        let payload = [0; 2 * MAX_BLOCK_SIZE];
        let vid_common = vid_scheme(1).disperse(payload).unwrap().common;

        let state = ValidatedState::default();
        let instance = NodeState::mock().with_chain_config(ChainConfig {
            max_block_size: (MAX_BLOCK_SIZE as u64).into(),
            base_fee: 0.into(),
            ..Default::default()
        });
        let parent = Leaf::genesis(&instance.genesis_state, &instance).await;
        let header = parent.block_header();

        // Validation fails because the proposed block exceeds the maximum block size.
        let err = validate_proposal(&state, instance.chain_config, &parent, header, &vid_common)
            .unwrap_err();
        tracing::info!(%err, "task failed successfully");
    }

    #[async_std::test]
    async fn test_validation_base_fee() {
        setup_logging();
        setup_backtrace();

        let max_block_size = 10;
        let payload = [0; 1];
        let vid_common = vid_scheme(1).disperse(payload).unwrap().common;

        let state = ValidatedState::default();
        let instance = NodeState::mock().with_chain_config(ChainConfig {
            base_fee: 1000.into(), // High base fee
            max_block_size: max_block_size.into(),
            ..Default::default()
        });
        let parent = Leaf::genesis(&instance.genesis_state, &instance).await;
        let header = parent.block_header();

        // Validation fails because the genesis fee (0) is too low.
        let err = validate_proposal(&state, instance.chain_config, &parent, header, &vid_common)
            .unwrap_err();
        tracing::info!(%err, "task failed successfully");
    }

    #[test]
    fn test_charge_fee() {
        setup_logging();
        setup_backtrace();

        let src = FeeAccount::generated_from_seed_indexed([0; 32], 0).0;
        let dst = FeeAccount::generated_from_seed_indexed([0; 32], 1).0;
        let amt = FeeAmount::from(1);

        let fee_info = FeeInfo::new(src, amt);

        let new_state = || {
            let mut state = ValidatedState::default();
            state.prefund_account(src, amt);
            state
        };

        tracing::info!("test successful fee");
        let mut state = new_state();
        state.charge_fee(fee_info, dst).unwrap();
        assert_eq!(state.balance(src), Some(0.into()));
        assert_eq!(state.balance(dst), Some(amt));

        tracing::info!("test insufficient balance");
        state.charge_fee(fee_info, dst).unwrap_err();
        assert_eq!(state.balance(src), Some(0.into()));
        assert_eq!(state.balance(dst), Some(amt));

        tracing::info!("test src not in memory");
        let mut state = new_state();
        state.fee_merkle_tree.forget(src).expect_ok().unwrap();
        state.charge_fee(fee_info, dst).unwrap_err();

        tracing::info!("test dst not in memory");
        let mut state = new_state();
        state.prefund_account(dst, amt);
        state.fee_merkle_tree.forget(dst).expect_ok().unwrap();
        state.charge_fee(fee_info, dst).unwrap_err();
    }

    #[test]
    fn test_fee_amount_serde_json_as_decimal() {
        let amt = FeeAmount::from(123);
        let serialized = serde_json::to_string(&amt).unwrap();

        // The value is serialized as a decimal string.
        assert_eq!(serialized, "\"123\"");

        // Deserialization produces the original value
        let deserialized: FeeAmount = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, amt);
    }

    #[test]
    fn test_fee_amount_from_units() {
        for (unit, multiplier) in [
            ("wei", 1),
            ("gwei", 1_000_000_000),
            ("eth", 1_000_000_000_000_000_000),
        ] {
            let amt: FeeAmount = serde_json::from_str(&format!("\"1 {unit}\"")).unwrap();
            assert_eq!(amt, multiplier.into());
        }
    }

    #[test]
    fn test_fee_amount_serde_json_from_hex() {
        // For backwards compatibility, fee amounts can also be deserialized from a 0x-prefixed hex
        // string.
        let amt: FeeAmount = serde_json::from_str("\"0x123\"").unwrap();
        assert_eq!(amt, FeeAmount::from(0x123));
    }

    #[test]
    fn test_fee_amount_serde_json_from_number() {
        // For convenience, fee amounts can also be deserialized from a JSON number.
        let amt: FeeAmount = serde_json::from_str("123").unwrap();
        assert_eq!(amt, FeeAmount::from(123));
    }

    #[test]
    fn test_fee_amount_serde_bincode_unchanged() {
        // For non-human-readable formats, FeeAmount just serializes as the underlying U256.
        let n = U256::from(123);
        let amt = FeeAmount(n);
        assert_eq!(
            bincode::serialize(&n).unwrap(),
            bincode::serialize(&amt).unwrap(),
        );
    }
}
