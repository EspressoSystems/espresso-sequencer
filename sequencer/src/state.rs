use crate::{Header, L1BlockInfo, Leaf, NodeState, SeqTypes};
use anyhow::{ensure, Context};
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, Read, SerializationError, Valid, Validate,
};
use commit::{Commitment, Committable, RawCommitmentBuilder};
use derive_more::{Add, From, Into, Sub};
use ethers::{
    abi::Address,
    core::k256::ecdsa::SigningKey,
    signers::{coins_bip39::English, MnemonicBuilder, Signer, Wallet},
    types::{self, RecoveryMessage, U256},
};
use hotshot::traits::ValidatedState as HotShotState;
use hotshot_types::data::{BlockError, ViewNumber};
use jf_primitives::merkle_tree::{
    prelude::{LightWeightSHA3MerkleTree, Sha3Digest, Sha3Node},
    universal_merkle_tree::UniversalMerkleTree,
    AppendableMerkleTreeScheme, ForgetableMerkleTreeScheme, ForgetableUniversalMerkleTreeScheme,
    LookupResult, MerkleCommitment, MerkleTreeScheme,
};
use jf_primitives::merkle_tree::{ToTraversalPath, UniversalMerkleTreeScheme};
use num_traits::CheckedSub;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use typenum::Unsigned;

#[derive(Hash, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ValidatedState {
    /// Frontier of Block Merkle Tree
    pub block_merkle_tree: BlockMerkleTree,
    /// Fee Merkle Tree
    pub fee_merkle_tree: FeeMerkleTree,
}

impl Default for ValidatedState {
    fn default() -> Self {
        let block_merkle_tree =
            BlockMerkleTree::from_elems(Some(32), Vec::<Commitment<Header>>::new()).unwrap();

        // Words of wisdom from @mrain: "capacity = arity^height"
        // "For index space 2^160, arity 256 (2^8),
        // you should set the height as 160/8=20"
        let fee_merkle_tree =
            FeeMerkleTree::from_kv_set(20, Vec::<(FeeAccount, FeeAmount)>::new()).unwrap();
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
}

pub fn validate_proposal(
    state: &mut ValidatedState,
    parent: &Header,
    proposal: &Header,
) -> anyhow::Result<()> {
    // validate height
    anyhow::ensure!(
        proposal.height == parent.height + 1,
        anyhow::anyhow!(
            "Invalid Height Error: {}, {}",
            parent.height,
            proposal.height
        )
    );

    let ValidatedState {
        block_merkle_tree,
        fee_merkle_tree,
    } = state;

    // validate proposal is descendent of parent by appending to parent
    block_merkle_tree.push(parent.commit()).unwrap();
    let block_merkle_tree_root = block_merkle_tree.commitment();
    anyhow::ensure!(
        proposal.block_merkle_tree_root == block_merkle_tree_root,
        anyhow::anyhow!(
            "Invalid Block Root Error: {}, {}",
            block_merkle_tree_root,
            proposal.block_merkle_tree_root
        )
    );

    // fetch receipts from the l1
    let receipts = fetch_fee_receipts(parent.l1_finalized, proposal.l1_finalized);
    for FeeInfo { account, amount } in receipts {
        // Get the balance in order to add amount, ignoring the proof.
        match fee_merkle_tree.universal_lookup(account) {
            LookupResult::Ok(balance, _) => fee_merkle_tree
                .update(account, balance.add(amount))
                .unwrap(),
            // Handle `NotFound` and `NotInMemory` by initializing
            // state.
            _ => fee_merkle_tree.update(account, amount).unwrap(),
        };
    }

    let fee_merkle_tree_root = fee_merkle_tree.commitment();
    anyhow::ensure!(
        proposal.fee_merkle_tree_root == fee_merkle_tree_root,
        anyhow::anyhow!(
            "Invalid Fee Root Error: {}, {}",
            fee_merkle_tree_root,
            proposal.fee_merkle_tree_root
        )
    );
    Ok(())
}

/// Fetch receipts from the l1 and add them to local balance.
fn update_balance(fee_merkle_tree: &mut FeeMerkleTree, parent: &Header, proposed: &Header) {
    let receipts = fetch_fee_receipts(parent.l1_finalized, proposed.l1_finalized);
    for FeeInfo { account, amount } in receipts {
        // Add `amount` to `balance`, if `balance` is `None` set it to `amount`
        #[allow(clippy::single_match)]
        match fee_merkle_tree.update_with(account, |balance| {
            Some(balance.cloned().unwrap_or_default().add(amount))
        }) {
            Ok(LookupResult::Ok(..)) => (),
            // TODO handle `LookupResult::NotInMemory` by looking up the value from
            // a peer during catchup.
            _ => (),
        }
    }
}
/// Validate builder account by verifiying signature and charging the account.
fn validate_builder(
    fee_merkle_tree: &mut FeeMerkleTree,
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
    let mut fee_merkle_tree = fee_merkle_tree.clone();
    match fee_merkle_tree.universal_lookup(fee_info.account) {
        LookupResult::Ok(balance, _) => {
            let updated = balance
                .checked_sub(&fee_info.amount)
                .ok_or_else(|| anyhow::anyhow!("Insufficient funds"))?;
            fee_merkle_tree.update(fee_info.account, updated).unwrap();
        }
        LookupResult::NotFound(_) => {
            anyhow::bail!(format!(
                "Account Not Found {:?}",
                fee_info.account.address()
            ));
        }
        LookupResult::NotInMemory => {
            anyhow::bail!(format!(
                "Invalid Builder Account {:?}",
                fee_info.account.address()
            ));
        }
    };
    Ok(())
}

impl HotShotState<SeqTypes> for ValidatedState {
    type Error = BlockError;
    type Instance = NodeState;

    type Time = ViewNumber;

    fn on_commit(&self) {}
    /// Validate parent against known values (from state) and validate
    /// proposal descends from parent. Returns updated `ValidatedState`.
    async fn validate_and_apply_header(
        &self,
        _instance: &Self::Instance,
        parent_leaf: &Leaf,
        proposed_header: &Header,
    ) -> Result<Self, Self::Error> {
        // Clone state to avoid mutation. Consumer can take update
        // through returned value.
        let mut validated_state = self.clone();
        let parent_header = parent_leaf.get_block_header();
        // validate proposed header against parent
        match validate_proposal(&mut validated_state, parent_header, proposed_header) {
            // Note that currently only block state is updated.
            Ok(validated_state) => validated_state,
            Err(e) => {
                tracing::warn!("Invalid Proposal: {}", e);
                return Err(BlockError::InvalidBlockHeader);
            }
        };

        // Update account balance from the l1
        update_balance(
            &mut validated_state.fee_merkle_tree,
            parent_header,
            proposed_header,
        );

        // Validate builder by verifying signature and charging account
        if let Err(e) = validate_builder(&mut validated_state.fee_merkle_tree, proposed_header) {
            tracing::warn!("Invalid Builder: {}", e);
            return Err(BlockError::InvalidBlockHeader);
        }

        Ok(validated_state)
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
    fn genesis(instance: &Self::Instance) -> Self {
        instance.genesis_state.clone()
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
    pub fn new(account: FeeAccount) -> Self {
        let amount = FeeAmount::default(); // TODO grab from config (instance_state?)
        Self { account, amount }
    }
    pub fn account(&self) -> FeeAccount {
        self.account
    }
    pub fn amount(&self) -> FeeAmount {
        self.amount
    }
}

impl Default for FeeInfo {
    fn default() -> Self {
        Self {
            amount: FeeAmount::default(),
            account: FeeAccount::test_wallet().address().into(),
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
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    From,
    Into,
)]
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

impl<A: Unsigned> ToTraversalPath<A> for FeeAccount {
    fn to_traversal_path(&self, height: usize) -> Vec<usize> {
        self.0
            .to_fixed_bytes()
            .into_iter()
            .take(height)
            .map(|i| i as usize)
            .collect()
    }
}

/// Fetch all deposit receitps between `prev_l1_finalized` and
/// `new_l1_finalized`. This is currently a mock function to be
/// implemented in the future.
pub fn fetch_fee_receipts(
    _prev_l1_finalized: Option<L1BlockInfo>,
    _new_l1_finalized: Option<L1BlockInfo>,
) -> Vec<FeeInfo> {
    Vec::from([FeeInfo::default()])
}

pub type FeeMerkleTree =
    UniversalMerkleTree<FeeAmount, Sha3Digest, FeeAccount, typenum::U256, Sha3Node>;
pub type FeeMerkleCommitment = <FeeMerkleTree as MerkleTreeScheme>::Commitment;

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
