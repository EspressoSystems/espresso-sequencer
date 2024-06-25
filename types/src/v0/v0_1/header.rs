use crate::NsTable;

use super::{
    BlockMerkleCommitment, BuilderSignature, FeeInfo, FeeMerkleCommitment, L1BlockInfo,
    ResolvableChainConfig,
};
use hotshot_types::{utils::BuilderCommitment, vid::VidCommitment};
use serde::{Deserialize, Serialize};

/// A header is like a [`Block`] with the body replaced by a digest.
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Header {
    /// A commitment to a ChainConfig or a full ChainConfig.
    pub(crate) chain_config: ResolvableChainConfig,

    pub(crate) height: u64,
    pub(crate) timestamp: u64,

    /// The Espresso block header includes a reference to the current head of the L1 chain.
    ///
    /// Rollups can use this to facilitate bridging between the L1 and L2 in a deterministic way.
    /// This field deterministically associates an L2 block with a recent L1 block the instant the
    /// L2 block is sequenced. Rollups can then define the L2 state after this block as the state
    /// obtained by executing all the transactions in this block _plus_ all the L1 deposits up to
    /// the given L1 block number. Since there is no need to wait for the L2 block to be reflected
    /// on the L1, this bridge design retains the low confirmation latency of HotShot.
    ///
    /// This block number indicates the unsafe head of the L1 chain, so it is subject to reorgs. For
    /// this reason, the Espresso header does not include any information that might change in a
    /// reorg, such as the L1 block timestamp or hash. It includes only the L1 block number, which
    /// will always refer to _some_ block after a reorg: if the L1 head at the time this block was
    /// sequenced gets reorged out, the L1 chain will eventually (and probably quickly) grow to the
    /// same height once again, and a different block will exist with the same height. In this way,
    /// Espresso does not have to handle L1 reorgs, and the Espresso blockchain will always be
    /// reflective of the current state of the L1 blockchain. Rollups that use this block number
    /// _do_ have to handle L1 reorgs, but each rollup and each rollup client can decide how many
    /// confirmations they want to wait for on top of this `l1_head` before they consider an L2
    /// block finalized. This offers a tradeoff between low-latency L1-L2 bridges and finality.
    ///
    /// Rollups that want a stronger guarantee of finality, or that want Espresso to attest to data
    /// from the L1 block that might change in reorgs, can instead use the latest L1 _finalized_
    /// block at the time this L2 block was sequenced: `l1_finalized`.
    pub(crate) l1_head: u64,

    /// The Espresso block header includes information a bout the latest finalized L1 block.
    ///
    /// Similar to `l1_head`, rollups can use this information to implement a bridge between the L1
    /// and L2 while retaining the finality of low-latency block confirmations from HotShot. Since
    /// this information describes the finalized L1 block, a bridge using this L1 block will have
    /// much higher latency than a bridge using `l1_head`. In exchange, rollups that use the
    /// finalized block do not have to worry about L1 reorgs, and can inject verifiable attestations
    /// to the L1 block metadata (such as its timestamp or hash) into their execution layers, since
    /// Espresso replicas will sign this information for the finalized L1 block.
    ///
    /// This block may be `None` in the rare case where Espresso has started shortly after the
    /// genesis of the L1, and the L1 has yet to finalize a block. In all other cases it will be
    /// `Some`.
    pub(crate) l1_finalized: Option<L1BlockInfo>,

    pub(crate) payload_commitment: VidCommitment,
    pub(crate) builder_commitment: BuilderCommitment,
    pub(crate) ns_table: NsTable,
    /// Root Commitment of Block Merkle Tree
    pub(crate) block_merkle_tree_root: BlockMerkleCommitment,
    /// Root Commitment of `FeeMerkleTree`
    pub(crate) fee_merkle_tree_root: FeeMerkleCommitment,
    /// Fee paid by the block builder
    pub(crate) fee_info: FeeInfo,
    /// Account (etheruem address) of builder
    ///
    /// This signature is not considered formally part of the header; it is just evidence proving
    /// that other parts of the header (`fee_info`) are correct. It exists in the header so that it
    /// is available to all nodes to be used during validation. But since it is checked during
    /// consensus, any downstream client who has a proof of consensus finality of a header can trust
    /// that `fee_info` is correct without relying on the signature. Thus, this signature is not
    /// included in the header commitment.
    pub(crate) builder_signature: Option<BuilderSignature>,
}
