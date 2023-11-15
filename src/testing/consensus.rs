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

use super::mocks::{
    MockBlock, MockMembership, MockNodeImpl, MockTransaction, MockTypes, TestableDataSource,
};
use crate::data_source::FileSystemDataSource;
use async_std::{
    sync::{Arc, RwLock},
    task::spawn,
};
use futures::{future::join_all, stream::StreamExt};
use hotshot::{
    traits::{
        implementations::{MasterMap, MemoryCommChannel, MemoryNetwork, MemoryStorage},
        NodeImplementation,
    },
    types::SystemContextHandle,
    HotShotInitializer, SystemContext,
};
use hotshot_signature_key::bn254::{BN254Priv, BN254Pub};
use hotshot_types::{
    traits::{
        election::Membership, node_implementation::ExchangesType, signature_key::SignatureKey,
    },
    ExecutionType, HotShotConfig,
};
use std::num::NonZeroUsize;
use std::time::Duration;

struct MockNode<D: TestableDataSource> {
    hotshot: SystemContextHandle<MockTypes, MockNodeImpl>,
    data_source: Arc<RwLock<D>>,
    _tmp_data: D::TmpData,
}

pub struct MockNetwork<D: TestableDataSource> {
    nodes: Vec<MockNode<D>>,
}

// MockNetwork can be used with any TestableDataSource, but it's nice to have a default with a
// convenient type alias.
pub type MockDataSource = FileSystemDataSource<MockTypes, MockNodeImpl>;

const MINIMUM_NODES: usize = 2;

impl<D: TestableDataSource> MockNetwork<D> {
    pub async fn init() -> Self {
        let priv_keys = (0..MINIMUM_NODES)
            .map(|_| BN254Priv::generate())
            .collect::<Vec<_>>();
        let pub_keys = priv_keys
            .iter()
            .map(BN254Pub::from_private)
            .collect::<Vec<_>>();
        let total_nodes = NonZeroUsize::new(pub_keys.len()).unwrap();
        let master_map = MasterMap::new();
        let known_nodes_with_stake: Vec<<BN254Pub as SignatureKey>::StakeTableEntry> = (0
            ..total_nodes.into())
            .map(|id| pub_keys[id].get_stake_table_entry(1u64))
            .collect();
        let config = HotShotConfig {
            total_nodes,
            known_nodes: pub_keys.clone(),
            known_nodes_with_stake: known_nodes_with_stake.clone(),
            start_delay: 0,
            round_start_delay: 0,
            next_view_timeout: 10000,
            timeout_ratio: (11, 10),
            propose_min_round_time: Duration::from_secs(0),
            propose_max_round_time: Duration::from_secs(2),
            min_transactions: 1,
            max_transactions: NonZeroUsize::new(100).unwrap(),
            num_bootstrap: 0,
            execution_type: ExecutionType::Continuous,
            election_config: None,
            da_committee_size: total_nodes.into(),
        };
        let nodes = join_all(
            priv_keys
                .into_iter()
                .enumerate()
                .map(|(node_id, priv_key)| {
                    let pub_keys = pub_keys.clone();
                    let known_nodes_with_stake = known_nodes_with_stake.clone();
                    let config = config.clone();
                    let master_map = master_map.clone();
                    let election_config =
                        MockMembership::default_election_config(total_nodes.get() as u64);

                    async move {
                        let (data_source, tmp_data) = D::create(node_id).await;
                        let network = Arc::new(MemoryNetwork::new(
                            pub_keys[node_id],
                            data_source.populate_metrics(),
                            master_map.clone(),
                            None,
                        ));
                        let consensus_channel = MemoryCommChannel::new(network.clone());
                        let da_channel = MemoryCommChannel::new(network.clone());
                        let view_sync_channel = MemoryCommChannel::new(network.clone());

                        let exchanges =
                            <MockNodeImpl as NodeImplementation<MockTypes>>::Exchanges::create(
                                known_nodes_with_stake.clone(),
                                pub_keys.clone(),
                                (election_config.clone(), election_config.clone()),
                                (consensus_channel, view_sync_channel, da_channel),
                                pub_keys[node_id],
                                pub_keys[node_id].get_stake_table_entry(1u64),
                                priv_key.clone(),
                            );

                        let (hotshot, _) = SystemContext::init(
                            pub_keys[node_id],
                            priv_key,
                            node_id as u64,
                            config,
                            MemoryStorage::empty(),
                            exchanges,
                            HotShotInitializer::from_genesis(MockBlock::genesis()).unwrap(),
                            data_source.populate_metrics(),
                        )
                        .await
                        .unwrap();
                        MockNode {
                            hotshot,
                            data_source: Arc::new(RwLock::new(data_source)),
                            _tmp_data: tmp_data,
                        }
                    }
                }),
        )
        .await;

        Self { nodes }
    }
}

impl<D: TestableDataSource> MockNetwork<D> {
    pub fn handle(&self) -> SystemContextHandle<MockTypes, MockNodeImpl> {
        self.nodes[0].hotshot.clone()
    }

    pub async fn submit_transaction(&self, tx: MockTransaction) {
        self.handle().submit_transaction(tx).await.unwrap();
    }

    pub fn data_source(&self) -> Arc<RwLock<D>> {
        self.nodes[0].data_source.clone()
    }

    pub async fn shut_down(mut self) {
        self.shut_down_impl().await
    }

    async fn shut_down_impl(&mut self) {
        for mut node in std::mem::take(&mut self.nodes) {
            node.hotshot.shut_down().await;
        }
    }
}

impl<D: TestableDataSource> MockNetwork<D> {
    pub async fn start(&mut self) {
        // Spawn the update tasks.
        for node in &mut self.nodes {
            let ds = node.data_source.clone();
            let mut events = node.hotshot.get_event_stream(Default::default()).await.0;
            spawn(async move {
                while let Some(event) = events.next().await {
                    tracing::info!("EVENT {:?}", event.event);
                    let mut ds = ds.write().await;
                    ds.update(&event).await.unwrap();
                    ds.commit().await.unwrap();
                }
            });
        }

        join_all(
            self.nodes
                .iter()
                .map(|node| node.hotshot.hotshot.start_consensus()),
        )
        .await;
    }
}

impl<D: TestableDataSource> Drop for MockNetwork<D> {
    fn drop(&mut self) {
        async_std::task::block_on(self.shut_down_impl())
    }
}
