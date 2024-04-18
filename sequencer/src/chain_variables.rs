use committable::{Commitment, Committable};
use serde::{Deserialize, Serialize};

/// Global variables for an Espresso blockchain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainVariables {
    /// The version of the protocol this chain is currently using.
    ///
    /// The protocol version can be changed by committing an update transaction.
    // TODO
    // pub protocol_version: (u16, u16, u16),

    /// A unique identifier for this chain, to prevent cross-chain replay attacks.
    ///
    /// The chain ID is set at genesis and never changes.
    pub chain_id: u16,
}

impl Default for ChainVariables {
    fn default() -> Self {
        Self::new(
            35353, // Arbitrarily chosen.
        )
    }
}

impl ChainVariables {
    pub fn new(chain_id: u16) -> Self {
        Self { chain_id }
    }
}

impl Committable for ChainVariables {
    fn commit(&self) -> Commitment<Self> {
        committable::RawCommitmentBuilder::new("ChainVariables")
            .u64_field("chain_id", self.chain_id as u64)
            .finalize()
    }
}
