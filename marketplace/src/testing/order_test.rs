use hotshot_types::{
    data::{QuorumProposal, ViewNumber},
    traits::node_implementation::ConsensusTime,
};

use crate::builder_state::MessageType;

use std::fmt::Debug;

use async_compatibility_layer::art::async_sleep;
use async_std::prelude::FutureExt;

use hotshot_example_types::block_types::TestTransaction;

use crate::{builder_state::TransactionSource, testing::TestTypes};
use crate::{
    service::handle_received_txns,
    testing::{calc_proposal_msg, get_req_msg, start_builder_state},
};
use hotshot::rand::{self, seq::SliceRandom, thread_rng};
use std::time::Duration;

/// The function checks whether the common part of two transaction vectors have the same order
fn order_check<T: Eq + Clone + Debug>(
    transaction_history: Vec<T>,
    all_transactions: Vec<Vec<T>>,
) -> bool {
    let all_transactions_vec = all_transactions.into_iter().flatten().collect::<Vec<_>>();
    tracing::debug!(
        "Doing order check, transaction_history = {:?}, all_transactions = {:?}",
        transaction_history,
        all_transactions_vec
    );
    let common_txs: Vec<_> = transaction_history
        .iter()
        .filter(|item| all_transactions_vec.contains(item))
        .collect();

    let another_common_txs: Vec<_> = all_transactions_vec
        .iter()
        .filter(|item| transaction_history.contains(item))
        .collect();

    common_txs == another_common_txs
}

/// This test simulates multiple builder states receiving messages from the channels and processing them
/// and focus specifically on orders.
/// It's fine that leader doesn't include some of transactions we've given, or interspersed with other transactions,
/// as long as the order is correct it will be good.
#[async_std::test]
async fn test_builder_order() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    tracing::info!("Testing the builder core with multiple messages from the channels");

    /// Number of views to simulate, make sure it's larger than 5
    /// so that we have enough rounds to play with
    const NUM_ROUNDS: usize = 10;
    /// Number of transactions to submit per round
    const NUM_TXNS_PER_ROUND: usize = 5;
    /// Capacity of broadcast channels
    const CHANNEL_CAPACITY: usize = NUM_ROUNDS * 5;
    /// Number of nodes on DA committee
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

    // generate three different random number between (0..(NUM_ROUNDS-2)) to do some changes for output transactions
    // it's not the last two rounds as they'll be used to test propose_in_advance
    let round_range: Vec<_> = (0..(NUM_ROUNDS - 2)).collect();
    let mut rng = thread_rng();
    let random_rounds: Vec<_> = round_range.choose_multiple(&mut rng, 3).cloned().collect();
    // the round we want to skip all the transactions
    let skip_round = random_rounds[0];
    // the round we want to randomly add some transactions
    let adjust_add_round = random_rounds[1];
    // the round we want to cut off the end of the bundle
    let adjust_remove_tail_round = random_rounds[2];
    // the round we want to include tx in later round (NUM_ROUNDS -1 which is also the final round) to propose in advance
    let propose_in_advance_round = NUM_ROUNDS - 2;

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
            .broadcast(req_msg.2.clone())
            .await
            .unwrap();

        // get response
        // in the next round we will use received transactions to simulate
        // the block being proposed
        let res_msg = req_msg
            .0
            .recv()
            .timeout(Duration::from_secs(10))
            .await
            .unwrap()
            .unwrap();

        // play with transactions propsed by proposers: skip the whole round OR interspersed some txs randomly OR remove some txs randomly
        if let MessageType::<TestTypes>::RequestMessage(ref request) = req_msg.2 {
            let view_number = request.requested_view_number;
            if view_number == ViewNumber::new(skip_round as u64) {
                prev_proposed_transactions = None;
            } else {
                let mut proposed_transactions = res_msg.transactions.clone();
                if view_number == ViewNumber::new(adjust_add_round as u64) {
                    proposed_transactions.insert(
                        rand::random::<usize>() % NUM_TXNS_PER_ROUND,
                        TestTransaction::new(vec![
                            adjust_add_round as u8,
                            (NUM_TXNS_PER_ROUND + 1) as u8,
                        ]),
                    );
                } else if view_number == ViewNumber::new(adjust_remove_tail_round as u64) {
                    proposed_transactions.pop();
                } else if view_number == ViewNumber::new(propose_in_advance_round as u64) {
                    proposed_transactions.push(TestTransaction::new(vec![
                        (propose_in_advance_round + 1) as u8,
                        0_u8,
                    ]));
                }
                prev_proposed_transactions = Some(proposed_transactions);
            }
        } else {
            tracing::error!("Unable to get request from RequestMessage");
        }
        // save transactions to history
        if prev_proposed_transactions.is_some() {
            transaction_history.extend(prev_proposed_transactions.clone().unwrap());
        }
    }

    // we should've served all transactions submitted, and in correct order
    // the test will fail if the common part of two vectors of transactions don't have the same order
    assert!(order_check(transaction_history, all_transactions));
}

/// This test simulates multiple builder states receiving messages from the channels and processing them
/// and focus specifically on orders with chain fork.
/// with one chain proposing transactions we've given and the other not
/// (we should give out the next batch if responding for first chain and both batches for the other)
#[async_std::test]
async fn test_builder_order_chain_fork() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    tracing::info!("Testing the builder core with multiple messages from the channels");

    // Number of views to simulate
    const NUM_ROUNDS: usize = 4;
    // Number of transactions to submit per round
    const NUM_TXNS_PER_ROUND: usize = 5;
    // Capacity of broadcast channels
    const CHANNEL_CAPACITY: usize = NUM_ROUNDS * 5;
    // Number of nodes on DA committee
    const NUM_STORAGE_NODES: usize = 4;

    // the round we want to skip all the transactions for the fork chain
    // round 0 is pre-fork
    // round 1 is where the fork happens
    // round 2 is exactly after the fork, they have different parents and the builder should give out different batches
    // round 3 should be back to normal, there's no fork anymore
    let fork_round = 1;

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

    // set up state to track the fork-ed chain
    let mut prev_proposed_transactions_2: Option<Vec<TestTransaction>> = None;
    let mut prev_quorum_proposal_2: Option<QuorumProposal<TestTypes>> = None;
    let mut transaction_history_2 = Vec::new();
    // the parameter to track whether there's a fork by pending whether the transactions submitted in
    // the previous round are the same
    let mut fork: bool;

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
            calc_proposal_msg(
                NUM_STORAGE_NODES,
                round,
                prev_quorum_proposal,
                transactions.clone(),
            )
            .await;

        let transactions_2 = prev_proposed_transactions_2.take().unwrap_or_default();
        let (quorum_proposal_2, quorum_proposal_msg_2, da_proposal_msg_2, builder_state_id_2) =
            calc_proposal_msg(
                NUM_STORAGE_NODES,
                round,
                prev_quorum_proposal_2.clone(),
                transactions_2.clone(),
            )
            .await;
        if transactions_2 != transactions {
            fork = true;
            tracing::debug!("Fork Exist.")
        } else {
            fork = false;
            tracing::debug!("No fork.");
        }

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

        prev_quorum_proposal_2 = Some(quorum_proposal_2.clone());
        // send quorum and DA proposals for this round
        // we also need to send out the message for the fork-ed chain although it's not forked yet
        // to prevent builders resend the transactions we've already committeed
        senders
            .da_proposal
            .broadcast(da_proposal_msg_2)
            .await
            .unwrap();
        senders
            .quorum_proposal
            .broadcast(quorum_proposal_msg_2)
            .await
            .unwrap();

        let req_msg = get_req_msg(round as u64, builder_state_id).await;
        // give builder state time to fork
        // async_sleep(Duration::from_secs(1)).await;

        // get the builder state for parent view we've just simulated
        global_state
            .read_arc()
            .await
            .spawned_builder_states
            .get(&req_msg.1)
            .expect("Failed to get channel for matching builder")
            .broadcast(req_msg.2.clone())
            .await
            .unwrap();

        // get response
        // in the next round we will use received transactions to simulate
        // the block being proposed
        let res_msg = req_msg
            .0
            .recv()
            .timeout(Duration::from_secs(10))
            .await
            .unwrap()
            .unwrap();

        // we have to get separate request message and response message when there's a fork
        if fork {
            let req_msg_2 = get_req_msg(round as u64, builder_state_id_2).await;
            // give builder state time to fork
            // async_sleep(Duration::from_secs(1)).await;

            // get the builder state for parent view we've just simulated
            global_state
                .read_arc()
                .await
                .spawned_builder_states
                .get(&req_msg_2.1)
                .expect("Failed to get channel for matching builder")
                .broadcast(req_msg_2.2.clone())
                .await
                .unwrap();

            // get response
            let res_msg_2 = req_msg_2
                .0
                .recv()
                .timeout(Duration::from_secs(10))
                .await
                .unwrap()
                .unwrap();

            // play with transactions propsed by proposers: at the fork_round, one chain propose while the other chain does not propose any
            let proposed_transactions_2 = res_msg_2.transactions.clone();
            prev_proposed_transactions_2 = Some(proposed_transactions_2);
        }

        // play with transactions propsed by proposers: at the fork_round, one chain propose while the other chain does not propose any
        let proposed_transactions = res_msg.transactions.clone();
        prev_proposed_transactions = Some(proposed_transactions);
        // if it's the `fork_round` we'll change what we want to propse to `None` for the fork-ed chain
        if let MessageType::<TestTypes>::RequestMessage(ref request) = req_msg.2 {
            let view_number = request.requested_view_number;
            if view_number == ViewNumber::new(fork_round as u64) {
                prev_proposed_transactions_2 = None;
            } else {
                prev_proposed_transactions_2 = prev_proposed_transactions.clone();
            }
        } else {
            tracing::error!("Unable to get request from RequestMessage");
        }

        // save transactions to history
        if prev_proposed_transactions.is_some() {
            transaction_history.extend(prev_proposed_transactions.clone().unwrap());
        }
        if prev_proposed_transactions_2.is_some() {
            transaction_history_2.extend(prev_proposed_transactions_2.clone().unwrap());
        }
    }

    // we should've served all transactions submitted, and in correct order
    // the test will fail if any transaction is skipped or re-ordered
    assert_eq!(
        transaction_history,
        all_transactions
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
    );
    assert_eq!(
        transaction_history_2,
        all_transactions.into_iter().flatten().collect::<Vec<_>>()
    );
}

/// This test simulates multiple builder states receiving messages from the channels and processing them
/// and focus specifically on orders.
/// It should fail as the proposer randomly drop a subset of transactions within a bundle,
/// which leads to different order of transaction.
#[async_std::test]
async fn test_builder_order_should_fail() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();
    tracing::info!("Testing the builder core with multiple messages from the channels");

    // Number of views to simulate
    const NUM_ROUNDS: usize = 10;
    // Number of transactions to submit per round
    const NUM_TXNS_PER_ROUND: usize = 5;
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

    // generate a random number between (0..NUM_ROUNDS) to do some changes for output transactions
    // the round we want to skip some transactions (cannot be the final round), after it is enabled the test is expected to fail
    let adjust_remove_round = rand::random::<usize>() % (NUM_ROUNDS - 1);
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
            .broadcast(req_msg.2.clone())
            .await
            .unwrap();

        // get response
        // in the next round we will use received transactions to simulate
        // the block being proposed
        let res_msg = req_msg
            .0
            .recv()
            .timeout(Duration::from_secs(10))
            .await
            .unwrap()
            .unwrap();

        // play with transactions propsed by proposers: skip the whole round OR interspersed some txs randomly OR remove some txs randomly
        if let MessageType::<TestTypes>::RequestMessage(ref request) = req_msg.2 {
            let view_number = request.requested_view_number;
            let mut proposed_transactions = res_msg.transactions.clone();
            if view_number == ViewNumber::new(adjust_remove_round as u64) {
                proposed_transactions.remove(rand::random::<usize>() % (NUM_TXNS_PER_ROUND - 1));
                // cannot be the last transaction
            }
            prev_proposed_transactions = Some(proposed_transactions);
        } else {
            tracing::error!("Unable to get request from RequestMessage");
        }
        // save transactions to history
        if prev_proposed_transactions.is_some() {
            transaction_history.extend(prev_proposed_transactions.clone().unwrap());
        }
    }
    // we should've served all transactions submitted, and in correct order
    // the test will fail if the common part of two vectors of transactions don't have the same order
    assert!(!order_check(transaction_history, all_transactions));
}
