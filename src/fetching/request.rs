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

//! Requests for fetching resources.

use crate::{availability::LeafQueryData, Payload};
use derive_more::{From, Into};
use hotshot_types::{data::VidCommitment, traits::node_implementation::NodeType};

use std::fmt::Debug;
use std::hash::Hash;

/// A request for a resource.
pub trait Request<Types>: Copy + Debug + Eq + Hash + Send {
    /// The type of resource that will be returned as a successful response to this request.
    type Response: Clone;
}

/// A request for a payload with a given commitment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PayloadRequest(pub VidCommitment);

impl<Types: NodeType> Request<Types> for PayloadRequest {
    type Response = Payload<Types>;
}

/// A request for a leaf with a given height.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, From, Into)]
pub struct LeafRequest(usize);

impl<Types: NodeType> Request<Types> for LeafRequest {
    type Response = LeafQueryData<Types>;
}
