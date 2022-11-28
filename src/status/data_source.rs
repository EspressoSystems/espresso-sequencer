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

use super::query_data::MempoolQueryData;
use hotshot_types::traits::metrics::Metrics;
use std::error::Error;
use std::fmt::Debug;

pub trait StatusDataSource {
    type Error: Error + Debug;
    fn block_height(&self) -> Result<usize, Self::Error>;
    fn mempool_info(&self) -> Result<MempoolQueryData, Self::Error>;
    fn success_rate(&self) -> Result<f64, Self::Error>;

    /// Export all available metrics in the Prometheus text format.
    fn export_metrics(&self) -> Result<String, Self::Error>;
}

pub trait UpdateStatusData {
    fn metrics(&self) -> Box<dyn Metrics>;
}
