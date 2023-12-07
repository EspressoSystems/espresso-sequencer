use commit::Committable;
use hotshot::{
    tasks::add_consensus_task,
    types::{SignatureKey, SystemContextHandle},
    HotShotConsensusApi,
};
use hotshot_task::event_stream::ChannelStream;
use hotshot_task_impls::events::HotShotEvent;
use hotshot_testing::{
    node_types::{MemoryImpl, TestTypes},
    task_helpers::{build_quorum_proposal, key_pair_for_id},
};
use hotshot_types::simple_vote::QuorumVote;
use hotshot_types::vote::Certificate;
use hotshot_types::{
    data::{Leaf, QuorumProposal, ViewNumber},
    message::GeneralConsensusMessage,
    traits::state::ConsensusTime,
};
use hotshot_types::{
    simple_vote::QuorumData,
    traits::{consensus_api::ConsensusSharedApi, election::Membership},
};

use std::collections::HashMap;

async fn build_vote(
    handle: &SystemContextHandle<TestTypes, MemoryImpl>,
    proposal: QuorumProposal<TestTypes>,
) -> GeneralConsensusMessage<TestTypes> {
    let consensus_lock = handle.get_consensus();
    let consensus = consensus_lock.read().await;
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let membership = api.inner.memberships.quorum_membership.clone();

    let justify_qc = proposal.justify_qc.clone();
    let view = ViewNumber::new(*proposal.view_number);
    let parent = if justify_qc.is_genesis {
        let Some(genesis_view) = consensus.state_map.get(&ViewNumber::new(0)) else {
            panic!("Couldn't find genesis view in state map.");
        };
        let Some(leaf) = genesis_view.get_leaf_commitment() else {
            panic!("Genesis view points to a view without a leaf");
        };
        let Some(leaf) = consensus.saved_leaves.get(&leaf) else {
            panic!("Failed to find genesis leaf.");
        };
        leaf.clone()
    } else {
        consensus
            .saved_leaves
            .get(&justify_qc.get_data().leaf_commit)
            .cloned()
            .unwrap()
    };

    let parent_commitment = parent.commit();

    let leaf: Leaf<_> = Leaf {
        view_number: view,
        justify_qc: proposal.justify_qc.clone(),
        parent_commitment,
        block_header: proposal.block_header,
        block_payload: None,
        rejected: Vec::new(),
        timestamp: 0,
        proposer_id: membership.get_leader(view).to_bytes(),
    };
    let vote = QuorumVote::<TestTypes>::create_signed_vote(
        QuorumData {
            leaf_commit: leaf.commit(),
        },
        view,
        api.public_key(),
        api.private_key(),
    );
    GeneralConsensusMessage::<TestTypes>::Vote(vote)
}

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_task() {
    use hotshot_task_impls::harness::run_harness;
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

    output.insert(HotShotEvent::QCFormed(either::Left(qc)), 1);
    output.insert(
        HotShotEvent::QuorumProposalSend(proposal.clone(), public_key),
        1,
    );
    output.insert(
        HotShotEvent::QuorumProposalRecv(proposal.clone(), public_key),
        1,
    );
    output.insert(
        HotShotEvent::ViewChange(ViewNumber::new(1)),
        1,
    );

    if let GeneralConsensusMessage::Vote(vote) = build_vote(&handle, proposal.data).await {
        output.insert(HotShotEvent::QuorumVoteSend(vote.clone()), 1);
        input.push(HotShotEvent::QuorumVoteRecv(vote.clone()));
        output.insert(HotShotEvent::QuorumVoteRecv(vote), 1);
    }

    output.insert(HotShotEvent::Shutdown, 1);

    let build_fn = |task_runner, event_stream| {
        add_consensus_task(task_runner, event_stream, ChannelStream::new(), handle)
    };

    run_harness(input, output, None, build_fn).await;
}

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_with_vid_vote() {
    use hotshot_task_impls::harness::run_harness;
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_testing::block_types::TestTransaction;
    use hotshot_testing::task_helpers::vid_init;
    use hotshot_types::data::VidSchemeTrait;
    use hotshot_types::{
        data::VidDisperse, message::Proposal,
        traits::node_implementation::NodeType,
    };
    use std::marker::PhantomData;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    // We assign node's key pair rather than read from config file since it's a test
    let (private_key_view1, public_key_view1) = key_pair_for_id(1);
    // In view 2, node 2 is the leader.
    let (private_key_view2, public_key_view2) = key_pair_for_id(2);

    // For the test of vote logic with vid
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let pub_key = *api.public_key();
    let vid = vid_init();
    let transactions = vec![TestTransaction(vec![0])];
    let encoded_transactions = TestTransaction::encode(transactions.clone()).unwrap();
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();
    let payload_commitment = vid_disperse.commit;

    let signature =
        <TestTypes as NodeType>::SignatureKey::sign(api.private_key(), payload_commitment.as_ref());
    let vid_disperse = VidDisperse {
        view_number: ViewNumber::new(2),
        payload_commitment,
        shares: vid_disperse.shares,
        common: vid_disperse.common,
    };
    let vid_proposal: Proposal<TestTypes, VidDisperse<TestTypes>> = Proposal {
        data: vid_disperse.clone(),
        signature,
        _pd: PhantomData,
    };

    let mut input = Vec::new();
    let mut output = HashMap::new();

    let proposal_view1 = build_quorum_proposal(&handle, &private_key_view1, 1).await;
    // Do a view change, so that it's not the genesis view, and vid vote is needed
    input.push(HotShotEvent::ViewChange(ViewNumber::new(1)));
    input.push(HotShotEvent::QuorumProposalRecv(
        proposal_view1.clone(),
        public_key_view1,
    ));

    output.insert(
        HotShotEvent::QuorumProposalRecv(proposal_view1.clone(), public_key_view1),
        1,
    );

    // Since it's genesis view, node can vote without dac and vid share
    if let GeneralConsensusMessage::Vote(vote) = build_vote(&handle, proposal_view1.data).await {
        output.insert(HotShotEvent::QuorumVoteSend(vote.clone()), 1);
        input.push(HotShotEvent::QuorumVoteRecv(vote.clone()));
        output.insert(HotShotEvent::QuorumVoteRecv(vote), 1);
    }



    input.push(HotShotEvent::ViewChange(ViewNumber::new(2)));
    let proposal_view2 = build_quorum_proposal(&handle, &private_key_view2, 2).await;
    // Send a proposal, vote on said proposal, update view based on proposal QC, receive vote as next leader
    input.push(HotShotEvent::QuorumProposalRecv(
        proposal_view2.clone(),
        public_key_view2,
    ));
    // followings are for the test of vote logic with vid
    input.push(HotShotEvent::VidDisperseRecv(vid_proposal.clone(), pub_key));

    output.insert(
        HotShotEvent::QuorumProposalRecv(proposal_view2.clone(), public_key_view2),
        1,
    );
    output.insert(HotShotEvent::VidDisperseRecv(vid_proposal, pub_key), 1);
    // if let GeneralConsensusMessage::Vote(vote) = build_vote(&handle, proposal_view2.data).await {
    //     output.insert(HotShotEvent::QuorumVoteSend(vote.clone()), 1);
    // }

    output.insert(
        HotShotEvent::ViewChange(ViewNumber::new(1)), 
        2, // 2 occurrences: 1 from `QuorumProposalRecv`, 1 from input
    );
    output.insert(
        HotShotEvent::ViewChange(ViewNumber::new(2)), 
        2, // 2 occurrences: 1 from `QuorumProposalRecv`?, 1 from input
    );

    input.push(HotShotEvent::Shutdown);
    output.insert(HotShotEvent::Shutdown, 1);

    let build_fn = |task_runner, event_stream| {
        add_consensus_task(task_runner, event_stream, ChannelStream::new(), handle)
    };

    run_harness(input, output, None, build_fn).await;
}

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_no_vote_without_vid_share() {
    use hotshot_task_impls::harness::run_harness;
    use hotshot_testing::task_helpers::build_system_handle;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    // We assign node's key pair rather than read from config file since it's a test
    let (private_key_view1, public_key_view1) = key_pair_for_id(1);
    // In view 2, node 2 is the leader.
    let (private_key_view2, public_key_view2) = key_pair_for_id(2);

    let mut input = Vec::new();
    let mut output = HashMap::new();

    let proposal_view1 = build_quorum_proposal(&handle, &private_key_view1, 1).await;
    // Do a view change, so that it's not the genesis view, and vid vote is needed
    input.push(HotShotEvent::ViewChange(ViewNumber::new(1)));
    input.push(HotShotEvent::QuorumProposalRecv(
        proposal_view1.clone(),
        public_key_view1,
    ));


    output.insert(
        HotShotEvent::QuorumProposalRecv(proposal_view1.clone(), public_key_view1),
        1,
    );

    // Since it's genesis view, node can vote without dac and vid share
    if let GeneralConsensusMessage::Vote(vote) = build_vote(&handle, proposal_view1.data).await {
        output.insert(HotShotEvent::QuorumVoteSend(vote.clone()), 1);
        input.push(HotShotEvent::QuorumVoteRecv(vote.clone()));
        output.insert(HotShotEvent::QuorumVoteRecv(vote), 1);
    }

    input.push(HotShotEvent::ViewChange(ViewNumber::new(2)));
    let proposal_view2 = build_quorum_proposal(&handle, &private_key_view2, 2).await;
    input.push(HotShotEvent::QuorumProposalRecv(
        proposal_view2.clone(),
        public_key_view2,
    ));

    // Without vid share, there is no HotShotEvent::QuorumVoteSend in the output.
    output.insert(
        HotShotEvent::QuorumProposalRecv(proposal_view2.clone(), public_key_view2),
        1,
    );

    output.insert(
        HotShotEvent::ViewChange(ViewNumber::new(1)), 
        2, // 2 occurrences: 1 from `QuorumProposalRecv`, 1 from input
    );
    output.insert(
        HotShotEvent::ViewChange(ViewNumber::new(2)), 
        2, // 2 occurrences: 1 from `QuorumProposalRecv`?, 1 from input
    );

    input.push(HotShotEvent::Shutdown);
    output.insert(HotShotEvent::Shutdown, 1);

    let build_fn = |task_runner, event_stream| {
        add_consensus_task(task_runner, event_stream, ChannelStream::new(), handle)
    };

    run_harness(input, output, None, build_fn).await;
}

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_vote() {
    use hotshot_task_impls::harness::run_harness;
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
    output.insert(
        HotShotEvent::QuorumProposalRecv(proposal.clone(), public_key),
        1,
    );
    let proposal = proposal.data;
    if let GeneralConsensusMessage::Vote(vote) = build_vote(&handle, proposal).await {
        output.insert(HotShotEvent::QuorumVoteSend(vote.clone()), 1);
        input.push(HotShotEvent::QuorumVoteRecv(vote.clone()));
        output.insert(HotShotEvent::QuorumVoteRecv(vote), 1);
    }

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(1)), 1);

    input.push(HotShotEvent::Shutdown);
    output.insert(HotShotEvent::Shutdown, 1);

    let build_fn = |task_runner, event_stream| {
        add_consensus_task(task_runner, event_stream, ChannelStream::new(), handle)
    };

    run_harness(input, output, None, build_fn).await;
}

