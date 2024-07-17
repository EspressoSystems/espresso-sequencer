use hotshot_types::traits::node_implementation::NodeType;
use serde::{Deserialize, Serialize};

use super::block_info::AvailableBlockInfo;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound = "")]
pub struct AvailableBlocksQueryData<I: NodeType> {
    pub blocks: Vec<AvailableBlockInfo<I>>,
}
