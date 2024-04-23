#![allow(clippy::panic)]
use std::{fmt::Debug, hash::Hash, marker::PhantomData, sync::Arc};

use async_broadcast::{Receiver, Sender};
use bitvec::bitvec;
use committable::Committable;
use ethereum_types::U256;
use hotshot::{
    types::{BLSPubKey, SignatureKey, SystemContextHandle},
    HotShotInitializer, Memberships, Networks, SystemContext,
};
use hotshot_example_types::{
    block_types::TestTransaction,
    node_types::{MemoryImpl, TestTypes},
    state_types::TestInstanceState,
};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    data::{Leaf, QuorumProposal, VidDisperse, VidDisperseShare, ViewNumber},
    message::{GeneralConsensusMessage, Proposal},
    simple_certificate::DACertificate,
    simple_vote::{DAData, DAVote, QuorumData, QuorumVote, SimpleVote},
    traits::{
        block_contents::vid_commitment,
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
    },
    vid::{vid_scheme, VidCommitment, VidSchemeType},
    vote::{Certificate, HasViewNumber, Vote},
};
use jf_primitives::vid::VidScheme;
use serde::Serialize;

use crate::test_builder::TestDescription;

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
    let builder = TestDescription::default_multiple_rounds();

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

pub fn get_vid_share<TYPES: NodeType>(
    shares: &[Proposal<TYPES, VidDisperseShare<TYPES>>],
    pub_key: TYPES::SignatureKey,
) -> Proposal<TYPES, VidDisperseShare<TYPES>> {
    shares
        .iter()
        .filter(|s| s.data.recipient_key == pub_key)
        .cloned()
        .collect::<Vec<_>>()
        .first()
        .expect("No VID for key")
        .clone()
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
    let encoded_transactions = TestTransaction::encode(&transactions).unwrap();
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();

    vid_disperse.commit
}

pub fn da_payload_commitment(
    quorum_membership: &<TestTypes as NodeType>::Membership,
    transactions: Vec<TestTransaction>,
) -> VidCommitment {
    let encoded_transactions = TestTransaction::encode(&transactions).unwrap();

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
    let encoded_transactions = TestTransaction::encode(&transactions).unwrap();

    let vid_disperse = VidDisperse::from_membership(
        view_number,
        vid.disperse(&encoded_transactions).unwrap(),
        quorum_membership,
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
    let encoded_transactions = TestTransaction::encode(&transactions).unwrap();

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
