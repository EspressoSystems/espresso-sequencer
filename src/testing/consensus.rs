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

use super::mocks::{MockBlock, MockNodeImpl, MockTypes};
use crate::{data_source::QueryData, update::UpdateDataSource};
use async_std::{
    sync::{Arc, RwLock},
    task::spawn,
};
use futures::future::join_all;
use hotshot::{
    traits::{
        election::static_committee::StaticElectionConfig,
        implementations::{MemoryCommChannel, MemoryStorage},
    },
    types::{HotShotHandle, SignatureKey},
    HotShot, HotShotInitializer,
};
use hotshot_types::{
    traits::{
        election::{ConsensusExchange, QuorumExchange},
        network::TestableNetworkingImplementation,
        signature_key::ed25519::{Ed25519Priv, Ed25519Pub},
    },
    ExecutionType, HotShotConfig,
};
use std::num::NonZeroUsize;
use std::time::Duration;
use tempdir::TempDir;

struct MockNode<UserData> {
    query_data: Arc<RwLock<QueryData<MockTypes, MockNodeImpl, UserData>>>,
    hotshot: HotShotHandle<MockTypes, MockNodeImpl>,
}

pub struct MockNetwork<UserData> {
    nodes: Vec<MockNode<UserData>>,
    _dir: TempDir,
}

const MINIMUM_NODES: usize = 6;

impl<UserData: Clone + Send> MockNetwork<UserData> {
    pub async fn init(user_data: UserData) -> Self {
        let dir = TempDir::new("mock_network").unwrap();
        let priv_keys = (0..MINIMUM_NODES)
            .map(|_| Ed25519Priv::generate())
            .collect::<Vec<_>>();
        let pub_keys = priv_keys
            .iter()
            .map(Ed25519Pub::from_private)
            .collect::<Vec<_>>();
        // let master_map = MasterMap::new();
        let config = HotShotConfig {
            total_nodes: NonZeroUsize::new(pub_keys.len()).unwrap(),
            known_nodes: pub_keys.clone(),
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
        };
        let nodes = join_all(
            priv_keys
                .into_iter()
                .enumerate()
                .map(|(node_id, priv_key)| {
                    let path = dir.path().join(format!("node{}", node_id));
                    let user_data = user_data.clone();
                    let pub_keys = pub_keys.clone();
                    let config = config.clone();
                    // let master_map = master_map.clone();
                    // TODO: this probably won't work because of the randomly
                    // generated keys.
                    let channel_gen = MemoryCommChannel::generator(config.total_nodes.into(), 1, 0);
                    async move {
                        let query_data = QueryData::create(&path, user_data).unwrap();
                        let quorum_exchange = QuorumExchange::create(
                            pub_keys.clone(),
                            StaticElectionConfig {},
                            channel_gen(node_id as u64),
                            pub_keys[node_id],
                            priv_key.clone(),
                        );
                        let committee_exchange = QuorumExchange::create(
                            pub_keys.clone(),
                            StaticElectionConfig {},
                            channel_gen(node_id as u64),
                            pub_keys[node_id],
                            priv_key.clone(),
                        );
                        let hotshot = HotShot::init(
                            pub_keys[node_id],
                            priv_key,
                            node_id as u64,
                            config,
                            MemoryStorage::new(),
                            quorum_exchange,
                            committee_exchange,
                            HotShotInitializer::from_genesis(MockBlock::genesis()).unwrap(),
                            query_data.metrics(),
                        )
                        .await
                        .unwrap();
                        MockNode {
                            query_data: Arc::new(RwLock::new(query_data)),
                            hotshot,
                        }
                    }
                }),
        )
        .await;

        Self { nodes, _dir: dir }
    }
}

impl<UserData> MockNetwork<UserData> {
    pub fn handle(&self) -> HotShotHandle<MockTypes, MockNodeImpl> {
        self.nodes[0].hotshot.clone()
    }

    pub fn query_data(&self) -> Arc<RwLock<QueryData<MockTypes, MockNodeImpl, UserData>>> {
        self.nodes[0].query_data.clone()
    }

    pub async fn shut_down(mut self) {
        self.shut_down_impl().await
    }

    async fn shut_down_impl(&mut self) {
        for node in std::mem::take(&mut self.nodes) {
            node.hotshot.shut_down().await;
        }
    }
}

impl<UserData: Send + Sync + 'static> MockNetwork<UserData> {
    pub async fn start(&self) {
        // Spawn the update tasks.
        for node in &self.nodes {
            let mut hotshot = node.hotshot.clone();
            let qd = node.query_data.clone();
            spawn(async move {
                while let Ok(event) = hotshot.next_event().await {
                    tracing::info!("EVENT {:?}", event.event);
                    let mut qd = qd.write().await;
                    qd.update(&event).unwrap();
                    qd.commit_version().await.unwrap();
                }
            });
        }

        join_all(self.nodes.iter().map(|node| node.hotshot.start())).await;
    }
}

impl<UserData> Drop for MockNetwork<UserData> {
    fn drop(&mut self) {
        async_std::task::block_on(self.shut_down_impl())
    }
}
