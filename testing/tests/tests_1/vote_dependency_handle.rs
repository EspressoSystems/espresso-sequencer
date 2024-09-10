#![cfg(feature = "dependency-tasks")]

use std::time::Duration;

use async_broadcast::broadcast;
use async_compatibility_layer::art::async_timeout;
use futures::StreamExt;
use hotshot_example_types::node_types::{MemoryImpl, TestTypes, TestVersions};
use hotshot_task::dependency_task::HandleDepOutput;
use hotshot_task_impls::{events::HotShotEvent::*, quorum_vote::VoteDependencyHandle};
use hotshot_testing::{
    helpers::{build_fake_view_with_leaf, build_system_handle},
    predicates::{event::*, Predicate, PredicateResult},
    view_generator::TestViewGenerator,
};
use hotshot_types::{
    consensus::OuterConsensus,
    data::ViewNumber,
    traits::{consensus_api::ConsensusApi, node_implementation::ConsensusTime},
    vote::HasViewNumber,
};
use itertools::Itertools;

const TIMEOUT: Duration = Duration::from_millis(35);

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_vote_dependency_handle() {
    use std::sync::Arc;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // We use a node ID of 2 here abitrarily. We just need it to build the system handle.
    let node_id = 2;
    // Construct the system handle for the node ID to build all of the state objects.
    let handle = build_system_handle::<TestTypes, MemoryImpl, TestVersions>(node_id)
        .await
        .0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    // Generate our state for the test
    let mut proposals = Vec::new();
    let mut leaves = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    let consensus = handle.hotshot.consensus().clone();
    let mut consensus_writer = consensus.write().await;
    for view in (&mut generator).take(3).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaves.push(view.leaf.clone());
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        consensus_writer
            .update_validated_state_map(
                view.quorum_proposal.data.view_number(),
                build_fake_view_with_leaf(view.leaf.clone(), &handle.hotshot.upgrade_lock).await,
            )
            .unwrap();
        consensus_writer
            .update_saved_leaves(view.leaf.clone(), &handle.hotshot.upgrade_lock)
            .await;
    }
    drop(consensus_writer);

    // We permute all possible orderings of inputs. Ordinarily we'd use `random!` for this, but
    // the dependency handles do not (yet) work with the existing test suite.
    let all_inputs = vec![
        DaCertificateValidated(dacs[1].clone()),
        QuorumProposalValidated(proposals[1].data.clone(), leaves[0].clone()),
        VidShareValidated(vids[1].0[0].clone()),
    ]
    .into_iter()
    .permutations(3);

    // For each permutation...
    for inputs in all_inputs.into_iter() {
        // The outputs are static here, but we re-make them since we use `into_iter` below
        let outputs = vec![
            exact(QuorumVoteDependenciesValidated(ViewNumber::new(2))),
            validated_state_updated(),
            quorum_vote_send(),
        ];

        let (event_sender, mut event_receiver) = broadcast(1024);
        let view_number = ViewNumber::new(node_id);

        let vote_dependency_handle_state =
            VoteDependencyHandle::<TestTypes, MemoryImpl, TestVersions> {
                public_key: handle.public_key(),
                private_key: handle.private_key().clone(),
                consensus: OuterConsensus::new(consensus.clone()),
                instance_state: handle.hotshot.instance_state(),
                quorum_membership: handle.hotshot.memberships.quorum_membership.clone().into(),
                storage: Arc::clone(&handle.storage()),
                view_number,
                sender: event_sender.clone(),
                receiver: event_receiver.clone(),
                upgrade_lock: handle.hotshot.upgrade_lock.clone(),
                id: handle.hotshot.id,
            };

        vote_dependency_handle_state
            .handle_dep_result(inputs.clone().into_iter().map(|i| i.into()).collect())
            .await;

        // We need to avoid re-processing the inputs during our output evaluation. This part here is not
        // strictly necessary, but it makes writing the outputs easier.
        let mut output_events = vec![];
        while let Ok(Ok(received_output)) =
            async_timeout(TIMEOUT, event_receiver.recv_direct()).await
        {
            output_events.push(received_output);
        }

        assert_eq!(
            output_events.len(),
            outputs.len(),
            "Output event count differs from expected"
        );

        // Finally, evaluate that the test does what we expected. The control flow of the handle always
        // outputs in the same order.
        for (check, real) in outputs.into_iter().zip(output_events) {
            if check.evaluate(&real).await == PredicateResult::Fail {
                panic!("Output {real} did not match expected output {check:?}");
            }
        }
    }
}
