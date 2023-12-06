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

//! Simple HotShot query server
//!
//! This example demonstrates the most basic usage of hotshot-query-service. It starts a small
//! consensus network with two nodes and connects a query service to each node. It runs each query
//! server on local host. The program continues until it is manually killed.

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::sync::Arc;
use clap::Parser;
use futures::future::{join_all, try_join_all};
use hotshot::{
    traits::implementations::{
        MasterMap, MemoryCommChannel, MemoryNetwork, MemoryStorage, NetworkingMetricsValue,
    },
    types::{SignatureKey, SystemContextHandle},
    HotShotInitializer, Memberships, Networks, SystemContext,
};
use hotshot_query_service::{
    data_source, run_standalone_service,
    status::UpdateStatusData,
    testing::mocks::{MockMembership, MockNodeImpl, MockTypes, TestableDataSource},
    Error,
};
use hotshot_signature_key::bn254::{BLSPrivKey, BLSPubKey};
use hotshot_types::{
    consensus::ConsensusMetricsValue, light_client::StateKeyPair, ExecutionType, HotShotConfig,
    ValidatorConfig,
};
use std::{num::NonZeroUsize, time::Duration};

const NUM_NODES: usize = 2;

#[derive(Parser)]
struct Options {
    /// Port on which to host the query service for the first consensus node.
    #[clap(long, default_value = "18080")]
    port1: u16,

    /// Port on which to host the query service for the second consensus node.
    #[clap(long, default_value = "28080")]
    port2: u16,
}

#[cfg(not(target_os = "windows"))]
type DataSource = data_source::SqlDataSource<MockTypes>;

// To use SqlDataSource, we need to run the `postgres` Docker image, which doesn't work on Windows.
#[cfg(target_os = "windows")]
type DataSource = data_source::FileSystemDataSource<MockTypes>;

type Db = <DataSource as TestableDataSource>::Storage;

#[cfg(not(target_os = "windows"))]
async fn init_db() -> Db {
    Db::init().await
}

#[cfg(target_os = "windows")]
async fn init_db() -> Db {
    Db::new("simple-server-db").unwrap()
}

#[cfg(not(target_os = "windows"))]
async fn init_data_source(db: &Db) -> DataSource {
    data_source::sql::Config::default()
        .user("postgres")
        .password("password")
        .port(db.port())
        .connect()
        .await
        .unwrap()
}

#[cfg(target_os = "windows")]
async fn init_data_source(db: &Db) -> DataSource {
    DataSource::create(db.path()).unwrap()
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();

    // Start databases for the query services.
    let dbs = join_all((0..NUM_NODES).map(|_| init_db())).await;

    // Create the data sources for the query services.
    let data_sources = join_all(dbs.iter().map(init_data_source)).await;

    // Start consensus.
    let nodes = init_consensus(&data_sources).await;

    // Start the servers.
    try_join_all(
        data_sources
            .into_iter()
            .zip(nodes)
            .zip([opt.port1, opt.port2])
            .map(|((data_source, node), port)| async move {
                let opt = hotshot_query_service::Options {
                    port,
                    ..Default::default()
                };
                run_standalone_service(opt, data_source, node).await
            }),
    )
    .await?;

    Ok(())
}

async fn init_consensus(
    data_sources: &[DataSource],
) -> Vec<SystemContextHandle<MockTypes, MockNodeImpl>> {
    let priv_keys = (0..data_sources.len())
        .map(|_| BLSPrivKey::generate())
        .collect::<Vec<_>>();
    let pub_keys = priv_keys
        .iter()
        .map(BLSPubKey::from_private)
        .collect::<Vec<_>>();
    let master_map = MasterMap::new();
    let known_nodes_with_stake: Vec<<BLSPubKey as SignatureKey>::StakeTableEntry> = pub_keys
        .iter()
        .map(|pub_key| pub_key.get_stake_table_entry(1u64))
        .collect();
    let config = HotShotConfig {
        total_nodes: NonZeroUsize::new(pub_keys.len()).unwrap(),
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
        da_committee_size: pub_keys.len(),
        my_own_validator_config: Default::default(),
    };
    join_all(priv_keys.into_iter().zip(data_sources).enumerate().map(
        |(node_id, (priv_key, data_source))| {
            let pub_keys = pub_keys.clone();
            let known_nodes_with_stake = known_nodes_with_stake.clone();
            let mut config = config.clone();
            let master_map = master_map.clone();

            async move {
                config.my_own_validator_config = ValidatorConfig {
                    public_key: pub_keys[node_id],
                    private_key: priv_key.clone(),
                    stake_value: known_nodes_with_stake[node_id].stake_amount.as_u64(),
                    state_key_pair: StateKeyPair::generate(),
                };

                let membership = MockMembership::new(&pub_keys, known_nodes_with_stake.clone());
                let memberships = Memberships {
                    quorum_membership: membership.clone(),
                    da_membership: membership.clone(),
                    vid_membership: membership.clone(),
                    view_sync_membership: membership,
                };

                let metrics = data_source.populate_metrics();
                let metrics = NetworkingMetricsValue {
                    values: Default::default(),
                    connected_peers: metrics.create_gauge(String::from("connected_peers"), None),
                    incoming_direct_message_count: metrics
                        .create_counter(String::from("incoming_direct_message_count"), None),
                    incoming_broadcast_message_count: metrics
                        .create_counter(String::from("incoming_broadcast_message_count"), None),
                    outgoing_direct_message_count: metrics
                        .create_counter(String::from("outgoing_direct_message_count"), None),
                    outgoing_broadcast_message_count: metrics
                        .create_counter(String::from("outgoing_broadcast_message_count"), None),
                    message_failed_to_send: metrics
                        .create_counter(String::from("message_failed_to_send"), None),
                };
                let network = Arc::new(MemoryNetwork::new(
                    pub_keys[node_id],
                    metrics,
                    master_map.clone(),
                    None,
                ));
                let networks = Networks {
                    quorum_network: MemoryCommChannel::new(network.clone()),
                    da_network: MemoryCommChannel::new(network),
                    _pd: Default::default(),
                };

                let metrics = data_source.populate_metrics();
                let metrics = ConsensusMetricsValue {
                    values: Default::default(),
                    last_synced_block_height: metrics
                        .create_gauge(String::from("last_synced_block_height"), None),
                    last_decided_view: metrics
                        .create_gauge(String::from("last_decided_view"), None),
                    current_view: metrics.create_gauge(String::from("current_view"), None),
                    number_of_views_since_last_decide: metrics
                        .create_gauge(String::from("number_of_views_since_last_decide"), None),
                    number_of_views_per_decide_event: metrics
                        .create_histogram(String::from("number_of_views_per_decide_event"), None),
                    invalid_qc: metrics.create_gauge(String::from("invalid_qc"), None),
                    outstanding_transactions: metrics
                        .create_gauge(String::from("outstanding_transactions"), None),
                    outstanding_transactions_memory_size: metrics
                        .create_gauge(String::from("outstanding_transactions_memory_size"), None),
                    number_of_timeouts: metrics
                        .create_counter(String::from("number_of_timeouts"), None),
                };

                SystemContext::init(
                    pub_keys[node_id],
                    priv_key,
                    node_id as u64,
                    config,
                    MemoryStorage::empty(),
                    memberships,
                    networks,
                    HotShotInitializer::from_genesis().unwrap(),
                    metrics,
                )
                .await
                .unwrap()
                .0
            }
        },
    ))
    .await
}
