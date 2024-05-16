#![allow(clippy::panic)]
use std::{collections::BTreeSet, sync::Arc};

use async_compatibility_layer::logging::setup_logging;
use hotshot::{
    traits::{
        election::static_committee::GeneralStaticCommittee,
        implementations::{MasterMap, MemoryNetwork, NetworkingMetricsValue},
        NodeImplementation,
    },
    types::SignatureKey,
};
use hotshot_example_types::{
    block_types::{TestBlockHeader, TestBlockPayload, TestTransaction},
    state_types::{TestInstanceState, TestValidatedState},
    storage_types::TestStorage,
};
use hotshot_types::{
    constants::STATIC_VER_0_1,
    data::ViewNumber,
    message::{DataMessage, Message, MessageKind},
    signature_key::{BLSPubKey, BuilderKey},
    traits::{
        network::{ConnectedNetwork, TestableNetworkingImplementation},
        node_implementation::{ConsensusTime, NodeType},
    },
};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use tracing::{instrument, trace};

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
    type BlockHeader = TestBlockHeader;
    type BlockPayload = TestBlockPayload;
    type SignatureKey = BLSPubKey;
    type Transaction = TestTransaction;
    type ValidatedState = TestValidatedState;
    type InstanceState = TestInstanceState;
    type Membership = GeneralStaticCommittee<Test, Self::SignatureKey>;
    type BuilderSignatureKey = BuilderKey;
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct TestImpl {}

pub type DaNetwork = MemoryNetwork<Message<Test>, <Test as NodeType>::SignatureKey>;
pub type QuorumNetwork = MemoryNetwork<Message<Test>, <Test as NodeType>::SignatureKey>;

impl NodeImplementation<Test> for TestImpl {
    type QuorumNetwork = QuorumNetwork;
    type DaNetwork = DaNetwork;
    type Storage = TestStorage<Test>;
}

/// fake Eq
/// we can't compare the votetokentype for equality, so we can't
/// derive EQ on `VoteType<TYPES>` and thereby message
/// we are only sending data messages, though so we compare key and
/// data message
fn fake_message_eq(message_1: Message<Test>, message_2: Message<Test>) {
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
fn pubkey() -> BLSPubKey {
    // random 32 bytes
    let mut bytes = [0; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    BLSPubKey::generated_from_seed_indexed(bytes, 0).0
}

/// create a message
fn gen_messages(num_messages: u64, seed: u64, pk: BLSPubKey) -> Vec<Message<Test>> {
    let mut messages = Vec::new();
    for _ in 0..num_messages {
        // create a random transaction from seed
        let mut bytes = [0u8; 8];
        let mut rng = StdRng::seed_from_u64(seed);
        rng.fill_bytes(&mut bytes);

        let message = Message {
            sender: pk,
            kind: MessageKind::Data(DataMessage::SubmitTransaction(
                TestTransaction::new(bytes.to_vec()),
                <ViewNumber as ConsensusTime>::new(0),
            )),
        };
        messages.push(message);
    }
    messages
}

// Spawning a single MemoryNetwork should produce no errors
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn memory_network_spawn_single() {
    setup_logging();
    let group: Arc<MasterMap<Message<Test>, <Test as NodeType>::SignatureKey>> = MasterMap::new();
    trace!(?group);
    let _pub_key = pubkey();
}

// // Spawning a two MemoryNetworks and connecting them should produce no errors
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn memory_network_spawn_double() {
    setup_logging();
    let group: Arc<MasterMap<Message<Test>, <Test as NodeType>::SignatureKey>> = MasterMap::new();
    trace!(?group);
    let _pub_key_1 = pubkey();
    let _pub_key_2 = pubkey();
}

// Check to make sure direct queue works
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn memory_network_direct_queue() {
    setup_logging();
    // Create some dummy messages

    // Make and connect the networking instances
    let group: Arc<MasterMap<Message<Test>, <Test as NodeType>::SignatureKey>> = MasterMap::new();
    trace!(?group);

    let pub_key_1 = pubkey();
    let network1 = MemoryNetwork::new(
        pub_key_1,
        NetworkingMetricsValue::default(),
        group.clone(),
        Option::None,
    );

    let pub_key_2 = pubkey();
    let network2 = MemoryNetwork::new(
        pub_key_2,
        NetworkingMetricsValue::default(),
        group,
        Option::None,
    );

    let first_messages: Vec<Message<Test>> = gen_messages(5, 100, pub_key_1);

    // Test 1 -> 2
    // Send messages
    for sent_message in first_messages {
        network1
            .direct_message(sent_message.clone(), pub_key_2, STATIC_VER_0_1)
            .await
            .expect("Failed to message node");
        let mut recv_messages = network2
            .recv_msgs()
            .await
            .expect("Failed to receive message");
        let recv_message = recv_messages.pop().unwrap();
        assert!(recv_messages.is_empty());
        fake_message_eq(sent_message, recv_message);
    }

    let second_messages: Vec<Message<Test>> = gen_messages(5, 200, pub_key_2);

    // Test 2 -> 1
    // Send messages
    for sent_message in second_messages {
        network2
            .direct_message(sent_message.clone(), pub_key_1, STATIC_VER_0_1)
            .await
            .expect("Failed to message node");
        let mut recv_messages = network1
            .recv_msgs()
            .await
            .expect("Failed to receive message");
        let recv_message = recv_messages.pop().unwrap();
        assert!(recv_messages.is_empty());
        fake_message_eq(sent_message, recv_message);
    }
}

// Check to make sure direct queue works
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
async fn memory_network_broadcast_queue() {
    setup_logging();
    // Make and connect the networking instances
    let group: Arc<MasterMap<Message<Test>, <Test as NodeType>::SignatureKey>> = MasterMap::new();
    trace!(?group);
    let pub_key_1 = pubkey();
    let network1 = MemoryNetwork::new(
        pub_key_1,
        NetworkingMetricsValue::default(),
        group.clone(),
        Option::None,
    );
    let pub_key_2 = pubkey();
    let network2 = MemoryNetwork::new(
        pub_key_2,
        NetworkingMetricsValue::default(),
        group,
        Option::None,
    );

    let first_messages: Vec<Message<Test>> = gen_messages(5, 100, pub_key_1);

    // Test 1 -> 2
    // Send messages
    for sent_message in first_messages {
        network1
            .broadcast_message(
                sent_message.clone(),
                vec![pub_key_2].into_iter().collect::<BTreeSet<_>>(),
                STATIC_VER_0_1,
            )
            .await
            .expect("Failed to message node");
        let mut recv_messages = network2
            .recv_msgs()
            .await
            .expect("Failed to receive message");
        let recv_message = recv_messages.pop().unwrap();
        assert!(recv_messages.is_empty());
        fake_message_eq(sent_message, recv_message);
    }

    let second_messages: Vec<Message<Test>> = gen_messages(5, 200, pub_key_2);

    // Test 2 -> 1
    // Send messages
    for sent_message in second_messages {
        network2
            .broadcast_message(
                sent_message.clone(),
                vec![pub_key_1].into_iter().collect::<BTreeSet<_>>(),
                STATIC_VER_0_1,
            )
            .await
            .expect("Failed to message node");
        let mut recv_messages = network1
            .recv_msgs()
            .await
            .expect("Failed to receive message");
        let recv_message = recv_messages.pop().unwrap();
        assert!(recv_messages.is_empty());
        fake_message_eq(sent_message, recv_message);
    }
}

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[instrument]
#[allow(deprecated)]
async fn memory_network_test_in_flight_message_count() {
    setup_logging();

    let group: Arc<MasterMap<Message<Test>, <Test as NodeType>::SignatureKey>> = MasterMap::new();
    trace!(?group);
    let pub_key_1 = pubkey();
    let network1 = MemoryNetwork::new(
        pub_key_1,
        NetworkingMetricsValue::default(),
        group.clone(),
        Option::None,
    );
    let pub_key_2 = pubkey();
    let network2 = MemoryNetwork::new(
        pub_key_2,
        NetworkingMetricsValue::default(),
        group,
        Option::None,
    );

    // Create some dummy messages
    let messages: Vec<Message<Test>> = gen_messages(5, 100, pub_key_1);
    let broadcast_recipients = BTreeSet::from([pub_key_1, pub_key_2]);

    assert_eq!(network1.in_flight_message_count(), Some(0));
    assert_eq!(network2.in_flight_message_count(), Some(0));

    for (count, message) in messages.iter().enumerate() {
        network1
            .direct_message(message.clone(), pub_key_2, STATIC_VER_0_1)
            .await
            .unwrap();
        // network 2 has received `count` broadcast messages and `count + 1` direct messages
        assert_eq!(network2.in_flight_message_count(), Some(count + count + 1));

        network2
            .broadcast_message(
                message.clone(),
                broadcast_recipients.clone(),
                STATIC_VER_0_1,
            )
            .await
            .unwrap();
        // network 1 has received `count` broadcast messages
        assert_eq!(network1.in_flight_message_count(), Some(count + 1));

        // network 2 has received `count + 1` broadcast messages and `count + 1` direct messages
        assert_eq!(network2.in_flight_message_count(), Some((count + 1) * 2));
    }

    while network1.in_flight_message_count().unwrap() > 0 {
        network1.recv_msgs().await.unwrap();
    }

    while network2.in_flight_message_count().unwrap() > messages.len() {
        network2.recv_msgs().await.unwrap();
    }

    while network2.in_flight_message_count().unwrap() > 0 {
        network2.recv_msgs().await.unwrap();
    }

    assert_eq!(network1.in_flight_message_count(), Some(0));
    assert_eq!(network2.in_flight_message_count(), Some(0));
}
