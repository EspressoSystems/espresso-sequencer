//! A collection of generator functions for mock data used in tests
use std::{marker::PhantomData, sync::Arc, time::Duration};

use async_broadcast::broadcast;
use committable::Commitment;
use committable::Committable;
use hotshot_example_types::block_types::{TestBlockHeader, TestBlockPayload, TestTransaction};
use hotshot_example_types::{
    node_types::{TestTypes, TestVersions},
    state_types::{TestInstanceState, TestValidatedState},
};
use hotshot_types::data::QuorumProposal2;
use hotshot_types::data::ViewNumber;
use hotshot_types::drb::INITIAL_DRB_RESULT;
use hotshot_types::drb::INITIAL_DRB_SEED_INPUT;
use hotshot_types::event::LeafInfo;
use hotshot_types::simple_certificate::QuorumCertificate2;
use hotshot_types::simple_vote::QuorumData2;
use hotshot_types::traits::block_contents::{vid_commitment, GENESIS_VID_NUM_STORAGE_NODES};
use hotshot_types::{
    data::{random_commitment, DaProposal, Leaf, Leaf2},
    message::UpgradeLock,
    simple_certificate::QuorumCertificate,
    simple_vote::VersionedVoteData,
    traits::node_implementation::{ConsensusTime, NodeType},
    traits::BlockPayload,
    utils::BuilderCommitment,
    vid::vid_scheme,
};
use jf_vid::VidScheme;
use rand::{distributions::Standard, thread_rng, Rng};

use crate::block::ParentBlockReferences;
use crate::state::BuilderState;

use super::constants::{TEST_CHANNEL_BUFFER_SIZE, TEST_NUM_NODES_IN_VID_COMPUTATION};

pub fn transaction() -> TestTransaction {
    TestTransaction::new(
        thread_rng()
            .sample_iter(Standard)
            .take(100)
            .collect::<Vec<_>>(),
    )
}

pub async fn decide_leaf_chain(decided_view: u64) -> Arc<Vec<LeafInfo<TestTypes>>> {
    decide_leaf_chain_with_transactions(decided_view, vec![transaction()]).await
}

pub async fn decide_leaf_chain_with_transactions(
    decided_view: u64,
    transactions: Vec<TestTransaction>,
) -> Arc<Vec<LeafInfo<TestTypes>>> {
    let (da_proposal, quorum_proposal) =
        proposals_with_transactions(decided_view, transactions).await;
    let mut leaf = Leaf2::from_quorum_proposal(&quorum_proposal);
    let payload = <TestBlockPayload as BlockPayload<TestTypes>>::from_bytes(
        &da_proposal.encoded_transactions,
        &da_proposal.metadata,
    );
    leaf.fill_block_payload_unchecked(payload);
    Arc::new(vec![LeafInfo {
        leaf,
        state: Default::default(),
        delta: None,
        vid_share: None,
    }])
}

/// Create mock pair of DA and Quorum proposals
pub async fn proposals(view: u64) -> (DaProposal<TestTypes>, QuorumProposal2<TestTypes>) {
    let transaction = transaction();
    proposals_with_transactions(view, vec![transaction]).await
}

/// Create mock pair of DA and Quorum proposals with given transactions
pub async fn proposals_with_transactions(
    view: u64,
    transactions: Vec<TestTransaction>,
) -> (DaProposal<TestTypes>, QuorumProposal2<TestTypes>) {
    let view_number = <TestTypes as NodeType>::View::new(view);
    let upgrade_lock = UpgradeLock::<TestTypes, TestVersions>::new();
    let validated_state = TestValidatedState::default();
    let instance_state = TestInstanceState::default();

    let (payload, metadata) = <TestBlockPayload as BlockPayload<TestTypes>>::from_transactions(
        transactions.clone(),
        &validated_state,
        &instance_state,
    )
    .await
    .unwrap();
    let encoded_transactions = TestTransaction::encode(&transactions);

    let header = TestBlockHeader::new(
        &Leaf::<TestTypes>::genesis(&Default::default(), &Default::default())
            .await
            .into(),
        vid_commitment(&encoded_transactions, GENESIS_VID_NUM_STORAGE_NODES),
        <TestBlockPayload as BlockPayload<TestTypes>>::builder_commitment(&payload, &metadata),
        metadata,
    );

    let genesis_qc = QuorumCertificate::<TestTypes>::genesis::<TestVersions>(
        &TestValidatedState::default(),
        &TestInstanceState::default(),
    )
    .await
    .to_qc2();
    let parent_proposal = QuorumProposal2 {
        block_header: header,
        view_number: ViewNumber::new(view_number.saturating_sub(1)),
        justify_qc: genesis_qc,
        upgrade_certificate: None,
        view_change_evidence: None,
        drb_seed: INITIAL_DRB_SEED_INPUT,
        drb_result: INITIAL_DRB_RESULT,
    };
    let leaf = Leaf2::from_quorum_proposal(&parent_proposal);

    let quorum_data = QuorumData2 {
        leaf_commit: leaf.commit(),
    };

    let versioned_data = VersionedVoteData::<_, _, _>::new_infallible(
        quorum_data.clone(),
        view_number,
        &upgrade_lock,
    )
    .await;

    let commitment = Commitment::from_raw(versioned_data.commit().into());

    let justify_qc =
        QuorumCertificate2::new(quorum_data, commitment, view_number, None, PhantomData);

    (
        DaProposal {
            encoded_transactions: encoded_transactions.into(),
            metadata,
            view_number,
        },
        QuorumProposal2 {
            block_header: leaf.block_header().clone(),
            view_number,
            justify_qc,
            upgrade_certificate: None,
            view_change_evidence: None,
            drb_seed: INITIAL_DRB_SEED_INPUT,
            drb_result: INITIAL_DRB_RESULT,
        },
    )
}

pub fn builder_state(view: u64) -> Arc<BuilderState<TestTypes>> {
    let references = parent_references(view);
    let (_, receiver) = broadcast(TEST_CHANNEL_BUFFER_SIZE);
    BuilderState::new(
        references,
        Duration::from_secs(1),
        receiver,
        TestValidatedState::default(),
    )
}

/// Generate references for given view number with random
/// commitments for use in testing code
pub fn parent_references(view: u64) -> ParentBlockReferences<TestTypes> {
    let rng = &mut thread_rng();
    ParentBlockReferences {
        view_number: <TestTypes as NodeType>::View::new(view),
        leaf_commit: random_commitment(rng),
        vid_commitment: vid_scheme(TEST_NUM_NODES_IN_VID_COMPUTATION)
            .commit_only(rng.sample_iter(Standard).take(100).collect::<Vec<_>>())
            .unwrap(),
        builder_commitment: BuilderCommitment::from_bytes(
            rng.sample_iter(Standard).take(32).collect::<Vec<_>>(),
        ),
        tx_count: rng.gen(),
        last_nonempty_view: None,
    }
}
