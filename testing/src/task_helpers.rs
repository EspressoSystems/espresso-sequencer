use std::marker::PhantomData;

use crate::{
    block_types::{TestBlockHeader, TestBlockPayload},
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
    consensus::ConsensusMetricsValue,
    data::{Leaf, QuorumProposal, VidScheme, ViewNumber},
    message::Proposal,
    simple_certificate::QuorumCertificate,
    traits::{
        block_contents::BlockHeader,
        block_contents::{vid_commitment, NUM_CHUNKS, NUM_STORAGE_NODES},
        consensus_api::ConsensusSharedApi,
        election::Membership,
        node_implementation::NodeType,
        signature_key::EncodedSignature,
        state::{ConsensusTime, TestableBlock},
        BlockPayload,
    },
    vote::HasViewNumber,
};

use async_std::sync::RwLockUpgradableReadGuard;
use bincode::Options;
use bitvec::bitvec;
use hotshot_types::simple_vote::QuorumData;
use hotshot_types::simple_vote::QuorumVote;
use hotshot_types::utils::View;
use hotshot_types::utils::ViewInner;
use hotshot_types::vote::Certificate;
use hotshot_types::vote::Vote;
use hotshot_utils::bincode::bincode_opts;

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

pub fn build_assembled_sig<
    TYPES: NodeType<SignatureKey = BLSPubKey>,
    VOTE: Vote<TYPES>,
    CERT: Certificate<TYPES, Voteable = VOTE::Commitment>,
>(
    leaf: Leaf<TYPES>,
    handle: &SystemContextHandle<TestTypes, MemoryImpl>,
    view: TYPES::Time,
) -> <TYPES::SignatureKey as SignatureKey>::QCType {
    // Assemble QC
    let stake_table = handle.get_committee_qc_stake_table();
    let real_qc_pp: <TYPES::SignatureKey as SignatureKey>::QCParams =
        <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
            stake_table.clone(),
            handle.get_threshold(),
        );
    let total_nodes = stake_table.len();
    let signers = bitvec![1; total_nodes];
    let mut sig_lists = Vec::new();

    // calculate vote
    for node_id in 0..total_nodes {
        let (private_key, public_key) = key_pair_for_id(node_id.try_into().unwrap());
        let vote = QuorumVote::<TYPES>::create_signed_vote(
            QuorumData {
                leaf_commit: leaf.commit(),
            },
            view,
            &public_key,
            &private_key,
        );
        let original_signature: <TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType =
            bincode_opts()
                .deserialize(&vote.get_signature().0)
                .expect("Deserialization on the signature shouldn't be able to fail.");
        sig_lists.push(original_signature);
    }

    let real_qc_sig = <TYPES::SignatureKey as SignatureKey>::assemble(
        &real_qc_pp,
        signers.as_bitslice(),
        &sig_lists[..],
    );

    real_qc_sig
}

pub fn build_qc<
    TYPES: NodeType,
    VOTE: Vote<TYPES, Commitment = QuorumData<TYPES>>,
    CERT: Certificate<TYPES, Voteable = VOTE::Commitment>,
>(
    real_qc_sig: <TYPES::SignatureKey as SignatureKey>::QCType,
    leaf: Leaf<TYPES>,
    view: TYPES::Time,
    public_key: &TYPES::SignatureKey,
    private_key: &<TYPES::SignatureKey as SignatureKey>::PrivateKey,
) -> CERT {
    let vote = QuorumVote::<TYPES>::create_signed_vote(
        QuorumData {
            leaf_commit: leaf.commit(),
        },
        view,
        public_key,
        private_key,
    );
    let cert = CERT::create_signed_certificate(
        vote.get_data_commitment(),
        vote.get_data().clone(),
        real_qc_sig,
        vote.get_view_number(),
    );
    cert
}

async fn build_quorum_proposal_and_signature(
    handle: &SystemContextHandle<TestTypes, MemoryImpl>,
    private_key: &<BLSPubKey as SignatureKey>::PrivateKey,
    public_key: &BLSPubKey,
    view: u64,
) -> (QuorumProposal<TestTypes>, EncodedSignature) {
    let temp_consensus = handle.get_consensus();
    let cur_consensus = temp_consensus.upgradable_read().await;
    let mut consensus = RwLockUpgradableReadGuard::upgrade(cur_consensus).await;

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
    let block = <TestBlockPayload as TestableBlock>::genesis();
    let payload_commitment = vid_commitment(&block.encode().unwrap().collect());
    let block_header = TestBlockHeader::new(payload_commitment, (), &parent_leaf.block_header);
    let leaf = Leaf {
        view_number: ViewNumber::new(1),
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
        block_header: block_header.clone(),
        view_number: ViewNumber::new(1),
        justify_qc: QuorumCertificate::genesis(),
        timeout_certificate: None,
        proposer_id: leaf.proposer_id.clone(),
    };

    if view == 2 {
        consensus.state_map.insert(
            ViewNumber::new(1),
            View {
                view_inner: ViewInner::Leaf {
                    leaf: leaf.commit(),
                },
            },
        );
        consensus.saved_leaves.insert(leaf.commit(), leaf.clone());
        let created_assembled_sig = build_assembled_sig::<
            TestTypes,
            QuorumVote<TestTypes>,
            QuorumCertificate<TestTypes>,
        >(leaf.clone(), handle, ViewNumber::new(1));
        let created_qc = build_qc::<TestTypes, QuorumVote<TestTypes>, QuorumCertificate<TestTypes>>(
            created_assembled_sig,
            leaf.clone(),
            ViewNumber::new(1),
            public_key,
            private_key,
        );
        let parent_leaf = leaf.clone();
        let leaf_view2 = Leaf {
            view_number: ViewNumber::new(2),
            justify_qc: created_qc.clone(),
            parent_commitment: parent_leaf.commit(),
            block_header: block_header.clone(),
            block_payload: None,
            rejected: vec![],
            timestamp: 0,
            proposer_id: api.public_key().to_bytes(),
        };
        let signature_view2 =
            <BLSPubKey as SignatureKey>::sign(private_key, leaf_view2.commit().as_ref());
        let proposal_view2 = QuorumProposal::<TestTypes> {
            block_header,
            view_number: ViewNumber::new(2),
            justify_qc: created_qc,
            timeout_certificate: None,
            proposer_id: leaf_view2.proposer_id,
        };
        return (proposal_view2, signature_view2);
    }

    (proposal, signature)
}

pub async fn build_quorum_proposal(
    handle: &SystemContextHandle<TestTypes, MemoryImpl>,
    private_key: &<BLSPubKey as SignatureKey>::PrivateKey,
    view: u64,
) -> Proposal<TestTypes, QuorumProposal<TestTypes>> {
    let public_key = &BLSPubKey::from_private(private_key);
    let (proposal, signature) =
        build_quorum_proposal_and_signature(handle, private_key, public_key, view).await;
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
