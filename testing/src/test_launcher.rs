use std::{collections::HashMap, sync::Arc};

use futures::future::BoxFuture;
use hotshot::traits::{NodeImplementation, TestableNodeImplementation};
use hotshot_task::{
    event_stream::ChannelStream,
    global_registry::{GlobalRegistry, HotShotTaskId},
    task::HotShotTaskCompleted,
    task_launcher::TaskRunner,
};
use hotshot_types::{
    message::Message,
    traits::{
        election::ConsensusExchange,
        network::CommunicationChannel,
        node_implementation::{
            ExchangesType, NodeType, QuorumCommChannel, QuorumEx, QuorumNetwork,
        },
    },
    HotShotConfig,
};

use crate::spinning_task::SpinningTask;

use super::{
    completion_task::CompletionTask, overall_safety_task::OverallSafetyTask,
    test_builder::TestMetadata, test_runner::TestRunner, txn_task::TxnTask, GlobalTestEvent,
};

pub type Networks<TYPES, I> = (
    <<<I as NodeImplementation<TYPES>>::Exchanges as ExchangesType<
        TYPES,
        Message<TYPES, I>,
    >>::QuorumExchange as ConsensusExchange<TYPES, Message<TYPES, I>>>::Networking,
    <<<I as NodeImplementation<TYPES>>::Exchanges as ExchangesType<
        TYPES,
        Message<TYPES, I>,
    >>::CommitteeExchange as ConsensusExchange<TYPES, Message<TYPES, I>>>::Networking,
    <<<I as NodeImplementation<TYPES>>::Exchanges as ExchangesType<
        TYPES,
        Message<TYPES, I>,
    >>::ViewSyncExchange as ConsensusExchange<TYPES, Message<TYPES, I>>>::Networking,
    <<<I as NodeImplementation<TYPES>>::Exchanges as ExchangesType<
        TYPES,
        Message<TYPES, I>,
    >>::VIDExchange as ConsensusExchange<TYPES, Message<TYPES, I>>>::Networking,
);

/// Wrapper for a function that takes a `node_id` and returns an instance of `T`.
pub type Generator<T> = Box<dyn Fn(u64) -> T + 'static>;

/// Wrapper Type for quorum function that takes a `ConnectedNetwork` and returns a `CommunicationChannel`
pub type QuorumNetworkGenerator<TYPES, I, T> =
    Box<dyn Fn(Arc<QuorumNetwork<TYPES, I>>) -> T + 'static>;

/// Wrapper Type for committee function that takes a `ConnectedNetwork` and returns a `CommunicationChannel`
pub type CommitteeNetworkGenerator<N, T> = Box<dyn Fn(Arc<N>) -> T + 'static>;

pub type ViewSyncNetworkGenerator<N, T> = Box<dyn Fn(Arc<N>) -> T + 'static>;

/// Wrapper type for a task generator.
pub type TaskGenerator<TASK> = Box<
    dyn FnOnce(
        TASK,
        GlobalRegistry,
        ChannelStream<GlobalTestEvent>,
    )
        -> BoxFuture<'static, (HotShotTaskId, BoxFuture<'static, HotShotTaskCompleted>)>,
>;

/// Wrapper type for a hook.
pub type Hook = Box<
    dyn FnOnce(
        GlobalRegistry,
        ChannelStream<GlobalTestEvent>,
    )
        -> BoxFuture<'static, (HotShotTaskId, BoxFuture<'static, HotShotTaskCompleted>)>,
>;

/// generators for resources used by each node
pub struct ResourceGenerators<TYPES: NodeType, I: TestableNodeImplementation<TYPES>>
where
    QuorumCommChannel<TYPES, I>: CommunicationChannel<
        TYPES,
        Message<TYPES, I>,
        <QuorumEx<TYPES, I> as ConsensusExchange<TYPES, Message<TYPES, I>>>::Membership,
    >,
{
    // generate channels
    pub channel_generator: Generator<Networks<TYPES, I>>,
    /// generate a new storage for each node
    pub storage: Generator<<I as NodeImplementation<TYPES>>::Storage>,
    /// configuration used to generate each hotshot node
    pub config: HotShotConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
}

/// test launcher
pub struct TestLauncher<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// generator for resources
    pub resource_generator: ResourceGenerators<TYPES, I>,
    /// metadasta used for tasks
    pub metadata: TestMetadata,
    /// overrideable txn task generator function
    pub txn_task_generator: TaskGenerator<TxnTask<TYPES, I>>,
    /// overrideable timeout task generator function
    pub completion_task_generator: TaskGenerator<CompletionTask<TYPES, I>>,
    /// overall safety task generator
    pub overall_safety_task_generator: TaskGenerator<OverallSafetyTask<TYPES, I>>,

    pub spinning_task_generator: TaskGenerator<SpinningTask<TYPES, I>>,

    pub hooks: Vec<Hook>,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TestLauncher<TYPES, I> {
    /// launch the test
    pub fn launch(self) -> TestRunner<TYPES, I> {
        TestRunner {
            launcher: self,
            nodes: Vec::new(),
            late_start: HashMap::new(),
            next_node_id: 0,
            task_runner: TaskRunner::default(),
        }
    }

    /// override the safety task generator
    pub fn with_overall_safety_task_generator(
        self,
        overall_safety_task_generator: TaskGenerator<OverallSafetyTask<TYPES, I>>,
    ) -> Self {
        Self {
            overall_safety_task_generator,
            ..self
        }
    }

    /// override the safety task generator
    pub fn with_spinning_task_generator(
        self,
        spinning_task_generator: TaskGenerator<SpinningTask<TYPES, I>>,
    ) -> Self {
        Self {
            spinning_task_generator,
            ..self
        }
    }

    /// overridde the completion task generator
    pub fn with_completion_task_generator(
        self,
        completion_task_generator: TaskGenerator<CompletionTask<TYPES, I>>,
    ) -> Self {
        Self {
            completion_task_generator,
            ..self
        }
    }

    /// override the txn task generator
    pub fn with_txn_task_generator(
        self,
        txn_task_generator: TaskGenerator<TxnTask<TYPES, I>>,
    ) -> Self {
        Self {
            txn_task_generator,
            ..self
        }
    }

    /// override resource generators
    pub fn with_resource_generator(self, resource_generator: ResourceGenerators<TYPES, I>) -> Self {
        Self {
            resource_generator,
            ..self
        }
    }

    /// add a hook
    pub fn add_hook(mut self, hook: Hook) -> Self {
        self.hooks.push(hook);
        self
    }

    /// overwrite hooks with more hooks
    pub fn with_hooks(self, hooks: Vec<Hook>) -> Self {
        Self { hooks, ..self }
    }

    /// Modifies the config used when generating nodes with `f`
    pub fn modify_default_config(
        mut self,
        mut f: impl FnMut(&mut HotShotConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>),
    ) -> Self {
        f(&mut self.resource_generator.config);
        self
    }
}
