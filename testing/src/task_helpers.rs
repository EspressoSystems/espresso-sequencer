use crate::{
    node_types::{SequencingMemoryImpl, SequencingTestTypes},
    test_builder::TestMetadata,
};
use commit::Committable;
use either::{Either::Left, Right};
use hotshot::{
    certificate::QuorumCertificate,
    traits::{NodeImplementation, TestableNodeImplementation},
    types::{bn254::BLSPubKey, SignatureKey, SystemContextHandle},
    HotShotInitializer, HotShotSequencingConsensusApi, SystemContext,
};
use hotshot_task::event_stream::ChannelStream;
use hotshot_task_impls::events::SequencingHotShotEvent;
use hotshot_types::{
    block_impl::{VIDBlockHeader, VIDBlockPayload, NUM_CHUNKS, NUM_STORAGE_NODES},
    consensus::ConsensusMetricsValue,
    data::{QuorumProposal, SequencingLeaf, VidScheme, ViewNumber},
    message::{Message, Proposal},
    traits::{
        block_contents::BlockHeader,
        consensus_api::ConsensusSharedApi,
        election::{ConsensusExchange, Membership, SignedCertificate},
        node_implementation::{CommitteeEx, ExchangesType, NodeType, QuorumEx},
        signature_key::EncodedSignature,
        state::{ConsensusTime, TestableBlock},
    },
};

pub async fn build_system_handle(
    node_id: u64,
) -> (
    SystemContextHandle<SequencingTestTypes, SequencingMemoryImpl>,
    ChannelStream<SequencingHotShotEvent<SequencingTestTypes, SequencingMemoryImpl>>,
) {
    let builder = TestMetadata::default_multiple_rounds();

    let launcher = builder.gen_launcher::<SequencingTestTypes, SequencingMemoryImpl>();

    let networks = (launcher.resource_generator.channel_generator)(node_id);
    let storage = (launcher.resource_generator.storage)(node_id);
    let config = launcher.resource_generator.config.clone();

    let initializer = HotShotInitializer::<
        SequencingTestTypes,
        <SequencingMemoryImpl as NodeImplementation<SequencingTestTypes>>::Leaf,
    >::from_genesis(<SequencingMemoryImpl as TestableNodeImplementation<
        SequencingTestTypes,
    >>::block_genesis())
    .unwrap();

    let known_nodes_with_stake = config.known_nodes_with_stake.clone();
    let private_key =
        <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0u8; 32], node_id).1;
    let public_key = <SequencingTestTypes as NodeType>::SignatureKey::from_private(&private_key);
    let quorum_election_config = config.election_config.clone().unwrap_or_else(|| {
        <QuorumEx<SequencingTestTypes, SequencingMemoryImpl> as ConsensusExchange<
            SequencingTestTypes,
            Message<SequencingTestTypes, SequencingMemoryImpl>,
        >>::Membership::default_election_config(config.total_nodes.get() as u64)
    });

    let committee_election_config = config.election_config.clone().unwrap_or_else(|| {
        <CommitteeEx<SequencingTestTypes, SequencingMemoryImpl> as ConsensusExchange<
            SequencingTestTypes,
            Message<SequencingTestTypes, SequencingMemoryImpl>,
        >>::Membership::default_election_config(config.total_nodes.get() as u64)
    });
    let exchanges =
        <SequencingMemoryImpl as NodeImplementation<SequencingTestTypes>>::Exchanges::create(
            known_nodes_with_stake.clone(),
            (quorum_election_config, committee_election_config),
            networks,
            public_key,
            public_key.get_stake_table_entry(1u64),
            private_key.clone(),
        );
    SystemContext::init(
        public_key,
        private_key,
        node_id,
        config,
        storage,
        exchanges,
        initializer,
        ConsensusMetricsValue::new(),
    )
    .await
    .expect("Could not init hotshot")
}

async fn build_quorum_proposal_and_signature(
    handle: &SystemContextHandle<SequencingTestTypes, SequencingMemoryImpl>,
    private_key: &<BLSPubKey as SignatureKey>::PrivateKey,
    view: u64,
) -> (
    QuorumProposal<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>,
    EncodedSignature,
) {
    let consensus_lock = handle.get_consensus();
    let consensus = consensus_lock.read().await;
    let api: HotShotSequencingConsensusApi<SequencingTestTypes, SequencingMemoryImpl> =
        HotShotSequencingConsensusApi {
            inner: handle.hotshot.inner.clone(),
        };
    let _quorum_exchange = api.inner.exchanges.quorum_exchange().clone();

    let parent_view_number = &consensus.high_qc.view_number();
    let Some(parent_view) = consensus.state_map.get(parent_view_number) else {
        panic!("Couldn't find high QC parent in state map.");
    };
    let Some(leaf) = parent_view.get_leaf_commitment() else {
        panic!("Parent of high QC points to a view without a proposal");
    };
    let Some(leaf) = consensus.saved_leaves.get(&leaf) else {
        panic!("Failed to find high QC parent.");
    };
    let parent_leaf = leaf.clone();
    let parent_header = match parent_leaf.deltas {
        Left((block_number, ref payload)) => VIDBlockHeader {
            block_number,
            payload_commitment: payload.commit(),
        },
        Right(ref header) => header.clone(),
    };

    // every event input is seen on the event stream in the output.
    let block = <VIDBlockPayload as TestableBlock>::genesis();
    let payload_commitment = block.commit();
    let block_header = VIDBlockHeader::new(payload_commitment, parent_header);
    let leaf = SequencingLeaf {
        view_number: ViewNumber::new(view),
        justify_qc: consensus.high_qc.clone(),
        parent_commitment: parent_leaf.commit(),
        deltas: Right(block_header.clone()),
        rejected: vec![],
        timestamp: 0,
        proposer_id: api.public_key().to_bytes(),
    };
    let signature = <BLSPubKey as SignatureKey>::sign(private_key, leaf.commit().as_ref());
    let proposal = QuorumProposal::<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>> {
        block_header,
        view_number: ViewNumber::new(view),
        justify_qc: QuorumCertificate::genesis(),
        timeout_certificate: None,
        proposer_id: leaf.proposer_id,
        dac: None,
    };

    (proposal, signature)
}

pub async fn build_quorum_proposal(
    handle: &SystemContextHandle<SequencingTestTypes, SequencingMemoryImpl>,
    private_key: &<BLSPubKey as SignatureKey>::PrivateKey,
    view: u64,
) -> Proposal<QuorumProposal<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>> {
    let (proposal, signature) =
        build_quorum_proposal_and_signature(handle, private_key, view).await;
    Proposal {
        data: proposal,
        signature,
    }
}

pub fn key_pair_for_id(node_id: u64) -> (<BLSPubKey as SignatureKey>::PrivateKey, BLSPubKey) {
    let private_key =
        <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0u8; 32], node_id).1;
    let public_key = <SequencingTestTypes as NodeType>::SignatureKey::from_private(&private_key);
    (private_key, public_key)
}

pub fn vid_init() -> VidScheme {
    let srs = hotshot_types::data::test_srs(NUM_STORAGE_NODES);
    VidScheme::new(NUM_CHUNKS, NUM_STORAGE_NODES, &srs).unwrap()
}
