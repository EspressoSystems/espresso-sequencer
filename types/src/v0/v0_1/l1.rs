use committable::{Commitment, Committable, RawCommitmentBuilder};
use ethers::prelude::{H256, U256};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct L1BlockInfo {
    pub(crate) number: u64,
    pub(crate) timestamp: U256,
    pub(crate) hash: H256,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct L1Snapshot {
    /// The relevant snapshot of the L1 includes a reference to the current head of the L1 chain.
    ///
    /// Note that the L1 head is subject to changing due to a reorg. However, no reorg will change
    /// the _number_ of this block in the chain: L1 block numbers will always be sequentially
    /// increasing. Therefore, the sequencer does not have to worry about reorgs invalidating this
    /// snapshot.
    pub head: u64,

    /// The snapshot also includes information about the latest finalized L1 block.
    ///
    /// Since this block is finalized (ie cannot be reorged) we can include specific information
    /// about the particular block, such as its hash and timestamp.
    ///
    /// This block may be `None` in the rare case where Espresso has started shortly after the
    /// genesis of the L1, and the L1 has yet to finalize a block. In all other cases it will be
    /// `Some`.
    pub finalized: Option<L1BlockInfo>,
}
