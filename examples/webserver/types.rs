use std::fmt::Debug;

use hotshot::traits::implementations::WebServerNetwork;
use hotshot_example_types::{state_types::TestTypes, storage_types::TestStorage};
use hotshot_types::{constants::WebServerVersion, traits::node_implementation::NodeImplementation};
use serde::{Deserialize, Serialize};

use crate::infra::WebServerDARun;

/// dummy struct so we can choose types
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

/// convenience type alias
pub type DANetwork = WebServerNetwork<TestTypes, WebServerVersion>;
/// convenience type alias
pub type VIDNetwork = WebServerNetwork<TestTypes, WebServerVersion>;
/// convenience type alias
pub type QuorumNetwork = WebServerNetwork<TestTypes, WebServerVersion>;
/// convenience type alias
pub type ViewSyncNetwork = WebServerNetwork<TestTypes, WebServerVersion>;

impl NodeImplementation<TestTypes> for NodeImpl {
    type CommitteeNetwork = DANetwork;
    type QuorumNetwork = QuorumNetwork;
    type Storage = TestStorage<TestTypes>;
}
/// convenience type alias
pub type ThisRun = WebServerDARun<TestTypes, WebServerVersion>;
