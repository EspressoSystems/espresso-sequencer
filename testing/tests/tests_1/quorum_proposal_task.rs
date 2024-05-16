#![cfg(feature = "dependency-tasks")]

use hotshot::{tasks::task_state::CreateTaskState, types::SystemContextHandle};
use hotshot_example_types::{
    block_types::TestMetadata,
    node_types::{MemoryImpl, TestTypes},
    state_types::TestInstanceState,
};
use hotshot_task_impls::{events::HotShotEvent::*, quorum_proposal::QuorumProposalTaskState};
use hotshot_testing::{
    predicates::event::quorum_proposal_send,
    script::{run_test_script, TestScriptStage},
    task_helpers::{build_cert, build_system_handle, key_pair_for_id, vid_scheme_from_view_number},
    view_generator::TestViewGenerator,
};
use hotshot_types::{
    consensus::{CommitmentAndMetadata, ProposalDependencyData},
    data::{null_block, VidDisperseShare, ViewChangeEvidence, ViewNumber},
    message::Proposal,
    simple_certificate::{TimeoutCertificate, ViewSyncFinalizeCertificate2},
    simple_vote::{TimeoutData, TimeoutVote, ViewSyncFinalizeData, ViewSyncFinalizeVote},
    traits::{
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
    },
    utils::BuilderCommitment,
    vid::VidSchemeType,
};
use jf_vid::VidScheme;
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

async fn insert_vid_shares_for_view(
    view: <TestTypes as NodeType>::Time,
    handle: &SystemContextHandle<TestTypes, MemoryImpl>,
    vid: (
        Vec<Proposal<TestTypes, VidDisperseShare<TestTypes>>>,
        <TestTypes as NodeType>::SignatureKey,
    ),
) {
    let consensus = handle.consensus();
    let mut consensus = consensus.write().await;

    // `create_and_send_proposal` depends on the `vid_shares` obtaining a vid dispersal.
    // to avoid needing to spin up the vote task, we can just insert it in here.
    consensus.update_vid_shares(view, vid.0[0].clone());
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_quorum_proposal_view_1() {
    use hotshot_example_types::block_types::TestMetadata;
    use hotshot_types::data::null_block;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 1;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(node_id));

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        vids.push(view.vid_proposal.clone());
    }

    insert_vid_shares_for_view(ViewNumber::new(node_id), &handle, vids[0].clone()).await;

    let cert = proposals[0].data.justify_qc.clone();
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let view = TestScriptStage {
        inputs: vec![
            QCFormed(either::Left(cert.clone())),
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(1),
                null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                    .unwrap(),
            ),
        ],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let script = vec![view];
    run_test_script(script, quorum_proposal_task_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_quorum_proposal_view_gt_1() {
    use hotshot_example_types::{block_types::TestMetadata, state_types::TestValidatedState};
    use hotshot_types::{
        data::null_block,
        utils::{View, ViewInner},
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 3;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(node_id));

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(3) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        vids.push(view.vid_proposal.clone());
    }

    insert_vid_shares_for_view(ViewNumber::new(node_id), &handle, vids[2].clone()).await;
    let consensus = handle.consensus();
    let mut consensus = consensus.write().await;

    // `validate_proposal_safety_and_liveness` depends on the existence of prior values in the consensus
    // state, but since we do not spin up the consensus task, these values must be manually filled
    // out.

    // First, insert a parent view whose leaf commitment will be returned in the lower function
    // call.
    consensus.update_validated_state_map(
        ViewNumber::new(2),
        View {
            view_inner: ViewInner::Leaf {
                leaf: leaves[1].parent_commitment(),
                state: TestValidatedState::default().into(),
                delta: None,
            },
        },
    );

    // Match an entry into the saved leaves for the parent commitment, returning the generated leaf
    // for this call.
    consensus.update_saved_leaves(leaves[1].clone());

    // Release the write lock before proceeding with the test
    drop(consensus);

    let cert = proposals[2].data.justify_qc.clone();
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let view = TestScriptStage {
        inputs: vec![
            QuorumProposalValidated(proposals[1].data.clone(), leaves[1].clone()),
            QCFormed(either::Left(cert.clone())),
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(node_id),
                null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                    .unwrap(),
            ),
        ],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let script = vec![view];
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

    let node_id = 2;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(node_id));
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
    }
    let timeout_data = TimeoutData {
        view: ViewNumber::new(1),
    };
    generator.add_timeout(timeout_data);
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
    }

    insert_vid_shares_for_view(ViewNumber::new(node_id), &handle, vids[1].clone()).await;
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
                null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                    .unwrap(),
            ),
        ],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

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
    let node_id = 2;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(node_id));
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
    }

    let view_sync_finalize_data = ViewSyncFinalizeData {
        relay: 2,
        round: ViewNumber::new(node_id),
    };
    generator.add_view_sync_finalize(view_sync_finalize_data);
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
    }

    insert_vid_shares_for_view(ViewNumber::new(node_id), &handle, vids[1].clone()).await;
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
                null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                    .unwrap(),
            ),
        ],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let script = vec![view_2];
    run_test_script(script, quorum_proposal_task_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_propose_now() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 2;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(node_id));

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
    }
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
    }

    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());
    // proposal dependency data - quorum proposal and cert
    let pdd_qp = ProposalDependencyData {
        commitment_and_metadata: CommitmentAndMetadata {
            commitment: payload_commitment,
            builder_commitment: builder_commitment.clone(),
            metadata: TestMetadata,
            fee: null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                .unwrap(),
            block_view: ViewNumber::new(2),
        },
        secondary_proposal_information:
            hotshot_types::consensus::SecondaryProposalInformation::QuorumProposalAndCertificate(
                proposals[1].data.clone(),
                proposals[1].data.justify_qc.clone(),
            ),
    };

    let view_qp = TestScriptStage {
        inputs: vec![ProposeNow(ViewNumber::new(node_id), pdd_qp)],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    insert_vid_shares_for_view(ViewNumber::new(node_id), &handle, vids[1].clone()).await;

    let script = vec![view_qp];
    run_test_script(script, quorum_proposal_task_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_propose_now_timeout() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 2;
    let handle = build_system_handle(node_id).await.0;
    let (private_key, public_key) = key_pair_for_id(node_id);
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(node_id));

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
    }
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
    }

    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());
    // proposal dependency data - timeout cert
    let pdd_timeout = ProposalDependencyData {
        commitment_and_metadata: CommitmentAndMetadata {
            commitment: payload_commitment,
            builder_commitment: builder_commitment.clone(),
            metadata: TestMetadata,
            fee: null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                .unwrap(),
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

    let view_timeout = TestScriptStage {
        inputs: vec![ProposeNow(ViewNumber::new(node_id), pdd_timeout)],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    insert_vid_shares_for_view(ViewNumber::new(node_id), &handle, vids[1].clone()).await;

    let script = vec![view_timeout];
    run_test_script(script, quorum_proposal_task_state).await;
}
#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_propose_now_view_sync() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 2;
    let handle = build_system_handle(node_id).await.0;
    let (private_key, public_key) = key_pair_for_id(node_id);
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(node_id));

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
    }
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
    }

    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    // proposal dependency data - view sync cert
    let pdd_view_sync = ProposalDependencyData {
        commitment_and_metadata: CommitmentAndMetadata {
            commitment: payload_commitment,
            builder_commitment,
            metadata: TestMetadata,
            fee: null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                .unwrap(),
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
                ViewNumber::new(node_id),
                &public_key,
                &private_key,
            )),
    };

    let view_view_sync = TestScriptStage {
        inputs: vec![ProposeNow(ViewNumber::new(node_id), pdd_view_sync)],
        outputs: vec![quorum_proposal_send()],
        asserts: vec![],
    };

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    insert_vid_shares_for_view(ViewNumber::new(node_id), &handle, vids[1].clone()).await;

    let script = vec![view_view_sync];
    run_test_script(script, quorum_proposal_task_state).await;
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
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

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

    let script = vec![view_2];
    run_test_script(script, quorum_proposal_task_state).await;
}
