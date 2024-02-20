use hotshot::tasks::task_state::CreateTaskState;
use hotshot::types::SignatureKey;
use hotshot::types::SystemContextHandle;
use hotshot_example_types::node_types::MemoryImpl;
use hotshot_example_types::{block_types::TestTransaction, node_types::TestTypes};
use hotshot_task_impls::{da::DATaskState, events::HotShotEvent};
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
    use hotshot_task_impls::harness::run_harness;
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_types::message::Proposal;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 2.
    let handle = build_system_handle(2).await.0;
    let pub_key = *handle.public_key();
    let transactions = vec![TestTransaction(vec![0])];
    let encoded_transactions = TestTransaction::encode(transactions.clone()).unwrap();
    let payload_commitment = vid_commitment(
        &encoded_transactions,
        handle.hotshot.memberships.quorum_membership.total_nodes(),
    );
    let encoded_transactions_hash = Sha256::digest(&encoded_transactions);

    let signature = <TestTypes as NodeType>::SignatureKey::sign(
        handle.private_key(),
        &encoded_transactions_hash,
    )
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

    output.insert(HotShotEvent::DAProposalSend(message.clone(), pub_key), 1);
    let da_vote = DAVote::create_signed_vote(
        DAData {
            payload_commit: payload_commitment,
        },
        ViewNumber::new(2),
        handle.public_key(),
        handle.private_key(),
    )
    .expect("Failed to sign DAData");
    output.insert(HotShotEvent::DAVoteSend(da_vote), 1);

    let da_state = DATaskState::<TestTypes, MemoryImpl, SystemContextHandle<TestTypes, MemoryImpl>>::create_from(&handle);
    run_harness(input, output, da_state, false).await;
}
