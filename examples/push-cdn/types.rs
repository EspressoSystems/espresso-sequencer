use hotshot::traits::{implementations::PushCdnNetwork, NodeImplementation};
use hotshot_example_types::{state_types::TestTypes, storage_types::TestStorage};
use serde::{Deserialize, Serialize};

use crate::infra::PushCdnDaRun;

#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
/// Convenience type alias
pub struct NodeImpl {}

/// Convenience type alias
pub type Network = PushCdnNetwork<TestTypes>;

impl NodeImplementation<TestTypes> for NodeImpl {
    type Network = Network;
    type Storage = TestStorage<TestTypes>;
}

/// Convenience type alias
pub type ThisRun = PushCdnDaRun<TestTypes>;
