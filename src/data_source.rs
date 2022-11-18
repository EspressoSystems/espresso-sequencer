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

use crate::{
    availability::{
        data_source::AvailabilityDataSource,
        query_data::{BlockHash, BlockQueryData, LeafHash, LeafQueryData, TransactionHash},
    },
    metrics::{MetricsError, PrometheusMetrics},
    status::{data_source::StatusDataSource, query_data::MempoolQueryData},
};
use hotshot_types::traits::{node_implementation::NodeTypes, signature_key::EncodedPublicKey};

pub trait ExtensibleDataSource {
    type UserData;
    fn user_data(&self) -> &Self::UserData;
    fn user_data_mut(&mut self) -> &mut Self::UserData;
}

#[derive(Debug)]
pub struct QueryData<Types: NodeTypes, UserData> {
    metrics: PrometheusMetrics,
    user_data: UserData,
    _marker: std::marker::PhantomData<Types>,
}

impl<Types: NodeTypes, UserData> ExtensibleDataSource for QueryData<Types, UserData> {
    type UserData = UserData;

    fn user_data(&self) -> &Self::UserData {
        &self.user_data
    }

    fn user_data_mut(&mut self) -> &mut Self::UserData {
        &mut self.user_data
    }
}

impl<Types: NodeTypes, UserData> AvailabilityDataSource<Types> for QueryData<Types, UserData> {
    type LeafIterType<'a> = std::vec::IntoIter<Option<LeafQueryData<Types>>> where UserData: 'a;
    type BlockIterType<'a> = std::vec::IntoIter<Option<BlockQueryData<Types>>> where UserData: 'a;

    fn get_nth_leaf_iter(&self, _n: usize) -> Self::LeafIterType<'_> {
        unimplemented!()
    }

    fn get_nth_block_iter(&self, _n: usize) -> Self::BlockIterType<'_> {
        unimplemented!()
    }

    fn get_leaf_index_by_hash(&self, _hash: LeafHash<Types>) -> Option<u64> {
        unimplemented!()
    }

    fn get_block_index_by_hash(&self, _hash: BlockHash<Types>) -> Option<u64> {
        unimplemented!()
    }

    fn get_txn_index_by_hash(&self, _hash: TransactionHash<Types>) -> Option<(u64, u64)> {
        unimplemented!()
    }

    fn get_block_ids_by_proposer_id(&self, _id: EncodedPublicKey) -> Vec<u64> {
        unimplemented!()
    }
}

impl<Types: NodeTypes, UserData> StatusDataSource for QueryData<Types, UserData> {
    type Error = MetricsError;

    fn block_height(&self) -> Result<usize, Self::Error> {
        unimplemented!()
    }

    fn mempool_info(&self) -> Result<MempoolQueryData, Self::Error> {
        unimplemented!()
    }

    fn success_rate(&self) -> Result<f64, Self::Error> {
        let failure_rate = (self.metrics.get_counter("invalid_qc_views")?.get() as f64)
            / (self.metrics.get_counter("currenv_view")?.get() as f64);
        Ok(1f64 - failure_rate)
    }

    fn export_metrics(&self) -> Result<String, Self::Error> {
        self.metrics.prometheus()
    }
}
