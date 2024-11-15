use std::marker::PhantomData;

use async_broadcast::Sender;
use committable::Committable;
use hotshot::{
    rand::{seq::SliceRandom, thread_rng},
    traits::BlockPayload,
    types::{BLSPubKey, Event, EventType, SignatureKey},
};
use hotshot_example_types::{
    block_types::{TestBlockHeader, TestBlockPayload, TestMetadata, TestTransaction},
    node_types::{TestTypes, TestVersions},
    state_types::{TestInstanceState, TestValidatedState},
};
use hotshot_types::{
    data::{DaProposal, Leaf, QuorumProposal, ViewNumber},
    message::Proposal,
    simple_certificate::{QuorumCertificate, SimpleCertificate, SuccessThreshold},
    simple_vote::QuorumData,
    traits::{block_contents::vid_commitment, node_implementation::ConsensusTime},
};
use marketplace_builder_shared::block::BuilderStateId;
use marketplace_builder_shared::testing::constants::TEST_NUM_NODES_IN_VID_COMPUTATION;
use sha2::{Digest, Sha256};

pub mod basic_test;
pub mod integration;
pub mod order_test;

pub(crate) struct SimulatedChainState {
    round: ViewNumber,
    previous_quorum_proposal: Option<QuorumProposal<TestTypes>>,
    event_stream_sender: Sender<Event<TestTypes>>,
}

impl SimulatedChainState {
    pub fn new(event_stream_sender: Sender<Event<TestTypes>>) -> Self {
        Self {
            round: ViewNumber::genesis(),
            previous_quorum_proposal: None,
            event_stream_sender,
        }
    }

    pub async fn simulate_consensus_round(
        &mut self,
        transactions: Option<Vec<TestTransaction>>,
    ) -> BuilderStateId<TestTypes> {
        let transactions = transactions.unwrap_or_default();
        let num_transactions = transactions.len() as u64;
        let encoded_transactions = TestTransaction::encode(&transactions);
        let block_payload = TestBlockPayload { transactions };
        let block_vid_commitment =
            vid_commitment(&encoded_transactions, TEST_NUM_NODES_IN_VID_COMPUTATION);
        let metadata = TestMetadata { num_transactions };
        let block_builder_commitment =
            <TestBlockPayload as BlockPayload<TestTypes>>::builder_commitment(
                &block_payload,
                &metadata,
            );

        // generate key for leader of this round
        let seed = [self.round.u64() as u8; 32];
        let (pub_key, private_key) = BLSPubKey::generated_from_seed_indexed(seed, self.round.u64());

        let quorum_signature =
            <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey::sign(
                &private_key,
                block_vid_commitment.as_ref(),
            )
            .expect("Failed to sign payload commitment while preparing Quorum proposal");
        let da_signature =
            <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey::sign(
                &private_key,
                Sha256::digest(&encoded_transactions).as_ref(),
            )
            .expect("Failed to sign payload commitment while preparing DA proposal");

        let da_proposal = DaProposal {
            encoded_transactions: encoded_transactions.into(),
            metadata,
            view_number: self.round,
        };

        let block_header = TestBlockHeader {
            block_number: self.round.u64(),
            payload_commitment: block_vid_commitment,
            builder_commitment: block_builder_commitment,
            timestamp: self.round.u64(),
            metadata,
            random: 1, // arbitrary
        };

        let justify_qc = match self.previous_quorum_proposal.as_ref() {
            None => {
                QuorumCertificate::<TestTypes>::genesis::<TestVersions>(
                    &TestValidatedState::default(),
                    &TestInstanceState::default(),
                )
                .await
            }
            Some(prev_proposal) => {
                let prev_justify_qc = &prev_proposal.justify_qc;
                let quorum_data = QuorumData::<TestTypes> {
                    leaf_commit: Committable::commit(&Leaf::from_quorum_proposal(prev_proposal)),
                };

                // form a justify qc
                SimpleCertificate::<TestTypes, QuorumData<TestTypes>, SuccessThreshold>::new(
                    quorum_data.clone(),
                    quorum_data.commit(),
                    prev_proposal.view_number,
                    prev_justify_qc.signatures.clone(),
                    PhantomData,
                )
            }
        };

        tracing::debug!("Iteration: {} justify_qc: {:?}", self.round, justify_qc);

        let quorum_proposal = QuorumProposal::<TestTypes> {
            block_header,
            view_number: self.round,
            justify_qc: justify_qc.clone(),
            upgrade_certificate: None,
            proposal_certificate: None,
        };

        let quorum_proposal_event = EventType::QuorumProposal {
            proposal: Proposal {
                data: quorum_proposal.clone(),
                signature: quorum_signature,
                _pd: PhantomData,
            },
            sender: pub_key,
        };

        let da_proposal_event = EventType::DaProposal {
            proposal: Proposal {
                data: da_proposal,
                signature: da_signature,
                _pd: PhantomData,
            },
            sender: pub_key,
        };

        let builder_state_id = BuilderStateId {
            parent_commitment: block_vid_commitment,
            parent_view: self.round,
        };

        let mut events = vec![quorum_proposal_event, da_proposal_event];
        // Shuffle the events to shake out possible bugs that depend on event ordering
        events.shuffle(&mut thread_rng());

        for evt in events {
            self.event_stream_sender
                .broadcast(Event {
                    view_number: self.round,
                    event: evt,
                })
                .await
                .unwrap();
        }

        // Update own state
        self.round = ViewNumber::new(self.round.u64() + 1);
        self.previous_quorum_proposal = Some(quorum_proposal);

        builder_state_id
    }
}
