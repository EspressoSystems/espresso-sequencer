#![cfg(feature = "dependency-tasks")]

use std::sync::Arc;

use committable::Committable;
use hotshot::tasks::task_state::CreateTaskState;
use hotshot_example_types::{
    block_types::TestMetadata,
    node_types::{MemoryImpl, TestTypes},
    state_types::{TestInstanceState, TestValidatedState},
};
use hotshot_task_impls::{
    events::HotShotEvent::{self, *},
    quorum_proposal::QuorumProposalTaskState,
};
use hotshot_testing::{
    predicates::{
        event::{exact, leaf_decided, quorum_proposal_send},
        Predicate,
    },
    script::{run_test_script, TestScriptStage},
    task_helpers::{
        build_cert, build_system_handle, key_pair_for_id, vid_scheme_from_view_number, vid_share,
    },
    view_generator::TestViewGenerator,
};
use hotshot_types::{
    consensus::{CommitmentAndMetadata, ProposalDependencyData},
    data::{null_block, Leaf, ViewChangeEvidence, ViewNumber},
    simple_certificate::{TimeoutCertificate, ViewSyncFinalizeCertificate2},
    simple_vote::{TimeoutData, TimeoutVote, ViewSyncFinalizeData, ViewSyncFinalizeVote},
    traits::{
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
    },
    utils::{BuilderCommitment, View, ViewInner},
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

fn create_fake_view_with_leaf(leaf: Leaf<TestTypes>) -> View<TestTypes> {
    View {
        view_inner: ViewInner::Leaf {
            leaf: leaf.commit(),
            state: TestValidatedState::default().into(),
            delta: None,
        },
    }
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_quorum_proposal_view_1() {
    use hotshot_types::simple_certificate::QuorumCertificate;

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
    let consensus = handle.hotshot.consensus();
    let mut consensus_writer = consensus.write().await;
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        vids.push(view.vid_proposal.clone());

        // We don't have a `QuorumProposalRecv` task handler, so we'll just manually insert the proposals
        // to make sure they show up during tests.
        consensus_writer
            .update_saved_leaves(Leaf::from_quorum_proposal(&view.quorum_proposal.data));
    }
    drop(consensus_writer);

    // We must send the genesis cert here to initialize hotshot successfully.
    let genesis_cert = QuorumCertificate::genesis(&*handle.hotshot.instance_state());
    let genesis_leaf = Leaf::genesis(&*handle.hotshot.instance_state());
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let view = TestScriptStage {
        inputs: vec![
            QcFormed(either::Left(genesis_cert.clone())),
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(1),
                null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                    .unwrap(),
            ),
            VidShareValidated(vid_share(&vids[0].0, handle.public_key())),
            ValidatedStateUpdated(ViewNumber::new(0), create_fake_view_with_leaf(genesis_leaf)),
        ],
        outputs: vec![
            exact(UpdateHighQc(genesis_cert.clone())),
            quorum_proposal_send(),
        ],
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
    use hotshot_types::vote::HasViewNumber;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 3;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    let mut vids = Vec::new();
    let consensus = handle.hotshot.consensus();
    let mut consensus_writer = consensus.write().await;
    for view in (&mut generator).take(5) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        vids.push(view.vid_proposal.clone());

        // We don't have a `QuorumProposalRecv` task handler, so we'll just manually insert the proposals
        // to make sure they show up during tests.
        consensus_writer
            .update_saved_leaves(Leaf::from_quorum_proposal(&view.quorum_proposal.data));
    }
    drop(consensus_writer);

    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    // We need to handle the views where we aren't the leader to ensure that the states are
    // updated properly.

    let genesis_cert = proposals[0].data.justify_qc.clone();
    let genesis_leaf = Leaf::genesis(&*handle.hotshot.instance_state());

    let genesis_view = TestScriptStage {
        inputs: vec![
            QcFormed(either::Left(genesis_cert.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(1)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(1),
                null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                    .unwrap(),
            ),
            VidShareValidated(vid_share(&vids[0].0, handle.public_key())),
            ValidatedStateUpdated(
                ViewNumber::new(0),
                create_fake_view_with_leaf(genesis_leaf.clone()),
            ),
        ],
        outputs: vec![exact(UpdateHighQc(genesis_cert.clone()))],
        asserts: vec![],
    };

    // We send all the events that we'd have otherwise received to ensure the states are updated.
    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalValidated(proposals[0].data.clone(), leaves[0].clone()),
            QcFormed(either::Left(proposals[1].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(2)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(2),
                null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                    .unwrap(),
            ),
            VidShareValidated(vid_share(&vids[1].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[0].data.view_number(),
                create_fake_view_with_leaf(leaves[0].clone()),
            ),
        ],
        outputs: vec![exact(UpdateHighQc(proposals[1].data.justify_qc.clone()))],
        asserts: vec![],
    };

    // Proposing for this view since we've received a proposal for view 2.
    let view_2 = TestScriptStage {
        inputs: vec![
            QuorumProposalValidated(proposals[1].data.clone(), leaves[1].clone()),
            QcFormed(either::Left(proposals[2].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(3)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(3),
                null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                    .unwrap(),
            ),
            VidShareValidated(vid_share(&vids[2].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[1].data.view_number(),
                create_fake_view_with_leaf(leaves[1].clone()),
            ),
        ],
        outputs: vec![
            exact(LockedViewUpdated(ViewNumber::new(1))),
            exact(UpdateHighQc(proposals[2].data.justify_qc.clone())),
            quorum_proposal_send(),
        ],
        asserts: vec![],
    };

    // Now, let's verify that we get the decide on the 3-chain.
    let view_3 = TestScriptStage {
        inputs: vec![
            QuorumProposalValidated(proposals[2].data.clone(), leaves[2].clone()),
            QcFormed(either::Left(proposals[3].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(4)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(4),
                null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                    .unwrap(),
            ),
            VidShareValidated(vid_share(&vids[3].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[2].data.view_number(),
                create_fake_view_with_leaf(leaves[2].clone()),
            ),
        ],
        outputs: vec![exact(UpdateHighQc(proposals[3].data.justify_qc.clone()))],
        asserts: vec![],
    };

    let view_4 = TestScriptStage {
        inputs: vec![
            QuorumProposalValidated(proposals[3].data.clone(), leaves[3].clone()),
            QcFormed(either::Left(proposals[4].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(5)),
                builder_commitment,
                TestMetadata,
                ViewNumber::new(5),
                null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                    .unwrap(),
            ),
            VidShareValidated(vid_share(&vids[4].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[3].data.view_number(),
                create_fake_view_with_leaf(leaves[3].clone()),
            ),
        ],
        outputs: vec![
            exact(LockedViewUpdated(ViewNumber::new(3))),
            exact(LastDecidedViewUpdated(ViewNumber::new(2))),
            leaf_decided(),
            exact(UpdateHighQc(proposals[4].data.justify_qc.clone())),
        ],
        asserts: vec![],
    };

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let script = vec![genesis_view, view_1, view_2, view_3, view_4];
    run_test_script(script, quorum_proposal_task_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_qc_timeout() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 3;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(node_id));
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut vids = Vec::new();
    let mut leaves = Vec::new();
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
        leaves.push(view.leaf.clone());
    }
    let timeout_data = TimeoutData {
        view: ViewNumber::new(1),
    };
    generator.add_timeout(timeout_data);
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
        leaves.push(view.leaf.clone());
    }

    // Get the proposal cert out for the view sync input
    let cert = match proposals[1].data.proposal_certificate.clone().unwrap() {
        ViewChangeEvidence::Timeout(tc) => tc,
        _ => panic!("Found a View Sync Cert when there should have been a Timeout cert"),
    };

    // Run at view 2, propose at view 3.
    let view_2 = TestScriptStage {
        inputs: vec![
            QcFormed(either::Right(cert.clone())),
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(3),
                null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                    .unwrap(),
            ),
            VidShareValidated(vid_share(&vids[2].0.clone(), handle.public_key())),
            ValidatedStateUpdated(
                ViewNumber::new(2),
                create_fake_view_with_leaf(leaves[1].clone()),
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
    let mut leaves = Vec::new();
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
        leaves.push(view.leaf.clone());
    }

    let view_sync_finalize_data = ViewSyncFinalizeData {
        relay: 2,
        round: ViewNumber::new(node_id),
    };
    generator.add_view_sync_finalize(view_sync_finalize_data);
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
        leaves.push(view.leaf.clone());
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
                null_block::builder_fee(quorum_membership.total_nodes(), &TestInstanceState {})
                    .unwrap(),
            ),
            VidShareValidated(vid_share(&vids[1].0.clone(), handle.public_key())),
            ValidatedStateUpdated(
                ViewNumber::new(1),
                create_fake_view_with_leaf(leaves[1].clone()),
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

#[ignore]
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

    let script = vec![view_qp];
    run_test_script(script, quorum_proposal_task_state).await;
}

#[ignore]
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

    let script = vec![view_timeout];
    run_test_script(script, quorum_proposal_task_state).await;
}

#[ignore]
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

fn generate_outputs(
    chain_length: i32,
    current_view_number: u64,
) -> Vec<Box<dyn Predicate<Arc<HotShotEvent<TestTypes>>>>> {
    match chain_length {
        // This is not - 2 because we start from the parent
        2 => vec![exact(LockedViewUpdated(ViewNumber::new(
            current_view_number - 1,
        )))],
        // This is not - 3 because we start from the parent
        3 => vec![
            exact(LockedViewUpdated(ViewNumber::new(current_view_number - 1))),
            exact(LastDecidedViewUpdated(ViewNumber::new(
                current_view_number - 2,
            ))),
            leaf_decided(),
        ],
        _ => vec![],
    }
}

/// This test validates the the ascension of the leaf chain across a large input space with
/// consistently increasing inputs to ensure that decides and locked view updates
/// occur as expected.
///
/// This test will never propose, instead, we focus exclusively on the processing of the
/// [`HotShotEvent::QuorumProposalValidated`] event in a number of different circumstances. We want to
/// guarantee that a particular space of outputs is generated.
///
/// These outputs should be easy to run since we'll be deterministically incrementing our iterator from
/// 0..100 proposals, inserting the valid state into the map (hence "happy path"). Since we'll know ahead
/// of time, we can essentially anticipate the formation of a valid chain.
///
/// The output sequence is essentially:
/// view 0/1 = No outputs
/// view 2
/// ```rust
/// LockedViewUpdated(1)
/// ```
///
/// view 3
/// ```rust
/// LockedViewUpdated(2)
/// LastDecidedViewUpdated(1)
/// LeafDecided()
/// ```
///
/// view i in 4..n
/// ```rust
/// LockedViewUpdated(i - 1)
/// LastDecidedViewUpdated(i - 2)
/// LeafDecided()
/// ```
///
/// Because we've inserted all of the valid data, the traversals should go exactly as we expect them to.
#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_happy_path_leaf_ascension() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id: usize = 1;
    let handle = build_system_handle(node_id.try_into().unwrap()).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();
    let mut generator = TestViewGenerator::generate(quorum_membership, da_membership);

    let mut current_chain_length = 0;
    let mut script = Vec::new();
    for view_number in 1..100u64 {
        current_chain_length += 1;
        if current_chain_length > 3 {
            current_chain_length = 3;
        }
        // This unwrap is safe here
        let view = generator.next().unwrap();
        let proposal = view.quorum_proposal.clone();
        let leaf = view.leaf.clone();

        // update the consensus shared state
        {
            let consensus = handle.consensus();
            let mut consensus_writer = consensus.write().await;
            consensus_writer.update_validated_state_map(
                ViewNumber::new(view_number),
                create_fake_view_with_leaf(leaf.clone()),
            );
            consensus_writer.update_saved_leaves(leaf.clone());
            consensus_writer.update_vid_shares(
                ViewNumber::new(view_number),
                view.vid_proposal.0[node_id].clone(),
            );
        }

        let view = TestScriptStage {
            inputs: vec![QuorumProposalValidated(proposal.data, leaf)],
            outputs: generate_outputs(current_chain_length, view_number.try_into().unwrap()),
            asserts: vec![],
        };
        script.push(view);
    }

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    run_test_script(script, quorum_proposal_task_state).await;
}

/// This test non-deterministically injects faults into the leaf ascension process where we randomly
/// drop states, views, etc from the proposals to ensure that we get decide events only when a three
/// chain is detected. This is accomplished by simply looking up in the state map and checking if the
/// parents for a given node indeed exist and, if so, updating the current chain depending on how recent
/// the dropped parent was.
///
/// We utilize the same method to generate the outputs in both cases since it's quite easy to get a predictable
/// output set depending on where we are in the chain. Again, we do *not* propose in this method and instead
/// verify that the leaf ascension is reliable. We also use non-determinism to make sure that our fault
/// injection is randomized to some degree. This helps smoke out corner cases (i.e. the start and end).
#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_fault_injection_leaf_ascension() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id: usize = 1;
    let handle = build_system_handle(node_id.try_into().unwrap()).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();
    let mut generator = TestViewGenerator::generate(quorum_membership, da_membership);

    let mut current_chain_length = 0;
    let mut script = Vec::new();
    let mut dropped_views = Vec::new();
    for view_number in 1..15u64 {
        current_chain_length += 1;
        // If the chain keeps going, then let it keep going
        if current_chain_length > 3 {
            current_chain_length = 3;
        }
        // This unwrap is safe here
        let view = generator.next().unwrap();
        let proposal = view.quorum_proposal.clone();
        let leaf = view.leaf.clone();

        {
            let consensus = handle.consensus();
            let mut consensus_writer = consensus.write().await;

            // Break the chain depending on the prior state. If the immediate parent is not found, we have a chain of
            // 1, but, if it is, and the parent 2 away is not found, we have a 2 chain.
            if consensus_writer
                .validated_state_map()
                .get(&ViewNumber::new(view_number - 1))
                .is_none()
            {
                current_chain_length = 1;
            } else if view_number > 2
                && consensus_writer
                    .validated_state_map()
                    .get(&ViewNumber::new(view_number - 2))
                    .is_none()
            {
                current_chain_length = 2;
            }

            // Update the consensus shared state with a 10% failure rate
            if rand::random::<f32>() < 0.9 {
                // if view_number != 7 && view_number != 13 {
                consensus_writer.update_validated_state_map(
                    ViewNumber::new(view_number),
                    create_fake_view_with_leaf(leaf.clone()),
                );
                consensus_writer.update_saved_leaves(leaf.clone());
                consensus_writer.update_vid_shares(
                    ViewNumber::new(view_number),
                    view.vid_proposal.0[node_id].clone(),
                );
            } else {
                dropped_views.push(view_number);
            }
        }

        let view = TestScriptStage {
            inputs: vec![QuorumProposalValidated(proposal.data, leaf)],
            outputs: generate_outputs(current_chain_length, view_number.try_into().unwrap()),
            asserts: vec![],
        };
        script.push(view);
    }

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    run_test_script(script, quorum_proposal_task_state).await;
}
