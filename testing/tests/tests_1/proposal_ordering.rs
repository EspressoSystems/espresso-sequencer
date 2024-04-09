use hotshot::{tasks::task_state::CreateTaskState, types::SystemContextHandle};
use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
use hotshot_task_impls::{consensus::ConsensusTaskState, events::HotShotEvent::*};
use hotshot_testing::{
    predicates::{exact, is_at_view_number, quorum_proposal_send},
    task_helpers::vid_scheme_from_view_number,
    test_helpers::permute_input_with_index_order,
    view_generator::TestViewGenerator,
};
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime};
use jf_primitives::vid::VidScheme;

/// Runs a basic test where a qualified proposal occurs (i.e. not initiated by the genesis view or node 1).
/// This proposal should happen no matter how the `input_permutation` is specified.
async fn test_ordering_with_specific_order(input_permutation: Vec<usize>) {
    use hotshot_testing::script::{run_test_script, TestScriptStage};
    use hotshot_testing::task_helpers::build_system_handle;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 2;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let mut vid =
        vid_scheme_from_view_number::<TestTypes>(&quorum_membership, ViewNumber::new(node_id));

    // Make some empty encoded transactions, we just care about having a commitment handy for the
    // later calls.
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
        votes.push(view.create_quorum_vote(&handle));
        leaders.push(view.leader_public_key);
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // This stage transitions from the initial view to view 1
    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DACertificateRecv(dacs[0].clone()),
            VidDisperseRecv(vids[0].0[0].clone()),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(1))),
            exact(QuorumProposalValidated(proposals[0].data.clone())),
            exact(QuorumVoteSend(votes[0].clone())),
        ],
        asserts: vec![is_at_view_number(1)],
    };

    // Node 2 is the leader up next, so we form the QC for it.
    let cert = proposals[1].data.justify_qc.clone();
    let inputs = vec![
        QuorumProposalRecv(proposals[1].clone(), leaders[1]),
        QCFormed(either::Left(cert)),
        SendPayloadCommitmentAndMetadata(payload_commitment, (), ViewNumber::new(node_id)),
    ];

    let view_2_outputs = 
        vec![
            exact(ViewChange(ViewNumber::new(2))),
            exact(QuorumProposalValidated(proposals[1].data.clone())),
            quorum_proposal_send(),
        ];

    let view_2_inputs = permute_input_with_index_order(inputs, input_permutation);

    // This stage transitions from view 1 to view 2.
    let view_2 = TestScriptStage {
        inputs: view_2_inputs,
        outputs: view_2_outputs,
        // We should end on view 2.
        asserts: vec![is_at_view_number(2)],
    };

    let script = vec![view_1, view_2];

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;

    run_test_script(script, consensus_state).await;
}

#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
/// A leader node may receive one of a couple of possible events which can trigger a proposal. This
/// test ensures that, no matter what order these events are received in, the node will still
/// trigger the proposal event regardless. This is to catch a regression in which
/// `SendPayloadCommitmentAndMetadata`, when received last, resulted in no proposal occurring.
async fn test_proposal_ordering() {
    test_ordering_with_specific_order(vec![0, 1, 2]).await;
    test_ordering_with_specific_order(vec![0, 2, 1]).await;
    test_ordering_with_specific_order(vec![1, 0, 2]).await;
    test_ordering_with_specific_order(vec![2, 0, 1]).await;
    test_ordering_with_specific_order(vec![1, 2, 0]).await;
    test_ordering_with_specific_order(vec![2, 1, 0]).await;
}
