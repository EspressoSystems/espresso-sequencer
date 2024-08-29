use std::hash::Hash;

use crate::{
    builder_state::{BuilderState, MessageType},
    service::BroadcastSenders,
};
use async_broadcast::broadcast;
use hotshot::{traits::election::static_committee::GeneralStaticCommittee, types::BLSPubKey};
use hotshot_types::{
    data::{Leaf, ViewNumber},
    signature_key::BuilderKey,
    traits::{
        block_contents::vid_commitment,
        node_implementation::{ConsensusTime, NodeType},
    },
    utils::BuilderCommitment,
};

use hotshot_example_types::{
    auction_results_provider_types::TestAuctionResult,
    block_types::{TestBlockHeader, TestBlockPayload, TestTransaction},
    state_types::{TestInstanceState, TestValidatedState},
};
use serde::{Deserialize, Serialize};

use crate::builder_state::BuiltFromProposedBlock;
use crate::service::{broadcast_channels, GlobalState};
use async_lock::RwLock;
use committable::{Commitment, CommitmentBoundsArkless};
use std::sync::Arc;
use std::time::Duration;
pub mod basic_test;

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
