use crate::{
    api::data_source::CatchupDataSource, catchup::SqlStateCatchup, eth_signature_key::EthKeyPair,
    ChainConfig, Header, Leaf, NodeState, SeqTypes,
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
use derive_more::{Add, Display, From, Into, Sub};
use ethers::{abi::Address, types::U256};
use futures::future::Future;
use hotshot::traits::ValidatedState as HotShotState;
use hotshot_query_service::{
    availability::{AvailabilityDataSource, LeafQueryData},
    data_source::VersionedDataSource,
    merklized_state::{MerklizedState, MerklizedStateHeightPersistence, UpdateStateData},
    types::HeightIndexed,
};
use hotshot_types::{
    data::{BlockError, ViewNumber},
    traits::{
        node_implementation::ConsensusTime, signature_key::BuilderSignatureKey, states::StateDelta,
    },
};
use itertools::Itertools;
use jf_primitives::merkle_tree::{
    prelude::MerkleProof, ToTraversalPath, UniversalMerkleTreeScheme,
};
use jf_primitives::{
    errors::PrimitivesError,
    merkle_tree::{
        prelude::{LightWeightSHA3MerkleTree, Sha3Digest, Sha3Node},
        universal_merkle_tree::UniversalMerkleTree,
        AppendableMerkleTreeScheme, ForgetableMerkleTreeScheme,
        ForgetableUniversalMerkleTreeScheme, LookupResult, MerkleCommitment, MerkleTreeScheme,
    },
};
use num_traits::CheckedSub;
use sequencer_utils::impl_to_fixed_bytes;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashSet, ops::Add, str::FromStr};

const BLOCK_MERKLE_TREE_HEIGHT: usize = 32;
const FEE_MERKLE_TREE_HEIGHT: usize = 20;

#[derive(Hash, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ValidatedState {
    /// Frontier of Block Merkle Tree
    pub block_merkle_tree: BlockMerkleTree,
    /// Fee Merkle Tree
    pub fee_merkle_tree: FeeMerkleTree,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
    ) -> Result<LookupResult<FeeAmount, (), ()>, PrimitivesError> {
        self.fee_merkle_tree
            .update_with(fee_info.account, |balance| {
                Some(balance.cloned().unwrap_or_default().add(fee_info.amount))
            })
    }

    /// Charge a fee to an account.
    pub fn charge_fee(&mut self, fee_info: FeeInfo) -> anyhow::Result<()> {
        let FeeInfo { account, amount } = fee_info;
        let mut err = None;
        let res = self.fee_merkle_tree.update_with(account, |balance| {
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
        // Check if we were unable to do the update because the required Merkle path is missing.
        ensure!(
            res.expect_not_in_memory().is_err(),
            format!("missing account state for {account}")
        );
        // Fail if there was an error during `update_with`, otherwise succeed.
        if let Some(err) = err {
            Err(err)
        } else {
            Ok(())
        }
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

// TODO remove this, data will come from HotShot:
// https://github.com/EspressoSystems/HotShot/issues/2744
fn get_proposed_payload_size() -> u64 {
    1
}

pub fn validate_proposal(
    state: &ValidatedState,
    expected_chain_config: ChainConfig,
    parent_leaf: &Leaf,
    proposal: &Header,
) -> anyhow::Result<()> {
    let parent_header = parent_leaf.get_block_header();

    // validate `ChainConfig`
    anyhow::ensure!(
        proposal.chain_config.commit() == expected_chain_config.commit(),
        anyhow::anyhow!(
            "Invalid Chain Config: local={:?}, proposal={:?}",
            expected_chain_config,
            proposal.chain_config
        )
    );

    anyhow::ensure!(
        get_proposed_payload_size() < expected_chain_config.max_block_size(),
        anyhow::anyhow!(
            "Invalid Payload Size: local={:?}, proposal={:?}",
            expected_chain_config,
            proposal.chain_config
        )
    );

    // validate height
    anyhow::ensure!(
        proposal.height == parent_header.height + 1,
        anyhow::anyhow!(
            "Invalid Height Error: {}, {}",
            parent_header.height,
            proposal.height
        )
    );

    let ValidatedState {
        block_merkle_tree,
        fee_merkle_tree,
    } = state;

    let block_merkle_tree_root = block_merkle_tree.commitment();
    anyhow::ensure!(
        proposal.block_merkle_tree_root == block_merkle_tree_root,
        anyhow::anyhow!(
            "Invalid Block Root Error: local={}, proposal={}",
            block_merkle_tree_root,
            proposal.block_merkle_tree_root
        )
    );

    let fee_merkle_tree_root = fee_merkle_tree.commitment();
    anyhow::ensure!(
        proposal.fee_merkle_tree_root == fee_merkle_tree_root,
        anyhow::anyhow!(
            "Invalid Fee Root Error: local={}, proposal={}",
            fee_merkle_tree_root,
            proposal.fee_merkle_tree_root
        )
    );
    Ok(())
}

fn charge_fee(
    state: &mut ValidatedState,
    delta: &mut Delta,
    fee_info: FeeInfo,
) -> anyhow::Result<()> {
    state.charge_fee(fee_info)?;
    delta.fees_delta.insert(fee_info.account);
    Ok(())
}

/// Validate builder account by verifying signature
fn validate_builder_fee(proposed_header: &Header) -> anyhow::Result<()> {
    // Beware of Malice!
    let signature = proposed_header
        .builder_signature
        .ok_or_else(|| anyhow::anyhow!("Builder signature not found"))?;
    let msg = proposed_header.fee_message().context("invalid fee")?;
    // verify signature
    anyhow::ensure!(
        proposed_header
            .fee_info
            .account
            .validate_builder_signature(&signature, msg.as_ref()),
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
    let header = proposed_leaf.get_block_header();

    // Check internal consistency.
    let parent_header = parent_leaf.get_block_header();
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
        view = ?parent_leaf.leaf().get_view_number(),
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
    async fn apply_header(
        &self,
        instance: &NodeState,
        parent_leaf: &Leaf,
        proposed_header: &Header,
    ) -> anyhow::Result<(Self, Delta)> {
        // Clone state to avoid mutation. Consumer can take update
        // through returned value.

        let l1_deposits = get_l1_deposits(instance, proposed_header, parent_leaf).await;

        let mut validated_state = self.clone();

        let accounts = std::iter::once(proposed_header.fee_info.account);

        // Find missing state entries
        let missing_accounts = self.forgotten_accounts(
            accounts.chain(l1_deposits.iter().map(|fee_info| fee_info.account)),
        );

        let parent_height = parent_leaf.get_height();
        let parent_view = parent_leaf.get_view_number();

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

        charge_fee(&mut validated_state, &mut delta, proposed_header.fee_info)?;

        Ok((validated_state, delta))
    }
}

pub async fn get_l1_deposits(
    instance: &NodeState,
    header: &Header,
    parent_leaf: &Leaf,
) -> Vec<FeeInfo> {
    if let Some(block_info) = header.l1_finalized {
        instance
            .l1_client
            .get_finalized_deposits(
                parent_leaf
                    .get_block_header()
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
pub fn apply_proposal(
    validated_state: &ValidatedState,
    delta: &mut Delta,
    parent_leaf: &Leaf,
    l1_deposits: Vec<FeeInfo>,
) -> ValidatedState {
    let mut validated_state = validated_state.clone();
    // pushing a block into merkle tree shouldn't fail
    validated_state
        .block_merkle_tree
        .push(parent_leaf.get_block_header().commit())
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
            view = ?parent_leaf.get_view_number(),
            height = parent_leaf.get_height(),
        ),
    )]
    async fn validate_and_apply_header(
        &self,
        instance: &Self::Instance,
        parent_leaf: &Leaf,
        proposed_header: &Header,
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
        ) {
            tracing::error!("invalid proposal: {err:#}");
            return Err(BlockError::InvalidBlockHeader);
        }

        // log successful progress about once in 10 - 20 seconds,
        // TODO: we may want to make this configurable
        if parent_leaf.get_view_number().get_u64() % 10 == 0 {
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
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Add,
    Sub,
    From,
    Into,
)]
pub struct FeeAmount(U256);

impl_to_fixed_bytes!(FeeAmount, U256);

impl From<u64> for FeeAmount {
    fn from(amt: u64) -> Self {
        Self(amt.into())
    }
}

impl CheckedSub for FeeAmount {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        self.0.checked_sub(v.0).map(FeeAmount)
    }
}

impl FeeAmount {
    pub(crate) fn as_u64(&self) -> Option<u64> {
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
}
