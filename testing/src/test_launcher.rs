// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use hotshot::{
    traits::{NodeImplementation, TestableNodeImplementation},
    MarketplaceConfig,
};
use hotshot_example_types::storage_types::TestStorage;
use hotshot_types::{
    traits::{
        network::{AsyncGenerator, ConnectedNetwork},
        node_implementation::NodeType,
    },
    HotShotConfig,
};

use super::{test_builder::TestDescription, test_runner::TestRunner};

/// A type alias to help readability
pub type Network<TYPES, I> = Arc<<I as NodeImplementation<TYPES>>::Network>;

/// Wrapper for a function that takes a `node_id` and returns an instance of `T`.
pub type Generator<T> = Box<dyn Fn(u64) -> T + 'static>;

/// generators for resources used by each node
pub struct ResourceGenerators<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// generate channels
    pub channel_generator: AsyncGenerator<Network<TYPES, I>>,
    /// generate new storage for each node
    pub storage: Generator<TestStorage<TYPES>>,
    /// configuration used to generate each hotshot node
    pub config: HotShotConfig<TYPES::SignatureKey>,
    /// generate a new marketplace config for each node
    pub marketplace_config: Generator<MarketplaceConfig<TYPES, I>>,
}

/// test launcher
pub struct TestLauncher<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> {
    /// generator for resources
    pub resource_generator: ResourceGenerators<TYPES, I>,
    /// metadata used for tasks
    pub metadata: TestDescription<TYPES, I>,
}

impl<TYPES: NodeType, I: TestableNodeImplementation<TYPES>> TestLauncher<TYPES, I> {
    /// launch the test
    #[must_use]
    pub fn launch<N: ConnectedNetwork<TYPES::SignatureKey>>(self) -> TestRunner<TYPES, I, N> {
        TestRunner::<TYPES, I, N> {
            launcher: self,
            nodes: Vec::new(),
            solver_server: None,
            late_start: HashMap::new(),
            next_node_id: 0,
            _pd: PhantomData,
        }
    }
    /// Modifies the config used when generating nodes with `f`
    #[must_use]
    pub fn modify_default_config(
        mut self,
        mut f: impl FnMut(&mut HotShotConfig<TYPES::SignatureKey>),
    ) -> Self {
        f(&mut self.resource_generator.config);
        self
    }
}
