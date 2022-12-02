use commit::{Commitment, Committable};
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

    // TODO: MA: this is currently not used anywhere.
    /// Committee size
    pub committee_size: u64,
}

impl Default for ChainVariables {
    fn default() -> Self {
        Self::new(
            35353, // Arbitrarily chosen.
            3,     // Arbitrarily chosen.
        )
    }
}

impl ChainVariables {
    pub fn new(chain_id: u16, committee_size: u64) -> Self {
        Self {
            chain_id,
            committee_size,
        }
    }
}

impl Committable for ChainVariables {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("ChainVariables")
            .u64_field("chain_id", self.chain_id as u64)
            .u64_field("committee_size", self.committee_size)
            .finalize()
    }
}
