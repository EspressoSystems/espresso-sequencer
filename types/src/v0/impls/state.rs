use std::{ops::Add, sync::Arc};

use anyhow::bail;
use async_trait::async_trait;
use committable::{Commitment, Committable};
use ethers::types::Address;
use hotshot_query_service::merklized_state::MerklizedState;
use hotshot_types::{
    data::{BlockError, ViewNumber},
    traits::{
        block_contents::BlockHeader, node_implementation::ConsensusTime,
        signature_key::BuilderSignatureKey, states::StateDelta, ValidatedState as HotShotState,
    },
    vid::{VidCommon, VidSchemeType},
};
use itertools::Itertools;
use jf_merkle_tree::{
    prelude::{MerkleProof, Sha3Digest, Sha3Node},
    AppendableMerkleTreeScheme, ForgetableMerkleTreeScheme, ForgetableUniversalMerkleTreeScheme,
    LookupResult, MerkleCommitment, MerkleTreeError, MerkleTreeScheme,
    PersistentUniversalMerkleTreeScheme, UniversalMerkleTreeScheme,
};
use jf_vid::VidScheme;
use num_traits::CheckedSub;
use vbs::version::Version;

use crate::{
    constants::{BLOCK_MERKLE_TREE_HEIGHT, FEE_MERKLE_TREE_HEIGHT},
    traits::StateCatchup,
    v0_1::BackoffParams,
    AccountQueryData, BlockMerkleTree, BuilderValidationError, ChainConfig, Delta, FeeAccount,
    FeeAmount, FeeError, FeeInfo, FeeMerkleCommitment, FeeMerkleTree, Header, Leaf, NodeState,
    NsTableValidationError, PayloadByteLen, ProposalValidationError, ResolvableChainConfig,
    SeqTypes, UpgradeType, ValidatedState,
};

impl StateDelta for Delta {}

impl Default for ValidatedState {
    fn default() -> Self {
        let block_merkle_tree =
            BlockMerkleTree::from_elems(Some(BLOCK_MERKLE_TREE_HEIGHT), Vec::<[u8; 32]>::new())
                .unwrap();

        // Words of wisdom from @mrain: "capacity = arity^height"
        // "For index space 2^160, arity 256 (2^8),
        // you should set the height as 160/8=20"
        let fee_merkle_tree = FeeMerkleTree::from_kv_set(
            FEE_MERKLE_TREE_HEIGHT,
            Vec::<(FeeAccount, FeeAmount)>::new(),
        )
        .unwrap();

        let chain_config = ResolvableChainConfig::from(ChainConfig::default());

        Self {
            block_merkle_tree,
            fee_merkle_tree,
            chain_config,
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
    pub fn charge_fee(&mut self, fee_info: FeeInfo, recipient: FeeAccount) -> Result<(), FeeError> {
        let fee_state = self.fee_merkle_tree.clone();

        // Deduct the fee from the paying account.
        let FeeInfo { account, amount } = fee_info;
        let mut err = None;
        let fee_state = fee_state.persistent_update_with(account, |balance| {
            let balance = balance.copied();
            let Some(updated) = balance.unwrap_or_default().checked_sub(&amount) else {
                // Return an error without updating the account.
                err = Some(FeeError::InsufficientFunds { balance, amount });
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
            chain_config: ResolvableChainConfig::from(self.chain_config.commit()),
        }
    }
}

impl From<NsTableValidationError> for ProposalValidationError {
    fn from(err: NsTableValidationError) -> Self {
        Self::InvalidNsTable { err }
    }
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
    if proposal.chain_config().commit() != expected_chain_config.commit() {
        return Err(ProposalValidationError::InvalidChainConfig {
            expected: format!("{:?}", expected_chain_config),
            proposal: format!("{:?}", proposal.chain_config()),
        });
    }

    // validate block size and fee
    let block_size = VidSchemeType::get_payload_byte_len(vid_common) as u64;
    if block_size > *expected_chain_config.max_block_size {
        return Err(ProposalValidationError::MaxBlockSizeExceeded {
            max_block_size: expected_chain_config.max_block_size,
            block_size: block_size.into(),
        });
    }

    // TODO here we are validating the each fee amount is at least
    // base_fee * block_size. Seems inappropriate to use current block
    // to calculate fees for bid on future block.
    for fee_info in proposal.fee_info().clone() {
        if fee_info.amount() < expected_chain_config.base_fee * block_size {
            return Err(ProposalValidationError::InsufficientFee {
                max_block_size: expected_chain_config.max_block_size,
                base_fee: expected_chain_config.base_fee,
                proposed_fee: fee_info.amount(),
            });
        }
    }

    // validate height
    if proposal.height() != parent_header.height() + 1 {
        return Err(ProposalValidationError::InvalidHeight {
            parent_height: parent_header.height(),
            proposal_height: proposal.height(),
        });
    }

    let ValidatedState {
        block_merkle_tree,
        fee_merkle_tree,
        ..
    } = state;

    let block_merkle_tree_root = block_merkle_tree.commitment();
    if proposal.block_merkle_tree_root() != block_merkle_tree_root {
        return Err(ProposalValidationError::InvalidBlockRoot {
            expected_root: block_merkle_tree_root,
            proposal_root: proposal.block_merkle_tree_root(),
        });
    }

    let fee_merkle_tree_root = fee_merkle_tree.commitment();
    if proposal.fee_merkle_tree_root() != fee_merkle_tree_root {
        return Err(ProposalValidationError::InvalidFeeRoot {
            expected_root: fee_merkle_tree_root,
            proposal_root: proposal.fee_merkle_tree_root(),
        });
    }

    proposal
        .ns_table()
        .validate(&PayloadByteLen::from_vid_common(vid_common))?;

    Ok(())
}

impl From<MerkleTreeError> for FeeError {
    fn from(item: MerkleTreeError) -> Self {
        Self::MerkleTreeError(item)
    }
}

fn charge_fee(
    state: &mut ValidatedState,
    delta: &mut Delta,
    fee_info: FeeInfo,
    recipient: FeeAccount,
) -> Result<(), FeeError> {
    state.charge_fee(fee_info, recipient)?;
    delta.fees_delta.extend([fee_info.account, recipient]);
    Ok(())
}

/// Validate builder accounts by verifying signatures. All fees are
/// verified against signature by index.
fn validate_builder_fee(proposed_header: &Header) -> Result<(), BuilderValidationError> {
    // TODO since we are iterating, should we include account/amount in errors?
    for (fee_info, signature) in proposed_header
        .fee_info()
        .iter()
        .zip(proposed_header.builder_signature())
    {
        // check that `amount` fits in a u64
        fee_info
            .amount
            .as_u64()
            .ok_or(BuilderValidationError::FeeAmountOutOfRange(fee_info.amount))?;

        // verify signature
        fee_info
            .account
            // TODO remove metadata, payload from trait `validate_fee_signature`
            .validate_fee_signature(&signature, fee_info.amount.as_u64().unwrap())
            .then_some(())
            .ok_or(BuilderValidationError::InvalidBuilderSignature)?;
    }

    Ok(())
}

impl ValidatedState {
    pub async fn apply_header(
        &self,
        instance: &NodeState,
        parent_leaf: &Leaf,
        proposed_header: &Header,
        version: Version,
    ) -> anyhow::Result<(Self, Delta)> {
        // Clone state to avoid mutation. Consumer can take update
        // through returned value.

        let mut validated_state = self.clone();
        validated_state.apply_upgrade(instance, version);

        let chain_config = validated_state
            .get_chain_config(instance, proposed_header.chain_config())
            .await?;

        if Some(chain_config) != validated_state.chain_config.resolve() {
            validated_state.chain_config = chain_config.into();
        }

        let l1_deposits = get_l1_deposits(
            instance,
            proposed_header,
            parent_leaf,
            chain_config.fee_contract,
        )
        .await;

        // Find missing fee state entries. We will need to use the builder account which is paying a
        // fee and the recipient account which is receiving it, plus any counts receiving deposits
        // in this block.
        let missing_accounts = self.forgotten_accounts(
            [
                proposed_header.fee_info().account,
                chain_config.fee_recipient,
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
            proposed_header.fee_info(),
            chain_config.fee_recipient,
        )?;

        Ok((validated_state, delta))
    }

    /// Updates the `ValidatedState` if a protocol upgrade has occurred.
    pub(crate) fn apply_upgrade(&mut self, instance: &NodeState, version: Version) {
        // Check for protocol upgrade based on sequencer version
        if version <= instance.current_version {
            return;
        }

        let Some(upgrade) = instance.upgrades.get(&version) else {
            return;
        };

        match upgrade.upgrade_type {
            UpgradeType::ChainConfig { chain_config } => {
                self.chain_config = chain_config.into();
            }
        }
    }

    /// Retrieves the `ChainConfig`.
    ///
    ///  Returns the `NodeState` `ChainConfig` if the `ValidatedState` `ChainConfig` commitment matches the `NodeState` `ChainConfig`` commitment.
    ///  If the commitments do not match, it returns the `ChainConfig` available in either `ValidatedState` or proposed header.
    ///  If neither has the `ChainConfig`, it fetches the config from the peers.
    ///
    /// Returns an error if it fails to fetch the `ChainConfig` from the peers.
    pub(crate) async fn get_chain_config(
        &self,
        instance: &NodeState,
        header_cf: &ResolvableChainConfig,
    ) -> anyhow::Result<ChainConfig> {
        let state_cf = self.chain_config;

        if state_cf.commit() == instance.chain_config.commit() {
            return Ok(instance.chain_config);
        }

        let cf = match (state_cf.resolve(), header_cf.resolve()) {
            (Some(cf), _) => cf,
            (_, Some(cf)) if cf.commit() == state_cf.commit() => cf,
            (_, Some(_)) | (None, None) => {
                instance
                    .peers
                    .as_ref()
                    .fetch_chain_config(state_cf.commit())
                    .await
            }
        };

        Ok(cf)
    }
}

pub async fn get_l1_deposits(
    instance: &NodeState,
    header: &Header,
    parent_leaf: &Leaf,
    fee_contract_address: Option<Address>,
) -> Vec<FeeInfo> {
    if let (Some(addr), Some(block_info)) = (fee_contract_address, header.l1_finalized()) {
        instance
            .l1_client
            .get_finalized_deposits(
                addr,
                parent_leaf
                    .block_header()
                    .l1_finalized()
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
        .push(parent_leaf.block_header().commit().as_ref())
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
        version: Version,
    ) -> Result<(Self, Self::Delta), Self::Error> {
        //validate builder fee
        if let Err(err) = validate_builder_fee(proposed_header) {
            tracing::error!("invalid builder fee: {err:#}");
            return Err(BlockError::InvalidBlockHeader);
        }

        // Unwrapping here is okay as we retry in a loop
        //so we should either get a validated state or until hotshot cancels the task
        let (validated_state, delta) = self
            .apply_header(instance, parent_leaf, proposed_header, version)
            .await
            .unwrap();

        let chain_config = validated_state
            .chain_config
            .resolve()
            .expect("Chain Config not found in validated state");

        // validate the proposal
        if let Err(err) = validate_proposal(
            &validated_state,
            chain_config,
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
        let fee_merkle_tree = if block_header.fee_merkle_tree_root().size() == 0 {
            // If the commitment tells us that the tree is supposed to be empty, it is convenient to
            // just create an empty tree, rather than a commitment-only tree.
            FeeMerkleTree::new(FEE_MERKLE_TREE_HEIGHT)
        } else {
            FeeMerkleTree::from_commitment(block_header.fee_merkle_tree_root())
        };
        let block_merkle_tree = if block_header.block_merkle_tree_root().size() == 0 {
            // If the commitment tells us that the tree is supposed to be empty, it is convenient to
            // just create an empty tree, rather than a commitment-only tree.
            BlockMerkleTree::new(BLOCK_MERKLE_TREE_HEIGHT)
        } else {
            BlockMerkleTree::from_commitment(block_header.block_merkle_tree_root())
        };
        Self {
            fee_merkle_tree,
            block_merkle_tree,
            chain_config: *block_header.chain_config(),
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

#[async_trait]
impl<T: StateCatchup + ?Sized> StateCatchup for Box<T> {
    async fn try_fetch_account(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        account: FeeAccount,
    ) -> anyhow::Result<AccountQueryData> {
        (**self)
            .try_fetch_account(height, view, fee_merkle_tree_root, account)
            .await
    }

    async fn fetch_accounts(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<AccountQueryData>> {
        (**self)
            .fetch_accounts(height, view, fee_merkle_tree_root, accounts)
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
    async fn try_fetch_account(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        account: FeeAccount,
    ) -> anyhow::Result<AccountQueryData> {
        (**self)
            .try_fetch_account(height, view, fee_merkle_tree_root, account)
            .await
    }

    async fn fetch_accounts(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<AccountQueryData>> {
        (**self)
            .fetch_accounts(height, view, fee_merkle_tree_root, accounts)
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
    #[tracing::instrument(skip(self))]
    async fn try_fetch_account(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        account: FeeAccount,
    ) -> anyhow::Result<AccountQueryData> {
        for provider in self {
            match provider
                .try_fetch_account(height, view, fee_merkle_tree_root, account)
                .await
            {
                Ok(account) => return Ok(account),
                Err(err) => {
                    tracing::warn!(%account, ?provider, "failed to fetch account: {err:#}");
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

impl MerklizedState<SeqTypes, { Self::ARITY }> for BlockMerkleTree {
    type Key = Self::Index;
    type Entry = [u8; 32];
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

#[cfg(test)]
mod test {
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use ethers::types::U256;
    use hotshot_types::vid::vid_scheme;
    use jf_vid::VidScheme;
    use sequencer_utils::ser::FromStringOrInteger;

    use super::*;
    use crate::{BlockSize, FeeAccountProof, FeeMerkleProof};

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
        assert_eq!(
            ProposalValidationError::MaxBlockSizeExceeded {
                max_block_size: instance.chain_config.max_block_size,
                block_size: BlockSize::from_integer(
                    VidSchemeType::get_payload_byte_len(&vid_common).into()
                )
                .unwrap()
            },
            err
        );
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
        assert_eq!(
            ProposalValidationError::InsufficientFee {
                max_block_size: instance.chain_config.max_block_size,
                base_fee: instance.chain_config.base_fee,
                proposed_fee: header.fee_info().amount()
            },
            err
        );
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
        let err = state.charge_fee(fee_info, dst).unwrap_err();
        assert_eq!(state.balance(src), Some(0.into()));
        assert_eq!(state.balance(dst), Some(amt));
        assert_eq!(
            FeeError::InsufficientFunds {
                balance: None,
                amount: amt
            },
            err
        );

        tracing::info!("test src not in memory");
        let mut state = new_state();
        state.fee_merkle_tree.forget(src).expect_ok().unwrap();
        assert_eq!(
            FeeError::MerkleTreeError(MerkleTreeError::ForgottenLeaf),
            state.charge_fee(fee_info, dst).unwrap_err()
        );

        tracing::info!("test dst not in memory");
        let mut state = new_state();
        state.prefund_account(dst, amt);
        state.fee_merkle_tree.forget(dst).expect_ok().unwrap();
        assert_eq!(
            FeeError::MerkleTreeError(MerkleTreeError::ForgottenLeaf),
            state.charge_fee(fee_info, dst).unwrap_err()
        );
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
