use crate::{
    block::{entry::TxTableEntryWord, tables::NameSpaceTable, NsTable},
    chain_config::ResolvableChainConfig,
    eth_signature_key::BuilderSignature,
    l1_client::L1Snapshot,
    state::{BlockMerkleCommitment, FeeAccount, FeeAmount, FeeInfo, FeeMerkleCommitment},
    ChainConfig, L1BlockInfo, Leaf, NamespaceId, NodeState, SeqTypes, ValidatedState,
};
use anyhow::{ensure, Context};
use ark_serialize::CanonicalSerialize;
use committable::{Commitment, Committable, RawCommitmentBuilder};
use hotshot_query_service::{availability::QueryableHeader, explorer::ExplorerHeader};
use hotshot_types::{
    traits::{
        block_contents::{BlockHeader, BlockPayload, BuilderFee},
        node_implementation::NodeType,
        signature_key::BuilderSignatureKey,
        ValidatedState as HotShotState,
    },
    utils::BuilderCommitment,
    vid::{VidCommitment, VidCommon},
};
use jf_merkle_tree::prelude::*;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use time::OffsetDateTime;
use vbs::version::Version;

// /// A header is like a [`Block`] with the body replaced by a digest.
// #[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
// pub struct Header {
//     pub(crate) chain_config: ResolvableChainConfig,
//     pub(crate) height: u64,
//     pub(crate) timestamp: u64,
//     pub(crate) l1_head: u64,
//     pub(crate) l1_finalized: Option<L1BlockInfo>,
//     pub(crate) payload_commitment: VidCommitment,
//     pub(crate) builder_commitment: BuilderCommitment,
//     pub(crate) ns_table: NameSpaceTable<TxTableEntryWord>,
//     pub(crate) block_merkle_tree_root: BlockMerkleCommitment,
//     pub(crate) fee_merkle_tree_root: FeeMerkleCommitment,
//     pub(crate) fee_info: FeeInfo,
//     pub(crate) builder_signature: Option<BuilderSignature>,
// }

// impl Header {
//     /// A commitment to a ChainConfig or a full ChainConfig.
//     pub fn chain_config(&self) -> &ResolvableChainConfig {
//         &self.chain_config
//     }

//     pub fn height(&self) -> u64 {
//         self.height
//     }

//     pub fn timestamp(&self) -> u64 {
//         self.timestamp
//     }

//     /// The Espresso block header includes a reference to the current head of the L1 chain.
//     ///
//     /// Rollups can use this to facilitate bridging between the L1 and L2 in a deterministic way.
//     /// This field deterministically associates an L2 block with a recent L1 block the instant the
//     /// L2 block is sequenced. Rollups can then define the L2 state after this block as the state
//     /// obtained by executing all the transactions in this block _plus_ all the L1 deposits up to
//     /// the given L1 block number. Since there is no need to wait for the L2 block to be reflected
//     /// on the L1, this bridge design retains the low confirmation latency of HotShot.
//     ///
//     /// This block number indicates the unsafe head of the L1 chain, so it is subject to reorgs. For
//     /// this reason, the Espresso header does not include any information that might change in a
//     /// reorg, such as the L1 block timestamp or hash. It includes only the L1 block number, which
//     /// will always refer to _some_ block after a reorg: if the L1 head at the time this block was
//     /// sequenced gets reorged out, the L1 chain will eventually (and probably quickly) grow to the
//     /// same height once again, and a different block will exist with the same height. In this way,
//     /// Espresso does not have to handle L1 reorgs, and the Espresso blockchain will always be
//     /// reflective of the current state of the L1 blockchain. Rollups that use this block number
//     /// _do_ have to handle L1 reorgs, but each rollup and each rollup client can decide how many
//     /// confirmations they want to wait for on top of this `l1_head` before they consider an L2
//     /// block finalized. This offers a tradeoff between low-latency L1-L2 bridges and finality.
//     ///
//     /// Rollups that want a stronger guarantee of finality, or that want Espresso to attest to data
//     /// from the L1 block that might change in reorgs, can instead use the latest L1 _finalized_
//     /// block at the time this L2 block was sequenced: [`Self::l1_finalized`].
//     pub fn l1_head(&self) -> u64 {
//         self.l1_head
//     }

//     /// The Espresso block header includes information a bout the latest finalized L1 block.
//     ///
//     /// Similar to [`l1_head`](Self::l1_head), rollups can use this information to implement a
//     /// bridge between the L1 and L2 while retaining the finality of low-latency block confirmations
//     /// from HotShot. Since this information describes the finalized L1 block, a bridge using this
//     /// L1 block will have much higher latency than a bridge using [`l1_head`](Self::l1_head). In
//     /// exchange, rollups that use the finalized block do not have to worry about L1 reorgs, and can
//     /// inject verifiable attestations to the L1 block metadata (such as its timestamp or hash) into
//     /// their execution layers, since Espresso replicas will sign this information for the finalized
//     /// L1 block.
//     ///
//     /// This block may be `None` in the rare case where Espresso has started shortly after the
//     /// genesis of the L1, and the L1 has yet to finalize a block. In all other cases it will be
//     /// `Some`.
//     pub fn l1_finalized(&self) -> Option<L1BlockInfo> {
//         self.l1_finalized
//     }

//     pub fn payload_commitment(&self) -> VidCommitment {
//         self.payload_commitment
//     }

//     pub fn builder_commitment(&self) -> &BuilderCommitment {
//         &self.builder_commitment
//     }

//     pub fn ns_table(&self) -> &NameSpaceTable<TxTableEntryWord> {
//         &self.ns_table
//     }

//     /// Root Commitment of Block Merkle Tree
//     pub fn block_merkle_tree_root(&self) -> BlockMerkleCommitment {
//         self.block_merkle_tree_root
//     }

//     /// Root Commitment of `FeeMerkleTree`
//     pub fn fee_merkle_tree_root(&self) -> FeeMerkleCommitment {
//         self.fee_merkle_tree_root
//     }

//     /// Fee paid by the block builder
//     pub fn fee_info(&self) -> FeeInfo {
//         self.fee_info
//     }

//     /// Account (etheruem address) of builder
//     ///
//     /// This signature is not considered formally part of the header; it is just evidence proving
//     /// that other parts of the header ([`fee_info`](Self::fee_info)) are correct. It exists in the
//     /// header so that it is available to all nodes to be used during validation. But since it is
//     /// checked during consensus, any downstream client who has a proof of consensus finality of a
//     /// header can trust that [`fee_info`](Self::fee_info) is correct without relying on the
//     /// signature. Thus, this signature is not included in the header commitment.
//     pub fn builder_signature(&self) -> Option<BuilderSignature> {
//         self.builder_signature
//     }
// }

// #[cfg(any(test, feature = "testing"))]
// /// Expose some fields directly for tests that need to directly manipulate or construct headers.
// impl Header {
//     pub fn set_height(&mut self, height: u64) {
//         self.height = height;
//     }
// }

// impl Committable for Header {
//     fn commit(&self) -> Commitment<Self> {
//         let mut bmt_bytes = vec![];
//         self.block_merkle_tree_root()
//             .serialize_with_mode(&mut bmt_bytes, ark_serialize::Compress::Yes)
//             .unwrap();
//         let mut fmt_bytes = vec![];
//         self.fee_merkle_tree_root()
//             .serialize_with_mode(&mut fmt_bytes, ark_serialize::Compress::Yes)
//             .unwrap();

//         RawCommitmentBuilder::new(&Self::tag())
//             .field("chain_config", self.chain_config().commit())
//             .u64_field("height", self.height())
//             .u64_field("timestamp", self.timestamp())
//             .u64_field("l1_head", self.l1_head())
//             .optional("l1_finalized", &self.l1_finalized())
//             .constant_str("payload_commitment")
//             .fixed_size_bytes(self.payload_commitment().as_ref().as_ref())
//             .constant_str("builder_commitment")
//             .fixed_size_bytes(self.builder_commitment().as_ref())
//             .field("ns_table", self.ns_table().commit())
//             .var_size_field("block_merkle_tree_root", &bmt_bytes)
//             .var_size_field("fee_merkle_tree_root", &fmt_bytes)
//             .field("fee_info", self.fee_info().commit())
//             .finalize()
//     }

//     fn tag() -> String {
//         // We use the tag "BLOCK" since blocks are identified by the hash of their header. This will
//         // thus be more intuitive to users than "HEADER".
//         "BLOCK".into()
//     }
// }

// impl Committable for NameSpaceTable<TxTableEntryWord> {
//     fn commit(&self) -> Commitment<Self> {
//         RawCommitmentBuilder::new(&Self::tag())
//             .var_size_bytes(self.get_bytes())
//             .finalize()
//     }

//     fn tag() -> String {
//         "NSTABLE".into()
//     }
// }

use espresso_types::Header;

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
            height = parent_leaf.block_header().height,
        ),
    )]

    async fn new(
        parent_state: &ValidatedState,
        instance_state: &NodeState,
        parent_leaf: &Leaf,
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        metadata: <<SeqTypes as NodeType>::BlockPayload as BlockPayload>::Metadata,
        builder_fee: BuilderFee<SeqTypes>,
        _vid_common: VidCommon,
        version: Version,
    ) -> Result<Self, Self::Error> {
        let chain_config = instance_state.chain_config;
        let height = parent_leaf.height();
        let view = parent_leaf.view_number();

        let mut validated_state = parent_state.clone();

        // Fetch the latest L1 snapshot.
        let l1_snapshot = instance_state.l1_client().snapshot().await;
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
                        .l1_finalized
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
        ns_table: <<SeqTypes as NodeType>::BlockPayload as BlockPayload>::Metadata,
    ) -> Self {
        let ValidatedState {
            fee_merkle_tree,
            block_merkle_tree,
        } = ValidatedState::genesis(instance_state).0;
        let block_merkle_tree_root = block_merkle_tree.commitment();
        let fee_merkle_tree_root = fee_merkle_tree.commitment();

        Self {
            // The genesis header needs to be completely deterministic, so we can't sample real
            // timestamps or L1 values.
            chain_config: instance_state.chain_config.into(),
            height: 0,
            timestamp: 0,
            l1_head: 0,
            l1_finalized: instance_state.l1_genesis,
            payload_commitment,
            builder_commitment,
            ns_table,
            block_merkle_tree_root,
            fee_merkle_tree_root,
            fee_info: FeeInfo::genesis(),
            builder_signature: None,
        }
    }

    fn block_number(&self) -> u64 {
        self.height
    }

    fn payload_commitment(&self) -> VidCommitment {
        self.payload_commitment
    }

    fn metadata(&self) -> &<<SeqTypes as NodeType>::BlockPayload as BlockPayload>::Metadata {
        &self.ns_table
    }

    /// Commit over fee_amount, payload_commitment and metadata
    fn builder_commitment(&self) -> BuilderCommitment {
        self.builder_commitment.clone()
    }
}

impl QueryableHeader<SeqTypes> for Header {
    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

impl ExplorerHeader<SeqTypes> for Header {
    type BalanceAmount = FeeAmount;
    type WalletAddress = FeeAccount;
    type ProposerId = FeeAccount;
    type NamespaceId = NamespaceId;

    fn proposer_id(&self) -> Self::ProposerId {
        self.fee_info.account()
    }

    fn fee_info_account(&self) -> Self::WalletAddress {
        self.fee_info.account()
    }

    fn fee_info_balance(&self) -> Self::BalanceAmount {
        self.fee_info.amount()
    }

    /// reward_balance at the moment is only implemented as a stub, as block
    /// rewards have not yet been implemented.
    ///
    /// TODO: update implementation when rewards have been created / supported.
    ///       Issue: https://github.com/EspressoSystems/espresso-sequencer/issues/1453
    fn reward_balance(&self) -> Self::BalanceAmount {
        FeeAmount::from(0)
    }

    fn namespace_ids(&self) -> Vec<Self::NamespaceId> {
        let l = self.ns_table.len();
        let mut result: Vec<Self::NamespaceId> = Vec::with_capacity(l);
        for i in 0..l {
            let (ns_id, _) = self.ns_table.get_table_entry(i);
            result.push(ns_id);
        }

        result
    }
}

#[cfg(test)]
mod test_headers {
    use std::sync::Arc;

    use super::*;
    use crate::{
        catchup::mock::MockStateCatchup,
        eth_signature_key::EthKeyPair,
        l1_client::L1Client,
        state::{validate_proposal, BlockMerkleTree, FeeAccount, FeeMerkleTree},
        NodeState,
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use ethers::{
        types::{Address, U256},
        utils::Anvil,
    };
    use hotshot_types::{traits::signature_key::BuilderSignatureKey, vid::vid_scheme};
    use jf_vid::VidScheme;

    #[derive(Debug, Default)]
    #[must_use]
    struct TestCase {
        // Parent header info.
        parent_timestamp: u64,
        parent_l1_head: u64,
        parent_l1_finalized: Option<L1BlockInfo>,

        // Environment at the time the new header is created.
        l1_head: u64,
        l1_finalized: Option<L1BlockInfo>,
        timestamp: u64,
        l1_deposits: Vec<FeeInfo>,

        // Expected new header info.
        expected_timestamp: u64,
        expected_l1_head: u64,
        expected_l1_finalized: Option<L1BlockInfo>,
    }

    impl TestCase {
        fn run(self) {
            setup_logging();
            setup_backtrace();

            // Check test case validity.
            assert!(self.expected_timestamp >= self.parent_timestamp);
            assert!(self.expected_l1_head >= self.parent_l1_head);
            assert!(self.expected_l1_finalized >= self.parent_l1_finalized);

            let genesis = GenesisForTest::default();
            let mut parent = genesis.header.clone();
            parent.timestamp = self.parent_timestamp;
            parent.l1_head = self.parent_l1_head;
            parent.l1_finalized = self.parent_l1_finalized;

            let mut parent_leaf = genesis.leaf.clone();
            *parent_leaf.block_header_mut() = parent.clone();

            let block_merkle_tree =
                BlockMerkleTree::from_elems(Some(32), Vec::<Commitment<Header>>::new()).unwrap();

            let fee_info = FeeInfo::genesis();
            let fee_merkle_tree = FeeMerkleTree::from_kv_set(
                20,
                Vec::from([(fee_info.account(), fee_info.amount())]),
            )
            .unwrap();
            let mut validated_state = ValidatedState {
                block_merkle_tree: block_merkle_tree.clone(),
                fee_merkle_tree,
            };

            let (fee_account, fee_key) = FeeAccount::generated_from_seed_indexed([0; 32], 0);
            let fee_amount = 0;
            let fee_signature = FeeAccount::sign_fee(
                &fee_key,
                fee_amount,
                &genesis.ns_table,
                &genesis.header.payload_commitment,
            )
            .unwrap();

            let header = Header::from_info(
                genesis.header.payload_commitment,
                genesis.header.builder_commitment,
                genesis.ns_table,
                &parent_leaf,
                L1Snapshot {
                    head: self.l1_head,
                    finalized: self.l1_finalized,
                },
                &self.l1_deposits,
                BuilderFee {
                    fee_account,
                    fee_amount,
                    fee_signature,
                },
                self.timestamp,
                validated_state.clone(),
                genesis.instance_state.chain_config,
                Version { major: 0, minor: 1 },
            )
            .unwrap();
            assert_eq!(header.height, parent.height + 1);
            assert_eq!(header.timestamp, self.expected_timestamp);
            assert_eq!(header.l1_head, self.expected_l1_head);
            assert_eq!(header.l1_finalized, self.expected_l1_finalized);

            // Check deposits were inserted before computing the fee merkle tree root.
            for fee_info in self.l1_deposits {
                validated_state.insert_fee_deposit(fee_info).unwrap();
            }
            assert_eq!(
                validated_state.fee_merkle_tree.commitment(),
                header.fee_merkle_tree_root,
            );

            assert_eq!(
                block_merkle_tree,
                BlockMerkleTree::from_elems(Some(32), Vec::<Commitment<Header>>::new()).unwrap()
            );
        }
    }

    fn l1_block(number: u64) -> L1BlockInfo {
        L1BlockInfo {
            number,
            ..Default::default()
        }
    }

    #[test]
    fn test_new_header() {
        // Simplest case: building on genesis, L1 info and timestamp unchanged.
        TestCase::default().run()
    }

    #[test]
    fn test_new_header_advance_timestamp() {
        TestCase {
            timestamp: 1,
            expected_timestamp: 1,
            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_advance_l1_block() {
        TestCase {
            parent_l1_head: 0,
            parent_l1_finalized: Some(l1_block(0)),

            l1_head: 1,
            l1_finalized: Some(l1_block(1)),

            expected_l1_head: 1,
            expected_l1_finalized: Some(l1_block(1)),

            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_advance_l1_finalized_from_none() {
        TestCase {
            l1_finalized: Some(l1_block(1)),
            expected_l1_finalized: Some(l1_block(1)),
            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_timestamp_behind_finalized_l1_block() {
        let l1_finalized = Some(L1BlockInfo {
            number: 1,
            timestamp: 1.into(),
            ..Default::default()
        });
        TestCase {
            l1_head: 1,
            l1_finalized,
            timestamp: 0,

            expected_l1_head: 1,
            expected_l1_finalized: l1_finalized,
            expected_timestamp: 1,

            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_timestamp_behind() {
        TestCase {
            parent_timestamp: 1,
            timestamp: 0,
            expected_timestamp: 1,

            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_l1_head_behind() {
        TestCase {
            parent_l1_head: 1,
            l1_head: 0,
            expected_l1_head: 1,

            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_l1_finalized_behind_some() {
        TestCase {
            parent_l1_finalized: Some(l1_block(1)),
            l1_finalized: Some(l1_block(0)),
            expected_l1_finalized: Some(l1_block(1)),

            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_l1_finalized_behind_none() {
        TestCase {
            parent_l1_finalized: Some(l1_block(0)),
            l1_finalized: None,
            expected_l1_finalized: Some(l1_block(0)),

            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_deposits_one() {
        TestCase {
            l1_deposits: vec![FeeInfo::new(Address::default(), 1)],
            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_deposits_many() {
        TestCase {
            l1_deposits: [
                (Address::default(), 1),
                (Address::default(), 2),
                (Address::random(), 3),
            ]
            .iter()
            .map(|(address, amount)| FeeInfo::new(*address, *amount))
            .collect(),
            ..Default::default()
        }
        .run()
    }

    struct GenesisForTest {
        pub instance_state: NodeState,
        pub validated_state: ValidatedState,
        pub leaf: Leaf,
        pub header: Header,
        pub ns_table: NameSpaceTable<TxTableEntryWord>,
    }

    impl Default for GenesisForTest {
        fn default() -> Self {
            let instance_state = NodeState::mock();
            let validated_state = ValidatedState::genesis(&instance_state).0;
            let leaf = Leaf::genesis(&instance_state);
            let header = leaf.block_header().clone();
            let ns_table = leaf.block_payload().unwrap().get_ns_table().clone();
            Self {
                instance_state,
                validated_state,
                leaf,
                header,
                ns_table,
            }
        }
    }

    #[async_std::test]
    async fn test_validate_proposal_error_cases() {
        let genesis = GenesisForTest::default();
        let vid_common = vid_scheme(1).disperse([]).unwrap().common;

        let mut validated_state = ValidatedState::default();
        let mut block_merkle_tree = validated_state.block_merkle_tree.clone();

        let mut parent_header = genesis.header.clone();
        let mut parent_leaf = genesis.leaf.clone();
        *parent_leaf.block_header_mut() = parent_header.clone();

        // Populate the tree with an initial `push`.
        block_merkle_tree.push(genesis.header.commit()).unwrap();
        let block_merkle_tree_root = block_merkle_tree.commitment();
        validated_state.block_merkle_tree = block_merkle_tree.clone();
        parent_header.block_merkle_tree_root = block_merkle_tree_root;
        let mut proposal = parent_header.clone();

        // Pass a different chain config to trigger a chain config validation error.
        let state = validated_state
            .apply_header(&genesis.instance_state, &parent_leaf, &proposal)
            .await
            .unwrap()
            .0;

        let result = validate_proposal(
            &state,
            ChainConfig {
                chain_id: U256::zero().into(),
                ..Default::default()
            },
            &parent_leaf,
            &proposal,
            &vid_common,
        )
        .unwrap_err();

        assert!(format!("{}", result.root_cause()).starts_with("Invalid Chain Config:"));

        // Advance `proposal.height` to trigger validation error.

        let validated_state = validated_state
            .apply_header(&genesis.instance_state, &parent_leaf, &proposal)
            .await
            .unwrap()
            .0;
        let result = validate_proposal(
            &validated_state,
            genesis.instance_state.chain_config,
            &parent_leaf,
            &proposal,
            &vid_common,
        )
        .unwrap_err();
        assert_eq!(
            format!("{}", result.root_cause()),
            "Invalid Height Error: 0, 0"
        );

        // proposed `Header` root should include parent + parent.commit
        proposal.height += 1;

        let validated_state = validated_state
            .apply_header(&genesis.instance_state, &parent_leaf, &proposal)
            .await
            .unwrap()
            .0;

        let result = validate_proposal(
            &validated_state,
            genesis.instance_state.chain_config,
            &parent_leaf,
            &proposal,
            &vid_common,
        )
        .unwrap_err();
        // Fails b/c `proposal` has not advanced from `parent`
        assert!(format!("{}", result.root_cause()).contains("Invalid Block Root Error"));
    }

    #[async_std::test]
    async fn test_validate_proposal_success() {
        setup_logging();
        setup_backtrace();

        let anvil = Anvil::new().block_time(1u32).spawn();
        let mut genesis_state =
            NodeState::mock().with_l1(L1Client::new(anvil.endpoint().parse().unwrap(), 1));

        let genesis = GenesisForTest::default();
        let vid_common = vid_scheme(1).disperse([]).unwrap().common;

        let mut parent_state = genesis.validated_state.clone();

        let mut block_merkle_tree = parent_state.block_merkle_tree.clone();
        let fee_merkle_tree = parent_state.fee_merkle_tree.clone();

        // Populate the tree with an initial `push`.
        block_merkle_tree.push(genesis.header.commit()).unwrap();
        let block_merkle_tree_root = block_merkle_tree.commitment();
        let fee_merkle_tree_root = fee_merkle_tree.commitment();
        parent_state.block_merkle_tree = block_merkle_tree.clone();
        parent_state.fee_merkle_tree = fee_merkle_tree.clone();

        let mut parent_header = genesis.header.clone();
        parent_header.block_merkle_tree_root = block_merkle_tree_root;
        parent_header.fee_merkle_tree_root = fee_merkle_tree_root;

        let mut parent_leaf = genesis.leaf.clone();
        *parent_leaf.block_header_mut() = parent_header.clone();

        // Forget the state to trigger lookups in Header::new
        let forgotten_state = parent_state.forget();
        genesis_state.peers = Arc::new(MockStateCatchup::from_iter([(
            parent_leaf.view_number(),
            Arc::new(parent_state.clone()),
        )]));
        // Get a proposal from a parent

        // TODO this currently fails because after fetching the blocks frontier
        // the element (header commitment) does not match the one in the proof.
        let key_pair = EthKeyPair::for_test();
        let fee_amount = 0u64;
        let payload_commitment = parent_header.payload_commitment;
        let builder_commitment = parent_header.builder_commitment().clone();
        let ns_table = genesis.ns_table;
        let fee_signature =
            FeeAccount::sign_fee(&key_pair, fee_amount, &ns_table, &payload_commitment).unwrap();
        let builder_fee = BuilderFee {
            fee_amount,
            fee_account: key_pair.fee_account(),
            fee_signature,
        };
        let proposal = Header::new(
            &forgotten_state,
            &genesis_state,
            &parent_leaf,
            payload_commitment,
            builder_commitment,
            ns_table,
            builder_fee,
            vid_common.clone(),
            hotshot_types::constants::BASE_VERSION,
        )
        .await
        .unwrap();

        let mut proposal_state = parent_state.clone();
        for fee_info in genesis_state
            .l1_client
            .get_finalized_deposits(Address::default(), None, 0)
            .await
        {
            proposal_state.insert_fee_deposit(fee_info).unwrap();
        }

        let mut block_merkle_tree = proposal_state.block_merkle_tree.clone();
        block_merkle_tree.push(proposal.commit()).unwrap();

        let proposal_state = proposal_state
            .apply_header(&genesis_state, &parent_leaf, &proposal)
            .await
            .unwrap()
            .0;
        validate_proposal(
            &proposal_state,
            genesis.instance_state.chain_config,
            &parent_leaf,
            &proposal.clone(),
            &vid_common,
        )
        .unwrap();

        assert_eq!(
            proposal_state.block_merkle_tree.commitment(),
            proposal.block_merkle_tree_root
        );
    }

    #[test]
    fn verify_header_signature() {
        // simulate a fixed size hash by padding our message
        let message = ";)";
        let mut commitment = [0u8; 32];
        commitment[..message.len()].copy_from_slice(message.as_bytes());

        let key = FeeAccount::generated_from_seed_indexed([0; 32], 0).1;
        let signature = FeeAccount::sign_builder_message(&key, &commitment).unwrap();
        assert!(key
            .fee_account()
            .validate_builder_signature(&signature, &commitment));
    }
}
