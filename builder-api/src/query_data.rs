// Copyright (c) 2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot HotShot Builder Protocol.
//
// TODO: License

use hotshot_types::traits::node_implementation::NodeType;
use serde::{Deserialize, Serialize};

use crate::block_info::AvailableBlockInfo;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(bound = "")]
pub struct AvailableBlocksQueryData<I: NodeType> {
    pub blocks: Vec<AvailableBlockInfo<I>>,
}
