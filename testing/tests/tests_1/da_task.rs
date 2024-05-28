use std::sync::Arc;

use futures::StreamExt;
use hotshot::{tasks::task_state::CreateTaskState, types::SystemContextHandle};
use hotshot_example_types::{
    block_types::{TestMetadata, TestTransaction},
    node_types::{MemoryImpl, TestTypes},
};
use hotshot_task_impls::{da::DaTaskState, events::HotShotEvent::*};
use hotshot_testing::{
    predicates::event::exact,
    script::{run_test_script, TestScriptStage},
    helpers::build_system_handle,
    view_generator::TestViewGenerator,
};
use hotshot_types::{
    data::{null_block, ViewNumber},
    simple_vote::DaData,
    traits::{
        block_contents::precompute_vid_commitment, election::Membership,
        node_implementation::ConsensusTime,
    },
};

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_da_task() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    // Make some empty encoded transactions, we just care about having a commitment handy for the
    // later calls. We need the VID commitment to be able to propose later.
    let transactions = vec![TestTransaction::new(vec![0])];
    let encoded_transactions = Arc::from(TestTransaction::encode(&transactions));
    let (payload_commit, precompute) = precompute_vid_commitment(
        &encoded_transactions,
        handle.hotshot.memberships.quorum_membership.total_nodes(),
    );

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();

    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.da_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_da_vote(DaData { payload_commit }, &handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    generator.add_transactions(vec![TestTransaction::new(vec![0])]);

    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.da_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_da_vote(DaData { payload_commit }, &handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Run view 1 (the genesis stage).
    let view_1 = TestScriptStage {
        inputs: vec![
            ViewChange(ViewNumber::new(1)),
            ViewChange(ViewNumber::new(2)),
            BlockRecv(
                encoded_transactions,
                TestMetadata,
                ViewNumber::new(2),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
                precompute,
            ),
        ],
        outputs: vec![exact(DaProposalSend(proposals[1].clone(), leaders[1]))],
        asserts: vec![],
    };

    // Run view 2 and validate proposal.
    let view_2 = TestScriptStage {
        inputs: vec![DaProposalRecv(proposals[1].clone(), leaders[1])],
        outputs: vec![
            exact(DaProposalValidated(proposals[1].clone(), leaders[1])),
            exact(DaVoteSend(votes[1].clone())),
        ],
        asserts: vec![],
    };

    let da_state = DaTaskState::<TestTypes, MemoryImpl, SystemContextHandle<TestTypes, MemoryImpl>>::create_from(&handle).await;
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
    handle.storage().write().await.should_return_err = true;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    // Make some empty encoded transactions, we just care about having a commitment handy for the
    // later calls. We need the VID commitment to be able to propose later.
    let transactions = vec![TestTransaction::new(vec![0])];
    let encoded_transactions = Arc::from(TestTransaction::encode(&transactions));
    let (payload_commit, precompute) = precompute_vid_commitment(
        &encoded_transactions,
        handle.hotshot.memberships.quorum_membership.total_nodes(),
    );

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();

    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.da_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_da_vote(DaData { payload_commit }, &handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    generator.add_transactions(transactions);

    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.da_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_da_vote(DaData { payload_commit }, &handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Run view 1 (the genesis stage).
    let view_1 = TestScriptStage {
        inputs: vec![
            ViewChange(ViewNumber::new(1)),
            ViewChange(ViewNumber::new(2)),
            BlockRecv(
                encoded_transactions,
                TestMetadata,
                ViewNumber::new(2),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
                precompute,
            ),
        ],
        outputs: vec![exact(DaProposalSend(proposals[1].clone(), leaders[1]))],
        asserts: vec![],
    };

    // Run view 2 and validate proposal.
    let view_2 = TestScriptStage {
        inputs: vec![DaProposalRecv(proposals[1].clone(), leaders[1])],
        outputs: vec![exact(DaProposalValidated(proposals[1].clone(), leaders[1]))],
        asserts: vec![],
    };

    // Run view 3 and propose.
    let view_3 = TestScriptStage {
        inputs: vec![DaProposalValidated(proposals[1].clone(), leaders[1])],
        outputs: vec![
            /* No vote was sent due to the storage failure */
        ],
        asserts: vec![],
    };

    let da_state = DaTaskState::<TestTypes, MemoryImpl, SystemContextHandle<TestTypes, MemoryImpl>>::create_from(&handle).await;
    let stages = vec![view_1, view_2, view_3];

    run_test_script(stages, da_state).await;
}
