#![allow(clippy::panic)]
use hotshot::tasks::task_state::CreateTaskState;
use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime};
use hotshot_testing::task_helpers::get_vid_share;

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_vote_task_success() {
    use hotshot_task_impls::{events::HotShotEvent::*, quorum_vote::QuorumVoteTaskState};
    use hotshot_testing::{
        predicates::{exact, quorum_proposal_validated,quorum_vote_send},
        script::{run_test_script, TestScriptStage},
        task_helpers::build_system_handle,
        view_generator::TestViewGenerator,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    generator.next();
    let view = generator.current_view.clone().unwrap();

    // Send the quorum proposal, DAC, and VID disperse data, in which case a dummy vote can be
    // formed and the view number will be updated.
    let view_success = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(view.quorum_proposal.clone(), view.leader_public_key),
            DACertificateRecv(view.da_certificate.clone()),
            VIDShareRecv(view.vid_proposal.0[0].clone()),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(2))),
            quorum_proposal_validated(),
            exact(DACertificateValidated(view.da_certificate.clone())),
            exact(VIDShareValidated(view.vid_proposal.0[0].clone())),
            exact(QuorumVoteDependenciesValidated(ViewNumber::new(1))),
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
async fn test_quorum_vote_task_miss_dependency() {
    use hotshot_task_impls::{events::HotShotEvent::*, quorum_vote::QuorumVoteTaskState};
    use hotshot_testing::{
        predicates::{exact, quorum_proposal_validated},
        script::{run_test_script, TestScriptStage},
        task_helpers::build_system_handle,
        view_generator::TestViewGenerator,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(3) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Send two of quorum proposal, DAC, and VID disperse data, in which case there's no vote.
    let view_no_dac = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            VIDShareRecv(get_vid_share(&vids[0].0, handle.get_public_key())),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(2))),
            quorum_proposal_validated(),
            exact(VIDShareValidated(vids[0].0[0].clone())),
        ],
        asserts: vec![],
    };
    let view_no_vid = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            DACertificateRecv(dacs[1].clone()),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(3))),
            quorum_proposal_validated(),
            exact(DACertificateValidated(dacs[1].clone())),
        ],
        asserts: vec![],
    };
    let view_no_quorum_proposal = TestScriptStage {
        inputs: vec![
            DACertificateRecv(dacs[2].clone()),
            VIDShareRecv(get_vid_share(&vids[2].0, handle.get_public_key())),
        ],
        outputs: vec![
            exact(DACertificateValidated(dacs[2].clone())),
            exact(VIDShareValidated(vids[2].0[0].clone())),
        ],
        asserts: vec![],
    };

    let quorum_vote_state =
        QuorumVoteTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    run_test_script(
        vec![ view_no_dac, view_no_vid, view_no_quorum_proposal],
        quorum_vote_state,
    )
    .await;
}
