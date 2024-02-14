#![cfg(any(test, feature = "testing"))]

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
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use std::time::Duration;

pub mod consensus;
pub mod mocks;

pub async fn sleep(dur: Duration) {
    // For some reason, `async_std::task::sleep` doesn't work on the GitHub Windows runners (it
    // hangs forever). `spin_sleep::sleep` works fine.
    async_std::task::spawn_blocking(move || spin_sleep::sleep(dur)).await;
}

pub fn setup_test() {
    setup_logging();
    setup_backtrace();

    #[cfg(all(feature = "backtrace-on-stack-overflow", not(windows)))]
    unsafe {
        backtrace_on_stack_overflow::enable();
    }
}

/// Whether VID is expected for the given block height.
///
/// VID is skipped in HotShot for the genesis block.
pub fn has_vid(height: usize) -> bool {
    height >= FIRST_VID_VIEW
}

// Currently, HotShot also skips VID for view 1 (block 1). This should be fixed soon, but in the
// meantime, we need to skip VID tests for this block.
pub const FIRST_VID_VIEW: usize = 2;
