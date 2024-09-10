// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{sync::Arc, time::Duration};

use async_broadcast::Sender;
use async_compatibility_layer::art::async_timeout;
use async_lock::RwLock;
use hotshot::traits::implementations::MemoryNetwork;
use hotshot_example_types::node_types::{MemoryImpl, TestTypes, TestVersions};
use hotshot_task::task::{ConsensusTaskRegistry, Task};
use hotshot_task_impls::{
    events::HotShotEvent,
    network::{self, NetworkEventTaskState},
};
use hotshot_testing::{
    helpers::build_system_handle, test_builder::TestDescription,
    test_task::add_network_message_test_task, view_generator::TestViewGenerator,
};
use hotshot_types::{
    data::ViewNumber,
    message::UpgradeLock,
    traits::{
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
    },
};
// Test that the event task sends a message, and the message task receives it
// and emits the proper event
#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
#[allow(clippy::too_many_lines)]
async fn test_network_task() {
    use futures::StreamExt;
    use hotshot_types::traits::network::Topic;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let builder: TestDescription<TestTypes, MemoryImpl, TestVersions> =
        TestDescription::default_multiple_rounds();
    let upgrade_lock = UpgradeLock::<TestTypes, TestVersions>::new();
    let node_id = 1;
    let handle = build_system_handle::<TestTypes, MemoryImpl, TestVersions>(node_id)
        .await
        .0;
    let launcher = builder.gen_launcher(node_id);

    let network = (launcher.resource_generator.channel_generator)(node_id).await;

    let storage = Arc::new(RwLock::new((launcher.resource_generator.storage)(node_id)));
    let consensus = handle.hotshot.consensus();
    let config = launcher.resource_generator.config.clone();
    let public_key = config.my_own_validator_config.public_key;

    let all_nodes = config.known_nodes_with_stake.clone();

    let membership = <TestTypes as NodeType>::Membership::new(
        all_nodes.clone(),
        all_nodes,
        Topic::Global,
        #[cfg(feature = "fixed-leader-election")]
        config.fixed_leader_for_gpuvid,
    );
    let network_state: NetworkEventTaskState<TestTypes, TestVersions, MemoryNetwork<_>, _> =
        NetworkEventTaskState {
            channel: network.clone(),
            view: ViewNumber::new(0),
            membership: membership.clone(),
            filter: network::quorum_filter,
            upgrade_lock: upgrade_lock.clone(),
            storage,
            consensus,
        };
    let (tx, rx) = async_broadcast::broadcast(10);
    let mut task_reg = ConsensusTaskRegistry::new();

    let task = Task::new(network_state, tx.clone(), rx);
    task_reg.run_task(task);

    let mut generator = TestViewGenerator::generate(membership.clone(), membership);
    let view = generator.next().await.unwrap();

    let (out_tx_internal, mut out_rx_internal) = async_broadcast::broadcast(10);
    let (out_tx_external, _) = async_broadcast::broadcast(10);
    add_network_message_test_task(
        out_tx_internal.clone(),
        out_tx_external.clone(),
        upgrade_lock,
        network.clone(),
    )
    .await;

    tx.broadcast_direct(Arc::new(HotShotEvent::QuorumProposalSend(
        view.quorum_proposal,
        public_key,
    )))
    .await
    .unwrap();
    let res: Arc<HotShotEvent<TestTypes>> =
        async_timeout(Duration::from_millis(100), out_rx_internal.recv_direct())
            .await
            .expect("timed out waiting for response")
            .expect("channel closed");
    assert!(matches!(
        res.as_ref(),
        HotShotEvent::QuorumProposalRecv(_, _)
    ));
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_network_storage_fail() {
    use futures::StreamExt;
    use hotshot_types::traits::network::Topic;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let builder: TestDescription<TestTypes, MemoryImpl, TestVersions> =
        TestDescription::default_multiple_rounds();
    let node_id = 1;
    let handle = build_system_handle::<TestTypes, MemoryImpl, TestVersions>(node_id)
        .await
        .0;
    let launcher = builder.gen_launcher(node_id);

    let network = (launcher.resource_generator.channel_generator)(node_id).await;

    let consensus = handle.hotshot.consensus();
    let storage = Arc::new(RwLock::new((launcher.resource_generator.storage)(node_id)));
    storage.write().await.should_return_err = true;
    let config = launcher.resource_generator.config.clone();
    let public_key = config.my_own_validator_config.public_key;
    let all_nodes = config.known_nodes_with_stake.clone();
    let upgrade_lock = UpgradeLock::<TestTypes, TestVersions>::new();

    let membership = <TestTypes as NodeType>::Membership::new(
        all_nodes.clone(),
        all_nodes,
        Topic::Global,
        #[cfg(feature = "fixed-leader-election")]
        config.fixed_leader_for_gpuvid,
    );
    let network_state: NetworkEventTaskState<TestTypes, TestVersions, MemoryNetwork<_>, _> =
        NetworkEventTaskState {
            channel: network.clone(),
            view: ViewNumber::new(0),
            membership: membership.clone(),
            filter: network::quorum_filter,
            upgrade_lock: upgrade_lock.clone(),
            storage,
            consensus,
        };
    let (tx, rx) = async_broadcast::broadcast(10);
    let mut task_reg = ConsensusTaskRegistry::new();

    let task = Task::new(network_state, tx.clone(), rx);
    task_reg.run_task(task);

    let mut generator = TestViewGenerator::generate(membership.clone(), membership);
    let view = generator.next().await.unwrap();

    let (out_tx_internal, mut out_rx_internal): (Sender<Arc<HotShotEvent<TestTypes>>>, _) =
        async_broadcast::broadcast(10);
    let (out_tx_external, _) = async_broadcast::broadcast(10);
    add_network_message_test_task(
        out_tx_internal.clone(),
        out_tx_external.clone(),
        upgrade_lock,
        network.clone(),
    )
    .await;

    tx.broadcast_direct(Arc::new(HotShotEvent::QuorumProposalSend(
        view.quorum_proposal,
        public_key,
    )))
    .await
    .unwrap();
    let res = async_timeout(Duration::from_millis(100), out_rx_internal.recv_direct()).await;
    assert!(res.is_err());
}
