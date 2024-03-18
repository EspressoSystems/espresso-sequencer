use crate::infra::WebServerDARun;
use hotshot::traits::implementations::WebServerNetwork;
use hotshot_example_types::state_types::TestTypes;
use hotshot_types::traits::node_implementation::NodeImplementation;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// dummy struct so we can choose types
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NodeImpl {}

/// convenience type alias
pub type DANetwork = WebServerNetwork<TestTypes>;
/// convenience type alias
pub type VIDNetwork = WebServerNetwork<TestTypes>;
/// convenience type alias
pub type QuorumNetwork = WebServerNetwork<TestTypes>;
/// convenience type alias
pub type ViewSyncNetwork = WebServerNetwork<TestTypes>;

impl NodeImplementation<TestTypes> for NodeImpl {
    type CommitteeNetwork = DANetwork;
    type QuorumNetwork = QuorumNetwork;
}
/// convenience type alias
pub type ThisRun = WebServerDARun<TestTypes>;
