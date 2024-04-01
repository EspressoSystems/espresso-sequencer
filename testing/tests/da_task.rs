use hotshot::tasks::task_state::CreateTaskState;
use hotshot::types::SystemContextHandle;
use hotshot_example_types::{
    block_types::TestTransaction,
    node_types::{MemoryImpl, TestTypes},
};
use hotshot_task_impls::da::DATaskState;
use hotshot_task_impls::events::HotShotEvent::*;
use hotshot_testing::{
    predicates::exact,
    script::{run_test_script, TestScriptStage},
    task_helpers::build_system_handle,
    view_generator::TestViewGenerator,
};
use hotshot_types::{
    data::ViewNumber,
    simple_vote::DAData,
    traits::{
        block_contents::vid_commitment, election::Membership, node_implementation::ConsensusTime,
    },
};

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_da_task() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    // Make some empty encoded transactions, we just care about having a commitment handy for the
    // later calls. We need the VID commitment to be able to propose later.
    let transactions = vec![TestTransaction(vec![0])];
    let encoded_transactions = TestTransaction::encode(transactions.clone()).unwrap();
    let payload_commit = vid_commitment(
        &encoded_transactions,
        handle.hotshot.memberships.quorum_membership.total_nodes(),
    );

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();

    for view in (&mut generator).take(1) {
        proposals.push(view.da_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_da_vote(DAData { payload_commit }, &handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    generator.add_transactions(vec![TestTransaction(vec![0])]);

    for view in (&mut generator).take(1) {
        proposals.push(view.da_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_da_vote(DAData { payload_commit }, &handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Run view 1 (the genesis stage).
    let view_1 = TestScriptStage {
        inputs: vec![
            ViewChange(ViewNumber::new(1)),
            ViewChange(ViewNumber::new(2)),
            TransactionsSequenced(encoded_transactions.clone(), (), ViewNumber::new(2)),
        ],
        outputs: vec![exact(DAProposalSend(proposals[1].clone(), leaders[1]))],
        asserts: vec![],
    };

    // Run view 2 and validate proposal.
    let view_2 = TestScriptStage {
        inputs: vec![DAProposalRecv(proposals[1].clone(), leaders[1])],
        outputs: vec![
            exact(DAProposalValidated(proposals[1].clone(), leaders[1])),
            exact(DAVoteSend(votes[1].clone())),
        ],
        asserts: vec![],
    };

    let da_state = DATaskState::<TestTypes, MemoryImpl, SystemContextHandle<TestTypes, MemoryImpl>>::create_from(&handle).await;
    let stages = vec![view_1, view_2];

    run_test_script(stages, da_state).await;
}

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_da_task_storage_failure() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;

    // Set the error flag here for the system handle. This causes it to emit an error on append.
    handle.get_storage().write().await.should_return_err = true;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    // Make some empty encoded transactions, we just care about having a commitment handy for the
    // later calls. We need the VID commitment to be able to propose later.
    let transactions = vec![TestTransaction(vec![0])];
    let encoded_transactions = TestTransaction::encode(transactions.clone()).unwrap();
    let payload_commit = vid_commitment(
        &encoded_transactions,
        handle.hotshot.memberships.quorum_membership.total_nodes(),
    );

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();

    for view in (&mut generator).take(1) {
        proposals.push(view.da_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_da_vote(DAData { payload_commit }, &handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    generator.add_transactions(transactions);

    for view in (&mut generator).take(1) {
        proposals.push(view.da_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_da_vote(DAData { payload_commit }, &handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Run view 1 (the genesis stage).
    let view_1 = TestScriptStage {
        inputs: vec![
            ViewChange(ViewNumber::new(1)),
            ViewChange(ViewNumber::new(2)),
            TransactionsSequenced(encoded_transactions.clone(), (), ViewNumber::new(2)),
        ],
        outputs: vec![exact(DAProposalSend(proposals[1].clone(), leaders[1]))],
        asserts: vec![],
    };

    // Run view 2 and validate proposal.
    let view_2 = TestScriptStage {
        inputs: vec![DAProposalRecv(proposals[1].clone(), leaders[1])],
        outputs: vec![exact(DAProposalValidated(proposals[1].clone(), leaders[1]))],
        asserts: vec![],
    };

    // Run view 3 and propose.
    let view_3 = TestScriptStage {
        inputs: vec![DAProposalValidated(proposals[1].clone(), leaders[1])],
        outputs: vec![
            /* No vote was sent due to the storage failure */
        ],
        asserts: vec![],
    };

    let da_state = DATaskState::<TestTypes, MemoryImpl, SystemContextHandle<TestTypes, MemoryImpl>>::create_from(&handle).await;
    let stages = vec![view_1, view_2, view_3];

    run_test_script(stages, da_state).await;
}
