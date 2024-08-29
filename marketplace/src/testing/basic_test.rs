use hotshot_types::{
    data::{Leaf, QuorumProposal, ViewNumber},
    message::Proposal,
    signature_key::BLSPubKey,
    simple_certificate::{QuorumCertificate, SimpleCertificate, SuccessThreshold},
    traits::{block_contents::BlockPayload, node_implementation::ConsensusTime},
};

use crate::builder_state::MessageType;

use std::marker::PhantomData;

use async_compatibility_layer::art::async_sleep;
use async_compatibility_layer::channel::unbounded;
use async_std::prelude::FutureExt;
use hotshot::types::SignatureKey;
use hotshot_types::{simple_vote::QuorumData, traits::block_contents::vid_commitment};

use hotshot_example_types::{
    block_types::{TestBlockHeader, TestBlockPayload, TestMetadata, TestTransaction},
    state_types::{TestInstanceState, TestValidatedState},
};

use crate::utils::BuilderStateId;
use crate::{
    builder_state::{DaProposalMessage, QuorumProposalMessage, RequestMessage, TransactionSource},
    testing::TestTypes,
};
use crate::{service::handle_received_txns, testing::start_builder_state};
use committable::Committable;
use std::sync::Arc;
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
        let txn_commitments = transactions.iter().map(Committable::commit).collect();
        let encoded_transactions = TestTransaction::encode(&transactions);
        let block_payload = TestBlockPayload { transactions };
        let block_vid_commitment = vid_commitment(&encoded_transactions, NUM_STORAGE_NODES);
        let block_builder_commitment =
            <TestBlockPayload as BlockPayload<TestTypes>>::builder_commitment(
                &block_payload,
                &TestMetadata,
            );

        // generate key for leader of this round
        let seed = [round as u8; 32];
        let (pub_key, private_key) = BLSPubKey::generated_from_seed_indexed(seed, round as u64);

        let da_proposal = Arc::new(DaProposalMessage {
            view_number: ViewNumber::new(round as u64),
            txn_commitments,
            sender: pub_key,
            builder_commitment: block_builder_commitment.clone(),
        });

        let block_header = TestBlockHeader {
            block_number: round as u64,
            payload_commitment: block_vid_commitment,
            builder_commitment: block_builder_commitment,
            timestamp: round as u64,
        };

        let justify_qc = match prev_quorum_proposal.take() {
            None => {
                QuorumCertificate::<TestTypes>::genesis(
                    &TestValidatedState::default(),
                    &TestInstanceState::default(),
                )
                .await
            }
            Some(prev_proposal) => {
                let prev_justify_qc = &prev_proposal.justify_qc;
                let quorum_data = QuorumData::<TestTypes> {
                    leaf_commit: Leaf::from_quorum_proposal(&prev_proposal).commit(),
                };

                // form a justify qc
                SimpleCertificate::<TestTypes, QuorumData<TestTypes>, SuccessThreshold>::new(
                    quorum_data.clone(),
                    quorum_data.commit(),
                    ViewNumber::new(round as u64),
                    prev_justify_qc.signatures.clone(),
                    PhantomData,
                )
            }
        };
        tracing::debug!("Iteration: {} justify_qc: {:?}", round, justify_qc);

        let quorum_proposal = QuorumProposal::<TestTypes> {
            block_header,
            view_number: ViewNumber::new(round as u64),
            justify_qc: justify_qc.clone(),
            upgrade_certificate: None,
            proposal_certificate: None,
        };

        prev_quorum_proposal = Some(quorum_proposal.clone());

        let qc_signature = <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey::sign(
                        &private_key,
                        block_vid_commitment.as_ref(),
                        ).expect("Failed to sign payload commitment while preparing QC proposal");

        let quorum_proposal_msg =
            MessageType::QuorumProposalMessage(QuorumProposalMessage::<TestTypes> {
                proposal: Arc::new(Proposal {
                    data: quorum_proposal.clone(),
                    signature: qc_signature,
                    _pd: PhantomData,
                }),
                sender: pub_key,
            });

        // send quorum and DA proposals for this round
        senders
            .da_proposal
            .broadcast(MessageType::DaProposalMessage(Arc::clone(&da_proposal)))
            .await
            .unwrap();
        senders
            .quorum_proposal
            .broadcast(quorum_proposal_msg)
            .await
            .unwrap();

        let (response_sender, response_receiver) = unbounded();
        let request_message = MessageType::<TestTypes>::RequestMessage(RequestMessage {
            requested_view_number: ViewNumber::new(round as u64),
            response_channel: response_sender,
        });

        let req_msg = (
            response_receiver,
            BuilderStateId {
                parent_commitment: block_vid_commitment,
                parent_view: ViewNumber::new(round as u64),
            },
            request_message,
        );

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
