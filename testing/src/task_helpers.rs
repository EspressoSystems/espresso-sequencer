use std::marker::PhantomData;

use crate::{
    node_types::{MemoryImpl, TestTypes},
    test_builder::TestMetadata,
};
use commit::Committable;
use hotshot::{
    types::{bn254::BLSPubKey, SignatureKey, SystemContextHandle},
    HotShotConsensusApi, HotShotInitializer, Memberships, Networks, SystemContext,
};
use hotshot_task::event_stream::ChannelStream;
use hotshot_task_impls::events::HotShotEvent;
use hotshot_types::{
    block_impl::{VIDBlockHeader, VIDBlockPayload},
    consensus::ConsensusMetricsValue,
    data::{Leaf, QuorumProposal, VidScheme, ViewNumber},
    message::Proposal,
    simple_certificate::QuorumCertificate,
    traits::{
        block_contents::BlockHeader,
        block_contents::{NUM_CHUNKS, NUM_STORAGE_NODES},
        consensus_api::ConsensusSharedApi,
        election::Membership,
        node_implementation::NodeType,
        signature_key::EncodedSignature,
        state::{ConsensusTime, TestableBlock},
    },
    vote::HasViewNumber,
};

pub async fn build_system_handle(
    node_id: u64,
) -> (
    SystemContextHandle<TestTypes, MemoryImpl>,
    ChannelStream<HotShotEvent<TestTypes>>,
) {
    let builder = TestMetadata::default_multiple_rounds();

    let launcher = builder.gen_launcher::<TestTypes, MemoryImpl>(node_id);

    let networks = (launcher.resource_generator.channel_generator)(node_id);
    let storage = (launcher.resource_generator.storage)(node_id);
    let config = launcher.resource_generator.config.clone();

    let initializer = HotShotInitializer::<TestTypes>::from_genesis().unwrap();

    let known_nodes_with_stake = config.known_nodes_with_stake.clone();
    let private_key = config.my_own_validator_config.private_key.clone();
    let public_key = config.my_own_validator_config.public_key;
    let quorum_election_config =
        config.election_config.clone().unwrap_or_else(|| {
            <TestTypes as NodeType>::Membership::default_election_config(
                config.total_nodes.get() as u64
            )
        });

    let committee_election_config =
        config.election_config.clone().unwrap_or_else(|| {
            <TestTypes as NodeType>::Membership::default_election_config(
                config.total_nodes.get() as u64
            )
        });
    let networks_bundle = Networks {
        quorum_network: networks.0.clone(),
        da_network: networks.1.clone(),
        _pd: PhantomData,
    };

    let memberships = Memberships {
        quorum_membership: <TestTypes as NodeType>::Membership::create_election(
            known_nodes_with_stake.clone(),
            quorum_election_config.clone(),
        ),
        da_membership: <TestTypes as NodeType>::Membership::create_election(
            known_nodes_with_stake.clone(),
            committee_election_config,
        ),
        vid_membership: <TestTypes as NodeType>::Membership::create_election(
            known_nodes_with_stake.clone(),
            quorum_election_config.clone(),
        ),
        view_sync_membership: <TestTypes as NodeType>::Membership::create_election(
            known_nodes_with_stake.clone(),
            quorum_election_config,
        ),
    };

    SystemContext::init(
        public_key,
        private_key,
        node_id,
        config,
        storage,
        memberships,
        networks_bundle,
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
) -> (QuorumProposal<TestTypes>, EncodedSignature) {
    let consensus_lock = handle.get_consensus();
    let consensus = consensus_lock.read().await;
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let parent_view_number = &consensus.high_qc.get_view_number();
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

    // every event input is seen on the event stream in the output.
    let block = <VIDBlockPayload as TestableBlock>::genesis();
    let payload_commitment = block.commit();
    let block_header = VIDBlockHeader::new(payload_commitment, (), &parent_leaf.block_header);
    let leaf = Leaf {
        view_number: ViewNumber::new(view),
        justify_qc: consensus.high_qc.clone(),
        parent_commitment: parent_leaf.commit(),
        block_header: block_header.clone(),
        block_payload: None,
        rejected: vec![],
        timestamp: 0,
        proposer_id: api.public_key().to_bytes(),
    };
    let signature = <BLSPubKey as SignatureKey>::sign(private_key, leaf.commit().as_ref());
    let proposal = QuorumProposal::<TestTypes> {
        block_header,
        view_number: ViewNumber::new(view),
        justify_qc: QuorumCertificate::genesis(),
        timeout_certificate: None,
        proposer_id: leaf.proposer_id,
    };

    (proposal, signature)
}

pub async fn build_quorum_proposal(
    handle: &SystemContextHandle<TestTypes, MemoryImpl>,
    private_key: &<BLSPubKey as SignatureKey>::PrivateKey,
    view: u64,
) -> Proposal<TestTypes, QuorumProposal<TestTypes>> {
    let (proposal, signature) =
        build_quorum_proposal_and_signature(handle, private_key, view).await;
    Proposal {
        data: proposal,
        signature,
        _pd: PhantomData,
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
    VidScheme::new(NUM_CHUNKS, NUM_STORAGE_NODES, srs).unwrap()
}
