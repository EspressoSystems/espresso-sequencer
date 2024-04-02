#![allow(clippy::panic)]
use std::marker::PhantomData;

use hotshot_example_types::{
    block_types::{TestBlockHeader, TestBlockPayload, TestTransaction},
    node_types::{MemoryImpl, TestTypes},
    state_types::{TestInstanceState, TestValidatedState},
};

use crate::test_builder::TestMetadata;
use commit::Committable;
use ethereum_types::U256;
use hotshot::{
    types::{BLSPubKey, SignatureKey, SystemContextHandle},
    HotShotInitializer, Memberships, Networks, SystemContext,
};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    data::{Leaf, QuorumProposal, VidDisperse, ViewNumber},
    message::{GeneralConsensusMessage, Proposal},
    simple_certificate::{DACertificate, QuorumCertificate},
    simple_vote::{DAData, DAVote, SimpleVote},
    traits::{
        block_contents::{vid_commitment, BlockHeader, TestableBlock},
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
        states::ValidatedState,
        BlockPayload,
    },
    vid::{vid_scheme, VidCommitment, VidSchemeType},
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

use jf_primitives::vid::VidScheme;

use hotshot_types::data::VidDisperseShare;
use serde::Serialize;
use std::{fmt::Debug, hash::Hash, sync::Arc};

/// create the [`SystemContextHandle`] from a node id
/// # Panics
/// if cannot create a [`HotShotInitializer`]
pub async fn build_system_handle(
    node_id: u64,
) -> (
    SystemContextHandle<TestTypes, MemoryImpl>,
    Sender<Arc<HotShotEvent<TestTypes>>>,
    Receiver<Arc<HotShotEvent<TestTypes>>>,
) {
    let builder = TestMetadata::default_multiple_rounds();

    let launcher = builder.gen_launcher::<TestTypes, MemoryImpl>(node_id);

    let networks = (launcher.resource_generator.channel_generator)(node_id).await;
    let storage = (launcher.resource_generator.storage)(node_id);
    let config = launcher.resource_generator.config.clone();

    let initializer = HotShotInitializer::<TestTypes>::from_genesis(TestInstanceState {}).unwrap();

    let known_nodes_with_stake = config.known_nodes_with_stake.clone();
    let private_key = config.my_own_validator_config.private_key.clone();
    let public_key = config.my_own_validator_config.public_key;

    let _known_nodes_without_stake = config.known_nodes_without_stake.clone();

    let quorum_election_config = config.election_config.clone().unwrap_or_else(|| {
        <TestTypes as NodeType>::Membership::default_election_config(
            config.num_nodes_with_stake.get() as u64,
            config.num_nodes_without_stake as u64,
        )
    });

    let committee_election_config = config.election_config.clone().unwrap_or_else(|| {
        <TestTypes as NodeType>::Membership::default_election_config(
            config.num_nodes_with_stake.get() as u64,
            config.num_nodes_without_stake as u64,
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
            config.fixed_leader_for_gpuvid,
        ),
        da_membership: <TestTypes as NodeType>::Membership::create_election(
            known_nodes_with_stake.clone(),
            committee_election_config,
            config.fixed_leader_for_gpuvid,
        ),
        vid_membership: <TestTypes as NodeType>::Membership::create_election(
            known_nodes_with_stake.clone(),
            quorum_election_config.clone(),
            config.fixed_leader_for_gpuvid,
        ),
        view_sync_membership: <TestTypes as NodeType>::Membership::create_election(
            known_nodes_with_stake.clone(),
            quorum_election_config,
            config.fixed_leader_for_gpuvid,
        ),
    };

    SystemContext::init(
        public_key,
        private_key,
        node_id,
        config,
        memberships,
        networks_bundle,
        initializer,
        ConsensusMetricsValue::default(),
        storage,
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
        handle.hotshot.memberships.quorum_membership.total_nodes(),
    );
    let mut parent_state = Arc::new(
        <TestValidatedState as ValidatedState<TestTypes>>::from_header(
            parent_leaf.get_block_header(),
        ),
    );
    let block_header = TestBlockHeader::new(
        &*parent_state,
        &TestInstanceState {},
        &parent_leaf,
        payload_commitment,
        (),
    )
    .await;
    let mut proposal = QuorumProposal::<TestTypes> {
        block_header: block_header.clone(),
        view_number: ViewNumber::new(1),
        justify_qc: QuorumCertificate::genesis(),
        upgrade_certificate: None,
        proposal_certificate: None,
    };
    // current leaf that can be re-assigned everytime when entering a new view
    let mut leaf = Leaf::from_quorum_proposal(&proposal);

    let mut signature = <BLSPubKey as SignatureKey>::sign(private_key, leaf.commit().as_ref())
        .expect("Failed to sign leaf commitment!");

    // Only view 2 is tested, higher views are not tested
    for cur_view in 2..=view {
        let (state_new_view, delta_new_view) = parent_state
            .validate_and_apply_header(&TestInstanceState {}, &parent_leaf, &block_header)
            .await
            .unwrap();
        let state_new_view = Arc::new(state_new_view);
        // save states for the previous view to pass all the qc checks
        // In the long term, we want to get rid of this, do not manually update consensus state
        consensus.validated_state_map.insert(
            ViewNumber::new(cur_view - 1),
            View {
                view_inner: ViewInner::Leaf {
                    leaf: leaf.commit(),
                    state: state_new_view.clone(),
                    delta: Some(Arc::new(delta_new_view)),
                },
            },
        );
        consensus.saved_leaves.insert(leaf.commit(), leaf.clone());
        // create a qc by aggregate signatures on the previous view (the data signed is last leaf commitment)
        let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
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
        let proposal_new_view = QuorumProposal::<TestTypes> {
            block_header: block_header.clone(),
            view_number: ViewNumber::new(cur_view),
            justify_qc: created_qc,
            upgrade_certificate: None,
            proposal_certificate: None,
        };
        let leaf_new_view = Leaf::from_quorum_proposal(&proposal_new_view);
        let signature_new_view =
            <BLSPubKey as SignatureKey>::sign(private_key, leaf_new_view.commit().as_ref())
                .expect("Failed to sign leaf commitment!");
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
/// if unable to create a [`VidSchemeType`]
#[must_use]
pub fn vid_scheme_from_view_number<TYPES: NodeType>(
    membership: &TYPES::Membership,
    view_number: TYPES::Time,
) -> VidSchemeType {
    let num_storage_nodes = membership.get_staked_committee(view_number).len();
    vid_scheme(num_storage_nodes)
}

pub fn vid_payload_commitment(
    quorum_membership: &<TestTypes as NodeType>::Membership,
    view_number: ViewNumber,
    transactions: Vec<TestTransaction>,
) -> VidCommitment {
    let mut vid = vid_scheme_from_view_number::<TestTypes>(quorum_membership, view_number);
    let encoded_transactions = TestTransaction::encode(transactions.clone()).unwrap();
    let vid_disperse = vid.disperse(encoded_transactions).unwrap();

    vid_disperse.commit
}

pub fn da_payload_commitment(
    quorum_membership: &<TestTypes as NodeType>::Membership,
    transactions: Vec<TestTransaction>,
) -> VidCommitment {
    let encoded_transactions = TestTransaction::encode(transactions.clone()).unwrap();

    vid_commitment(&encoded_transactions, quorum_membership.total_nodes())
}

/// TODO: <https://github.com/EspressoSystems/HotShot/issues/2821>
pub fn build_vid_proposal(
    quorum_membership: &<TestTypes as NodeType>::Membership,
    view_number: ViewNumber,
    transactions: Vec<TestTransaction>,
    private_key: &<BLSPubKey as SignatureKey>::PrivateKey,
) -> Vec<Proposal<TestTypes, VidDisperseShare<TestTypes>>> {
    let mut vid = vid_scheme_from_view_number::<TestTypes>(quorum_membership, view_number);
    let encoded_transactions = TestTransaction::encode(transactions.clone()).unwrap();

    let vid_disperse = VidDisperse::from_membership(
        view_number,
        vid.disperse(encoded_transactions).unwrap(),
        &quorum_membership.clone().into(),
    );

    VidDisperseShare::from_vid_disperse(vid_disperse)
        .into_iter()
        .map(|vid_disperse| {
            vid_disperse
                .to_proposal(private_key)
                .expect("Failed to sign payload commitment")
        })
        .collect()
}

pub fn build_da_certificate(
    quorum_membership: &<TestTypes as NodeType>::Membership,
    view_number: ViewNumber,
    transactions: Vec<TestTransaction>,
    public_key: &<TestTypes as NodeType>::SignatureKey,
    private_key: &<BLSPubKey as SignatureKey>::PrivateKey,
) -> DACertificate<TestTypes> {
    let encoded_transactions = TestTransaction::encode(transactions.clone()).unwrap();

    let da_payload_commitment =
        vid_commitment(&encoded_transactions, quorum_membership.total_nodes());

    let da_data = DAData {
        payload_commit: da_payload_commitment,
    };

    build_cert::<TestTypes, DAData, DAVote<TestTypes>, DACertificate<TestTypes>>(
        da_data,
        quorum_membership,
        view_number,
        public_key,
        private_key,
    )
}

pub async fn build_vote(
    handle: &SystemContextHandle<TestTypes, MemoryImpl>,
    proposal: QuorumProposal<TestTypes>,
) -> GeneralConsensusMessage<TestTypes> {
    let view = ViewNumber::new(*proposal.view_number);

    let leaf: Leaf<_> = Leaf::from_quorum_proposal(&proposal);
    let vote = QuorumVote::<TestTypes>::create_signed_vote(
        QuorumData {
            leaf_commit: leaf.commit(),
        },
        view,
        handle.public_key(),
        handle.private_key(),
    )
    .expect("Failed to create quorum vote");
    GeneralConsensusMessage::<TestTypes>::Vote(vote)
}
