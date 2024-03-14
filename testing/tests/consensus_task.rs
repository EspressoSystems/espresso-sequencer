#![allow(clippy::panic)]
use hotshot::types::SystemContextHandle;
use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
use hotshot_testing::test_helpers::permute_input_with_index_order;
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime};
use jf_primitives::vid::VidScheme;

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_task() {
    use hotshot::tasks::{inject_consensus_polls, task_state::CreateTaskState};
    use hotshot_task_impls::{consensus::ConsensusTaskState, events::HotShotEvent::*};
    use hotshot_testing::{
        predicates::{exact, is_at_view_number, quorum_proposal_send},
        script::{run_test_script, TestScriptStage},
        task_helpers::{build_system_handle, vid_scheme_from_view_number},
        view_generator::TestViewGenerator,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    // Make some empty encoded transactions, we just care about having a commitment handy for the
    // later calls. We need the VID commitment to be able to propose later.
    let vid = vid_scheme_from_view_number::<TestTypes>(&quorum_membership, ViewNumber::new(2));
    let encoded_transactions = Vec::new();
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();
    let payload_commitment = vid_disperse.commit;

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

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

    // Run view 1 (the genesis stage).
    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DACRecv(dacs[0].clone()),
            VidDisperseRecv(vids[0].0.clone(), vids[0].1),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(1))),
            exact(QuorumProposalValidated(proposals[0].data.clone())),
            exact(QuorumVoteSend(votes[0].clone())),
        ],
        asserts: vec![is_at_view_number(1)],
    };

    let cert = proposals[1].data.justify_qc.clone();

    // Run view 2 and propose.
    let view_2 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            QCFormed(either::Left(cert)),
            // We must have a payload commitment and metadata to propose.
            SendPayloadCommitmentAndMetadata(payload_commitment, (), ViewNumber::new(2)),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(2))),
            exact(QuorumProposalValidated(proposals[1].data.clone())),
            quorum_proposal_send(),
        ],
        asserts: vec![is_at_view_number(2)],
    };

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;

    inject_consensus_polls(&consensus_state).await;

    run_test_script(vec![view_1, view_2], consensus_state).await;
}

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_vote() {
    use hotshot::tasks::{inject_consensus_polls, task_state::CreateTaskState};
    use hotshot_task_impls::{consensus::ConsensusTaskState, events::HotShotEvent::*};
    use hotshot_testing::{
        predicates::exact,
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
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Send a proposal, vote on said proposal, update view based on proposal QC, receive vote as next leader
    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DACRecv(dacs[0].clone()),
            VidDisperseRecv(vids[0].0.clone(), vids[0].1),
            QuorumVoteRecv(votes[0].clone()),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(1))),
            exact(QuorumProposalValidated(proposals[0].data.clone())),
            exact(QuorumVoteSend(votes[0].clone())),
        ],
        asserts: vec![],
    };

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;

    inject_consensus_polls(&consensus_state).await;
    run_test_script(vec![view_1], consensus_state).await;
}

/// Tests the voting behavior by allowing the input to be permuted in any order desired. This
/// assures that, no matter what, a vote is indeed sent no matter what order the precipitating
/// events occur. The permutation is specified as `input_permutation` and is a vector of indices.
async fn test_vote_with_specific_order(input_permutation: Vec<usize>) {
    use hotshot::tasks::{inject_consensus_polls, task_state::CreateTaskState};
    use hotshot_task_impls::{consensus::ConsensusTaskState, events::HotShotEvent::*};
    use hotshot_testing::{
        predicates::{exact, is_at_view_number},
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
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Get out of the genesis view first
    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DACRecv(dacs[0].clone()),
            VidDisperseRecv(vids[0].0.clone(), vids[0].1),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(1))),
            exact(QuorumProposalValidated(proposals[0].data.clone())),
            exact(QuorumVoteSend(votes[0].clone())),
        ],
        asserts: vec![is_at_view_number(1)],
    };

    let inputs = vec![
        // We need a VID share for view 2 otherwise we cannot vote at view 2 (as node 2).
        VidDisperseRecv(vids[1].0.clone(), vids[1].1),
        DACRecv(dacs[1].clone()),
        QuorumProposalRecv(proposals[1].clone(), leaders[1]),
    ];
    let view_2_inputs = permute_input_with_index_order(inputs, input_permutation);

    let view_2_outputs = vec![
        exact(ViewChange(ViewNumber::new(2))),
        exact(QuorumProposalValidated(proposals[1].data.clone())),
        exact(QuorumVoteSend(votes[1].clone())),
    ];

    // Use the permuted inputs for view 2 depending on the provided index ordering.
    let view_2 = TestScriptStage {
        inputs: view_2_inputs,
        outputs: view_2_outputs,
        asserts: vec![is_at_view_number(2)],
    };

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;

    inject_consensus_polls(&consensus_state).await;
    run_test_script(vec![view_1, view_2], consensus_state).await;
}

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_vote_with_permuted_dac() {
    // These tests verify that a vote is indeed sent no matter when it receives a DACRecv
    // event. In particular, we want to verify that receiving events in an unexpected (but still
    // valid) order allows the system to proceed as it normally would.
    test_vote_with_specific_order(vec![0, 1, 2]).await;
    test_vote_with_specific_order(vec![0, 2, 1]).await;
    test_vote_with_specific_order(vec![1, 0, 2]).await;
    test_vote_with_specific_order(vec![2, 0, 1]).await;
    test_vote_with_specific_order(vec![1, 2, 0]).await;
    test_vote_with_specific_order(vec![2, 1, 0]).await;
}
