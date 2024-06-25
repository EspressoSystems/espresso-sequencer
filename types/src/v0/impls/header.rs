use std::sync::Arc;

use crate::{
    v0_1::UpgradeType, BlockMerkleTree, ChainConfig, FeeMerkleTree, L1Snapshot, Leaf, NodeState,
    NsTable, SeqTypes, ValidatedState,
};

use super::{
    v0_1, v0_2, BlockMerkleCommitment, BuilderSignature, FeeInfo, FeeMerkleCommitment, Header,
    L1BlockInfo, ResolvableChainConfig,
};
use anyhow::{bail, ensure, Context};
use ark_serialize::CanonicalSerialize;
use committable::{Commitment, Committable, RawCommitmentBuilder};
use hotshot_types::{
    traits::{
        block_contents::{BlockHeader, BuilderFee},
        node_implementation::NodeType,
        signature_key::BuilderSignatureKey,
        BlockPayload, EncodeBytes, ValidatedState as _,
    },
    utils::BuilderCommitment,
    vid::{VidCommitment, VidCommon},
};
use jf_merkle_tree::{AppendableMerkleTreeScheme, MerkleTreeScheme};
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use snafu::Snafu;
use time::OffsetDateTime;
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

        //

        let mut c = RawCommitmentBuilder::new(&Self::tag());
        if self.version() > v0_1::VERSION {
            // The original commitment scheme (from version 0.1) did not include the version in the
            // commitment. We respect this for backwards compatibility, but for all future versions,
            // we want to include it.
            c = c
                .u64_field("version_major", self.version().major.into())
                .u64_field("version_minor", self.version().minor.into());
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

        // if self.version() > v0_1::VERSION {
        //     // Added in 0.2.
        //     c = c.fixed_size_field("fee_recipient", self.fee_recipient().to_fixed_bytes());
        // }
        c.finalize()
    }

    //

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
        match (v.major, v.minor) {
            (0, 1) => v0_1::Header::deserialize(d).map(Self::from),
            (0, 2) => v0_2::Header::deserialize(d).map(Self::from),
            _ => Err(D::Error::custom(format!("unsupported version {v}"))),
        }
    }

    pub fn version(&self) -> Version {
        match self {
            Self::V1(_) => Version { major: 0, minor: 1 },
            Self::V2(_) => Version { major: 0, minor: 1 },
        }
    }
}

use serde::de::Error;

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
    #[allow(clippy::too_many_arguments)]
    fn from_info(
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        ns_table: NsTable,
        parent_leaf: &Leaf,
        mut l1: L1Snapshot,
        l1_deposits: &[FeeInfo],
        builder_fee: BuilderFee<SeqTypes>,
        mut timestamp: u64,
        mut state: ValidatedState,
        chain_config: ChainConfig,
        version: Version,
    ) -> anyhow::Result<Self> {
        let Version { major, minor } = version;
        ensure!(major == 0, "Invalid major version {major}");

        // Increment height.
        let parent_header = parent_leaf.block_header();
        let height = parent_header.height() + 1;

        // Ensure the timestamp does not decrease. We can trust `parent.timestamp` because `parent`
        // has already been voted on by consensus. If our timestamp is behind, either f + 1 nodes
        // are lying about the current time, or our clock is just lagging.
        if timestamp < parent_header.timestamp() {
            tracing::warn!(
                "Espresso timestamp {timestamp} behind parent {}, local clock may be out of sync",
                parent_header.timestamp()
            );
            timestamp = parent_header.timestamp();
        }

        // Ensure the L1 block references don't decrease. Again, we can trust `parent.l1_*` are
        // accurate.
        if l1.head < parent_header.l1_head() {
            tracing::warn!(
                "L1 head {} behind parent {}, L1 client may be lagging",
                l1.head,
                parent_header.l1_head()
            );
            l1.head = parent_header.l1_head();
        }
        if l1.finalized < parent_header.l1_finalized() {
            tracing::warn!(
                "L1 finalized {:?} behind parent {:?}, L1 client may be lagging",
                l1.finalized,
                parent_header.l1_finalized()
            );
            l1.finalized = parent_header.l1_finalized();
        }

        // Enforce that the sequencer block timestamp is not behind the L1 block timestamp. This can
        // only happen if our clock is badly out of sync with L1.
        if let Some(l1_block) = &l1.finalized {
            let l1_timestamp = l1_block.timestamp.as_u64();
            if timestamp < l1_timestamp {
                tracing::warn!("Espresso timestamp {timestamp} behind L1 timestamp {l1_timestamp}, local clock may be out of sync");
                timestamp = l1_timestamp;
            }
        }

        state
            .block_merkle_tree
            .push(parent_header.commit().as_ref())
            .context("missing blocks frontier")?;
        let block_merkle_tree_root = state.block_merkle_tree.commitment();

        // Insert the new L1 deposits
        for fee_info in l1_deposits {
            state
                .insert_fee_deposit(*fee_info)
                .context(format!("missing fee account {}", fee_info.account()))?;
        }

        // Charge the builder fee.
        ensure!(
            builder_fee.fee_account.validate_fee_signature(
                &builder_fee.fee_signature,
                builder_fee.fee_amount,
                &ns_table,
                &payload_commitment,
            ),
            "invalid builder signature"
        );
        let builder_signature = Some(builder_fee.fee_signature);
        let fee_info = builder_fee.into();
        state
            .charge_fee(fee_info, chain_config.fee_recipient)
            .context(format!("invalid builder fee {fee_info:?}"))?;

        let fee_merkle_tree_root = state.fee_merkle_tree.commitment();

        // TODO: >>>>>>>>
        let header = match minor {
            0 => Self::V1(v0_1::Header {
                chain_config: chain_config.commit().into(),
                height,
                timestamp,
                l1_head: l1.head,
                l1_finalized: l1.finalized,
                payload_commitment,
                builder_commitment,
                ns_table,
                fee_merkle_tree_root,
                block_merkle_tree_root,
                fee_info,
                builder_signature,
            }),
            _ => Self::V2(v0_2::Header {
                chain_config: chain_config.commit().into(),
                height,
                timestamp,
                l1_head: l1.head,
                l1_finalized: l1.finalized,
                payload_commitment,
                builder_commitment,
                ns_table,
                fee_merkle_tree_root,
                block_merkle_tree_root,
                fee_info,
                builder_signature,
            }),
        };

        Ok(header)
    }

    async fn get_chain_config(
        validated_state: &ValidatedState,
        instance_state: &NodeState,
    ) -> ChainConfig {
        let validated_cf = validated_state.chain_config;
        let instance_cf = instance_state.chain_config;

        if validated_cf.commit() == instance_cf.commit() {
            return instance_cf;
        }

        match validated_cf.resolve() {
            Some(cf) => cf,
            None => {
                tracing::info!("fetching chain config {} from peers", validated_cf.commit());

                instance_state
                    .peers
                    .as_ref()
                    .fetch_chain_config(validated_cf.commit())
                    .await
            }
        }
    }
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

    pub fn ns_table(&self) -> &NsTable {
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

#[derive(Debug, Snafu)]
#[snafu(display("Invalid Block Header {msg}"))]
pub struct InvalidBlockHeader {
    msg: String,
}
impl InvalidBlockHeader {
    fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl From<anyhow::Error> for InvalidBlockHeader {
    fn from(err: anyhow::Error) -> Self {
        Self::new(format!("{err:#}"))
    }
}

impl BlockHeader<SeqTypes> for Header {
    type Error = InvalidBlockHeader;

    #[tracing::instrument(
        skip_all,
        fields(
            node_id = instance_state.node_id,
            view = ?parent_leaf.view_number(),
            height = parent_leaf.block_header().height(),
        ),
    )]

    async fn new(
        parent_state: &ValidatedState,
        instance_state: &NodeState,
        parent_leaf: &Leaf,
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        metadata: <<SeqTypes as NodeType>::BlockPayload as BlockPayload<SeqTypes>>::Metadata,
        builder_fee: BuilderFee<SeqTypes>,
        _vid_common: VidCommon,
        version: Version,
    ) -> Result<Self, Self::Error> {
        let height = parent_leaf.height();
        let view = parent_leaf.view_number();

        let mut validated_state = parent_state.clone();

        let chain_config = if version > instance_state.current_version {
            match instance_state
                .upgrades
                .get(&version)
                .map(|upgrade| match upgrade.upgrade_type {
                    UpgradeType::ChainConfig { chain_config } => chain_config,
                }) {
                Some(cf) => cf,
                None => Header::get_chain_config(&validated_state, instance_state).await,
            }
        } else {
            Header::get_chain_config(&validated_state, instance_state).await
        };

        validated_state.chain_config = chain_config.into();

        // Fetch the latest L1 snapshot.
        let l1_snapshot = instance_state.l1_client.snapshot().await;
        // Fetch the new L1 deposits between parent and current finalized L1 block.
        let l1_deposits = if let (Some(addr), Some(block_info)) =
            (chain_config.fee_contract, l1_snapshot.finalized)
        {
            instance_state
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
        };
        // Find missing fee state entries. We will need to use the builder account which is paying a
        // fee and the recipient account which is receiving it, plus any counts receiving deposits
        // in this block.
        let missing_accounts = parent_state.forgotten_accounts(
            [builder_fee.fee_account, chain_config.fee_recipient]
                .into_iter()
                .chain(l1_deposits.iter().map(|info| info.account())),
        );
        if !missing_accounts.is_empty() {
            tracing::warn!(
                height,
                ?view,
                ?missing_accounts,
                "fetching missing accounts from peers"
            );

            // Fetch missing fee state entries
            let missing_account_proofs = instance_state
                .peers
                .as_ref()
                .fetch_accounts(
                    height,
                    view,
                    parent_state.fee_merkle_tree.commitment(),
                    missing_accounts,
                )
                .await?;

            // Insert missing fee state entries
            for account in missing_account_proofs.iter() {
                account
                    .proof
                    .remember(&mut validated_state.fee_merkle_tree)
                    .context("remembering fee account")?;
            }
        }

        // Ensure merkle tree has frontier
        if validated_state.need_to_fetch_blocks_mt_frontier() {
            tracing::warn!(height, ?view, "fetching block frontier from peers");
            instance_state
                .peers
                .as_ref()
                .remember_blocks_merkle_tree(height, view, &mut validated_state.block_merkle_tree)
                .await
                .context("remembering block proof")?;
        }

        Ok(Self::from_info(
            payload_commitment,
            builder_commitment,
            metadata,
            parent_leaf,
            l1_snapshot,
            &l1_deposits,
            builder_fee,
            OffsetDateTime::now_utc().unix_timestamp() as u64,
            validated_state,
            chain_config,
            version,
        )?)
    }

    fn genesis(
        instance_state: &NodeState,
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        ns_table: <<SeqTypes as NodeType>::BlockPayload as BlockPayload<SeqTypes>>::Metadata,
    ) -> Self {
        let ValidatedState {
            fee_merkle_tree,
            block_merkle_tree,
            ..
        } = ValidatedState::genesis(instance_state).0;
        let block_merkle_tree_root = block_merkle_tree.commitment();
        let fee_merkle_tree_root = fee_merkle_tree.commitment();

        Self::V2(v0_2::Header {
            // The genesis header needs to be completely deterministic, so we can't sample real
            // timestamps or L1 values.
            chain_config: instance_state.chain_config.into(),
            height: 0,
            timestamp: instance_state.genesis_header.timestamp.unix_timestamp(),
            l1_finalized: instance_state.l1_genesis,
            // Make sure the L1 head is not behind the finalized block.
            l1_head: instance_state
                .l1_genesis
                .map(|block| block.number)
                .unwrap_or_default(),
            payload_commitment,
            builder_commitment,
            ns_table,
            block_merkle_tree_root,
            fee_merkle_tree_root,
            fee_info: FeeInfo::genesis(),
            builder_signature: None,
        })
    }

    fn block_number(&self) -> u64 {
        self.height()
    }

    fn payload_commitment(&self) -> VidCommitment {
        self.payload_commitment()
    }

    fn metadata(
        &self,
    ) -> &<<SeqTypes as NodeType>::BlockPayload as BlockPayload<SeqTypes>>::Metadata {
        &self.ns_table()
    }

    /// Commit over fee_amount, payload_commitment and metadata
    fn builder_commitment(&self) -> BuilderCommitment {
        self.builder_commitment().clone()
    }
}
