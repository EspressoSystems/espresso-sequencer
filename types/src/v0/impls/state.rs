use std::collections::HashSet;
use std::ops::Add;

use super::{
    v0_1, v0_2, BlockMerkleCommitment, BuilderSignature, FeeInfo, FeeMerkleCommitment, Header,
    L1BlockInfo, NameSpaceTable, ResolvableChainConfig, TxTableEntryWord,
};
use crate::{
    v0_1::ValidatedState, BlockMerkleTree, ChainConfig, FeeAccount, FeeAmount, FeeMerkleTree,
    NsTable,
};
use crate::{NodeState, SeqTypes};
use anyhow::ensure;
use anyhow::{anyhow, Context};
use ark_serialize::CanonicalSerialize;
use committable::{Commitment, Committable, RawCommitmentBuilder};
use hotshot_types::data::{BlockError, ViewNumber};
use hotshot_types::traits::block_contents::BlockHeader;
use hotshot_types::traits::node_implementation::ConsensusTime;
use hotshot_types::traits::signature_key::BuilderSignatureKey;
use hotshot_types::traits::states::StateDelta;
use hotshot_types::traits::ValidatedState as HotShotState;
use hotshot_types::vid::{VidCommon, VidSchemeType};
use hotshot_types::{
    data::Leaf, traits::block_contents::BuilderFee, utils::BuilderCommitment, vid::VidCommitment,
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
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use vbs::version::Version;

// todo: move
const BLOCK_MERKLE_TREE_HEIGHT: usize = 32;
const FEE_MERKLE_TREE_HEIGHT: usize = 20;

impl Default for ValidatedState {
    fn default() -> Self {
        let header_commit = Vec::<[u8; 32]>::new();

        let block_merkle_tree =
            BlockMerkleTree::from_elems(Some(BLOCK_MERKLE_TREE_HEIGHT), header_commit).unwrap();

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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Delta {
    pub fees_delta: HashSet<FeeAccount>,
}

impl StateDelta for Delta {}

impl ValidatedState {
    pub(crate) async fn apply_header(
        &self,
        instance: &NodeState,
        parent_leaf: &Leaf<SeqTypes>,
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
                proposed_header.fee_info().account,
                instance.chain_config().fee_recipient,
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
            instance.chain_config().fee_recipient,
        )?;

        Ok((validated_state, delta))
    }
}

pub async fn get_l1_deposits(
    instance: &NodeState,
    header: &Header,
    parent_leaf: &Leaf<SeqTypes>,
) -> Vec<FeeInfo> {
    if let (Some(addr), Some(block_info)) =
        (instance.chain_config.fee_contract, header.l1_finalized())
    {
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
    parent_leaf: &Leaf<SeqTypes>,
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
        parent_leaf: &Leaf<SeqTypes>,
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
        }
    }
    /// Construct a genesis validated state.
    #[must_use]
    fn genesis(instance: &Self::Instance) -> (Self, Self::Delta) {
        (instance.genesis_state.clone(), Delta::default())
    }
}

pub fn validate_proposal(
    state: &ValidatedState,
    expected_chain_config: ChainConfig,
    parent_leaf: &Leaf<SeqTypes>,
    proposal: &Header,
    vid_common: &VidCommon,
) -> anyhow::Result<()> {
    let parent_header = parent_leaf.block_header();

    // validate `ChainConfig`
    anyhow::ensure!(
        proposal.chain_config().commit() == expected_chain_config.commit(),
        anyhow::anyhow!(
            "Invalid Chain Config: local={:?}, proposal={:?}",
            expected_chain_config,
            proposal.chain_config()
        )
    );

    // validate block size and fee
    let block_size = VidSchemeType::get_payload_byte_len(vid_common) as u64;
    anyhow::ensure!(
        block_size < expected_chain_config.max_block_size,
        anyhow::anyhow!(
            "Invalid Payload Size: local={:?}, proposal={:?}",
            expected_chain_config,
            proposal.chain_config()
        )
    );
    anyhow::ensure!(
        proposal.fee_info().amount() >= expected_chain_config.base_fee * block_size,
        format!(
            "insufficient fee: block_size={block_size}, base_fee={:?}, proposed_fee={:?}",
            expected_chain_config.base_fee,
            proposal.fee_info().amount()
        )
    );

    // validate height
    anyhow::ensure!(
        proposal.height() == parent_header.height() + 1,
        anyhow::anyhow!(
            "Invalid Height Error: {}, {}",
            parent_header.height(),
            proposal.height()
        )
    );

    let ValidatedState {
        block_merkle_tree,
        fee_merkle_tree,
    } = state;

    let block_merkle_tree_root = block_merkle_tree.commitment();
    anyhow::ensure!(
        proposal.block_merkle_tree_root() == block_merkle_tree_root,
        anyhow::anyhow!(
            "Invalid Block Root Error: local={}, proposal={}",
            block_merkle_tree_root,
            proposal.block_merkle_tree_root()
        )
    );

    let fee_merkle_tree_root = fee_merkle_tree.commitment();
    anyhow::ensure!(
        proposal.fee_merkle_tree_root() == fee_merkle_tree_root,
        anyhow::anyhow!(
            "Invalid Fee Root Error: local={}, proposal={}",
            fee_merkle_tree_root,
            proposal.fee_merkle_tree_root()
        )
    );
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
        .builder_signature()
        .ok_or_else(|| anyhow::anyhow!("Builder signature not found"))?;
    let fee_amount = proposed_header
        .fee_info()
        .amount()
        .as_u64()
        .context(format!(
            "fee amount out of range: {:?}",
            proposed_header.fee_info().amount()
        ))?;
    // verify signature
    anyhow::ensure!(
        proposed_header.fee_info().account.validate_fee_signature(
            &signature,
            fee_amount,
            proposed_header.metadata(),
            &proposed_header.payload_commitment()
        ),
        "Invalid Builder Signature"
    );

    Ok(())
}
