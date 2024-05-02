// TODO: Remove after integration
#![allow(unused_imports)]

use hotshot::tasks::task_state::CreateTaskState;
use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
use hotshot_task_impls::{
    events::HotShotEvent::*, quorum_proposal_recv::QuorumProposalRecvTaskState,
};
use hotshot_testing::{
    predicates::event::exact,
    script::{run_test_script, TestScriptStage},
    task_helpers::build_system_handle,
    view_generator::TestViewGenerator,
};
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime};

#[cfg(test)]
#[cfg(feature = "dependency-tasks")]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_recv_task() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);
    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Run view 2 and propose.
    let view_2 = TestScriptStage {
        inputs: vec![QuorumProposalRecv(proposals[1].clone(), leaders[1])],
        outputs: vec![exact(ViewChange(ViewNumber::new(2)))],
        asserts: vec![],
    };

    let state = QuorumProposalRecvTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    run_test_script(vec![view_2], state).await;
}
