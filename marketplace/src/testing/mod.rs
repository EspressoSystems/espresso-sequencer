use std::{hash::Hash, marker::PhantomData};

use crate::{
    builder_state::{
        BuilderState, DaProposalMessage, MessageType, QuorumProposalMessage, RequestMessage,
        ResponseMessage,
    },
    service::BroadcastSenders,
    utils::BuilderStateId,
};
use async_broadcast::broadcast;
use async_compatibility_layer::channel::{unbounded, UnboundedReceiver};
use hotshot::{
    traits::{election::static_committee::GeneralStaticCommittee, BlockPayload},
    types::{BLSPubKey, SignatureKey},
};
use hotshot_types::{
    data::{Leaf, QuorumProposal, ViewNumber},
    message::Proposal,
    signature_key::BuilderKey,
    simple_certificate::{QuorumCertificate, SimpleCertificate, SuccessThreshold},
    simple_vote::QuorumData,
    traits::{
        block_contents::vid_commitment,
        node_implementation::{ConsensusTime, NodeType},
    },
    utils::BuilderCommitment,
};

use hotshot_example_types::{
    auction_results_provider_types::TestAuctionResult,
    block_types::{TestBlockHeader, TestBlockPayload, TestMetadata, TestTransaction},
    state_types::{TestInstanceState, TestValidatedState},
};
use serde::{Deserialize, Serialize};

use crate::builder_state::BuiltFromProposedBlock;
use crate::service::{broadcast_channels, GlobalState};
use async_lock::RwLock;
use committable::{Commitment, CommitmentBoundsArkless, Committable};
use std::sync::Arc;
use std::time::Duration;
pub mod basic_test;
pub mod order_test;

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
struct TestTypes;
impl NodeType for TestTypes {
    type Time = ViewNumber;
    type BlockHeader = TestBlockHeader;
    type BlockPayload = TestBlockPayload;
    type SignatureKey = BLSPubKey;
    type Transaction = TestTransaction;
    type ValidatedState = TestValidatedState;
    type InstanceState = TestInstanceState;
    type Membership = GeneralStaticCommittee<TestTypes, Self::SignatureKey>;
    type BuilderSignatureKey = BuilderKey;
    type AuctionResult = TestAuctionResult;
}

/// set up the broadcast channels and instatiate the global state with fixed channel capacity and num nodes
async fn start_builder_state(
    channel_capacity: usize,
    num_storage_nodes: usize,
) -> (
    BroadcastSenders<TestTypes>,
    Arc<RwLock<GlobalState<TestTypes>>>,
) {
    // set up the broadcast channels
    let (bootstrap_sender, bootstrap_receiver) =
        broadcast::<MessageType<TestTypes>>(channel_capacity);
    let (senders, receivers) = broadcast_channels(channel_capacity);

    let genesis_vid_commitment = vid_commitment(&[], num_storage_nodes);
    let genesis_builder_commitment = BuilderCommitment::from_bytes([]);
    let built_from_info = BuiltFromProposedBlock {
        view_number: ViewNumber::genesis(),
        vid_commitment: genesis_vid_commitment,
        leaf_commit: Commitment::<Leaf<TestTypes>>::default_commitment_no_preimage(),
        builder_commitment: genesis_builder_commitment,
    };

    // instantiate the global state
    let global_state = Arc::new(RwLock::new(GlobalState::<TestTypes>::new(
        bootstrap_sender,
        senders.transactions.clone(),
        genesis_vid_commitment,
        ViewNumber::genesis(),
    )));

    // instantiate the bootstrap builder state
    let builder_state = BuilderState::<TestTypes>::new(
        built_from_info,
        &receivers,
        bootstrap_receiver,
        Vec::new(),
        Arc::clone(&global_state),
        Duration::from_millis(10), // max time to wait for non-zero txn block
        0,                         // base fee
        Arc::new(TestInstanceState::default()),
        Duration::from_secs(3600), // duration for txn garbage collection
        Arc::new(TestValidatedState::default()),
    );

    // start the event loop
    builder_state.event_loop();

    (senders, global_state)
}

/// get transactions submitted in previous rounds, [] for genesis
/// and simulate the block built from those
async fn calc_proposal_msg(
    num_storage_nodes: usize,
    round: usize,
    prev_quorum_proposal: Option<QuorumProposal<TestTypes>>,
    transactions: Vec<TestTransaction>,
) -> (
    QuorumProposal<TestTypes>,
    MessageType<TestTypes>,
    MessageType<TestTypes>,
    BuilderStateId<TestTypes>,
) {
    // get transactions submitted in previous rounds, [] for genesis
    // and simulate the block built from those
    let txn_commitments = transactions.iter().map(Committable::commit).collect();
    let encoded_transactions = TestTransaction::encode(&transactions);
    let block_payload = TestBlockPayload { transactions };
    let block_vid_commitment = vid_commitment(&encoded_transactions, num_storage_nodes);
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

    let justify_qc = match prev_quorum_proposal.as_ref() {
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
                leaf_commit: Leaf::from_quorum_proposal(prev_proposal).commit(),
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

    tracing::debug!("Iteration: {} justify_qc: {:?}", round, justify_qc);

    let quorum_proposal = QuorumProposal::<TestTypes> {
        block_header,
        view_number: ViewNumber::new(round as u64),
        justify_qc: justify_qc.clone(),
        upgrade_certificate: None,
        proposal_certificate: None,
    };

    let qc_signature =
        <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey::sign(
            &private_key,
            block_vid_commitment.as_ref(),
        )
        .expect("Failed to sign payload commitment while preparing QC proposal");

    let quorum_proposal_msg =
        MessageType::QuorumProposalMessage(QuorumProposalMessage::<TestTypes> {
            proposal: Arc::new(Proposal {
                data: quorum_proposal.clone(),
                signature: qc_signature,
                _pd: PhantomData,
            }),
            sender: pub_key,
        });

    let da_proposal_msg = MessageType::DaProposalMessage(Arc::clone(&da_proposal));
    let builder_state_id = BuilderStateId {
        parent_commitment: block_vid_commitment,
        parent_view: ViewNumber::new(round as u64),
    };
    (
        quorum_proposal,
        quorum_proposal_msg,
        da_proposal_msg,
        builder_state_id,
    )
}

/// get request message
/// it contains receiver, builder state id ( which helps looking up builder state in global state) and request message in view number and response channel
async fn get_req_msg(
    round: u64,
    builder_state_id: BuilderStateId<TestTypes>,
) -> (
    UnboundedReceiver<ResponseMessage<TestTypes>>,
    BuilderStateId<TestTypes>,
    MessageType<TestTypes>,
) {
    let (response_sender, response_receiver) = unbounded();
    let request_message = MessageType::<TestTypes>::RequestMessage(RequestMessage {
        requested_view_number: ViewNumber::new(round),
        response_channel: response_sender,
    });

    (response_receiver, builder_state_id, request_message)
}
