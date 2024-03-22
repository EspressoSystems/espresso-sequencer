use hotshot_types::event::Event;
use hotshot_types::traits::node_implementation::NodeType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = ""))]
pub struct EventInfo<Types: NodeType> {
    pub event: Event<Types>,
}
