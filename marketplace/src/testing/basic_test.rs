use hotshot_types::data::QuorumProposal;

use async_compatibility_layer::art::async_sleep;
use async_std::prelude::FutureExt;

use hotshot_example_types::block_types::TestTransaction;

use crate::{builder_state::TransactionSource, testing::TestTypes};
use crate::{
    service::handle_received_txns,
    testing::{calc_proposal_msg, get_req_msg, start_builder_state},
};
use std::time::Duration;

/// This test simulates multiple builder states receiving messages from the channels and processing them
#[async_std::test]
async fn test_builder() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    tracing::info!("Testing the builder core with multiple messages from the channels");

    // Number of views to simulate
    const NUM_ROUNDS: usize = 5;
    // Number of transactions to submit per round
    const NUM_TXNS_PER_ROUND: usize = 4;
    // Capacity of broadcast channels
    const CHANNEL_CAPACITY: usize = NUM_ROUNDS * 5;
    // Number of nodes on DA committee
    const NUM_STORAGE_NODES: usize = 4;

    let (senders, global_state) = start_builder_state(CHANNEL_CAPACITY, NUM_STORAGE_NODES).await;

    // Transactions to send
    let all_transactions = (0..NUM_ROUNDS)
        .map(|round| {
            (0..NUM_TXNS_PER_ROUND)
                .map(|tx_num| TestTransaction::new(vec![round as u8, tx_num as u8]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // set up state to track between simulated consensus rounds
    let mut prev_proposed_transactions: Option<Vec<TestTransaction>> = None;
    let mut prev_quorum_proposal: Option<QuorumProposal<TestTypes>> = None;
    let mut transaction_history = Vec::new();

    // Simulate NUM_ROUNDS of consensus. First we submit the transactions for this round to the builder,
    // then construct DA and Quorum Proposals based on what we received from builder in the previous round
    // and request a new bundle.
    #[allow(clippy::needless_range_loop)] // intent is clearer this way
    for round in 0..NUM_ROUNDS {
        // simulate transaction being submitted to the builder
        for res in handle_received_txns(
            &senders.transactions,
            all_transactions[round].clone(),
            TransactionSource::HotShot,
        )
        .await
        {
            res.unwrap();
        }

        // get transactions submitted in previous rounds, [] for genesis
        // and simulate the block built from those
        let transactions = prev_proposed_transactions.take().unwrap_or_default();
        let (quorum_proposal, quorum_proposal_msg, da_proposal_msg, builder_state_id) =
            calc_proposal_msg(NUM_STORAGE_NODES, round, prev_quorum_proposal, transactions).await;

        prev_quorum_proposal = Some(quorum_proposal.clone());

        // send quorum and DA proposals for this round
        senders
            .da_proposal
            .broadcast(da_proposal_msg)
            .await
            .unwrap();
        senders
            .quorum_proposal
            .broadcast(quorum_proposal_msg)
            .await
            .unwrap();

        let req_msg = get_req_msg(round as u64, builder_state_id).await;

        // give builder state time to fork
        async_sleep(Duration::from_millis(100)).await;

        // get the builder state for parent view we've just simulated
        global_state
            .read_arc()
            .await
            .spawned_builder_states
            .get(&req_msg.1)
            .expect("Failed to get channel for matching builder")
            .1
            .broadcast(req_msg.2.clone())
            .await
            .unwrap();

        // get response
        let res_msg = req_msg
            .0
            .recv()
            .timeout(Duration::from_secs(10))
            .await
            .unwrap()
            .unwrap();
        // in the next round we will use received transactions to simulate
        // the block being proposed
        prev_proposed_transactions = Some(res_msg.transactions.clone());
        // save transactions to history
        transaction_history.extend(res_msg.transactions);
    }

    // we should've served all transactions submitted, and in correct order
    assert_eq!(
        transaction_history,
        all_transactions.into_iter().flatten().collect::<Vec<_>>()
    );
}
