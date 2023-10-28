use std::collections::HashSet;

use crate::{
    node_types::{MemoryImpl, TestTypes},
    test_builder::TestMetadata,
};
use commit::Committable;
use hotshot::{
    certificate::QuorumCertificate,
    traits::{NodeImplementation, TestableNodeImplementation},
    types::{bn254::BLSPubKey, SignatureKey, SystemContextHandle},
    HotShotConsensusApi, HotShotInitializer, SystemContext,
};
use hotshot_task::event_stream::ChannelStream;
use hotshot_task_impls::events::HotShotEvent;
use hotshot_types::{
    block_impl::{VIDBlockHeader, VIDBlockPayload, NUM_CHUNKS, NUM_STORAGE_NODES},
    consensus::ConsensusMetricsValue,
    data::{Leaf, QuorumProposal, VidScheme, ViewNumber},
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
    SystemContextHandle<TestTypes, MemoryImpl>,
    ChannelStream<HotShotEvent<TestTypes, MemoryImpl>>,
) {
    let builder = TestMetadata::default_multiple_rounds();

    let launcher = builder.gen_launcher::<TestTypes, MemoryImpl>();

    let networks = (launcher.resource_generator.channel_generator)(node_id);
    let storage = (launcher.resource_generator.storage)(node_id);
    let config = launcher.resource_generator.config.clone();

    let initializer = HotShotInitializer::<
        TestTypes,
        <MemoryImpl as NodeImplementation<TestTypes>>::Leaf,
    >::from_genesis(
        <MemoryImpl as TestableNodeImplementation<TestTypes>>::block_genesis()
    )
    .unwrap();

    let known_nodes_with_stake = config.known_nodes_with_stake.clone();
    let private_key =
        <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0u8; 32], node_id).1;
    let public_key = <TestTypes as NodeType>::SignatureKey::from_private(&private_key);
    let quorum_election_config = config.election_config.clone().unwrap_or_else(|| {
        <QuorumEx<TestTypes, MemoryImpl> as ConsensusExchange<
            TestTypes,
            Message<TestTypes, MemoryImpl>,
        >>::Membership::default_election_config(config.total_nodes.get() as u64)
    });

    let committee_election_config = config.election_config.clone().unwrap_or_else(|| {
        <CommitteeEx<TestTypes, MemoryImpl> as ConsensusExchange<
            TestTypes,
            Message<TestTypes, MemoryImpl>,
        >>::Membership::default_election_config(config.total_nodes.get() as u64)
    });
    let exchanges = <MemoryImpl as NodeImplementation<TestTypes>>::Exchanges::create(
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
    handle: &SystemContextHandle<TestTypes, MemoryImpl>,
    private_key: &<BLSPubKey as SignatureKey>::PrivateKey,
    view: u64,
) -> (QuorumProposal<TestTypes, Leaf<TestTypes>>, EncodedSignature) {
    let consensus_lock = handle.get_consensus();
    let consensus = consensus_lock.read().await;
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
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
    let parent_header = parent_leaf.block_header.clone();

    // every event input is seen on the event stream in the output.
    let block = <VIDBlockPayload as TestableBlock>::genesis();
    let payload_commitment = block.commit();
    let block_header = VIDBlockHeader::new(payload_commitment, &parent_header);
    let leaf = Leaf {
        view_number: ViewNumber::new(view),
        justify_qc: consensus.high_qc.clone(),
        parent_commitment: parent_leaf.commit(),
        block_header: block_header.clone(),
        transaction_commitments: HashSet::new(),
        rejected: vec![],
        timestamp: 0,
        proposer_id: api.public_key().to_bytes(),
    };
    let signature = <BLSPubKey as SignatureKey>::sign(private_key, leaf.commit().as_ref());
    let proposal = QuorumProposal::<TestTypes, Leaf<TestTypes>> {
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
    handle: &SystemContextHandle<TestTypes, MemoryImpl>,
    private_key: &<BLSPubKey as SignatureKey>::PrivateKey,
    view: u64,
) -> Proposal<QuorumProposal<TestTypes, Leaf<TestTypes>>> {
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
    let public_key = <TestTypes as NodeType>::SignatureKey::from_private(&private_key);
    (private_key, public_key)
}

pub fn vid_init() -> VidScheme {
    let srs = hotshot_types::data::test_srs(NUM_STORAGE_NODES);
    VidScheme::new(NUM_CHUNKS, NUM_STORAGE_NODES, &srs).unwrap()
}
