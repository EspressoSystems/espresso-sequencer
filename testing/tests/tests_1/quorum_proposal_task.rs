use hotshot::tasks::{inject_quorum_proposal_polls, task_state::CreateTaskState};

use hotshot_example_types::state_types::TestInstanceState;
use std::sync::Arc;

use hotshot_example_types::{
    node_types::{MemoryImpl, TestTypes},
    state_types::TestValidatedState,
};
use hotshot_task_impls::{events::HotShotEvent::*, quorum_proposal::QuorumProposalTaskState};
use hotshot_testing::{
    predicates::event::quorum_proposal_send,
    script::{run_test_script, TestScriptStage},
    task_helpers::{build_system_handle, vid_scheme_from_view_number},
    view_generator::TestViewGenerator,
};
use hotshot_types::{
    data::{ViewChangeEvidence, ViewNumber},
    simple_vote::ViewSyncFinalizeData,
    traits::{
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
    },
    utils::{BuilderCommitment, View, ViewInner},
    vid::VidSchemeType,
};
use jf_primitives::vid::VidScheme;
use sha2::Digest;

fn make_payload_commitment(
    membership: &<TestTypes as NodeType>::Membership,
    view: ViewNumber,
) -> <VidSchemeType as VidScheme>::Commit {
    // Make some empty encoded transactions, we just care about having a commitment handy for the
    // later calls. We need the VID commitment to be able to propose later.
    let mut vid = vid_scheme_from_view_number::<TestTypes>(membership, view);
    let encoded_transactions = Vec::new();
    vid.commit_only(&encoded_transactions).unwrap()
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_quorum_proposal() {
    use hotshot_example_types::block_types::TestMetadata;
    use hotshot_types::data::null_block;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // We need to propose as the leader for view 2, otherwise we get caught up with the special
    // case in the genesis view.
    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(2));

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
    }
    let consensus = handle.get_consensus();
    let mut consensus = consensus.write().await;

    // `find_parent_leaf_and_state` depends on the existence of prior values in the consensus
    // state, but since we do not spin up the consensus task, these values must be manually filled
    // out.

    // First, insert a parent view whose leaf commitment will be returned in the lower function
    // call.
    consensus.validated_state_map.insert(
        ViewNumber::new(1),
        View {
            view_inner: ViewInner::Leaf {
                leaf: leaves[1].get_parent_commitment(),
                state: TestValidatedState::default().into(),
                delta: None,
            },
        },
    );

    // Match an entry into the saved leaves for the parent commitment, returning the generated leaf
    // for this call.
    consensus
        .saved_leaves
        .insert(leaves[1].get_parent_commitment(), leaves[1].clone());

    // Release the write lock before proceeding with the test
    drop(consensus);
    let cert = proposals[1].data.justify_qc.clone();
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    // Run at view 2, the quorum proposal task shouldn't care as long as the bookkeeping is correct
    let view_2 = TestScriptStage {
        inputs: vec![
            QuorumProposalValidated(proposals[1].data.clone(), leaves[1].clone()),
            QCFormed(either::Left(cert.clone())),
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(2),
                null_block::builder_fee(quorum_membership.total_nodes(), Arc::new(TestInstanceState {})).unwrap(),
            ),
        ],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    inject_quorum_proposal_polls(&quorum_proposal_task_state).await;

    let script = vec![view_2];
    run_test_script(script, quorum_proposal_task_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_qc_timeout() {
    use hotshot_example_types::block_types::TestMetadata;
    use hotshot_types::{data::null_block, simple_vote::TimeoutData};
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(2));
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
    }
    let timeout_data = TimeoutData {
        view: ViewNumber::new(1),
    };
    generator.add_timeout(timeout_data);
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
    }

    // Get the proposal cert out for the view sync input
    let cert = match proposals[1].data.proposal_certificate.clone().unwrap() {
        ViewChangeEvidence::Timeout(tc) => tc,
        _ => panic!("Found a View Sync Cert when there should have been a Timeout cert"),
    };

    // Run at view 2, the quorum vote task shouldn't care as long as the bookkeeping is correct
    let view_2 = TestScriptStage {
        inputs: vec![
            QCFormed(either::Right(cert.clone())),
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(2),
                null_block::builder_fee(quorum_membership.total_nodes(), Arc::new(TestInstanceState {})).unwrap(),
            ),
        ],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    inject_quorum_proposal_polls(&quorum_proposal_task_state).await;

    let script = vec![view_2];
    run_test_script(script, quorum_proposal_task_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_view_sync() {
    use hotshot_example_types::block_types::TestMetadata;
    use hotshot_types::data::null_block;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // We need to propose as the leader for view 2, otherwise we get caught up with the special
    // case in the genesis view.
    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(2));
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
    }

    let view_sync_finalize_data = ViewSyncFinalizeData {
        relay: 2,
        round: ViewNumber::new(2),
    };
    generator.add_view_sync_finalize(view_sync_finalize_data);
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
    }

    // Get the proposal cert out for the view sync input
    let cert = match proposals[1].data.proposal_certificate.clone().unwrap() {
        ViewChangeEvidence::ViewSync(vsc) => vsc,
        _ => panic!("Found a TC when there should have been a view sync cert"),
    };

    // Run at view 2, the quorum vote task shouldn't care as long as the bookkeeping is correct
    let view_2 = TestScriptStage {
        inputs: vec![
            ViewSyncFinalizeCertificate2Recv(cert.clone()),
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(2),
                null_block::builder_fee(quorum_membership.total_nodes(), Arc::new(TestInstanceState {})).unwrap(),
            ),
        ],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    inject_quorum_proposal_polls(&quorum_proposal_task_state).await;

    let script = vec![view_2];
    run_test_script(script, quorum_proposal_task_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_propose_now() {
    use hotshot_example_types::block_types::TestMetadata;
    use hotshot_testing::task_helpers::{build_cert, key_pair_for_id};
    use hotshot_types::{
        consensus::{CommitmentAndMetadata, ProposalDependencyData},
        data::null_block,
        simple_certificate::{TimeoutCertificate, ViewSyncFinalizeCertificate2},
        simple_vote::{TimeoutData, TimeoutVote, ViewSyncFinalizeVote},
    };
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let (private_key, public_key) = key_pair_for_id(2);
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(2));

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
    }
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
    }
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());
    // proposal dependency data - quorum proposal and cert
    let pdd_qp = ProposalDependencyData {
        commitment_and_metadata: CommitmentAndMetadata {
            commitment: payload_commitment,
            builder_commitment: builder_commitment.clone(),
            metadata: TestMetadata,
            fee: null_block::builder_fee(quorum_membership.total_nodes(), Arc::new(TestInstanceState {})).unwrap(),
            block_view: ViewNumber::new(2),
        },
        secondary_proposal_information:
            hotshot_types::consensus::SecondaryProposalInformation::QuorumProposalAndCertificate(
                proposals[1].data.clone(),
                proposals[1].data.justify_qc.clone(),
            ),
    };

    // proposal dependency data - timeout cert
    let pdd_timeout = ProposalDependencyData {
        commitment_and_metadata: CommitmentAndMetadata {
            commitment: payload_commitment,
            builder_commitment: builder_commitment.clone(),
            metadata: TestMetadata,
            fee: null_block::builder_fee(quorum_membership.total_nodes(), Arc::new(TestInstanceState {})).unwrap(),
            block_view: ViewNumber::new(2),
        },
        secondary_proposal_information:
            hotshot_types::consensus::SecondaryProposalInformation::Timeout(build_cert::<
                TestTypes,
                TimeoutData<TestTypes>,
                TimeoutVote<TestTypes>,
                TimeoutCertificate<TestTypes>,
            >(
                TimeoutData {
                    view: ViewNumber::new(1),
                },
                &quorum_membership,
                ViewNumber::new(2),
                &public_key,
                &private_key,
            )),
    };

    // proposal dependency data - view sync cert
    let pdd_view_sync = ProposalDependencyData {
        commitment_and_metadata: CommitmentAndMetadata {
            commitment: payload_commitment,
            builder_commitment,
            metadata: TestMetadata,
            fee: null_block::builder_fee(quorum_membership.total_nodes(), Arc::new(TestInstanceState {})).unwrap(),
            block_view: ViewNumber::new(2),
        },
        secondary_proposal_information:
            hotshot_types::consensus::SecondaryProposalInformation::ViewSync(build_cert::<
                TestTypes,
                ViewSyncFinalizeData<TestTypes>,
                ViewSyncFinalizeVote<TestTypes>,
                ViewSyncFinalizeCertificate2<TestTypes>,
            >(
                ViewSyncFinalizeData {
                    relay: 1,
                    round: ViewNumber::new(1),
                },
                &quorum_membership,
                ViewNumber::new(2),
                &public_key,
                &private_key,
            )),
    };

    let view_qp = TestScriptStage {
        inputs: vec![ProposeNow(ViewNumber::new(2), pdd_qp)],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    let view_timeout = TestScriptStage {
        inputs: vec![ProposeNow(ViewNumber::new(2), pdd_timeout)],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    let view_view_sync = TestScriptStage {
        inputs: vec![ProposeNow(ViewNumber::new(2), pdd_view_sync)],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    for stage in vec![view_qp, view_timeout, view_view_sync] {
        let quorum_proposal_task_state =
            QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
        inject_quorum_proposal_polls(&quorum_proposal_task_state).await;

        let script = vec![stage];
        run_test_script(script, quorum_proposal_task_state).await;
    }
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_with_incomplete_events() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // We need to propose as the leader for view 2, otherwise we get caught up with the special
    // case in the genesis view.
    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
    }

    // We run the task here at view 2, but this time we ignore the crucial piece of evidence: the
    // payload commitment and metadata. Instead we send only one of the three "OR" required fields.
    // This should result in the proposal failing to be sent.
    let view_2 = TestScriptStage {
        inputs: vec![QuorumProposalValidated(
            proposals[1].data.clone(),
            leaves[1].clone(),
        )],
        outputs: vec![],
        asserts: vec![],
    };

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    inject_quorum_proposal_polls(&quorum_proposal_task_state).await;

    let script = vec![view_2];
    run_test_script(script, quorum_proposal_task_state).await;
}
