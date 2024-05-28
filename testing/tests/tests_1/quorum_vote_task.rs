#![allow(clippy::panic)]
use hotshot::tasks::task_state::CreateTaskState;
use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
use hotshot_testing::helpers::{build_fake_view_with_leaf,vid_share};
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime,vote::HasViewNumber};

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_vote_task_success() {
    use hotshot_task_impls::{events::HotShotEvent::*, quorum_vote::QuorumVoteTaskState};
    use hotshot_testing::{
        predicates::event::{exact, quorum_vote_send},
        script::{run_test_script, TestScriptStage},
        helpers::build_system_handle,
        view_generator::TestViewGenerator,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaves = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaves.push(view.leaf.clone());
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Send the quorum proposal, DAC, VID share data, and validated state, in which case a dummy
    // vote can be formed and the view number will be updated.
    let view_success = TestScriptStage {
        inputs: vec![
            QuorumProposalValidated(proposals[1].data.clone(), leaves[0].clone()),
            DaCertificateRecv(dacs[1].clone()),
            VidShareRecv(vids[1].0[0].clone()),
            ValidatedStateUpdated(
                proposals[1].data.view_number(),
                build_fake_view_with_leaf(leaves[1].clone()),
            ),
        ],
        outputs: vec![
            exact(DaCertificateValidated(dacs[1].clone())),
            exact(VidShareValidated(vids[1].0[0].clone())),
            exact(QuorumVoteDependenciesValidated(ViewNumber::new(2))),
            quorum_vote_send(),
        ],
        asserts: vec![],
    };

    let quorum_vote_state =
        QuorumVoteTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    run_test_script(vec![view_success], quorum_vote_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_vote_task_vote_now() {
    use hotshot_task_impls::{events::HotShotEvent::*, quorum_vote::QuorumVoteTaskState};
    use hotshot_testing::{
        predicates::event::{exact, quorum_vote_send},
        script::{run_test_script, TestScriptStage},
        helpers::build_system_handle,
        view_generator::TestViewGenerator,
    };
    use hotshot_types::vote::VoteDependencyData;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    generator.next();
    let view = generator.current_view.clone().unwrap();

    let vote_dependency_data = VoteDependencyData {
        quorum_proposal: view.quorum_proposal.data.clone(),
        parent_leaf: view.leaf.clone(),
        vid_share: view.vid_proposal.0[0].clone(),
        da_cert: view.da_certificate.clone(),
    };

    // Submit an event with just the `VoteNow` event which should successfully send a vote.
    let view_vote_now = TestScriptStage {
        inputs: vec![VoteNow(view.view_number, vote_dependency_data)],
        outputs: vec![
            exact(QuorumVoteDependenciesValidated(ViewNumber::new(1))),
            quorum_vote_send(),
        ],
        asserts: vec![],
    };

    let quorum_vote_state =
        QuorumVoteTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    run_test_script(vec![view_vote_now], quorum_vote_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_vote_task_miss_dependency() {
    use hotshot_task_impls::{events::HotShotEvent::*, quorum_vote::QuorumVoteTaskState};
    use hotshot_testing::{
        predicates::event::exact,
        script::{run_test_script, TestScriptStage},
        helpers::build_system_handle,
        view_generator::TestViewGenerator,
    };

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
    let mut leaves = Vec::new();
    for view in (&mut generator).take(5) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaves.push(view.leaf.clone());
    }

    // Send three of quorum proposal, DAC, VID share data, and validated state, in which case
    // there's no vote.
    let view_no_dac = TestScriptStage {
        inputs: vec![
            QuorumProposalValidated(proposals[1].data.clone(), leaves[0].clone()),
            VidShareRecv(vid_share(&vids[1].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[1].data.view_number(),
                build_fake_view_with_leaf(leaves[1].clone()),
            ),
        ],
        outputs: vec![exact(VidShareValidated(vids[1].0[0].clone()))],
        asserts: vec![],
    };
    let view_no_vid = TestScriptStage {
        inputs: vec![
            QuorumProposalValidated(proposals[2].data.clone(), leaves[1].clone()),
            DaCertificateRecv(dacs[2].clone()),
            ValidatedStateUpdated(
                proposals[2].data.view_number(),
                build_fake_view_with_leaf(leaves[2].clone()),
            ),
        ],
        outputs: vec![exact(DaCertificateValidated(dacs[2].clone()))],
        asserts: vec![],
    };
    let view_no_quorum_proposal = TestScriptStage {
        inputs: vec![
            DaCertificateRecv(dacs[3].clone()),
            VidShareRecv(vid_share(&vids[3].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[3].data.view_number(),
                build_fake_view_with_leaf(leaves[3].clone()),
            ),
        ],
        outputs: vec![
            exact(DaCertificateValidated(dacs[3].clone())),
            exact(VidShareValidated(vids[3].0[0].clone())),
        ],
        asserts: vec![],
    };
    let view_no_validated_state = TestScriptStage {
        inputs: vec![
            QuorumProposalValidated(proposals[4].data.clone(), leaves[3].clone()),
            DaCertificateRecv(dacs[4].clone()),
            VidShareRecv(vid_share(&vids[4].0, handle.public_key())),
        ],
        outputs: vec![
            exact(DaCertificateValidated(dacs[4].clone())),
            exact(VidShareValidated(vids[4].0[0].clone())),
        ],
        asserts: vec![],
    };

    let quorum_vote_state =
        QuorumVoteTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    run_test_script(
        vec![view_no_dac, view_no_vid, view_no_quorum_proposal, view_no_validated_state],
        quorum_vote_state,
    )
    .await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_vote_task_incorrect_dependency() {
    use hotshot_task_impls::{events::HotShotEvent::*, quorum_vote::QuorumVoteTaskState};
    use hotshot_testing::{
        predicates::event::exact,
        script::{run_test_script, TestScriptStage},
        helpers::build_system_handle,
        view_generator::TestViewGenerator,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaves = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaves.push(view.leaf.clone());
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Send the correct quorum proposal, DAC, and VID share data, and incorrect validated state.
    let view_incorrect_dependency = TestScriptStage {
        inputs: vec![
            QuorumProposalValidated(proposals[1].data.clone(), leaves[0].clone()),
            DaCertificateRecv(dacs[1].clone()),
            VidShareRecv(vids[1].0[0].clone()),
            // The validated state is for an earlier view.
            ValidatedStateUpdated(
                proposals[0].data.view_number(),
                build_fake_view_with_leaf(leaves[0].clone()),
            ),
        ],
        outputs: vec![
            exact(DaCertificateValidated(dacs[1].clone())),
            exact(VidShareValidated(vids[1].0[0].clone())),
        ],
        asserts: vec![],
    };

    let quorum_vote_state =
        QuorumVoteTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    run_test_script(vec![view_incorrect_dependency], quorum_vote_state).await;
}
