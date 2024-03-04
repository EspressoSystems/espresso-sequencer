#![allow(clippy::panic)]
use hotshot::types::SystemContextHandle;
use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_testing::task_helpers::{build_quorum_proposal, build_vote, key_pair_for_id};
use hotshot_types::traits::{consensus_api::ConsensusApi, election::Membership};
use hotshot_types::{
    data::ViewNumber, message::GeneralConsensusMessage, traits::node_implementation::ConsensusTime,
};
use jf_primitives::vid::VidScheme;
use std::collections::HashMap;

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_task() {
    use hotshot::tasks::{inject_consensus_polls, task_state::CreateTaskState};
    use hotshot_task_impls::{consensus::ConsensusTaskState, harness::run_harness};
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_types::simple_certificate::QuorumCertificate;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(1).await.0;
    // We assign node's key pair rather than read from config file since it's a test
    let (private_key, public_key) = key_pair_for_id(1);

    let mut input = Vec::new();
    let mut output = HashMap::new();

    // Trigger a proposal to send by creating a new QC.  Then recieve that proposal and update view based on the valid QC in the proposal
    let qc = QuorumCertificate::<TestTypes>::genesis();
    let proposal = build_quorum_proposal(&handle, &private_key, 1).await;

    input.push(HotShotEvent::QCFormed(either::Left(qc.clone())));
    input.push(HotShotEvent::QuorumProposalRecv(
        proposal.clone(),
        public_key,
    ));

    input.push(HotShotEvent::Shutdown);

    output.insert(
        HotShotEvent::QuorumProposalSend(proposal.clone(), public_key),
        1,
    );

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(1)), 1);

    if let GeneralConsensusMessage::Vote(vote) = build_vote(&handle, proposal.data).await {
        output.insert(HotShotEvent::QuorumVoteSend(vote.clone()), 1);
        input.push(HotShotEvent::QuorumVoteRecv(vote.clone()));
    }

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;

    inject_consensus_polls(&consensus_state).await;

    run_harness(input, output, consensus_state, false).await;
}

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_vote() {
    use hotshot::tasks::{inject_consensus_polls, task_state::CreateTaskState};
    use hotshot_task_impls::{consensus::ConsensusTaskState, harness::run_harness};
    use hotshot_testing::task_helpers::build_system_handle;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    // We assign node's key pair rather than read from config file since it's a test
    let (private_key, public_key) = key_pair_for_id(1);

    let mut input = Vec::new();
    let mut output = HashMap::new();

    let proposal = build_quorum_proposal(&handle, &private_key, 1).await;

    // Send a proposal, vote on said proposal, update view based on proposal QC, receive vote as next leader
    input.push(HotShotEvent::QuorumProposalRecv(
        proposal.clone(),
        public_key,
    ));

    let proposal = proposal.data;
    if let GeneralConsensusMessage::Vote(vote) = build_vote(&handle, proposal).await {
        output.insert(HotShotEvent::QuorumVoteSend(vote.clone()), 1);
        input.push(HotShotEvent::QuorumVoteRecv(vote.clone()));
    }

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(1)), 1);

    input.push(HotShotEvent::Shutdown);

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;

    inject_consensus_polls(&consensus_state).await;

    run_harness(input, output, consensus_state, false).await;
}

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
// TODO: re-enable this when HotShot/the sequencer needs the shares for something
// issue: https://github.com/EspressoSystems/HotShot/issues/2236
#[ignore]
async fn test_consensus_with_vid() {
    use hotshot::tasks::{inject_consensus_polls, task_state::CreateTaskState};
    use hotshot::traits::BlockPayload;
    use hotshot::types::SignatureKey;
    use hotshot_example_types::block_types::TestBlockPayload;
    use hotshot_example_types::block_types::TestTransaction;
    use hotshot_task_impls::{consensus::ConsensusTaskState, harness::run_harness};
    use hotshot_testing::task_helpers::build_cert;
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_testing::task_helpers::vid_scheme_from_view_number;
    use hotshot_types::simple_certificate::DACertificate;
    use hotshot_types::simple_vote::DAData;
    use hotshot_types::simple_vote::DAVote;
    use hotshot_types::traits::block_contents::{vid_commitment, TestableBlock};
    use hotshot_types::{
        data::VidDisperse, message::Proposal, traits::node_implementation::NodeType,
    };
    use std::marker::PhantomData;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let (handle, _tx, _rx) = build_system_handle(2).await;
    // We assign node's key pair rather than read from config file since it's a test
    // In view 2, node 2 is the leader.
    let (private_key_view2, public_key_view2) = key_pair_for_id(2);

    // For the test of vote logic with vid
    let pub_key = *handle.public_key();
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let vid = vid_scheme_from_view_number::<TestTypes>(&quorum_membership, ViewNumber::new(2));
    let transactions = vec![TestTransaction(vec![0])];
    let encoded_transactions = TestTransaction::encode(transactions.clone()).unwrap();
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();
    let payload_commitment = vid_disperse.commit;

    let vid_signature = <TestTypes as NodeType>::SignatureKey::sign(
        handle.private_key(),
        payload_commitment.as_ref(),
    )
    .expect("Failed to sign payload commitment");
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();
    let vid_disperse_inner = VidDisperse::from_membership(
        ViewNumber::new(2),
        vid_disperse,
        &quorum_membership.clone().into(),
    );
    // TODO for now reuse the same block payload commitment and signature as DA committee
    // https://github.com/EspressoSystems/jellyfish/issues/369
    let vid_proposal = Proposal {
        data: vid_disperse_inner.clone(),
        signature: vid_signature,
        _pd: PhantomData,
    };

    let mut input = Vec::new();
    let mut output = HashMap::new();

    // Do a view change, so that it's not the genesis view, and vid vote is needed
    input.push(HotShotEvent::ViewChange(ViewNumber::new(1)));

    // For the test of vote logic with vid, starting view 2 we need vid share
    input.push(HotShotEvent::ViewChange(ViewNumber::new(2)));
    let proposal_view2 = build_quorum_proposal(&handle, &private_key_view2, 2).await;
    let block = <TestBlockPayload as TestableBlock>::genesis();
    let da_payload_commitment = vid_commitment(
        &block.encode().unwrap().collect(),
        quorum_membership.total_nodes(),
    );
    let da_data = DAData {
        payload_commit: da_payload_commitment,
    };
    let created_dac_view2 =
        build_cert::<TestTypes, DAData, DAVote<TestTypes>, DACertificate<TestTypes>>(
            da_data,
            &quorum_membership,
            ViewNumber::new(2),
            &public_key_view2,
            &private_key_view2,
        );
    input.push(HotShotEvent::DACRecv(created_dac_view2.clone()));
    input.push(HotShotEvent::VidDisperseRecv(vid_proposal.clone(), pub_key));

    // Send a proposal, vote on said proposal, update view based on proposal QC, receive vote as next leader
    input.push(HotShotEvent::QuorumProposalRecv(
        proposal_view2.clone(),
        public_key_view2,
    ));

    if let GeneralConsensusMessage::Vote(vote) = build_vote(&handle, proposal_view2.data).await {
        output.insert(HotShotEvent::QuorumVoteSend(vote.clone()), 1);
    }

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(1)), 1);
    output.insert(HotShotEvent::ViewChange(ViewNumber::new(2)), 1);

    input.push(HotShotEvent::Shutdown);
    output.insert(HotShotEvent::Shutdown, 1);

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;

    inject_consensus_polls(&consensus_state).await;

    run_harness(input, output, consensus_state, false).await;
}
