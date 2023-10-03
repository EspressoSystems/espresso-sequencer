use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::sync::Arc;

use async_compatibility_layer::logging::setup_logging;
use hotshot::demo::SDemoState;
use hotshot::traits::election::static_committee::{
    GeneralStaticCommittee, StaticElectionConfig, StaticVoteToken,
};
use hotshot::traits::implementations::{
    MasterMap, MemoryCommChannel, MemoryNetwork, MemoryStorage,
};
use hotshot::traits::NodeImplementation;
use hotshot::types::bn254::{BLSPrivKey, BLSPubKey};
use hotshot::types::SignatureKey;
use hotshot_types::block_impl::{VIDBlockPayload, VIDTransaction};
use hotshot_types::certificate::ViewSyncCertificate;
use hotshot_types::data::{DAProposal, QuorumProposal, SequencingLeaf};
use hotshot_types::message::{Message, SequencingMessage};
use hotshot_types::traits::election::{CommitteeExchange, QuorumExchange, ViewSyncExchange};
use hotshot_types::traits::metrics::NoMetrics;
use hotshot_types::traits::network::TestableNetworkingImplementation;
use hotshot_types::traits::network::{ConnectedNetwork, TransmitType};
use hotshot_types::traits::node_implementation::{ChannelMaps, NodeType, SequencingExchanges};
use hotshot_types::vote::{DAVote, ViewSyncVote};
use hotshot_types::{
    data::ViewNumber,
    message::{DataMessage, MessageKind},
    traits::state::ConsensusTime,
    vote::QuorumVote,
};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use tracing::trace;

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Test;

impl NodeType for Test {
    type Time = ViewNumber;
    type BlockType = VIDBlockPayload;
    type SignatureKey = BLSPubKey;
    type VoteTokenType = StaticVoteToken<Self::SignatureKey>;
    type Transaction = VIDTransaction;
    type ElectionConfigType = StaticElectionConfig;
    type StateType = SDemoState;
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct TestImpl {}

pub type ThisLeaf = SequencingLeaf<Test>;
pub type ThisMembership = GeneralStaticCommittee<Test, ThisLeaf, <Test as NodeType>::SignatureKey>;
pub type DANetwork = MemoryCommChannel<Test, TestImpl, ThisMembership>;
pub type QuorumNetwork = MemoryCommChannel<Test, TestImpl, ThisMembership>;
pub type ViewSyncNetwork = MemoryCommChannel<Test, TestImpl, ThisMembership>;

pub type ThisDAProposal = DAProposal<Test>;
pub type ThisDAVote = DAVote<Test>;

pub type ThisQuorumProposal = QuorumProposal<Test, ThisLeaf>;
pub type ThisQuorumVote = QuorumVote<Test, ThisLeaf>;

pub type ThisViewSyncProposal = ViewSyncCertificate<Test>;
pub type ThisViewSyncVote = ViewSyncVote<Test>;

impl NodeImplementation<Test> for TestImpl {
    type Storage = MemoryStorage<Test, Self::Leaf>;
    type Leaf = SequencingLeaf<Test>;
    type Exchanges = SequencingExchanges<
        Test,
        Message<Test, Self>,
        QuorumExchange<
            Test,
            Self::Leaf,
            ThisQuorumProposal,
            ThisMembership,
            QuorumNetwork,
            Message<Test, Self>,
        >,
        CommitteeExchange<Test, ThisMembership, DANetwork, Message<Test, Self>>,
        ViewSyncExchange<
            Test,
            ThisViewSyncProposal,
            ThisMembership,
            ViewSyncNetwork,
            Message<Test, Self>,
        >,
    >;
    type ConsensusMessage = SequencingMessage<Test, Self>;

    fn new_channel_maps(
        start_view: <Test as NodeType>::Time,
    ) -> (ChannelMaps<Test, Self>, Option<ChannelMaps<Test, Self>>) {
        (ChannelMaps::new(start_view), None)
    }
}

/// fake Eq
/// we can't compare the votetokentype for equality, so we can't
/// derive EQ on `VoteType<TYPES>` and thereby message
/// we are only sending data messages, though so we compare key and
/// data message
fn fake_message_eq(message_1: Message<Test, TestImpl>, message_2: Message<Test, TestImpl>) {
    assert_eq!(message_1.sender, message_2.sender);
    if let MessageKind::Data(DataMessage::SubmitTransaction(d_1, _)) = message_1.kind {
        if let MessageKind::Data(DataMessage::SubmitTransaction(d_2, _)) = message_2.kind {
            assert_eq!(d_1, d_2);
        }
    } else {
        panic!("Got unexpected message type in memory test!");
    }
}

#[instrument]
fn get_pubkey() -> BLSPubKey {
    // random 32 bytes
    let mut bytes = [0; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    BLSPubKey::from_private(&BLSPrivKey::generate_from_seed(bytes))
}

/// create a message
fn gen_messages(num_messages: u64, seed: u64, pk: BLSPubKey) -> Vec<Message<Test, TestImpl>> {
    let mut messages = Vec::new();
    for _ in 0..num_messages {
        // create a random transaction from seed
        let mut bytes = [0u8; 8];
        let mut rng = StdRng::seed_from_u64(seed);
        rng.fill_bytes(&mut bytes);

        let message = Message {
            sender: pk,
            kind: MessageKind::Data(DataMessage::SubmitTransaction(
                VIDTransaction(bytes.to_vec()),
                <ViewNumber as ConsensusTime>::new(0),
            )),
            _phantom: PhantomData,
        };
        messages.push(message);
    }
    messages
}

// Spawning a single MemoryNetwork should produce no errors
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn memory_network_spawn_single() {
    setup_logging();
    let group: Arc<MasterMap<Message<Test, TestImpl>, <Test as NodeType>::SignatureKey>> =
        MasterMap::new();
    trace!(?group);
    let pub_key = get_pubkey();
    let _network = MemoryNetwork::new(pub_key, NoMetrics::boxed(), group, Option::None);
}

// // Spawning a two MemoryNetworks and connecting them should produce no errors
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn memory_network_spawn_double() {
    setup_logging();
    let group: Arc<MasterMap<Message<Test, TestImpl>, <Test as NodeType>::SignatureKey>> =
        MasterMap::new();
    trace!(?group);
    let pub_key_1 = get_pubkey();
    let _network_1 = MemoryNetwork::new(pub_key_1, NoMetrics::boxed(), group.clone(), Option::None);
    let pub_key_2 = get_pubkey();
    let _network_2 = MemoryNetwork::new(pub_key_2, NoMetrics::boxed(), group, Option::None);
}

// Check to make sure direct queue works
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn memory_network_direct_queue() {
    setup_logging();
    // Create some dummy messages

    // Make and connect the networking instances
    let group: Arc<MasterMap<Message<Test, TestImpl>, <Test as NodeType>::SignatureKey>> =
        MasterMap::new();
    trace!(?group);

    let pub_key_1 = get_pubkey();
    let network1 = MemoryNetwork::new(pub_key_1, NoMetrics::boxed(), group.clone(), Option::None);

    let pub_key_2 = get_pubkey();
    let network2 = MemoryNetwork::new(pub_key_2, NoMetrics::boxed(), group, Option::None);

    let first_messages: Vec<Message<Test, TestImpl>> = gen_messages(5, 100, pub_key_1);

    // Test 1 -> 2
    // Send messages
    for sent_message in first_messages {
        network1
            .direct_message(sent_message.clone(), pub_key_2)
            .await
            .expect("Failed to message node");
        let mut recv_messages = network2
            .recv_msgs(TransmitType::Direct)
            .await
            .expect("Failed to receive message");
        let recv_message = recv_messages.pop().unwrap();
        assert!(recv_messages.is_empty());
        fake_message_eq(sent_message, recv_message);
    }

    let second_messages: Vec<Message<Test, TestImpl>> = gen_messages(5, 200, pub_key_2);

    // Test 2 -> 1
    // Send messages
    for sent_message in second_messages {
        network2
            .direct_message(sent_message.clone(), pub_key_1)
            .await
            .expect("Failed to message node");
        let mut recv_messages = network1
            .recv_msgs(TransmitType::Direct)
            .await
            .expect("Failed to receive message");
        let recv_message = recv_messages.pop().unwrap();
        assert!(recv_messages.is_empty());
        fake_message_eq(sent_message, recv_message);
    }
}

// Check to make sure direct queue works
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn memory_network_broadcast_queue() {
    setup_logging();
    // Make and connect the networking instances
    let group: Arc<MasterMap<Message<Test, TestImpl>, <Test as NodeType>::SignatureKey>> =
        MasterMap::new();
    trace!(?group);
    let pub_key_1 = get_pubkey();
    let network1 = MemoryNetwork::new(pub_key_1, NoMetrics::boxed(), group.clone(), Option::None);
    let pub_key_2 = get_pubkey();
    let network2 = MemoryNetwork::new(pub_key_2, NoMetrics::boxed(), group, Option::None);

    let first_messages: Vec<Message<Test, TestImpl>> = gen_messages(5, 100, pub_key_1);

    // Test 1 -> 2
    // Send messages
    for sent_message in first_messages {
        network1
            .broadcast_message(
                sent_message.clone(),
                vec![pub_key_2].into_iter().collect::<BTreeSet<_>>(),
            )
            .await
            .expect("Failed to message node");
        let mut recv_messages = network2
            .recv_msgs(TransmitType::Broadcast)
            .await
            .expect("Failed to receive message");
        let recv_message = recv_messages.pop().unwrap();
        assert!(recv_messages.is_empty());
        fake_message_eq(sent_message, recv_message);
    }

    let second_messages: Vec<Message<Test, TestImpl>> = gen_messages(5, 200, pub_key_2);

    // Test 2 -> 1
    // Send messages
    for sent_message in second_messages {
        network2
            .broadcast_message(
                sent_message.clone(),
                vec![pub_key_1].into_iter().collect::<BTreeSet<_>>(),
            )
            .await
            .expect("Failed to message node");
        let mut recv_messages = network1
            .recv_msgs(TransmitType::Broadcast)
            .await
            .expect("Failed to receive message");
        let recv_message = recv_messages.pop().unwrap();
        assert!(recv_messages.is_empty());
        fake_message_eq(sent_message, recv_message);
    }
}

#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
#[allow(deprecated)]
async fn memory_network_test_in_flight_message_count() {
    setup_logging();

    let group: Arc<MasterMap<Message<Test, TestImpl>, <Test as NodeType>::SignatureKey>> =
        MasterMap::new();
    trace!(?group);
    let pub_key_1 = get_pubkey();
    let network1 = MemoryNetwork::new(pub_key_1, NoMetrics::boxed(), group.clone(), Option::None);
    let pub_key_2 = get_pubkey();
    let network2 = MemoryNetwork::new(pub_key_2, NoMetrics::boxed(), group, Option::None);

    // Create some dummy messages
    let messages: Vec<Message<Test, TestImpl>> = gen_messages(5, 100, pub_key_1);
    let broadcast_recipients = BTreeSet::from([pub_key_1, pub_key_2]);

    assert_eq!(network1.in_flight_message_count(), Some(0));
    assert_eq!(network2.in_flight_message_count(), Some(0));

    for (count, message) in messages.iter().enumerate() {
        network1
            .direct_message(message.clone(), pub_key_2)
            .await
            .unwrap();
        // network 2 has received `count` broadcast messages and `count + 1` direct messages
        assert_eq!(network2.in_flight_message_count(), Some(count + count + 1));

        network2
            .broadcast_message(message.clone(), broadcast_recipients.clone())
            .await
            .unwrap();
        // network 1 has received `count` broadcast messages
        assert_eq!(network1.in_flight_message_count(), Some(count + 1));

        // network 2 has received `count + 1` broadcast messages and `count + 1` direct messages
        assert_eq!(network2.in_flight_message_count(), Some((count + 1) * 2));
    }

    while network1.in_flight_message_count().unwrap() > 0 {
        network1.recv_msgs(TransmitType::Broadcast).await.unwrap();
    }

    while network2.in_flight_message_count().unwrap() > 0 {
        network2.recv_msgs(TransmitType::Broadcast).await.unwrap();
        network2.recv_msgs(TransmitType::Direct).await.unwrap();
    }

    assert_eq!(network1.in_flight_message_count(), Some(0));
    assert_eq!(network2.in_flight_message_count(), Some(0));
}
