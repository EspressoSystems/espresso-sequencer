use crate::{Header, NodeState, Payload};
use anyhow::{ensure, Context};
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, Read, SerializationError, Valid, Validate,
};
use commit::{Commitment, Committable, RawCommitmentBuilder};
use derive_more::{Add, From, Into, Sub};
use ethers::{abi::Address, types::U256};
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
fn update_balance(fee_merkle_tree: &mut FeeMerkleTree, parent: &Header) {
    let receipts = fetch_fee_receipts(parent);
    for FeeReceipt { recipient, amount } in receipts {
        // Add `amount` to `balance`, if `balance` is `None` set it to `amount`
        #[allow(clippy::single_match)]
        match fee_merkle_tree.update_with(recipient, |balance| {
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
                AsRef::<[u8]>::as_ref(&proposed_header.commit()),
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
            anyhow::bail!("Account Not Found");
        }
        LookupResult::NotInMemory => {
            anyhow::bail!("Invalid Builder Account");
        }
    };
    Ok(())
}

impl HotShotState for ValidatedState {
    type Error = BlockError;
    type Instance = NodeState;
    type BlockHeader = Header;
    type BlockPayload = Payload;

    type Time = ViewNumber;

    fn on_commit(&self) {}
    /// Validate parent against known values (from state) and validate
    /// proposal descends from parent. Returns updated `ValidatedState`.
    fn validate_and_apply_header(
        &self,
        _instance: &Self::Instance,
        parent_header: &Self::BlockHeader,
        proposed_header: &Self::BlockHeader,
    ) -> Result<Self, Self::Error> {
        // Clone state to avoid mutation. Consumer can take update
        // through returned value.
        let mut validated_state = self.clone();
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
        update_balance(&mut validated_state.fee_merkle_tree, parent_header);

        // Validate builder by verifying signature and charging account
        if let Err(e) = validate_builder(&mut validated_state.fee_merkle_tree, parent_header) {
            tracing::warn!("Invalid Builder: {}", e);
            return Err(BlockError::InvalidBlockHeader);
        }

        Ok(validated_state)
    }
    /// Construct the state with the given block header.
    ///
    /// This can also be used to rebuild the state for catchup.
    fn from_header(block_header: &Self::BlockHeader) -> Self {
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
impl hotshot_types::traits::states::TestableState for ValidatedState {
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
    Default,
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
}

impl Committable for FeeInfo {
    fn commit(&self) -> Commitment<Self> {
        let mut comm_bytes = vec![];
        self.serialize_with_mode(&mut comm_bytes, ark_serialize::Compress::Yes)
            .unwrap();
        RawCommitmentBuilder::new(&Self::tag())
            .var_size_field("fee_info", &comm_bytes)
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
    pub fn address(&self) -> Address {
        self.0
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
        let mut bytes = [0u8; core::mem::size_of::<U256>()];
        self.0.to_little_endian(&mut bytes);
        Ok(writer.write_all(&bytes)?)
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
        Ok(writer.write_all(self.0.as_bytes())?)
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
        Address::to_fixed_bytes(self.0)
            .into_iter()
            .take(height)
            .map(|i| i as usize)
            .collect()
    }
}

#[derive(Default, Hash, Clone, CanonicalDeserialize)]
pub struct FeeReceipt {
    pub recipient: FeeAccount,
    pub amount: FeeAmount,
}
/// Fetch fee receitps from l1. Currently a mock function to be
/// implemented in the future.
pub fn fetch_fee_receipts(_parent: &Header) -> Vec<FeeReceipt> {
    Vec::from([FeeReceipt::default()])
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
