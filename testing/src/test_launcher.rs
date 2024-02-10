use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use hotshot::traits::{NodeImplementation, TestableNodeImplementation};
use hotshot_types::{
    traits::{network::CommunicationChannel, node_implementation::NodeType},
    HotShotConfig,
};

use super::{test_builder::TestMetadata, test_runner::TestRunner};

/// convience type alias for the networks available
pub type Networks<TYPES, I> = (
    <I as NodeImplementation<TYPES>>::QuorumNetwork,
    <I as NodeImplementation<TYPES>>::QuorumNetwork,
);

/// Wrapper for a function that takes a `node_id` and returns an instance of `T`.
pub type Generator<T> = Box<dyn Fn(u64) -> T + 'static>;

/// Wrapper Type for committee function that takes a `ConnectedNetwork` and returns a `CommunicationChannel`
pub type CommitteeNetworkGenerator<N, T> = Box<dyn Fn(Arc<N>) -> T + 'static>;

/// Wrapper Type for view sync function that takes a `ConnectedNetwork` and returns a `CommunicationChannel`
pub type ViewSyncNetworkGenerator<N, T> = Box<dyn Fn(Arc<N>) -> T + 'static>;

/// generators for resources used by each node
pub struct ResourceGenerators<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// generate channels
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
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TestLauncher<TYPES, I> {
    /// launch the test
    #[must_use]
    pub fn launch<N: CommunicationChannel<TYPES>>(self) -> TestRunner<TYPES, I, N> {
        TestRunner::<TYPES, I, N> {
            launcher: self,
            nodes: Vec::new(),
            late_start: HashMap::new(),
            next_node_id: 0,
            _pd: PhantomData,
        }
    }
    /// Modifies the config used when generating nodes with `f`
    #[must_use]
    pub fn modify_default_config(
        mut self,
        mut f: impl FnMut(&mut HotShotConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>),
    ) -> Self {
        f(&mut self.resource_generator.config);
        self
    }
}
