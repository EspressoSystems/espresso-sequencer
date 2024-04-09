use crate::{
    api::endpoints::AccountQueryData, catchup::StateCatchup, Header, Leaf, NodeState, SeqTypes,
};
use anyhow::{bail, ensure, Context};
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, Read, SerializationError, Valid, Validate,
};
use async_std::{
    stream::StreamExt,
    sync::{RwLock, RwLockReadGuard},
};
use async_trait::async_trait;
use committable::{Commitment, Committable, RawCommitmentBuilder};
use contract_bindings::fee_contract::DepositFilter;
use derive_more::{Add, Display, From, Into, Sub};
use ethers::{
    abi::Address,
    core::k256::ecdsa::SigningKey,
    signers::{coins_bip39::English, MnemonicBuilder, Wallet},
    types::{self, RecoveryMessage, U256},
};
use hotshot::traits::ValidatedState as HotShotState;
use hotshot_query_service::{
    data_source::storage::SqlStorage,
    merklized_state::{MerklizedState, Snapshot},
};
use hotshot_types::{
    data::{BlockError, ViewNumber},
    traits::{node_implementation::ConsensusTime, states::StateDelta},
};
use itertools::Itertools;
use jf_primitives::merkle_tree::{
    prelude::{MerkleNode, MerkleProof},
    ToTraversalPath, UniversalMerkleTreeScheme,
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
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, ops::Add, str::FromStr, sync::Arc};
use typenum::{Unsigned, U3};

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

pub fn validate_and_apply_proposal(
    state: &mut ValidatedState,
    delta: &mut Delta,
    parent_leaf: &Leaf,
    proposal: &Header,
    receipts: Vec<FeeInfo>,
) -> anyhow::Result<()> {
    let parent_header = parent_leaf.get_block_header();
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

    // validate proposal is descendent of parent by appending to parent
    block_merkle_tree.push(parent_header.commit()).unwrap();
    let block_merkle_tree_root = block_merkle_tree.commitment();
    anyhow::ensure!(
        proposal.block_merkle_tree_root == block_merkle_tree_root,
        anyhow::anyhow!(
            "Invalid Block Root Error: local={}, proposal={}",
            block_merkle_tree_root,
            proposal.block_merkle_tree_root
        )
    );

    // Insert the fee deposits
    for FeeInfo { account, amount } in receipts {
        fee_merkle_tree
            .update_with(account, |balance| {
                Some(balance.cloned().unwrap_or_default().add(amount))
            })
            .expect("update_with succeeds");

        delta.fees_delta.insert(account);
    }

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

#[derive(Debug)]
enum ChargeFeeError {
    /// Account not in memory, needs to be fetched from peer
    NotInMemory,
    /// Account exists but has insufficient funds
    InsufficientFunds,
}

fn charge_fee(
    fee_merkle_tree: &mut FeeMerkleTree,
    fee_info: FeeInfo,
) -> Result<(), ChargeFeeError> {
    let FeeInfo { account, amount } = fee_info;
    let mut err = None;
    let res = fee_merkle_tree
        .update_with(account, |balance| {
            let balance = balance.copied();
            let Some(updated) = balance.unwrap_or_default().checked_sub(&amount) else {
                // Return an error without updating the account.
                err = Some(ChargeFeeError::InsufficientFunds);
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
        })
        .expect("updated succeeds");
    if res.expect_not_in_memory().is_ok() {
        return Err(ChargeFeeError::NotInMemory);
    }
    if let Some(err) = err {
        Err(err)
    } else {
        Ok(())
    }
}

/// Validate builder account by verifying signature and charging the account.
fn validate_and_charge_builder(
    fee_merkle_tree: &mut FeeMerkleTree,
    delta: &mut Delta,
    proposed_header: &Header,
) -> anyhow::Result<()> {
    // Beware of Malice!
    let builder_signature = proposed_header
        .builder_signature
        .ok_or_else(|| anyhow::anyhow!("Builder signature not found"))?;

    let fee_info = proposed_header.fee_info;
    // verify signature
    anyhow::ensure!(
        builder_signature
            .verify(
                RecoveryMessage::Hash(types::H256(proposed_header.commit().into())),
                fee_info.account.address()
            )
            .is_ok(),
        "Invalid Builder Signature"
    );

    // charge the fee to the builder
    if charge_fee(fee_merkle_tree, fee_info).is_err() {
        bail!("Insufficient funds")
    };

    delta.fees_delta.insert(fee_info.account);
    Ok(())
}

/// A pure function to validate and apply a header to the state.
///
/// It assumes that all state required to validate and apply the header
/// is available in the `validated_state`.
fn validate_and_apply_header(
    validated_state: &mut ValidatedState,
    delta: &mut Delta,
    parent_leaf: &Leaf,
    proposed_header: &Header,
    l1_deposits: Vec<FeeInfo>,
) -> Result<(), BlockError> {
    // validate proposed header against parent
    match validate_and_apply_proposal(
        validated_state,
        delta,
        parent_leaf,
        proposed_header,
        l1_deposits,
    ) {
        // Note that currently only block state is updated.
        Ok(validated_state) => validated_state,
        Err(e) => {
            tracing::warn!("Invalid Proposal: {}", e);
            return Err(BlockError::InvalidBlockHeader);
        }
    };

    // Validate builder by verifying signature and charging account
    if let Err(e) =
        validate_and_charge_builder(&mut validated_state.fee_merkle_tree, delta, proposed_header)
    {
        tracing::warn!("Invalid Builder: {}", e);
        return Err(BlockError::InvalidBlockHeader);
    };

    Ok(())
}

use core::fmt::Debug;
use hotshot_query_service::data_source::SqlDataSource;
#[derive(Debug)]
struct SqlStateCatchup<'a> {
    db: RwLockReadGuard<'a, SqlDataSource<SeqTypes, Provider>>,
    view_number: ViewNumber,
    block_height: u64,
}

#[async_trait]
impl<'a> StateCatchup for SqlStateCatchup<'a> {
    async fn fetch_accounts(
        &self,
        _view: ViewNumber,
        _fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> Vec<AccountQueryData> {
        let mut ret = vec![];
        // for account in accounts {
        //     let block_height = self.block_height;

        //     let path = self
        //         .db
        //         .get_path(
        //             Snapshot::<SeqTypes, FeeMerkleTree, 256>::Index(block_height),
        //             account,
        //         )
        //         .await
        //         .unwrap();
        // }
        ret
    }

    async fn remember_blocks_merkle_tree(&self, view: ViewNumber, mt: &mut BlockMerkleTree) {
        // let view = view.get_u64();

        // if view == 1 {
        //     return;
        // }

        // let path = self
        //     .db
        //     .get_path(
        //         Snapshot::<SeqTypes, BlockMerkleTree, 3>::Index(view),
        //         view - 1,
        //     )
        //     .await
        //     .unwrap();

        // let leaf = path.clone().proof.get(0).unwrap();

        // match leaf {
        //     MerkleNode::Leaf { value, pos, elem } => {
        //         mt.remember(pos, elem, path).unwrap();
        //     }
        //     _ => panic!("ss"),
        // }
    }
}
use crate::api::data_source::Provider;
use crate::api::data_source::SequencerDataSource;
use hotshot_query_service::availability::AvailabilityDataSource;
use hotshot_query_service::merklized_state::MerklizedStateDataSource;
async fn update_state_storage(
    storage: Arc<RwLock<SqlDataSource<SeqTypes, Provider>>>,
    instance: Arc<NodeState>,
) -> anyhow::Result<()> {
    // get last saved merklized state

    let mut leaves = storage.read().await.subscribe_leaves(0).await;

    while let Some(leaf) = leaves.next().await {
        let leaf = leaf.leaf();
        let header = leaf.get_block_header();
        let validated_state = ValidatedState::from_header(header);

        let catchup = SqlStateCatchup {
            db: storage.read().await,
            view_number: leaf.get_view_number(),
            block_height: leaf.get_height(),
        };

        let (state, delta) = validated_state
            .apply_header(&instance, leaf, header, catchup)
            .await?;

        let block_number = leaf.get_height();
        let ValidatedState {
            fee_merkle_tree,
            block_merkle_tree,
        } = state;

        let Delta { fees_delta } = delta;

        // Insert block merkle tree nodes
        let (_, proof) = block_merkle_tree
            .lookup(block_number - 1)
            .expect_ok()
            .context("Index not found in block merkle tree")?;
        let path = <u64 as ToTraversalPath<3>>::to_traversal_path(
            &(block_number - 1),
            block_merkle_tree.height(),
        );

        storage
            .write()
            .await
            .store_state::<BlockMerkleTree, 3>(proof.proof, path, block_number)
            .await
            .context("failed to insert merkle nodes for block merkle tree")?;

        // Insert fee merkle tree nodes
        for delta in fees_delta {
            let (_, proof) = fee_merkle_tree
                .universal_lookup(delta)
                .expect_ok()
                .context("Index not found in fee merkle tree")?;
            let path: Vec<usize> = <FeeAccount as ToTraversalPath<256>>::to_traversal_path(
                &delta,
                fee_merkle_tree.height(),
            );

            storage
                .write()
                .await
                .store_state::<FeeMerkleTree, 256>(proof.proof, path, block_number)
                .await
                .context("failed to insert merkle nodes for block merkle tree")?;
        }
    }

    Ok(())
}

impl ValidatedState {
    async fn apply_header(
        &self,
        instance: &NodeState,
        parent_leaf: &Leaf,
        proposed_header: &Header,
        catchup: impl StateCatchup,
    ) -> anyhow::Result<(Self, Delta)> {
        // Clone state to avoid mutation. Consumer can take update
        // through returned value.

        let mut validated_state = self.clone();

        // Fetch the new L1 deposits between parent and current finalized L1 block.
        let l1_deposits = if let Some(block_info) = proposed_header.l1_finalized {
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
        };

        let mut delta = Delta::default();

        catchup
            .remember_blocks_merkle_tree(
                parent_leaf.get_view_number(),
                &mut validated_state.block_merkle_tree,
            )
            .await;

        for FeeInfo { account, amount } in l1_deposits {
            let account_proof = catchup
                .fetch_accounts(
                    parent_leaf.get_view_number(),
                    validated_state.fee_merkle_tree.commitment(),
                    vec![account],
                )
                .await;

            let proof = &account_proof.get(0).unwrap().proof;

            proof
                .remember(&mut validated_state.fee_merkle_tree)
                .unwrap();
            delta.fees_delta.insert(account);
        }

        Ok((validated_state, delta))
    }
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
        fields(view = ?parent_leaf.get_view_number(), height = parent_leaf.get_block_header().height),
    )]
    async fn validate_and_apply_header(
        &self,
        instance: &Self::Instance,
        parent_leaf: &Leaf,
        proposed_header: &Header,
    ) -> Result<(Self, Self::Delta), Self::Error> {
        // Clone state to avoid mutation. Consumer can take update
        // through returned value.
        let mut validated_state = self.clone();

        let accounts = std::iter::once(proposed_header.fee_info.account);

        // Fetch the new L1 deposits between parent and current finalized L1 block.
        let l1_deposits = if let Some(block_info) = proposed_header.l1_finalized {
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
        };

        // Find missing state entries
        let missing_accounts = self.forgotten_accounts(
            accounts.chain(l1_deposits.iter().map(|fee_info| fee_info.account)),
        );

        let view = parent_leaf.get_view_number();

        // Ensure merkle tree has frontier
        if self.need_to_fetch_blocks_mt_frontier() {
            tracing::warn!("fetching block frontier from peers");
            instance
                .peers
                .as_ref()
                .remember_blocks_merkle_tree(view, &mut validated_state.block_merkle_tree)
                .await;
        }

        // Fetch missing fee state entries
        if !missing_accounts.is_empty() {
            tracing::warn!(
                "fetching {} missing accounts from peers",
                missing_accounts.len()
            );

            let missing_account_proofs = instance
                .peers
                .as_ref()
                .fetch_accounts(
                    view,
                    validated_state.fee_merkle_tree.commitment(),
                    missing_accounts,
                )
                .await;

            // Remember the fee state entries
            for account in missing_account_proofs.iter() {
                account
                    .proof
                    .remember(&mut validated_state.fee_merkle_tree)
                    .expect("proof previously verified");
            }
        }

        let mut delta = Delta::default();

        // Lastly validate and apply the header
        validate_and_apply_header(
            &mut validated_state,
            &mut delta,
            parent_leaf,
            proposed_header,
            l1_deposits,
        )?;

        Ok((validated_state, delta))
    }
    /// Construct the state with the given block header.
    ///
    /// This can also be used to rebuild the state for catchup.
    fn from_header(block_header: &Header) -> Self {
        let fee_merkle_tree = FeeMerkleTree::from_commitment(block_header.fee_merkle_tree_root);
        let block_merkle_tree = BlockMerkleTree::from_commitment(block_header.fee_merkle_tree_root);
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

impl MerklizedState<SeqTypes, 3> for BlockMerkleTree {
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
    Default, Hash, Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Add, Sub, From, Into,
)]
pub struct FeeAmount(U256);
impl FeeAmount {
    /// Return array containing underlying bytes of inner `U256` type
    fn to_fixed_bytes(self) -> [u8; 32] {
        let mut bytes = [0u8; core::mem::size_of::<U256>()];
        self.0.to_little_endian(&mut bytes);
        bytes
    }
}

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
    pub fn test_wallet() -> Wallet<SigningKey> {
        let phrase = "test test test test test test test test test test test junk";
        MnemonicBuilder::<English>::default()
            .phrase::<&str>(phrase)
            .build()
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
