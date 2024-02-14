#![allow(clippy::panic)]
use std::marker::PhantomData;

use hotshot_example_types::{
    block_types::{TestBlockHeader, TestBlockPayload},
    node_types::{MemoryImpl, TestTypes},
    state_types::{TestInstanceState, TestValidatedState},
};

use crate::test_builder::TestMetadata;
use commit::Committable;
use ethereum_types::U256;
use hotshot::{
    types::{BLSPubKey, SignatureKey, SystemContextHandle},
    HotShotConsensusApi, HotShotInitializer, Memberships, Networks, SystemContext,
};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    data::{Leaf, QuorumProposal, VidScheme, ViewNumber},
    message::Proposal,
    simple_certificate::QuorumCertificate,
    simple_vote::SimpleVote,
    traits::{
        block_contents::{vid_commitment, BlockHeader, TestableBlock},
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
        states::ValidatedState,
        BlockPayload,
    },
    vote::HasViewNumber,
};

use async_broadcast::{Receiver, Sender};
use async_lock::RwLockUpgradableReadGuard;
use bitvec::bitvec;
use hotshot_types::simple_vote::QuorumData;
use hotshot_types::simple_vote::QuorumVote;
use hotshot_types::utils::View;
use hotshot_types::utils::ViewInner;
use hotshot_types::vote::Certificate;
use hotshot_types::vote::Vote;

use serde::Serialize;
use std::{fmt::Debug, hash::Hash, sync::Arc};

/// create the [`SystemContextHandle`] from a node id
/// # Panics
/// if cannot create a [`HotShotInitializer`]
pub async fn build_system_handle(
    node_id: u64,
) -> (
    SystemContextHandle<TestTypes, MemoryImpl>,
    Sender<HotShotEvent<TestTypes>>,
    Receiver<HotShotEvent<TestTypes>>,
) {
    let builder = TestMetadata::default_multiple_rounds();

    let launcher = builder.gen_launcher::<TestTypes, MemoryImpl>(node_id);

    let networks = (launcher.resource_generator.channel_generator)(node_id);
    let storage = (launcher.resource_generator.storage)(node_id);
    let config = launcher.resource_generator.config.clone();

    let initializer = HotShotInitializer::<TestTypes>::from_genesis(&TestInstanceState {}).unwrap();

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
        ConsensusMetricsValue::default(),
    )
    .await
    .expect("Could not init hotshot")
}

/// create certificate
/// # Panics
/// if we fail to sign the data
pub fn build_cert<
    TYPES: NodeType<SignatureKey = BLSPubKey>,
    DATAType: Committable + Clone + Eq + Hash + Serialize + Debug + 'static,
    VOTE: Vote<TYPES, Commitment = DATAType>,
    CERT: Certificate<TYPES, Voteable = VOTE::Commitment>,
>(
    data: DATAType,
    membership: &TYPES::Membership,
    view: TYPES::Time,
    public_key: &TYPES::SignatureKey,
    private_key: &<TYPES::SignatureKey as SignatureKey>::PrivateKey,
) -> CERT {
    let real_qc_sig = build_assembled_sig::<TYPES, VOTE, CERT, DATAType>(&data, membership, view);

    let vote =
        SimpleVote::<TYPES, DATAType>::create_signed_vote(data, view, public_key, private_key)
            .expect("Failed to sign data!");
    let cert = CERT::create_signed_certificate(
        vote.get_data_commitment(),
        vote.get_data().clone(),
        real_qc_sig,
        vote.get_view_number(),
    );
    cert
}

/// create signature
/// # Panics
/// if fails to convert node id into keypair
pub fn build_assembled_sig<
    TYPES: NodeType<SignatureKey = BLSPubKey>,
    VOTE: Vote<TYPES>,
    CERT: Certificate<TYPES, Voteable = VOTE::Commitment>,
    DATAType: Committable + Clone + Eq + Hash + Serialize + Debug + 'static,
>(
    data: &DATAType,
    membership: &TYPES::Membership,
    view: TYPES::Time,
) -> <TYPES::SignatureKey as SignatureKey>::QCType {
    let stake_table = membership.get_committee_qc_stake_table();
    let real_qc_pp: <TYPES::SignatureKey as SignatureKey>::QCParams =
        <TYPES::SignatureKey as SignatureKey>::get_public_parameter(
            stake_table.clone(),
            U256::from(CERT::threshold(membership)),
        );
    let total_nodes = stake_table.len();
    let signers = bitvec![1; total_nodes];
    let mut sig_lists = Vec::new();

    // assemble the vote
    for node_id in 0..total_nodes {
        let (private_key_i, public_key_i) = key_pair_for_id(node_id.try_into().unwrap());
        let vote: SimpleVote<TYPES, DATAType> = SimpleVote::<TYPES, DATAType>::create_signed_vote(
            data.clone(),
            view,
            &public_key_i,
            &private_key_i,
        )
        .expect("Failed to sign data!");
        let original_signature: <TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType =
            vote.get_signature();
        sig_lists.push(original_signature);
    }

    let real_qc_sig = <TYPES::SignatureKey as SignatureKey>::assemble(
        &real_qc_pp,
        signers.as_bitslice(),
        &sig_lists[..],
    );

    real_qc_sig
}

/// build a quorum proposal and signature
#[allow(clippy::too_many_lines)]
async fn build_quorum_proposal_and_signature(
    handle: &SystemContextHandle<TestTypes, MemoryImpl>,
    private_key: &<BLSPubKey as SignatureKey>::PrivateKey,
    public_key: &BLSPubKey,
    view: u64,
) -> (
    QuorumProposal<TestTypes>,
    <BLSPubKey as SignatureKey>::PureAssembledSignatureType,
) {
    // build the genesis view
    let genesis_consensus = handle.get_consensus();
    let cur_consensus = genesis_consensus.upgradable_read().await;
    let mut consensus = RwLockUpgradableReadGuard::upgrade(cur_consensus).await;
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    // parent_view_number should be equal to 0
    let parent_view_number = &consensus.high_qc.get_view_number();
    assert_eq!(parent_view_number.get_u64(), 0);
    let Some(parent_view) = consensus.validated_state_map.get(parent_view_number) else {
        panic!("Couldn't find high QC parent in state map.");
    };
    let Some(leaf_view_0) = parent_view.get_leaf_commitment() else {
        panic!("Parent of high QC points to a view without a proposal");
    };
    let Some(leaf_view_0) = consensus.saved_leaves.get(&leaf_view_0) else {
        panic!("Failed to find high QC parent.");
    };
    let parent_leaf = leaf_view_0.clone();

    // every event input is seen on the event stream in the output.
    let block = <TestBlockPayload as TestableBlock>::genesis();
    let payload_commitment = vid_commitment(
        &block.encode().unwrap().collect(),
        handle
            .hotshot
            .inner
            .memberships
            .quorum_membership
            .total_nodes(),
    );
    let mut parent_state = Arc::new(<TestValidatedState as ValidatedState>::from_header(
        &parent_leaf.block_header,
    ));
    let block_header = TestBlockHeader::new(
        &*parent_state,
        &TestInstanceState {},
        &parent_leaf.block_header,
        payload_commitment,
        (),
    );
    // current leaf that can be re-assigned everytime when entering a new view
    let mut leaf = Leaf {
        view_number: ViewNumber::new(1),
        justify_qc: consensus.high_qc.clone(),
        parent_commitment: parent_leaf.commit(),
        block_header: block_header.clone(),
        block_payload: None,
        proposer_id: *api.public_key(),
    };

    let mut signature = <BLSPubKey as SignatureKey>::sign(private_key, leaf.commit().as_ref())
        .expect("Failed to sign leaf commitment!");
    let mut proposal = QuorumProposal::<TestTypes> {
        block_header: block_header.clone(),
        view_number: ViewNumber::new(1),
        justify_qc: QuorumCertificate::genesis(),
        timeout_certificate: None,
        upgrade_certificate: None,
        proposer_id: leaf.proposer_id,
    };

    // Only view 2 is tested, higher views are not tested
    for cur_view in 2..=view {
        let state_new_view = Arc::new(
            parent_state
                .validate_and_apply_header(&TestInstanceState {}, &block_header, &block_header)
                .unwrap(),
        );
        // save states for the previous view to pass all the qc checks
        // In the long term, we want to get rid of this, do not manually update consensus state
        consensus.validated_state_map.insert(
            ViewNumber::new(cur_view - 1),
            View {
                view_inner: ViewInner::Leaf {
                    leaf: leaf.commit(),
                    state: state_new_view.clone(),
                },
            },
        );
        consensus.saved_leaves.insert(leaf.commit(), leaf.clone());
        // create a qc by aggregate signatures on the previous view (the data signed is last leaf commitment)
        let quorum_membership = handle.hotshot.inner.memberships.quorum_membership.clone();
        let quorum_data = QuorumData {
            leaf_commit: leaf.commit(),
        };
        let created_qc = build_cert::<
            TestTypes,
            QuorumData<TestTypes>,
            QuorumVote<TestTypes>,
            QuorumCertificate<TestTypes>,
        >(
            quorum_data,
            &quorum_membership,
            ViewNumber::new(cur_view - 1),
            public_key,
            private_key,
        );
        // create a new leaf for the current view
        let parent_leaf = leaf.clone();
        let leaf_new_view = Leaf {
            view_number: ViewNumber::new(cur_view),
            justify_qc: created_qc.clone(),
            parent_commitment: parent_leaf.commit(),
            block_header: block_header.clone(),
            block_payload: None,
            proposer_id: quorum_membership.get_leader(ViewNumber::new(cur_view)),
        };
        let signature_new_view =
            <BLSPubKey as SignatureKey>::sign(private_key, leaf_new_view.commit().as_ref())
                .expect("Failed to sign leaf commitment!");
        let proposal_new_view = QuorumProposal::<TestTypes> {
            block_header: block_header.clone(),
            view_number: ViewNumber::new(cur_view),
            justify_qc: created_qc,
            timeout_certificate: None,
            upgrade_certificate: None,
            proposer_id: leaf_new_view.clone().proposer_id,
        };
        proposal = proposal_new_view;
        signature = signature_new_view;
        leaf = leaf_new_view;
        parent_state = state_new_view;
    }

    (proposal, signature)
}

/// create a quorum proposal
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

/// get the keypair for a node id
#[must_use]
pub fn key_pair_for_id(node_id: u64) -> (<BLSPubKey as SignatureKey>::PrivateKey, BLSPubKey) {
    let private_key =
        <BLSPubKey as SignatureKey>::generated_from_seed_indexed([0u8; 32], node_id).1;
    let public_key = <TestTypes as NodeType>::SignatureKey::from_private(&private_key);
    (private_key, public_key)
}

/// initialize VID
/// # Panics
/// if unable to create a [`VidScheme`]
#[must_use]
pub fn vid_init<TYPES: NodeType>(
    membership: &TYPES::Membership,
    view_number: TYPES::Time,
) -> VidScheme {
    let num_committee = membership.get_committee(view_number).len();

    // calculate the last power of two
    // TODO change after https://github.com/EspressoSystems/jellyfish/issues/339
    // issue: https://github.com/EspressoSystems/HotShot/issues/2152
    let chunk_size = 1 << num_committee.ilog2();

    // TODO <https://github.com/EspressoSystems/HotShot/issues/1686>
    let srs = hotshot_types::data::test_srs(num_committee);

    VidScheme::new(chunk_size, num_committee, srs).unwrap()
}
