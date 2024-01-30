use hotshot::{types::SignatureKey, HotShotConsensusApi};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_testing::{
    block_types::TestTransaction,
    node_types::{MemoryImpl, TestTypes},
};
use hotshot_types::{
    data::{DAProposal, ViewNumber},
    simple_vote::{DAData, DAVote},
    traits::{
        block_contents::vid_commitment,
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
    },
};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, marker::PhantomData};

#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_da_task() {
    use hotshot::tasks::add_da_task;
    use hotshot_task_impls::harness::run_harness;
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_types::message::Proposal;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 2.
    let handle = build_system_handle(2).await.0;
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let pub_key = *api.public_key();
    let transactions = vec![TestTransaction(vec![0])];
    let encoded_transactions = TestTransaction::encode(transactions.clone()).unwrap();
    let payload_commitment = vid_commitment(
        &encoded_transactions,
        handle
            .hotshot
            .inner
            .memberships
            .quorum_membership
            .total_nodes(),
    );
    let encoded_transactions_hash = Sha256::digest(&encoded_transactions);

    let signature =
        <TestTypes as NodeType>::SignatureKey::sign(api.private_key(), &encoded_transactions_hash)
            .expect("Failed to sign block payload");
    let proposal = DAProposal {
        encoded_transactions: encoded_transactions.clone(),
        metadata: (),
        view_number: ViewNumber::new(2),
    };
    let message = Proposal {
        data: proposal,
        signature,
        _pd: PhantomData,
    };

    // TODO for now reuse the same block payload commitment and signature as DA committee
    // https://github.com/EspressoSystems/jellyfish/issues/369

    // Every event input is seen on the event stream in the output.
    let mut input = Vec::new();
    let mut output = HashMap::new();

    // In view 1, node 2 is the next leader.
    input.push(HotShotEvent::ViewChange(ViewNumber::new(1)));
    input.push(HotShotEvent::ViewChange(ViewNumber::new(2)));
    input.push(HotShotEvent::TransactionsSequenced(
        encoded_transactions.clone(),
        (),
        ViewNumber::new(2),
    ));
    input.push(HotShotEvent::DAProposalRecv(message.clone(), pub_key));

    input.push(HotShotEvent::Shutdown);

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(1)), 1);
    output.insert(
        HotShotEvent::TransactionsSequenced(encoded_transactions, (), ViewNumber::new(2)),
        1,
    );
    output.insert(HotShotEvent::DAProposalSend(message.clone(), pub_key), 1);
    let da_vote = DAVote::create_signed_vote(
        DAData {
            payload_commit: payload_commitment,
        },
        ViewNumber::new(2),
        api.public_key(),
        api.private_key(),
    )
    .expect("Failed to sign DAData");
    output.insert(HotShotEvent::DAVoteSend(da_vote), 1);

    output.insert(HotShotEvent::DAProposalRecv(message, pub_key), 1);

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(2)), 1);
    output.insert(HotShotEvent::Shutdown, 1);

    let build_fn = |task_runner, event_stream| add_da_task(task_runner, event_stream, handle);

    run_harness(input, output, None, build_fn, false).await;
}
