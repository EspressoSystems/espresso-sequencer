// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! elections used for consensus

/// static (round robin) committee election
pub mod static_committee;
/// static (round robin leader for 2 consecutive views) committee election
pub mod static_committee_leader_two_views;
