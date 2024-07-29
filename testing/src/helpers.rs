#![allow(clippy::panic)]
use std::{fmt::Debug, hash::Hash, marker::PhantomData, sync::Arc};

use async_broadcast::{Receiver, Sender};
use bitvec::bitvec;
use committable::Committable;
use ethereum_types::U256;
use hotshot::{
    traits::{NodeImplementation, TestableNodeImplementation},
    types::{BLSPubKey, SignatureKey, SystemContextHandle},
    HotShotInitializer, Memberships, SystemContext,
};
use hotshot_example_types::{
    auction_results_provider_types::TestAuctionResultsProvider,
    block_types::TestTransaction,
    node_types::{MemoryImpl, TestTypes},
    state_types::{TestInstanceState, TestValidatedState},
    storage_types::TestStorage,
};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    data::{Leaf, QuorumProposal, VidDisperse, VidDisperseShare, ViewNumber},
    message::{GeneralConsensusMessage, Proposal},
    simple_certificate::DaCertificate,
    simple_vote::{DaData, DaVote, QuorumData, QuorumVote, SimpleVote},
    traits::{
        block_contents::vid_commitment,
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
    },
    utils::{View, ViewInner},
    vid::{vid_scheme, VidCommitment, VidSchemeType},
    vote::{Certificate, HasViewNumber, Vote},
};
use jf_vid::VidScheme;
use serde::Serialize;

use crate::test_builder::TestDescription;

/// create the [`SystemContextHandle`] from a node id
/// # Panics
/// if cannot create a [`HotShotInitializer`]
pub async fn build_system_handle<
    TYPES: NodeType<InstanceState = TestInstanceState>,
    I: NodeImplementation<
            TYPES,
            Storage = TestStorage<TYPES>,
            AuctionResultsProvider = TestAuctionResultsProvider,
        > + TestableNodeImplementation<TYPES>,
>(
    node_id: u64,
) -> (
    SystemContextHandle<TYPES, I>,
    Sender<Arc<HotShotEvent<TYPES>>>,
    Receiver<Arc<HotShotEvent<TYPES>>>,
) {
    let builder: TestDescription<TYPES, I> = TestDescription::default_multiple_rounds();

    let launcher = builder.gen_launcher(node_id);

    let network = (launcher.resource_generator.channel_generator)(node_id).await;
    let storage = (launcher.resource_generator.storage)(node_id);
    let auction_results_provider = (launcher.resource_generator.auction_results_provider)(node_id);
    let config = launcher.resource_generator.config.clone();

    let initializer = HotShotInitializer::<TYPES>::from_genesis(TestInstanceState {})
        .await
        .unwrap();

    let known_nodes_with_stake = config.known_nodes_with_stake.clone();
    let private_key = config.my_own_validator_config.private_key.clone();
    let public_key = config.my_own_validator_config.public_key.clone();

    let _known_nodes_without_stake = config.known_nodes_without_stake.clone();

    let memberships = Memberships {
        quorum_membership: TYPES::Membership::create_election(
            known_nodes_with_stake.clone(),
            known_nodes_with_stake.clone(),
            config.fixed_leader_for_gpuvid,
        ),
        da_membership: TYPES::Membership::create_election(
            known_nodes_with_stake.clone(),
            config.known_da_nodes.clone(),
            config.fixed_leader_for_gpuvid,
        ),
        vid_membership: TYPES::Membership::create_election(
            known_nodes_with_stake.clone(),
            known_nodes_with_stake.clone(),
            config.fixed_leader_for_gpuvid,
        ),
        view_sync_membership: TYPES::Membership::create_election(
            known_nodes_with_stake.clone(),
            known_nodes_with_stake,
            config.fixed_leader_for_gpuvid,
        ),
    };

    SystemContext::init(
        public_key,
        private_key,
        node_id,
        config,
        memberships,
        network,
        initializer,
        ConsensusMetricsValue::default(),
        storage,
        auction_results_provider,
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
        vote.date_commitment(),
        vote.date().clone(),
        real_qc_sig,
        vote.view_number(),
    );
    cert
}

pub fn vid_share<TYPES: NodeType>(
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
) -> <TYPES::SignatureKey as SignatureKey>::QcType {
    let stake_table = membership.committee_qc_stake_table();
    let real_qc_pp: <TYPES::SignatureKey as SignatureKey>::QcParams =
        <TYPES::SignatureKey as SignatureKey>::public_parameter(
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
            vote.signature();
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
    let num_storage_nodes = membership.staked_committee(view_number).len();
    vid_scheme(num_storage_nodes)
}

pub fn vid_payload_commitment(
    quorum_membership: &<TestTypes as NodeType>::Membership,
    view_number: ViewNumber,
    transactions: Vec<TestTransaction>,
) -> VidCommitment {
    let mut vid = vid_scheme_from_view_number::<TestTypes>(quorum_membership, view_number);
    let encoded_transactions = TestTransaction::encode(&transactions);
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();

    vid_disperse.commit
}

pub fn da_payload_commitment(
    quorum_membership: &<TestTypes as NodeType>::Membership,
    transactions: Vec<TestTransaction>,
) -> VidCommitment {
    let encoded_transactions = TestTransaction::encode(&transactions);

    vid_commitment(&encoded_transactions, quorum_membership.total_nodes())
}

pub fn build_payload_commitment(
    membership: &<TestTypes as NodeType>::Membership,
    view: ViewNumber,
) -> <VidSchemeType as VidScheme>::Commit {
    // Make some empty encoded transactions, we just care about having a commitment handy for the
    // later calls. We need the VID commitment to be able to propose later.
    let mut vid = vid_scheme_from_view_number::<TestTypes>(membership, view);
    let encoded_transactions = Vec::new();
    vid.commit_only(&encoded_transactions).unwrap()
}

/// TODO: <https://github.com/EspressoSystems/HotShot/issues/2821>
#[allow(clippy::type_complexity)]
pub fn build_vid_proposal(
    quorum_membership: &<TestTypes as NodeType>::Membership,
    view_number: ViewNumber,
    transactions: Vec<TestTransaction>,
    private_key: &<BLSPubKey as SignatureKey>::PrivateKey,
) -> (
    Proposal<TestTypes, VidDisperse<TestTypes>>,
    Vec<Proposal<TestTypes, VidDisperseShare<TestTypes>>>,
) {
    let mut vid = vid_scheme_from_view_number::<TestTypes>(quorum_membership, view_number);
    let encoded_transactions = TestTransaction::encode(&transactions);

    let vid_disperse = VidDisperse::from_membership(
        view_number,
        vid.disperse(&encoded_transactions).unwrap(),
        quorum_membership,
    );

    let signature =
        <BLSPubKey as SignatureKey>::sign(private_key, vid_disperse.payload_commitment.as_ref())
            .expect("Failed to sign VID commitment");
    let vid_disperse_proposal = Proposal {
        data: vid_disperse.clone(),
        signature,
        _pd: PhantomData,
    };

    (
        vid_disperse_proposal,
        VidDisperseShare::from_vid_disperse(vid_disperse)
            .into_iter()
            .map(|vid_disperse| {
                vid_disperse
                    .to_proposal(private_key)
                    .expect("Failed to sign payload commitment")
            })
            .collect(),
    )
}

pub fn build_da_certificate(
    quorum_membership: &<TestTypes as NodeType>::Membership,
    da_membership: &<TestTypes as NodeType>::Membership,
    view_number: ViewNumber,
    transactions: Vec<TestTransaction>,
    public_key: &<TestTypes as NodeType>::SignatureKey,
    private_key: &<BLSPubKey as SignatureKey>::PrivateKey,
) -> DaCertificate<TestTypes> {
    let encoded_transactions = TestTransaction::encode(&transactions);

    let da_payload_commitment =
        vid_commitment(&encoded_transactions, quorum_membership.total_nodes());

    let da_data = DaData {
        payload_commit: da_payload_commitment,
    };

    build_cert::<TestTypes, DaData, DaVote<TestTypes>, DaCertificate<TestTypes>>(
        da_data,
        da_membership,
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
        &handle.public_key(),
        handle.private_key(),
    )
    .expect("Failed to create quorum vote");
    GeneralConsensusMessage::<TestTypes>::Vote(vote)
}

/// This function permutes the provided input vector `inputs`, given some order provided within the
/// `order` vector.
///
/// # Examples
/// let output = permute_input_with_index_order(vec![1, 2, 3], vec![2, 1, 0]);
/// // Output is [3, 2, 1] now
pub fn permute_input_with_index_order<T>(inputs: Vec<T>, order: Vec<usize>) -> Vec<T>
where
    T: Clone,
{
    let mut ordered_inputs = Vec::with_capacity(inputs.len());
    for &index in &order {
        ordered_inputs.push(inputs[index].clone());
    }
    ordered_inputs
}

/// This function will create a fake [`View`] from a provided [`Leaf`].
pub fn build_fake_view_with_leaf(leaf: Leaf<TestTypes>) -> View<TestTypes> {
    build_fake_view_with_leaf_and_state(leaf, TestValidatedState::default())
}

/// This function will create a fake [`View`] from a provided [`Leaf`] and `state`.
pub fn build_fake_view_with_leaf_and_state(
    leaf: Leaf<TestTypes>,
    state: TestValidatedState,
) -> View<TestTypes> {
    View {
        view_inner: ViewInner::Leaf {
            leaf: leaf.commit(),
            state: state.into(),
            delta: None,
        },
    }
}
