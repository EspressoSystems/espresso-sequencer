use super::{FeeAccount, FeeAmount};
use anyhow::{ensure, Context};
use ethers::types::{Address, U256};
use jf_merkle_tree::{
    prelude::{LightWeightSHA3MerkleTree, Sha3Digest, Sha3Node},
    universal_merkle_tree::UniversalMerkleTree,
    ForgetableMerkleTreeScheme, ForgetableUniversalMerkleTreeScheme, LookupResult,
    MerkleCommitment, MerkleTreeScheme, UniversalMerkleTreeScheme,
};
use serde::{Deserialize, Serialize};

// The block merkle tree accumulates header commitments. However, since the underlying
// representation of the commitment type remains the same even while the header itself changes,
// using the underlying type `[u8; 32]` allows us to use the same state type across minor versions.
pub type BlockMerkleTree = LightWeightSHA3MerkleTree<[u8; 32]>;
pub type BlockMerkleCommitment = <BlockMerkleTree as MerkleTreeScheme>::Commitment;

pub type FeeMerkleTree = UniversalMerkleTree<FeeAmount, Sha3Digest, FeeAccount, 256, Sha3Node>;
pub type FeeMerkleCommitment = <FeeMerkleTree as MerkleTreeScheme>::Commitment;

#[derive(Hash, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ValidatedState {
    /// Frontier of Block Merkle Tree
    pub block_merkle_tree: BlockMerkleTree,
    /// Fee Merkle Tree
    pub fee_merkle_tree: FeeMerkleTree,
}

/// A proof of the balance of an account in the fee ledger.
///
/// If the account of interest does not exist in the fee state, this is a Merkle non-membership
/// proof, and the balance is implicitly zero. Otherwise, this is a normal Merkle membership proof.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FeeAccountProof {
    pub account: Address,
    pub proof: FeeMerkleProof,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum FeeMerkleProof {
    Presence(<FeeMerkleTree as MerkleTreeScheme>::MembershipProof),
    Absence(<FeeMerkleTree as UniversalMerkleTreeScheme>::NonMembershipProof),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountQueryData {
    pub balance: U256,
    pub proof: FeeAccountProof,
}

impl From<(FeeAccountProof, U256)> for AccountQueryData {
    fn from((proof, balance): (FeeAccountProof, U256)) -> Self {
        Self { balance, proof }
    }
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
