use super::{
    v0_1, v0_2, BlockMerkleCommitment, BuilderSignature, FeeInfo, FeeMerkleCommitment, Header,
    L1BlockInfo, NameSpaceTable, ResolvableChainConfig, TxTableEntryWord,
};
use committable::{Commitment, Committable, RawCommitmentBuilder};
use hotshot_types::{utils::BuilderCommitment, vid::VidCommitment};
use serde::{de::Deserializer, ser::Serializer};
use vbs::version::Version;

impl Committable for Header {
    fn commit(&self) -> Commitment<Self> {
        let mut bmt_bytes = vec![];
        self.block_merkle_tree_root()
            .serialize_with_mode(&mut bmt_bytes, ark_serialize::Compress::Yes)
            .unwrap();
        let mut fmt_bytes = vec![];
        self.fee_merkle_tree_root()
            .serialize_with_mode(&mut fmt_bytes, ark_serialize::Compress::Yes)
            .unwrap();

        let mut c = RawCommitmentBuilder::new(&Self::tag());
        if self.version() > v0_1::VERSION {
            // The original commitment scheme (from version 0.1) did not include the version in the
            // commitment. We respect this for backwards compatibility, but for all future versions,
            // we want to include it.
            c = c
                .u32_field("version_major", self.version().major)
                .u32_field("version_minor", self.version().minor);
        }
        c = c
            .field("chain_config", self.chain_config().commit())
            .u64_field("height", self.height())
            .u64_field("timestamp", self.timestamp())
            .u64_field("l1_head", self.l1_head())
            .optional("l1_finalized", &self.l1_finalized())
            .constant_str("payload_commitment")
            .fixed_size_bytes(self.payload_commitment().as_ref().as_ref())
            .constant_str("builder_commitment")
            .fixed_size_bytes(self.builder_commitment().as_ref())
            .field("ns_table", self.ns_table().commit())
            .var_size_field("block_merkle_tree_root", &bmt_bytes)
            .var_size_field("fee_merkle_tree_root", &fmt_bytes)
            .field("fee_info", self.fee_info().commit());

        if self.version() > v0_1::VERSION {
            // Added in 0.2.
            c = c.fixed_size_field("fee_recipient", self.fee_recipient().to_fixed_bytes());
        }
        c.finalize()
    }

    fn tag() -> String {
        // We use the tag "BLOCK" since blocks are identified by the hash of their header. This will
        // thus be more intuitive to users than "HEADER".
        "BLOCK".into()
    }
}

impl Header {
    pub fn versioned_serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        // Write out the underlying data directly, with no tag or version information. This
        // maintains exact compatibility with older versions, even in formats that aren't
        // self-describing. Deserialization will always use `versioned_deserialize`, and the caller
        // is responsible for knowing which version they are deserializing (for example, they should
        // know that all headers before a certain block height are 0.1 and after are 0.2).
        match self {
            Self::V1(data) => data.serialize(s),
            Self::V2(data) => data.serialize(s),
        }
    }

    pub fn versioned_deserialize<'de, D: Deserializer<'de>>(
        d: D,
        v: Version,
    ) -> Result<Self, D::Error> {
        match (v.major(), v.minor()) {
            (0, 1) => v0_1::Header::deserialize(d).map(Self::from),
            (0, 2) => v0_2::Header::deserialize(d).map(Self::from),
            _ => D::Error::custom(format!("unsupported version {v}")),
        }
    }

    pub fn version(&self) -> Version {
        match self {
            Self::V1(_) => Version::new(0, 1),
            Self::V2(_) => Version::new(0, 2),
        }
    }
}

// Getter for a field which is the same across all versions.
macro_rules! field {
    ($obj:ident.$name:ident) => {
        match $obj {
            Self::V1(data) => &data.$name,
            Self::V2(data) => &data.$name,
        }
    };
}

impl Header {
    /// A commitment to a ChainConfig or a full ChainConfig.
    pub fn chain_config(&self) -> &ResolvableChainConfig {
        field!(self.chain_config)
    }

    pub fn height(&self) -> u64 {
        *field!(self.height)
    }

    pub fn timestamp(&self) -> u64 {
        *field!(self.timestamp)
    }

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
    /// block at the time this L2 block was sequenced: [`Self::l1_finalized`].
    pub fn l1_head(&self) -> u64 {
        *field!(self.l1_head)
    }

    /// The Espresso block header includes information a bout the latest finalized L1 block.
    ///
    /// Similar to [`l1_head`](Self::l1_head), rollups can use this information to implement a
    /// bridge between the L1 and L2 while retaining the finality of low-latency block confirmations
    /// from HotShot. Since this information describes the finalized L1 block, a bridge using this
    /// L1 block will have much higher latency than a bridge using [`l1_head`](Self::l1_head). In
    /// exchange, rollups that use the finalized block do not have to worry about L1 reorgs, and can
    /// inject verifiable attestations to the L1 block metadata (such as its timestamp or hash) into
    /// their execution layers, since Espresso replicas will sign this information for the finalized
    /// L1 block.
    ///
    /// This block may be `None` in the rare case where Espresso has started shortly after the
    /// genesis of the L1, and the L1 has yet to finalize a block. In all other cases it will be
    /// `Some`.
    pub fn l1_finalized(&self) -> Option<L1BlockInfo> {
        *field!(self.l1_finalized)
    }

    pub fn payload_commitment(&self) -> VidCommitment {
        *field!(self.payload_commitment)
    }

    pub fn builder_commitment(&self) -> &BuilderCommitment {
        field!(self.builder_commitment)
    }

    pub fn ns_table(&self) -> &NameSpaceTable<TxTableEntryWord> {
        field!(self.ns_table)
    }

    /// Root Commitment of Block Merkle Tree
    pub fn block_merkle_tree_root(&self) -> BlockMerkleCommitment {
        *field!(self.block_merkle_tree_root)
    }

    /// Root Commitment of `FeeMerkleTree`
    pub fn fee_merkle_tree_root(&self) -> FeeMerkleCommitment {
        *field!(self.fee_merkle_tree_root)
    }

    /// Fee paid by the block builder
    pub fn fee_info(&self) -> FeeInfo {
        *field!(self.fee_info)
    }

    /// Account (etheruem address) of builder
    ///
    /// This signature is not considered formally part of the header; it is just evidence proving
    /// that other parts of the header ([`fee_info`](Self::fee_info)) are correct. It exists in the
    /// header so that it is available to all nodes to be used during validation. But since it is
    /// checked during consensus, any downstream client who has a proof of consensus finality of a
    /// header can trust that [`fee_info`](Self::fee_info) is correct without relying on the
    /// signature. Thus, this signature is not included in the header commitment.
    pub fn builder_signature(&self) -> Option<BuilderSignature> {
        *field!(self.builder_signature)
    }
}
