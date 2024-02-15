use std::ops::Add;

use crate::{Header, NodeState, Payload};
use anyhow::{ensure, Context};
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, Read, SerializationError, Valid, Validate,
};
use commit::{Commitment, Committable};
use derive_more::Add;
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
use serde::{Deserialize, Serialize};
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
    pub fn validate_proposal(&self, parent: &Header, proposal: &Header) -> anyhow::Result<Self> {
        // validate height
        anyhow::ensure!(
            proposal.height == parent.height + 1,
            anyhow::anyhow!(
                "Invalid Height Error: {}, {}",
                parent.height,
                proposal.height
            )
        );

        let mut fee_merkle_tree = self.fee_merkle_tree.clone();
        let mut block_merkle_tree = self.block_merkle_tree.clone();

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
        let receipts = fetch_fee_receipts(parent);
        for FeeReceipt { recipient, amount } in receipts {
            // Get the balance in order to add amount, ignoring the proof.
            match fee_merkle_tree.universal_lookup(recipient) {
                LookupResult::Ok(balance, _) => fee_merkle_tree
                    .update(recipient, balance.add(amount))
                    .unwrap(),
                // Handle `NotFound` and `NotInMemory` by initializing
                // state.
                _ => fee_merkle_tree.update(recipient, amount).unwrap(),
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

        Ok(ValidatedState {
            block_merkle_tree,
            fee_merkle_tree,
        })
    }
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
        match self.validate_proposal(parent_header, proposed_header) {
            // Note that currently only block state is updated.
            Ok(validated_state) => Ok(validated_state),
            Err(e) => {
                tracing::warn!("Invalid Proposal: {}", e);
                Err(BlockError::InvalidBlockHeader)
            }
        }
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
    fn genesis(_instance: &Self::Instance) -> Self {
        ValidatedState::default()
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

// New Type for `U256` in order to implement `CanonicalSerialize` and
// `CanonicalDeserialize`
#[derive(Default, Hash, Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Add)]
pub struct FeeAmount(U256);
// New Type for `Address` in order to implement `CanonicalSerialize` and
// `CanonicalDeserialize`
#[derive(
    Default, Hash, Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct FeeAccount(Address);

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
impl std::convert::From<u64> for FeeAccount {
    fn from(item: u64) -> Self {
        FeeAccount(Address::from_low_u64_le(item))
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
