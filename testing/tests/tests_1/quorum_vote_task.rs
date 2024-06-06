#![allow(clippy::panic)]
use futures::StreamExt;
use hotshot::tasks::task_state::CreateTaskState;
use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
use hotshot_macros::{run_test, test_scripts};
use hotshot_testing::{
    all_predicates,
    helpers::{build_fake_view_with_leaf, vid_share},
    predicates::event::all_predicates,
    random,
    script::{Expectations, InputOrder, TaskScript},
    serial,
};
use hotshot_types::{
    data::ViewNumber, traits::node_implementation::ConsensusTime, vote::HasViewNumber,
};
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_millis(35);

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_vote_task_success() {
    use hotshot_task_impls::{events::HotShotEvent::*, quorum_vote::QuorumVoteTaskState};
    use hotshot_testing::{
        helpers::build_system_handle,
        predicates::event::{exact, quorum_vote_send, validated_state_updated},
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
    let consensus = handle.hotshot.consensus().clone();
    let mut consensus_writer = consensus.write().await;
    for view in (&mut generator).take(2).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaves.push(view.leaf.clone());
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        consensus_writer.update_validated_state_map(
            view.quorum_proposal.data.view_number(),
            build_fake_view_with_leaf(view.leaf.clone()),
        );
        consensus_writer.update_saved_leaves(view.leaf.clone());
    }
    drop(consensus_writer);

    // Send the quorum proposal, DAC, VID share data, and validated state, in which case a dummy
    // vote can be formed and the view number will be updated.
    let inputs = vec![random![
        QuorumProposalValidated(proposals[1].data.clone(), leaves[0].clone()),
        DaCertificateRecv(dacs[1].clone()),
        VidShareRecv(vids[1].0[0].clone()),
    ]];

    let expectations = vec![Expectations::from_outputs(all_predicates![
        exact(DaCertificateValidated(dacs[1].clone())),
        exact(VidShareValidated(vids[1].0[0].clone())),
        exact(QuorumVoteDependenciesValidated(ViewNumber::new(2))),
        validated_state_updated(),
        quorum_vote_send(),
    ])];

    let quorum_vote_state =
        QuorumVoteTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let mut script = TaskScript {
        timeout: TIMEOUT,
        state: quorum_vote_state,
        expectations,
    };
    run_test![inputs, script].await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_vote_task_vote_now() {
    use hotshot_task_impls::{events::HotShotEvent::*, quorum_vote::QuorumVoteTaskState};
    use hotshot_testing::{
        helpers::build_system_handle,
        predicates::event::{exact, quorum_vote_send, validated_state_updated},
        view_generator::TestViewGenerator,
    };
    use hotshot_types::vote::VoteDependencyData;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    generator.next().await;
    let view = generator.current_view.clone().unwrap();

    let vote_dependency_data = VoteDependencyData {
        quorum_proposal: view.quorum_proposal.data.clone(),
        parent_leaf: view.leaf.clone(),
        vid_share: view.vid_proposal.0[0].clone(),
        da_cert: view.da_certificate.clone(),
    };

    // Submit an event with just the `VoteNow` event which should successfully send a vote.
    let inputs = vec![serial![VoteNow(view.view_number, vote_dependency_data),]];

    let expectations = vec![Expectations::from_outputs(vec![
        exact(QuorumVoteDependenciesValidated(ViewNumber::new(1))),
        validated_state_updated(),
        quorum_vote_send(),
    ])];

    let quorum_vote_state =
        QuorumVoteTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let mut script = TaskScript {
        timeout: TIMEOUT,
        state: quorum_vote_state,
        expectations,
    };
    run_test![inputs, script].await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_vote_task_miss_dependency() {
    use hotshot_task_impls::{events::HotShotEvent::*, quorum_vote::QuorumVoteTaskState};
    use hotshot_testing::{
        helpers::build_system_handle, predicates::event::exact, view_generator::TestViewGenerator,
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
    let consensus = handle.hotshot.consensus().clone();
    let mut consensus_writer = consensus.write().await;
    for view in (&mut generator).take(5).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaves.push(view.leaf.clone());

        consensus_writer.update_validated_state_map(
            view.quorum_proposal.data.view_number(),
            build_fake_view_with_leaf(view.leaf.clone()),
        );
        consensus_writer.update_saved_leaves(view.leaf.clone());
    }
    drop(consensus_writer);

    // Send three of quorum proposal, DAC, VID share data, and validated state, in which case
    // there's no vote.
    let inputs = vec![
        random![
            QuorumProposalValidated(proposals[1].data.clone(), leaves[0].clone()),
            VidShareRecv(vid_share(&vids[1].0, handle.public_key())),
        ],
        random![
            QuorumProposalValidated(proposals[2].data.clone(), leaves[1].clone()),
            DaCertificateRecv(dacs[2].clone()),
        ],
        random![
            DaCertificateRecv(dacs[3].clone()),
            VidShareRecv(vid_share(&vids[3].0, handle.public_key())),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(vec![exact(VidShareValidated(vids[1].0[0].clone()))]),
        Expectations::from_outputs(vec![exact(DaCertificateValidated(dacs[2].clone()))]),
        Expectations::from_outputs(all_predicates![
            exact(DaCertificateValidated(dacs[3].clone())),
            exact(VidShareValidated(vids[3].0[0].clone())),
        ]),
    ];

    let quorum_vote_state =
        QuorumVoteTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let mut script = TaskScript {
        timeout: TIMEOUT,
        state: quorum_vote_state,
        expectations,
    };
    run_test![inputs, script].await;
}
