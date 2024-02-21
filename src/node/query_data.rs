// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

use serde::{Deserialize, Serialize};

pub use crate::availability::{BlockId, LeafQueryData};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct SyncStatus {
    pub missing_blocks: usize,
    pub missing_leaves: usize,
    pub missing_vid_common: usize,
    pub missing_vid_shares: usize,
}

impl SyncStatus {
    pub fn fully_synced() -> Self {
        Self {
            missing_blocks: 0,
            missing_leaves: 0,
            missing_vid_common: 0,
            missing_vid_shares: 0,
        }
    }

    pub fn is_fully_synced(&self) -> bool {
        *self == Self::fully_synced()
    }
}
