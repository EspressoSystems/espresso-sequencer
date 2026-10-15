use super::*;

async fn run_catchup_test(url_suffix: &str) {
    // Start a sequencer network, using the query service for catchup. Node 0 keeps merklized
    // state in SQL: in memory it holds only the states from its decided view up, and a node
    // restarted under load handles proposals behind the network, so by the time it asks for
    // a parent's frontier a peer without storage has dropped that view, and the request
    // retries forever.
    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    const NUM_NODES: usize = 5;
    let storage = SqlDataSource::create_storage().await;

    let url: url::Url = format!("http://localhost:{port}{url_suffix}")
        .parse()
        .unwrap();

    let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
        .api_config(SqlDataSource::options(&storage, Options::with_port(port)))
        .network_config(TestConfigBuilder::default().build())
        .without_api_catchup()
        .catchups(std::array::from_fn(|_| {
            StatePeers::<StaticVersion<0, 1>>::from_urls(
                vec![url.clone()],
                Default::default(),
                Duration::from_secs(2),
                &NoMetrics,
            )
        }))
        .build();
    let mut network = TestNetwork::new(config, TEST_UPGRADE).await;
    let catchup = StatePeers::<StaticVersion<0, 1>>::from_urls(
        vec![url],
        Default::default(),
        Duration::from_secs(2),
        &NoMetrics,
    );
    restarted_node_catches_up(&mut network, catchup).await;
}

/// Restarts replica 1 from genesis after the network has moved on without it, and waits for
/// it to decide blocks past the height the network had reached when it came back, with a
/// state it validated itself. We don't just stop consensus and restart it; we fully drop the
/// node and recreate it so it loses all of its temporary state. It should be able to catch up
/// by listening to proposals and then rebuild its state with `catchup`.
async fn restarted_node_catches_up<const NUM_NODES: usize>(
    network: &mut TestNetwork<no_storage::Options, NUM_NODES>,
    catchup: impl StateCatchup + 'static,
) {
    // A decide proves only that the node hears the network: it takes a proposal and its two
    // certificates, all broadcast, and never a validated state. A state this node computed
    // carries a delta, where the `from_header` stub it holds otherwise has none.
    restart_from_genesis_and_decide(network, catchup, |info| info.delta.is_some()).await;
}

/// Restarts replica 1 from genesis after the network has moved on without it, then waits for
/// it to decide a leaf past the height the network had reached when it came back that also
/// satisfies `caught_up`. Returns every leaf it decided on the way, oldest first.
async fn restart_from_genesis_and_decide<const NUM_NODES: usize>(
    network: &mut TestNetwork<no_storage::Options, NUM_NODES>,
    catchup: impl StateCatchup + 'static,
    caught_up: impl Fn(&LeafInfo<SeqTypes>) -> bool,
) -> Vec<LeafInfo<SeqTypes>> {
    // Wait for replica 1 to reach a (non-genesis) decide, before disconnecting it.
    let mut events = network.node(1).event_stream();
    loop {
        let event = events.next().await.unwrap();
        let Some(leaf_chain) = decided_leaves(&event) else {
            continue;
        };
        if leaf_chain[0].leaf.height() > 0 {
            break;
        }
    }

    tracing::info!("shutting down node");
    network.stop_node(1).await;

    // Wait for a few blocks to pass while the node is down, so it falls behind.
    network
        .server
        .event_stream()
        .filter(|event| future::ready(decided_leaves(event).is_some()))
        .take(3)
        .collect::<Vec<_>>()
        .await;

    tracing::info!("restarting node");
    let target = network.server.decided_leaf().await.height() + 3;
    let node = network
        .start_stopped_node(
            1,
            network.genesis_state(1),
            no_storage::Options,
            catchup,
            TEST_UPGRADE,
        )
        .await;
    let mut events = node.event_stream();
    let mut decided = Vec::new();
    let done = timeout(Duration::from_secs(120), async {
        loop {
            let event = events.next().await.unwrap();
            let Some(chain) = decided_leaves(&event) else {
                continue;
            };
            decided.extend(chain.iter().rev().cloned());
            let newest = &chain[0];
            if newest.leaf.height() >= target && caught_up(newest) {
                break;
            }
        }
    })
    .await;
    assert!(
        done.is_ok(),
        "restarted node never caught up to height {target}; decided (height, validated): {:?}",
        decided
            .iter()
            .map(|info| (info.leaf.height(), info.delta.is_some()))
            .collect::<Vec<_>>()
    );
    decided
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_catchup() {
    run_catchup_test("").await;
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_catchup_v0() {
    run_catchup_test("/v0").await;
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_catchup_v1() {
    run_catchup_test("/v1").await;
}

/// Without HTTP state peers, a node restarted from genesis seeds a stub for its first decided
/// leaf and catches up over request-response at that decided view, which peers still hold.
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_restart_no_state_peers_catches_up_over_request_response() {
    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    const NUM_NODES: usize = 5;
    let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
        .api_config(Options::with_port(port))
        .network_config(TestConfigBuilder::default().build())
        .without_api_catchup()
        .build();
    let mut network = TestNetwork::new(config, TEST_UPGRADE).await;
    restart_from_genesis_and_decide(&mut network, NullStateCatchup::default(), |info| {
        info.delta.is_some()
    })
    .await;
}

/// A query node with a gap must fill it from a peer over the path production runs: the
/// proactive scanner, `LightClientProvider`, the light-client HTTP client and the peer's
/// endpoints. One node takes part in consensus while the chain is built, then is shut down and
/// brought back with a query database holding every leaf and one empty payload, the state a
/// node is in once its leaf scan is done: payload dedup reads every empty height as present,
/// and the non-empty ones are the fragmented set mainnet leaves behind.
///
/// What this pins is that the production wiring converges on the peer's data, not that the
/// ranges endpoints carried it. Consensus is not restarted, but two other writers remain: the
/// startup replay of decides persisted before the shutdown, and the aggregator's payload
/// fetches by hash. Both are bounded and neither uses a ranges request, so they cost the test
/// its claim on the route rather than its result;
/// `test_scanner_backfills_over_ranges_endpoints_only` is what covers the route.
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_query_service_catchup_from_peer() {
    const NUM_NODES: usize = 5;
    const LATE: usize = NUM_NODES - 1;
    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    let late_port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    let peer_url: Url = format!("http://localhost:{port}").parse().unwrap();
    let state_peers = || {
        StatePeers::<SequencerApiVersion>::from_urls(
            vec![peer_url.clone()],
            Default::default(),
            Duration::from_secs(2),
            &NoMetrics,
        )
    };

    let dbs = join_all((0..NUM_NODES).map(|_| SqlDataSource::create_storage())).await;
    let persistence: [_; NUM_NODES] = dbs
        .iter()
        .map(<SqlDataSource as TestableSequencerDataSource>::persistence_options)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
        .api_config(
            SqlDataSource::options(&dbs[0], Options::with_port(port))
                .light_client(Default::default()),
        )
        .persistences(persistence)
        .catchups(std::array::from_fn(|_| state_peers()))
        .network_config(TestConfigBuilder::default().build())
        .build();
    let mut network = TestNetwork::new(config, TEST_UPGRADE).await;

    // Interleave transactions with idle views so the chain gets both non-empty and empty
    // blocks. Which heights land either way is up to the builder, so the shape is read back
    // from the peer below rather than assumed here.
    let namespace = NamespaceId::from(7_u32);
    let mut decided = network.server.event_stream().filter_map(|event| {
        future::ready(match event {
            CoordinatorEvent::LegacyEvent(Event {
                event: EventType::Decide { leaf_chain, .. },
                ..
            }) => Some(leaf_chain[0].leaf.clone()),
            CoordinatorEvent::NewDecide { leaf_infos, .. } => Some(leaf_infos[0].leaf.clone()),
            _ => None,
        })
    });
    let height = tokio::time::timeout(Duration::from_secs(180), async {
        let mut height = 0;
        for i in 0..4u8 {
            network
                .server
                .submit_transaction(Transaction::new(namespace, vec![i; 8]))
                .await
                .unwrap();
            // Wait for it to be sequenced, then let a couple more views decide.
            loop {
                let leaf = decided.next().await.unwrap();
                height = leaf.height();
                if leaf
                    .block_header()
                    .ns_table()
                    .find_ns_id(&namespace)
                    .is_some()
                {
                    break;
                }
            }
            for _ in 0..2 {
                height = decided.next().await.unwrap().height();
            }
        }
        // A finality proof for a leaf needs a QC two-chain above it, so leave the last
        // non-empty block well below the tip.
        for _ in 0..3 {
            height = decided.next().await.unwrap().height();
        }
        height
    })
    .await
    .expect("network did not sequence the test transactions");
    drop(decided);

    // Take the last node out of consensus; it comes back below as the late query node. As in
    // `restart_node`, its coordinator port is only free once the aborted tasks let go of it.
    network.peers[LATE - 1].shut_down().await;
    let addr = network.cfg.coordinator_addr(LATE).to_string();
    tokio::time::timeout(Duration::from_secs(60), async {
        while std::net::TcpListener::bind(&addr).is_err() {
            sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("shut-down node did not release its coordinator port");

    // Seed the late node with every leaf plus one empty block, the state a node reaches once
    // its leaf scan is done. Payload dedup then makes every empty height read as present. Not
    // the genesis block: its payload commitment is not the one the chain's empty blocks share.
    let peer: Client<ClientErr, SequencerApiVersion> = Client::new(peer_url.clone());
    let leaves: Vec<LeafQueryData<SeqTypes>> = peer
        .get(&format!("availability/leaf/0/{}", height + 1))
        .send()
        .await
        .unwrap();
    let blocks: Vec<BlockQueryData<SeqTypes>> = peer
        .get(&format!("availability/block/0/{}", height + 1))
        .send()
        .await
        .unwrap();
    let (empty, non_empty): (Vec<_>, Vec<_>) = blocks
        .iter()
        .filter(|block| block.height() > 0)
        .partition(|block| block.num_transactions() == 0);
    assert!(
        non_empty.len() >= 2 && !empty.is_empty(),
        "chain of {} heights is not a mix of empty and non-empty blocks",
        blocks.len()
    );
    {
        // No proactive fetching: this handle only seeds, and a scanner of its own would race
        // the late node for the same database.
        let mut opt = tmp_options(&dbs[LATE]);
        opt.disable_proactive_fetching = true;
        let ds = SqlDataSource::create(opt, Default::default(), false)
            .await
            .unwrap();
        let mut tx = ds.write().await.unwrap();
        for leaf in &leaves {
            tx.insert_leaf(leaf).await.unwrap();
        }
        // Genesis too, so the only missing blocks are the non-empty heights: its payload
        // commitment is its own, so it dedups with nothing and would otherwise read as one
        // more missing run of its own.
        tx.insert_block(&blocks[0]).await.unwrap();
        tx.insert_block(empty[0]).await.unwrap();
        tx.commit().await.unwrap();

        // Read the shape back rather than over HTTP, where the scanner could have run first.
        // The missing blocks are exactly the non-empty heights now, so more than one run is
        // the fragmentation this whole path exists for; one contiguous gap would be a much
        // easier case.
        let status = ds.sync_status().await.unwrap();
        let missing_runs = status
            .blocks
            .ranges
            .iter()
            .filter(|range| range.status == SyncStatus::Missing)
            .count();
        assert!(
            missing_runs > 1,
            "missing set is not fragmented: {status:#?}"
        );
        assert!(status.vid_common.missing > 0, "{status:#?}");
    }

    // Back as a query node whose service fetches from node 0 the way a production node fetches
    // from its configured peers. Consensus is not restarted: a decided leaf would have the node
    // chase parents one at a time, and that is not the path under test.
    let mut late_db = tmp_options(&dbs[LATE]);
    late_db.proactive_scan_interval = Some(Duration::from_secs(1));
    // A ranges request that cannot be served costs this much before the per-chunk fallback, and the
    // 120 second default would not fit in the budget below.
    late_db.proactive_fetch_timeout = Some(Duration::from_secs(5));
    // The sync status the scanner reads and the endpoint serves is cached for five minutes by
    // default, which would hide the catch-up from both for the whole test.
    late_db.sync_status_ttl = Some(Duration::from_secs(1));
    // Chunks smaller than the gap, so the missing runs pack into several requests.
    late_db.proactive_scan_chunk_size = Some(4);
    let api = Options::with_port(late_port).query_sql(
        Query {
            peers: vec![peer_url.clone()],
            ..Default::default()
        },
        late_db,
    );
    let cfg = network.cfg.clone();
    let upgrades_map = cfg.upgrades();
    let persistence =
        <SqlDataSource as TestableSequencerDataSource>::persistence_options(&dbs[LATE]);
    let catchup = state_peers();
    let _late = api
        .serve(|metrics, consumer, storage| {
            async move {
                Ok(cfg
                    .init_node(
                        LATE,
                        ValidatedState::default(),
                        persistence,
                        Some(catchup),
                        storage,
                        &*metrics,
                        STAKE_TABLE_CAPACITY_FOR_TEST,
                        consumer,
                        TEST_UPGRADE,
                        upgrades_map,
                    )
                    .await)
            }
            .boxed()
        })
        .await
        .unwrap();

    let client: Client<ClientErr, SequencerApiVersion> =
        Client::new(format!("http://localhost:{late_port}").parse().unwrap());
    assert!(client.connect(Some(Duration::from_secs(60))).await);
    let mut last = None;
    let synced = tokio::time::timeout(Duration::from_secs(180), async {
        loop {
            if let Ok(status) = client
                .get::<SyncStatusQueryData>("node/sync-status")
                .send()
                .await
            {
                if status.is_fully_synced() {
                    return;
                }
                tracing::info!(?status, height, "waiting for the late node to catch up");
                last = Some(status);
            }
            sleep(Duration::from_secs(1)).await;
        }
    })
    .await;
    assert!(
        synced.is_ok(),
        "late node did not catch its query service up from its peer; last status {last:#?}"
    );

    for h in 0..=height {
        let (ours, theirs) = try_join!(
            client
                .get::<BlockQueryData<SeqTypes>>(&format!("availability/block/{h}"))
                .send(),
            peer.get::<BlockQueryData<SeqTypes>>(&format!("availability/block/{h}"))
                .send(),
        )
        .unwrap();
        assert_eq!(ours, theirs);
        let (ours, theirs) = try_join!(
            client
                .get::<VidCommonQueryData<SeqTypes>>(&format!("availability/vid/common/{h}"))
                .send(),
            peer.get::<VidCommonQueryData<SeqTypes>>(&format!("availability/vid/common/{h}"))
                .send(),
        )
        .unwrap();
        assert_eq!(ours, theirs);
    }
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_chain_config_catchup() {
    // Only node 0 starts with the full chain config; every other node holds just its
    // commitment and must fetch the config from node 0.
    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");

    const NUM_NODES: usize = 5;
    let builder = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
        .api_config(Options::from(options::Http {
            port,
            max_connections: None,
            tonic_port: None,
        }))
        .catchups(std::array::from_fn(|_| {
            StatePeers::<StaticVersion<0, 1>>::from_urls(
                vec![format!("http://localhost:{port}").parse().unwrap()],
                Default::default(),
                Duration::from_secs(2),
                &NoMetrics,
            )
        }))
        .network_config(TestConfigBuilder::default().build())
        .pos_hook(
            DelegationConfig::MultipleDelegators,
            StakeTableContractVersion::V3,
            TEST_UPGRADE,
        )
        .await
        .unwrap();

    // Differ from the default so a node can't get it from its own instance state.
    let cf = ChainConfig {
        max_block_size: 300.into(),
        ..builder.chain_config()
    };
    let commitment_only = ValidatedState {
        chain_config: cf.commit().into(),
        ..Default::default()
    };
    let mut states = std::array::from_fn(|_| commitment_only.clone());
    states[0] = ValidatedState {
        chain_config: cf.into(),
        ..Default::default()
    };
    let config = builder.states(states).genesis_chain_config(cf).build();

    let mut network = TestNetwork::new(config, TEST_UPGRADE).await;

    // Wait for a few blocks to be decided.
    network
        .server
        .event_stream()
        .filter(|event| future::ready(decided_leaves(event).is_some()))
        .take(3)
        .collect::<Vec<_>>()
        .await;

    for node in std::iter::once(&network.server).chain(&network.peers) {
        let state = node.consensus_handle().decided_state().await.unwrap();

        assert_eq!(state.chain_config.resolve().unwrap(), cf)
    }

    network.server.shut_down().await;
    drop(network);
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
pub(crate) async fn test_restart() {
    const NUM_NODES: usize = 5;
    // Initialize nodes.
    let storage = join_all((0..NUM_NODES).map(|_| SqlDataSource::create_storage())).await;
    let persistence: [_; NUM_NODES] = storage
        .iter()
        .map(<SqlDataSource as TestableSequencerDataSource>::persistence_options)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    let config = TestNetworkConfigBuilder::default()
        .api_config(SqlDataSource::options(
            &storage[0],
            Options::with_port(port),
        ))
        .persistences(persistence.clone())
        .network_config(TestConfigBuilder::default().build())
        .build();
    let mut network = TestNetwork::new(config, TEST_UPGRADE).await;

    // Connect client.
    let client: Client<ClientErr, SequencerApiVersion> =
        Client::new(format!("http://localhost:{port}").parse().unwrap());
    client.connect(None).await;
    tracing::info!(port, "server running");

    // Wait until some blocks have been decided.
    client
        .socket("availability/stream/blocks/0")
        .subscribe::<BlockQueryData<SeqTypes>>()
        .await
        .unwrap()
        .take(3)
        .collect::<Vec<_>>()
        .await;

    // Shut down the consensus nodes.
    tracing::info!("shutting down nodes");
    network.stop_consensus().await;

    // Get the block height we reached.
    let height = client
        .get::<usize>("status/block-height")
        .send()
        .await
        .unwrap();
    tracing::info!("decided {height} blocks before shutting down");

    // Get the decided chain, so we can check consistency after the restart.
    let chain: Vec<LeafQueryData<SeqTypes>> = client
        .socket("availability/stream/leaves/0")
        .subscribe()
        .await
        .unwrap()
        .take(height)
        .try_collect()
        .await
        .unwrap();
    let decided_view = chain.last().unwrap().leaf().view_number();

    // Get the most recent state, for catchup.

    let state = network.server.decided_state().await.unwrap();
    tracing::info!(?decided_view, ?state, "consensus state");

    // Fully shut down the API servers.
    drop(network);

    // Start up again, resuming from the last decided leaf.
    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");

    let config = TestNetworkConfigBuilder::default()
        .api_config(SqlDataSource::options(
            &storage[0],
            Options::with_port(port),
        ))
        .persistences(persistence)
        .catchups(std::array::from_fn(|_| {
            // Catchup using node 0 as a peer. Node 0 was running the archival state service
            // before the restart, so it should be able to resume without catching up by loading
            // state from storage.
            StatePeers::<StaticVersion<0, 1>>::from_urls(
                vec![format!("http://localhost:{port}").parse().unwrap()],
                Default::default(),
                Duration::from_secs(2),
                &NoMetrics,
            )
        }))
        .network_config(TestConfigBuilder::default().build())
        .build();
    let _network = TestNetwork::new(config, TEST_UPGRADE).await;
    let client: Client<ClientErr, StaticVersion<0, 1>> =
        Client::new(format!("http://localhost:{port}").parse().unwrap());
    client.connect(None).await;
    tracing::info!(port, "server running");

    // Make sure we can decide new blocks after the restart.
    tracing::info!("waiting for decide, height {height}");
    let new_leaf: LeafQueryData<SeqTypes> = client
        .socket(&format!("availability/stream/leaves/{height}"))
        .subscribe()
        .await
        .unwrap()
        .next()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(new_leaf.height(), height as u64);
    assert_eq!(
        new_leaf.leaf().parent_commitment(),
        chain[height - 1].hash()
    );

    // Ensure the new chain is consistent with the old chain.
    let new_chain: Vec<LeafQueryData<SeqTypes>> = client
        .socket("availability/stream/leaves/0")
        .subscribe()
        .await
        .unwrap()
        .take(height)
        .try_collect()
        .await
        .unwrap();
    assert_eq!(chain, new_chain);
}

/// A query node whose database is wiped has to rebuild itself from its peer
/// and rejoin consensus.
///
/// Six nodes run the new protocol from genesis. Nodes 0 and 1 both serve the
/// query API and are peered at each other so either can backfill from the
/// other; nodes 2 through 5 are plain validators. Once the network decides
/// past epoch seven, node 1 is taken down and started again on an empty
/// database: no consensus state, no archive, no merklized state. To recover
/// it has to bootstrap its stake-table window, resync consensus, backfill
/// the chain it lost from node 0 (including the new protocol's cert2
/// finality certificates), and resume proposing.
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_new_protocol_query_node_restart_with_fresh_storage() -> anyhow::Result<()> {
    const NUM_NODES: usize = 6;
    const EPOCH_HEIGHT: u64 = 10;
    const EPOCHS_BEFORE_RESTART: u64 = 7;
    const TEST_UPGRADE: Upgrade = Upgrade::trivial(NEW_PROTOCOL_VERSION);
    /// Bound on each stage of the restarted node's recovery.
    const RECOVERY_TIMEOUT: Duration = Duration::from_secs(240);

    let network_config = TestConfigBuilder::default()
        .epoch_height(EPOCH_HEIGHT)
        .epoch_start_block(0)
        .build();

    let api_port = reserve_tcp_port().expect("No ports free for query service");
    let query_port = reserve_tcp_port().expect("No ports free for query service");
    let api_url: Url = format!("http://localhost:{api_port}").parse()?;
    let query_url: Url = format!("http://localhost:{query_port}").parse()?;

    let (storage, persistence) = node_storage::<SqlDataSource, NUM_NODES>().await;

    // Both query nodes need the catchup and light client modules: state
    // catchup is served over the former, and the query service validates
    // leaves fetched from a peer with light client proofs served over the
    // latter. Every node can catch up from either query node.
    let query_urls = vec![api_url.clone(), query_url.clone()];
    let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
        .api_config(
            Options::with_port(api_port)
                .catchup(Default::default())
                .light_client(Default::default())
                .query_sql(
                    Query {
                        peers: vec![query_url.clone()],
                        ..Default::default()
                    },
                    tmp_options(&storage[0]),
                ),
        )
        .network_config(network_config)
        .persistences(persistence.clone())
        .catchups(std::array::from_fn(|_| {
            StatePeers::<SequencerApiVersion>::from_urls(
                query_urls.clone(),
                Default::default(),
                Duration::from_secs(2),
                &NoMetrics,
            )
        }))
        .build();
    let mut network = TestNetwork::new(config, TEST_UPGRADE).await;
    let genesis_state = network.genesis_state(0);

    // `TestNetwork` only gives node 0 an API, so node 1 has to be served
    // separately to become the second query node. Its keys are already in the
    // config, so it rejoins under the same index.
    network.peers[0].shut_down().await;
    network.peers.remove(0);

    let cfg = network.cfg.clone();
    let peer_url = api_url.clone();
    let start_query_node = move |db: persistence::sql::Options| {
        let cfg = cfg.clone();
        let genesis_state = genesis_state.clone();
        let peer_url = peer_url.clone();
        async move {
            let opt = Options::with_port(query_port)
                .catchup(Default::default())
                .light_client(Default::default())
                .query_sql(
                    Query {
                        peers: vec![peer_url.clone()],
                        ..Default::default()
                    },
                    db.clone(),
                );
            let ctx = opt
                .serve(move |metrics, consumer, storage| {
                    async move {
                        Ok(cfg
                            .init_node(
                                1,
                                genesis_state,
                                db,
                                Some(StatePeers::<SequencerApiVersion>::from_urls(
                                    vec![peer_url],
                                    Default::default(),
                                    Duration::from_secs(2),
                                    &NoMetrics,
                                )),
                                storage,
                                &*metrics,
                                test_helpers::STAKE_TABLE_CAPACITY_FOR_TEST,
                                consumer,
                                TEST_UPGRADE,
                                Default::default(),
                            )
                            .await)
                    }
                    .boxed()
                })
                .await
                .expect("second query node should start");
            ctx.start_consensus().await;
            ctx
        }
    };

    let mut query_node = start_query_node(tmp_options(&storage[1])).await;

    let api_client: Client<ClientErr, SequencerApiVersion> = Client::new(api_url);
    let query_client: Client<ClientErr, SequencerApiVersion> = Client::new(query_url);
    assert!(
        api_client.connect(Some(Duration::from_secs(60))).await,
        "node 0 query API did not come up"
    );
    assert!(
        query_client.connect(Some(Duration::from_secs(60))).await,
        "node 1 query API did not come up"
    );

    // Run the network for seven epochs, watching a validator that stays up
    // for the whole test.
    let mut events = network.peers[0].event_stream();
    wait_for_epochs(&mut events, EPOCH_HEIGHT, EPOCHS_BEFORE_RESTART).await;

    // The node about to be wiped must be healthy first, otherwise the test
    // would prove nothing about recovering from a wipe.
    let height_before_restart = query_node.decided_leaf().await.height();
    timeout(
        RECOVERY_TIMEOUT,
        wait_until_block_height(
            &query_client,
            "block-state/block-height",
            height_before_restart,
        ),
    )
    .await
    .context("second query node was not caught up before the restart")?;

    // Pick a height the new protocol finalized with a cert2 to check the
    // restarted node backfills those too. A cert2 is only stored at the
    // height it finalizes, so scan back from the tip for one.
    let mut finalized_height = None;
    for height in (1..height_before_restart).rev() {
        if api_client
            .get::<espresso_types::Certificate2<SeqTypes>>(&format!("availability/cert2/{height}"))
            .send()
            .await
            .is_ok()
        {
            finalized_height = Some(height);
            break;
        }
    }
    let finalized_height = finalized_height.context("no cert2 stored on a new protocol chain")?;

    tracing::info!(
        height_before_restart,
        finalized_height,
        "restarting the second query node with fresh storage"
    );
    query_node.shut_down().await;
    // The node has to come back on the same ports: node 0 dials the query
    // API at the peer URL baked in at network construction, and cliquenet
    // peers dial the address registered in the stake table. `shut_down`
    // aborts the server tasks without waiting for them to finish, so poll
    // until the old listeners are actually gone before rebinding.
    let cliquenet_port = network.cfg.known_nodes_with_stake()[1]
        .connect_info
        .as_ref()
        .expect("node 1 registered cliquenet connect info")
        .p2p_addr
        .port();
    timeout(RECOVERY_TIMEOUT, async {
        for port in [query_port, cliquenet_port] {
            while std::net::TcpListener::bind(("127.0.0.1", port)).is_err() {
                sleep(Duration::from_millis(100)).await;
            }
        }
    })
    .await
    .context("shut-down node did not release its ports")?;

    let fresh_storage = SqlDataSource::create_storage().await;
    let query_node = start_query_node(tmp_options(&fresh_storage)).await;
    assert!(
        query_client.connect(Some(Duration::from_secs(60))).await,
        "restarted query API did not come up"
    );

    // First it has to get back to the height it lost, in both the archive and
    // the merklized state derived from it.
    timeout(
        RECOVERY_TIMEOUT,
        wait_until_block_height(&query_client, "status/block-height", height_before_restart),
    )
    .await
    .context("restarted node did not rebuild its archive")?;
    timeout(
        RECOVERY_TIMEOUT,
        wait_until_block_height(
            &query_client,
            "block-state/block-height",
            height_before_restart,
        ),
    )
    .await
    .context("restarted node did not rebuild its merklized state")?;

    // Reaching the tip only proves it is following consensus again, so stream
    // the whole range it lost from both nodes: the restarted node's copy can
    // only have come from node 0, and it has to be the chain the network
    // actually decided. Streaming also forces the restarted node to backfill
    // every leaf in the range, since the stream endpoint fetches on demand.
    let wiped_range = (height_before_restart - 1) as usize;
    let stream_leaves = |client: Client<ClientErr, SequencerApiVersion>, who: &'static str| async move {
        let leaves: Vec<LeafQueryData<SeqTypes>> = client
            .socket("availability/stream/leaves/1")
            .subscribe()
            .await
            .with_context(|| format!("subscribing to {who}'s leaf stream"))?
            .take(wiped_range)
            .try_collect()
            .await
            .with_context(|| format!("{who}'s leaf stream errored"))?;
        anyhow::Ok(leaves)
    };
    // Node 0's stream returns immediately, so the shared timeout is in
    // practice a bound on the restarted node's backfill.
    let (ours, theirs) = timeout(
        RECOVERY_TIMEOUT,
        future::try_join(
            stream_leaves(query_client.clone(), "the restarted node"),
            stream_leaves(api_client.clone(), "node 0"),
        ),
    )
    .await
    .context("streaming the wiped range stalled")??;
    assert_eq!(
        ours.len(),
        wiped_range,
        "restarted node's leaf stream ended early"
    );
    assert_eq!(
        theirs.len(),
        wiped_range,
        "node 0's leaf stream ended early"
    );
    for (ours, theirs) in ours.iter().zip(&theirs) {
        assert_eq!(
            ours.hash(),
            theirs.hash(),
            "restarted node's leaf at height {} diverges from node 0",
            ours.height(),
        );
        assert_eq!(
            ours.header().version(),
            NEW_PROTOCOL_VERSION,
            "block {} should have been produced under the new protocol",
            ours.height(),
        );
    }

    let cert2 = timeout(RECOVERY_TIMEOUT, async {
        loop {
            match query_client
                .get::<espresso_types::Certificate2<SeqTypes>>(&format!(
                    "availability/cert2/{finalized_height}"
                ))
                .send()
                .await
            {
                Ok(cert2) => break cert2,
                Err(err) => {
                    tracing::info!(finalized_height, %err, "cert2 not backfilled yet")
                },
            }
            sleep(Duration::from_secs(2)).await;
        }
    })
    .await
    .context("restarted node did not backfill the cert2")?;
    assert_eq!(cert2.data.block_number, finalized_height);

    // Tracking decides is not enough to call the node a participant: wait for
    // a block it proposed itself to be decided after the restart.
    let node_1_key = network.cfg.known_nodes_with_stake()[1]
        .stake_table_entry
        .stake_key;
    let coordinator = query_node.node_state().coordinator;
    let mut events = query_node.event_stream();
    let proposed = timeout(RECOVERY_TIMEOUT, async {
        while let Some(event) = events.next().await {
            let leaf_infos: &[LeafInfo<SeqTypes>] = match &event {
                CoordinatorEvent::NewDecide { leaf_infos, .. } => leaf_infos,
                _ => continue,
            };
            for LeafInfo { leaf, .. } in leaf_infos {
                if leaf.height() <= height_before_restart {
                    continue;
                }
                let membership = match coordinator.membership_for_epoch(leaf.epoch(EPOCH_HEIGHT)) {
                    Ok(membership) => membership,
                    Err(err) => {
                        tracing::warn!(
                            height = leaf.height(),
                            %err,
                            "no membership for epoch",
                        );
                        continue;
                    },
                };
                match membership.leader(leaf.view_number()) {
                    Ok(leader) if leader == node_1_key => return Some(leaf.height()),
                    Ok(_) => {},
                    Err(err) => {
                        tracing::warn!(view = ?leaf.view_number(), %err, "leader unresolved");
                    },
                }
            }
        }
        None
    })
    .await
    .context("restarted node's event stream stalled")?;
    let proposed = proposed.context("restarted node's event stream ended")?;
    tracing::info!(proposed, "restarted node proposed a decided block");

    Ok(())
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_merklized_state_catchup_on_restart() -> anyhow::Result<()> {
    // This test verifies that a query node can catch up on
    // merklized state after being offline for multiple epochs.
    //
    // Steps:
    // 1. Start a test network with 5 sequencer nodes.
    // 2. Start a separate node with the query module enabled, connected to the network.
    //    - This node stores merklized state
    // 3. Shut down the query node after 1 epoch.
    // 4. Allow the network to progress 3 more epochs (query node remains offline).
    // 5. Restart the query node.
    //    - The node is expected to reconstruct or catch up on its own
    use espresso_types::{DECAF_CHAIN_ID, v0_3::ChainConfig};

    const EPOCH_HEIGHT: u64 = 10;

    let network_config = TestConfigBuilder::default()
        .epoch_height(EPOCH_HEIGHT)
        .build();

    let api_port = reserve_tcp_port().expect("OS should have ephemeral ports available");

    tracing::info!("API PORT = {api_port}");
    const NUM_NODES: usize = 5;

    let (storage, persistence) = node_storage::<SqlDataSource, NUM_NODES>().await;

    // The light client skips epoch-root stake-table-hash verification for pre-DRB headers only
    // on the Decaf chain id, so use it to keep the V3->V4 catchup path covered. The stake
    // table deploy in `TestNetwork::new` keeps the chain id from `state[0]`.
    let decaf_state = ValidatedState {
        chain_config: ChainConfig {
            chain_id: DECAF_CHAIN_ID,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };

    let config = TestNetworkConfigBuilder::with_num_nodes()
        .api_config(SqlDataSource::options(
            &storage[0],
            Options::with_port(api_port)
                .catchup(Default::default())
                .light_client(Default::default()),
        ))
        .network_config(network_config)
        .persistences(persistence.clone())
        .states(std::array::from_fn(|_| decaf_state.clone()))
        .build();
    let mut network = TestNetwork::new(config, TEST_UPGRADE).await;
    let state = network.genesis_state(0);

    // Remove peer 0 and restart it with the query module enabled.
    // Adding an additional node to the test network is not straight forward,
    // as the keys have already been initialized in the config above.
    // So, we remove this node and re-add it using the same index.
    network.peers[0].shut_down().await;
    network.peers.remove(0);
    let node_0_storage = &storage[1];
    let node_0_persistence = persistence[1].clone();
    let node_0_port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    tracing::info!("node_0_port {node_0_port}");
    // enable query module with api peers
    let opt = Options::with_port(node_0_port).query_sql(
        Query {
            peers: vec![format!("http://localhost:{api_port}").parse().unwrap()],
            ..Default::default()
        },
        tmp_options(node_0_storage),
    );

    // start the query node so that it builds the merklized state
    let mut node_0 = opt
        .clone()
        .serve(|metrics, consumer, storage| {
            let cfg = network.cfg.clone();
            let node_0_persistence = node_0_persistence.clone();
            let state = state.clone();
            async move {
                Ok(cfg
                    .init_node(
                        1,
                        state,
                        node_0_persistence.clone(),
                        Some(StatePeers::<StaticVersion<0, 1>>::from_urls(
                            vec![format!("http://localhost:{api_port}").parse().unwrap()],
                            Default::default(),
                            Duration::from_secs(2),
                            &NoMetrics,
                        )),
                        storage,
                        &*metrics,
                        test_helpers::STAKE_TABLE_CAPACITY_FOR_TEST,
                        consumer,
                        TEST_UPGRADE,
                        Default::default(),
                    )
                    .await)
            }
            .boxed()
        })
        .await
        .unwrap();
    node_0.start_consensus().await;

    let mut events = network.peers[2].event_stream();
    // wait for 1 epoch
    wait_for_epochs(&mut events, EPOCH_HEIGHT, 1).await;

    // Shut the node down for 3 epochs. Await it so its cliquenet listener releases its port
    // before the restarted node binds the same address.
    node_0.shut_down().await;

    // wait for 4 epochs
    wait_for_epochs(&mut events, EPOCH_HEIGHT, 4).await;

    // start the node again.
    tracing::info!("restarting node");
    let node_0 = opt
        .serve(|metrics, consumer, storage| {
            let cfg = network.cfg.clone();
            async move {
                Ok(cfg
                    .init_node(
                        1,
                        state,
                        node_0_persistence,
                        Some(StatePeers::<StaticVersion<0, 1>>::from_urls(
                            vec![format!("http://localhost:{api_port}").parse().unwrap()],
                            Default::default(),
                            Duration::from_secs(2),
                            &NoMetrics,
                        )),
                        storage,
                        &*metrics,
                        test_helpers::STAKE_TABLE_CAPACITY_FOR_TEST,
                        consumer,
                        TEST_UPGRADE,
                        Default::default(),
                    )
                    .await)
            }
            .boxed()
        })
        .await
        .unwrap();
    node_0.start_consensus().await;

    let client: Client<ClientErr, SequencerApiVersion> =
        Client::new(format!("http://localhost:{node_0_port}").parse().unwrap());
    client.connect(None).await;

    wait_for_epochs(&mut events, EPOCH_HEIGHT, 6).await;

    let epoch_7_block = EPOCH_HEIGHT * 6 + 1;

    // check that the node's state has reward accounts
    let mut retries = 0;
    loop {
        sleep(Duration::from_secs(1)).await;
        let leaves = node_0
            .decided_state()
            .await
            .map_or(0, |state| state.reward_merkle_tree_v2.num_leaves());

        if leaves > 0 {
            tracing::info!("Node's state has reward accounts");
            break;
        }

        retries += 1;
        if retries > 120 {
            panic!("max retries reached. failed to catchup reward state");
        }
    }

    retries = 0;
    // check that the node has stored atleast 6 epochs merklized state in persistence
    loop {
        sleep(Duration::from_secs(3)).await;

        let bh = client
            .get::<u64>("block-state/block-height")
            .send()
            .await
            .expect("block height not found");

        tracing::info!("block state: block height={bh}");
        if bh > epoch_7_block {
            break;
        }

        retries += 1;
        if retries > 30 {
            panic!(
                "max retries reached. block state block height is less than epoch 7 start block"
            );
        }
    }

    // Read before shutting consensus down: once the coordinator stops, these fall back to the
    // legacy handle, which never ran at 0.6.
    let decided_leaf = node_0.decided_leaf().await;
    let state = node_0.decided_state().await.unwrap();
    node_0.shutdown_consensus().await;
    tracing::info!(
        height = decided_leaf.height(),
        ?decided_leaf,
        ?state,
        "final state"
    );

    let height = decided_leaf.height();
    let num_leaves = state.block_merkle_tree.num_leaves();
    tracing::info!(height, num_leaves, "checking block merkle tree state");
    state
        .block_merkle_tree
        .lookup(height - 1)
        .expect_ok()
        .unwrap_or_else(|err| {
            panic!(
                "block state not found ({err:#}):\n{:#?}",
                state.block_merkle_tree
            )
        });

    Ok(())
}

/// A node that was down for longer than its state retention comes back with its merklized
/// state well behind the chain. The state loop can only continue from its newest snapshot, so
/// the state pruner must not get ahead of it. Restart a query node behind a pruned height that
/// an unbounded pruner run has already stamped past its state head, and check that the loop
/// still resumes and reaches the tip, and that every pruner run from then on stamps below the
/// state head.
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_state_pruner_stays_below_state_head() -> anyhow::Result<()> {
    const NUM_NODES: usize = 5;
    // Blocks the chain moves on by while the query node is down, and the margin by which the
    // loop has to get past the stale pruned height before the check ends.
    const GAP: u64 = 10;

    let (storage, persistence) = node_storage::<SqlDataSource, NUM_NODES>().await;
    let api_port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    let api_url: Url = format!("http://localhost:{api_port}").parse().unwrap();
    // The query node fetches what it missed from the API node, so the API node serves catchup
    // and light client proofs.
    let config = TestNetworkConfigBuilder::with_num_nodes()
        .api_config(SqlDataSource::options(
            &storage[0],
            Options::with_port(api_port)
                .catchup(Default::default())
                .light_client(Default::default()),
        ))
        .network_config(TestConfigBuilder::default().build())
        .persistences(persistence.clone())
        .build();
    let mut network = TestNetwork::new(config, TEST_UPGRADE).await;
    let genesis_state = network.genesis_state(0);

    // Replace peer 0 with a query node whose state pruner runs every second with zero
    // retention: left to itself, every run would prune right up to the chain tip. Consensus
    // data stays out of it: the test genesis has timestamp 0, so any data retention would
    // delete the genesis leaf the loop's first catchup reads.
    network.peers[0].shut_down().await;
    network.peers.remove(0);
    let query_port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    let mut db_opt = tmp_options(&storage[1]);
    db_opt.prune = true;
    db_opt.pruning = <persistence::sql::PruningOptions as clap::Parser>::parse_from([
        "pruning",
        "--target-retention",
        "4294967295s",
        "--state-target-retention",
        "0s",
        "--interval",
        "1s",
    ]);
    let start_query_node = {
        let cfg = network.cfg.clone();
        let node_persistence = persistence[1].clone();
        let db_opt = db_opt.clone();
        let api_url = api_url.clone();
        move || {
            let cfg = cfg.clone();
            let genesis_state = genesis_state.clone();
            let node_persistence = node_persistence.clone();
            let db_opt = db_opt.clone();
            let api_url = api_url.clone();
            async move {
                let opt = Options::with_port(query_port).query_sql(
                    Query {
                        peers: vec![api_url.clone()],
                        ..Default::default()
                    },
                    db_opt,
                );
                let ctx = opt
                    .serve(move |metrics, consumer, storage| {
                        async move {
                            Ok(cfg
                                .init_node(
                                    1,
                                    genesis_state,
                                    node_persistence,
                                    Some(StatePeers::<StaticVersion<0, 1>>::from_urls(
                                        vec![api_url],
                                        Default::default(),
                                        Duration::from_secs(2),
                                        &NoMetrics,
                                    )),
                                    storage,
                                    &*metrics,
                                    STAKE_TABLE_CAPACITY_FOR_TEST,
                                    consumer,
                                    TEST_UPGRADE,
                                    Default::default(),
                                )
                                .await)
                        }
                        .boxed()
                    })
                    .await
                    .expect("query node should start");
                ctx.start_consensus().await;
                ctx
            }
        }
    };

    let mut query_node = start_query_node().await;
    let query_client: Client<ClientErr, StaticVersion<0, 1>> =
        Client::new(format!("http://localhost:{query_port}").parse().unwrap());
    assert!(query_client.connect(Some(Duration::from_secs(60))).await);
    let api_client: Client<ClientErr, StaticVersion<0, 1>> = Client::new(api_url.clone());
    assert!(api_client.connect(Some(Duration::from_secs(60))).await);

    // Let the loop build some state, then take the node down while the chain moves on.
    wait_until_block_height(&query_client, "block-state/block-height", 5).await;
    tracing::info!("query node has state; shutting it down");
    query_node.shut_down().await;
    drop(query_node);
    // `shut_down` aborts the server task without waiting for it, so wait for the port before
    // rebinding it.
    timeout(Duration::from_secs(30), async {
        while std::net::TcpListener::bind(("127.0.0.1", query_port)).is_err() {
            sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .context("shut-down query node did not release its port")?;
    let db = SqlStorage::connect(Config::try_from(&db_opt)?, StorageConnectionType::Query).await?;
    let (_, head_at_shutdown) = state_heights(&db).await;
    wait_until_block_height(&api_client, "status/block-height", head_at_shutdown + GAP).await;
    // `status/block-height` counts blocks, so the tip is one below it.
    let stale_cursor = api_client.get::<u64>("status/block-height").send().await? - 1;
    tracing::info!(
        head_at_shutdown,
        stale_cursor,
        "chain moved on; restarting query node"
    );

    // A pruner from before the state pruner was bounded by the state head could have stamped
    // past the loop while it was behind. Stamp the tip as such a run would have; id 2 is the
    // state cursor.
    {
        let mut tx = db.write().await?;
        tx.upsert(
            "pruned_height",
            ["id", "last_height"],
            ["id"],
            [(2i32, stale_cursor as i64)],
        )
        .await?;
        tx.commit().await?;
    }
    ensure!(
        state_heights(&db).await.0 == Some(stale_cursor),
        "the stale cursor was not stamped"
    );

    // The loop has to resume from its head, get past the stale cursor and reach the tip, and
    // every pruner run from then on has to stamp below the head.
    let _query_node = start_query_node().await;
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        let (pruned, head) = state_heights(&db).await;
        let pruned = pruned.context("state pruned height is stamped")?;
        tracing::info!(pruned, head, stale_cursor, "state loop progress");
        if pruned > stale_cursor {
            ensure!(
                pruned < head,
                "state pruner passed the state head: pruned {pruned}, head {head}"
            );
            if head > stale_cursor + GAP {
                break;
            }
        }
        ensure!(
            Instant::now() < deadline,
            "state loop did not get past the stale pruned height {stale_cursor}: pruned {pruned}, \
             head {head}"
        );
        sleep(Duration::from_millis(200)).await;
    }

    // The head snapshot is served. The loop and the pruner keep moving, so a head read just
    // before a pruner run can be below the cursor by the time the query runs; read it again.
    for attempt in 1.. {
        let (_, head) = state_heights(&db).await;
        let frontier = query_client
            .get::<MerkleProof<Commitment<Header>, u64, Sha3Node, 3>>(&format!(
                "block-state/{head}/{}",
                head - 1
            ))
            .send()
            .await;
        match frontier {
            Ok(_) => break,
            Err(err) if attempt < 10 => {
                tracing::info!(head, %err, "retrying the head frontier query");
                sleep(Duration::from_millis(200)).await;
            },
            Err(err) => bail!("block frontier at the state head {head} is not served: {err}"),
        }
    }
    Ok(())
}

/// When the state loop meets the data pruned height, as on a node that was down for longer
/// than its data retention.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DataPruned {
    /// The marker was stamped past the state head while the node was down: the loop finds
    /// its head leaf gone and starts on peers.
    BeforeRestart,
    /// The marker is stamped while the loop waits on the local stream for a leaf that never
    /// comes: the loop has to notice and switch to peers. The loop is put well behind first,
    /// as a long outage would, since a gap above the consensus anchor is re-decided on
    /// restart and cannot be kept open.
    WhileWaiting,
}

/// The availability layer never fetches a leaf at or below the data pruned height, so the
/// state loop has to get the leaves it still needs from peers, without storing them, and
/// carry on from the local stream once it is past the pruned height. Peers' leaf chains are
/// verified against per-epoch stake tables, so the network runs with epochs.
#[rstest]
#[case::before_restart(DataPruned::BeforeRestart)]
#[case::while_waiting(DataPruned::WhileWaiting)]
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_state_loop_fetches_pruned_leaves_from_peers(
    #[case] pruned_when: DataPruned,
) -> anyhow::Result<()> {
    const NUM_NODES: usize = 5;
    const EPOCH_HEIGHT: u64 = 10;
    // Leaves the node holds beyond the state head when it goes down, the margin by which the
    // loop has to get past the pruned height before the check ends, and the head the loop is
    // put back to when it has to be left waiting.
    const SYNCED: u64 = 5;
    const GAP: u64 = 10;
    const BEHIND: u64 = 2;

    let storage = join_all((0..NUM_NODES).map(|_| SqlDataSource::create_storage())).await;
    let persistence: [_; NUM_NODES] = storage
        .iter()
        .map(<SqlDataSource as TestableSequencerDataSource>::persistence_options)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let api_port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    let api_url: Url = format!("http://localhost:{api_port}").parse().unwrap();
    // The API node is the peer the query node fetches leaves from.
    let config = TestNetworkConfigBuilder::with_num_nodes()
        .api_config(SqlDataSource::options(
            &storage[0],
            Options::with_port(api_port)
                .catchup(Default::default())
                .light_client(Default::default()),
        ))
        .network_config(
            TestConfigBuilder::default()
                .epoch_height(EPOCH_HEIGHT)
                .build(),
        )
        .persistences(persistence.clone())
        .catchups(std::array::from_fn(|_| {
            StatePeers::<StaticVersion<0, 1>>::from_urls(
                vec![api_url.clone()],
                Default::default(),
                Duration::from_secs(2),
                &NoMetrics,
            )
        }))
        .pos_hook(
            DelegationConfig::MultipleDelegators,
            StakeTableContractVersion::V3,
            TEST_UPGRADE,
        )
        .await?
        .build();
    let genesis_state = config.states()[0].clone();
    let mut network = TestNetwork::new(config, TEST_UPGRADE).await;

    // Replace peer 0 with a query node. Its availability fetcher gets the given peers: the
    // API node, except when the loop has to be left waiting for a leaf that never arrives.
    network.peers[0].shut_down().await;
    network.peers.remove(0);
    let query_port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    let db_opt = tmp_options(&storage[1]);
    let start_query_node = {
        let cfg = network.cfg.clone();
        let node_persistence = persistence[1].clone();
        let db_opt = db_opt.clone();
        let api_url = api_url.clone();
        move |availability_peers: Vec<Url>| {
            let cfg = cfg.clone();
            let genesis_state = genesis_state.clone();
            let node_persistence = node_persistence.clone();
            let db_opt = db_opt.clone();
            let api_url = api_url.clone();
            async move {
                let opt = Options::with_port(query_port).query_sql(
                    Query {
                        peers: availability_peers,
                        ..Default::default()
                    },
                    db_opt,
                );
                let ctx = opt
                    .serve(move |metrics, consumer, storage| {
                        async move {
                            Ok(cfg
                                .init_node(
                                    1,
                                    genesis_state,
                                    node_persistence,
                                    Some(StatePeers::<StaticVersion<0, 1>>::from_urls(
                                        vec![api_url],
                                        Default::default(),
                                        Duration::from_secs(2),
                                        &NoMetrics,
                                    )),
                                    storage,
                                    &*metrics,
                                    STAKE_TABLE_CAPACITY_FOR_TEST,
                                    consumer,
                                    TEST_UPGRADE,
                                    Default::default(),
                                )
                                .await)
                        }
                        .boxed()
                    })
                    .await
                    .expect("query node should start");
                ctx.start_consensus().await;
                ctx
            }
        }
    };

    let mut query_node = start_query_node(vec![api_url.clone()]).await;
    let query_client: Client<ClientErr, StaticVersion<0, 1>> =
        Client::new(format!("http://localhost:{query_port}").parse().unwrap());
    assert!(query_client.connect(Some(Duration::from_secs(60))).await);
    let api_client: Client<ClientErr, StaticVersion<0, 1>> = Client::new(api_url.clone());
    assert!(api_client.connect(Some(Duration::from_secs(60))).await);

    // Let the loop build some state, then take the node down while the chain moves on.
    wait_until_block_height(&query_client, "block-state/block-height", 5).await;
    query_node.shut_down().await;
    drop(query_node);
    // `shut_down` aborts the server task without waiting for it, so wait for the port before
    // rebinding it.
    timeout(Duration::from_secs(30), async {
        while std::net::TcpListener::bind(("127.0.0.1", query_port)).is_err() {
            sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .context("shut-down query node did not release its port")?;
    let db = SqlStorage::connect(Config::try_from(&db_opt)?, StorageConnectionType::Query).await?;
    let (_, head_at_shutdown) = state_heights(&db).await;
    let local_tip = local_block_height(&db).await? - 1;

    let _restarted;
    let pruned = match pruned_when {
        DataPruned::BeforeRestart => {
            // Prune consensus data past the state head, as a pruner run during the downtime
            // would have once the retention passed. A run never stamps past the node's own
            // tip, so first give the node a leaf beyond the marker, as its fetcher would have
            // before the outage outran the loop.
            let synced_tip = local_tip + SYNCED;
            let pruned = synced_tip - 1;
            wait_until_block_height(&api_client, "status/block-height", synced_tip + 1).await;
            let synced_leaf = api_client
                .get::<LeafQueryData<SeqTypes>>(&format!("availability/leaf/{synced_tip}"))
                .send()
                .await?;
            let mut tx = db.write().await?;
            tx.insert_leaf(&synced_leaf).await?;
            tx.commit().await?;
            prune_data_to(&db, pruned).await?;
            _restarted = start_query_node(vec![api_url.clone()]).await;
            pruned
        },
        DataPruned::WhileWaiting => {
            // Put the loop behind its own storage and take the leaves above its new head out
            // of local reach: below the consensus anchor they are never re-decided, and
            // without availability peers they are never fetched. The loop then waits on the
            // stream for the leaf above its head while consensus moves the tip on.
            {
                let mut tx = db.write().await?;
                UpdateStateData::<SeqTypes, BlockMerkleTree, { BlockMerkleTree::ARITY }>::set_last_state_height(
                    &mut tx,
                    BEHIND as usize,
                )
                .await?;
                tx.commit().await?;
            }
            delete_data_between(&db, BEHIND, head_at_shutdown).await?;
            _restarted = start_query_node(vec![]).await;
            let deadline = Instant::now() + Duration::from_secs(120);
            loop {
                if local_block_height(&db).await? > local_tip + GAP {
                    break;
                }
                ensure!(
                    Instant::now() < deadline,
                    "consensus did not move the local tip past {}",
                    local_tip + GAP
                );
                sleep(Duration::from_millis(200)).await;
            }
            let (_, head) = state_heights(&db).await;
            ensure!(
                head == BEHIND,
                "the loop moved to {head} without the leaf above {BEHIND}"
            );
            // Now stamp past the height the loop is waiting for. The old head is well below
            // the tip, as a real run leaves it.
            let pruned = head_at_shutdown;
            prune_data_to(&db, pruned).await?;
            pruned
        },
    };

    // The loop has to get past the pruned height on leaves from the API node, and then follow
    // the local stream.
    let deadline = Instant::now() + Duration::from_secs(180);
    loop {
        let (_, head) = state_heights(&db).await;
        if head > pruned + GAP {
            break;
        }
        ensure!(
            Instant::now() < deadline,
            "state loop did not get past the data pruned height {pruned}: head {head}, started \
             from {head_at_shutdown}"
        );
        sleep(Duration::from_millis(200)).await;
    }

    // The leaves the loop fetched from peers were not stored: the pruner would only delete
    // them again.
    let mut tx = db.read().await?;
    let stored: i64 = sqlx::query_scalar("SELECT count(*) FROM leaf2 WHERE height <= $1")
        .bind(pruned as i64)
        .fetch_one(tx.as_mut())
        .await?;
    ensure!(
        stored == 0,
        "{stored} leaves at or below the data pruned height {pruned} were stored"
    );
    Ok(())
}

/// The number of blocks in the node's own availability storage.
async fn local_block_height(db: &SqlStorage) -> anyhow::Result<u64> {
    let mut tx = db.read().await?;
    Ok(NodeStorage::<SeqTypes>::block_height(&mut tx).await? as u64)
}

/// Prune consensus data to `pruned` as a pruner batch would: stamp the data cursor, id 1,
/// then delete what `delete_batch` deletes. The node may be writing new blocks meanwhile, so
/// a serialization conflict is retried.
async fn prune_data_to(db: &SqlStorage, pruned: u64) -> anyhow::Result<()> {
    serializable_retry!(db, || async {
        let mut tx = db.write().await?;
        tx.upsert(
            "pruned_height",
            ["id", "last_height"],
            ["id"],
            [(1i32, pruned as i64)],
        )
        .await?;
        for statement in [
            "DELETE FROM transactions WHERE block_height <= $1",
            "DELETE FROM leaf2 WHERE height <= $1",
            "DELETE FROM header WHERE height <= $1",
        ] {
            sqlx::query(statement)
                .bind(pruned as i64)
                .execute(tx.as_mut())
                .await?;
        }
        tx.commit().await
    })
    .await
}

/// Delete consensus data above `from` up to and including `to` without moving the data
/// cursor, as if the node had never had it.
async fn delete_data_between(db: &SqlStorage, from: u64, to: u64) -> anyhow::Result<()> {
    let mut tx = db.write().await?;
    for statement in [
        "DELETE FROM transactions WHERE block_height > $1 AND block_height <= $2",
        "DELETE FROM leaf2 WHERE height > $1 AND height <= $2",
        "DELETE FROM header WHERE height > $1 AND height <= $2",
    ] {
        sqlx::query(statement)
            .bind(from as i64)
            .bind(to as i64)
            .execute(tx.as_mut())
            .await?;
    }
    tx.commit().await
}

/// The state pruned height, then the state head. Read in that order: every move of the
/// cursor, up by a batch or down by the startup repair, lands below the head, and the head
/// only grows, so a head read after the cursor is always above it.
async fn state_heights(db: &SqlStorage) -> (Option<u64>, u64) {
    let mut tx = db.read().await.unwrap();
    let pruned = tx.load_state_pruned_height().await.unwrap();
    let head = tx.get_last_state_height().await.unwrap() as u64;
    (pruned, head)
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_state_reconstruction() -> anyhow::Result<()> {
    // This test verifies that a query node can successfully reconstruct its state
    // after being shut down from the database
    //
    // Steps:
    // 1. Start a test network with 5 nodes.
    // 2. Add a query node connected to the network.
    // 3. Let the network run until 3 epochs have passed.
    // 4. Shut down the query node.
    // 5. Attempt to reconstruct its state from storage using:
    //    - No fee/reward accounts
    //    - Only fee accounts
    //    - Only reward accounts
    //    - Both fee and reward accounts
    // 6. Assert that the reconstructed state is correct in all scenarios.

    const EPOCH_HEIGHT: u64 = 10;

    let network_config = TestConfigBuilder::default()
        .epoch_height(EPOCH_HEIGHT)
        .build();

    let api_port = reserve_tcp_port().expect("OS should have ephemeral ports available");

    tracing::info!("API PORT = {api_port}");
    const NUM_NODES: usize = 5;

    let (storage, persistence) = node_storage::<SqlDataSource, NUM_NODES>().await;

    let config = TestNetworkConfigBuilder::with_num_nodes()
        .api_config(SqlDataSource::options(
            &storage[0],
            Options::with_port(api_port).light_client(Default::default()),
        ))
        .network_config(network_config)
        .persistences(persistence.clone())
        .build();
    let mut network = TestNetwork::new(config, TEST_UPGRADE).await;
    let state = network.genesis_state(0);
    // Remove peer 0 and restart it with the query module enabled.
    // Adding an additional node to the test network is not straight forward,
    // as the keys have already been initialized in the config above.
    // So, we remove this node and re-add it using the same index.
    // Await the shutdown so the node's cliquenet listener releases its
    // port before the replacement node binds the same address.
    network.peers[0].shut_down().await;
    network.peers.remove(0);

    let node_0_storage = &storage[1];
    let node_0_persistence = persistence[1].clone();
    let node_0_port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    tracing::info!("node_0_port {node_0_port}");
    let opt = Options::with_port(node_0_port).query_sql(
        Query {
            peers: vec![format!("http://localhost:{api_port}").parse().unwrap()],
            ..Query::test()
        },
        tmp_options(node_0_storage),
    );
    let node_0 = opt
        .clone()
        .serve(|metrics, consumer, storage| {
            let cfg = network.cfg.clone();
            let node_0_persistence = node_0_persistence.clone();
            let state = state.clone();
            async move {
                Ok(cfg
                    .init_node(
                        1,
                        state,
                        node_0_persistence.clone(),
                        Some(StatePeers::<StaticVersion<0, 1>>::from_urls(
                            vec![format!("http://localhost:{api_port}").parse().unwrap()],
                            Default::default(),
                            Duration::from_secs(2),
                            &NoMetrics,
                        )),
                        storage,
                        &*metrics,
                        test_helpers::STAKE_TABLE_CAPACITY_FOR_TEST,
                        consumer,
                        TEST_UPGRADE,
                        Default::default(),
                    )
                    .await)
            }
            .boxed()
        })
        .await
        .unwrap();
    node_0.start_consensus().await;

    let mut events = network.peers[2].event_stream();
    // Wait until at least 3 epochs have passed
    wait_for_epochs(&mut events, EPOCH_HEIGHT, 3).await;

    // Read before shutting consensus down: once the coordinator stops, the decided state falls
    // back to the legacy handle, which never ran at 0.6.
    let instance = node_0.node_state();
    // A node can decide a leaf before it has validated that leaf's state itself.
    let state = timeout(Duration::from_secs(60), async {
        loop {
            if let Some(state) = node_0.decided_state().await {
                break state;
            }
            sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .context("node 0 never validated a decided state")?;
    let fee_accounts = state
        .fee_merkle_tree
        .clone()
        .into_iter()
        .map(|(acct, _)| acct)
        .collect::<Vec<_>>();
    let reward_accounts = state
        .reward_merkle_tree_v2
        .clone()
        .into_iter()
        .map(|(acct, _)| acct)
        .collect::<Vec<_>>();

    // At 0.6 a leaf can be decided before the next view's proposal arrives, and decided
    // proposals are pruned from consensus storage, so shut down only once an undecided
    // proposal is stored for `reconstruct_state` to reach.
    let persistence = node_0.persistence();
    timeout(Duration::from_secs(60), async {
        loop {
            let decided_view = node_0.decided_leaf().await.view_number();
            let proposals = persistence.load_quorum_proposals().await?;
            if proposals.keys().any(|view| *view > decided_view) {
                break anyhow::Ok(());
            }
            sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .context("no undecided proposal was stored")??;

    tracing::warn!("shutting down node 0");
    node_0.shutdown_consensus().await;

    // The oldest stored proposal. It may already be decided, since pruning lags the decide.
    let (to_view, proposal) = persistence
        .load_quorum_proposals()
        .await?
        .into_iter()
        .next()
        .expect("an undecided proposal survives the shutdown");
    let from_height = proposal.data.block_header().height() - 1;

    let client: Client<ClientErr, SequencerApiVersion> =
        Client::new(format!("http://localhost:{node_0_port}").parse().unwrap());
    client.connect(Some(Duration::from_secs(10))).await;
    wait_until_block_height(&client, "node/block-height", from_height + 1).await;
    // `reconstruct_state` loads accounts from the merklized state at `from_height`, which a
    // separate loop writes after the block is indexed.
    wait_until_block_height(&client, "fee-state/block-height", from_height).await;

    let ds = SqlStorage::connect(
        Config::try_from(&node_0_persistence).unwrap(),
        StorageConnectionType::Sequencer,
    )
    .await
    .unwrap();
    let mut tx = ds.read().await?;

    let (state, leaf) = reconstruct_state(&instance, &ds, &mut tx, from_height, to_view, &[], &[])
        .await
        .unwrap();
    assert_eq!(leaf.view_number(), to_view);
    assert!(
        state
            .block_merkle_tree
            .lookup(from_height)
            .expect_ok()
            .is_ok(),
        "inconsistent block merkle tree"
    );

    // Reconstruct fee state
    let (state, leaf) = reconstruct_state(
        &instance,
        &ds,
        &mut tx,
        from_height,
        to_view,
        &fee_accounts,
        &[],
    )
    .await
    .unwrap();

    assert_eq!(leaf.view_number(), to_view);
    assert!(
        state
            .block_merkle_tree
            .lookup(from_height)
            .expect_ok()
            .is_ok(),
        "inconsistent block merkle tree"
    );

    for account in &fee_accounts {
        state.fee_merkle_tree.lookup(account).expect_ok().unwrap();
    }

    // Reconstruct reward state

    let (state, leaf) = reconstruct_state(
        &instance,
        &ds,
        &mut tx,
        from_height,
        to_view,
        &[],
        &reward_accounts,
    )
    .await
    .unwrap();

    for account in &reward_accounts {
        state
            .reward_merkle_tree_v2
            .lookup(account)
            .expect_ok()
            .unwrap();
    }

    assert_eq!(leaf.view_number(), to_view);
    assert!(
        state
            .block_merkle_tree
            .lookup(from_height)
            .expect_ok()
            .is_ok(),
        "inconsistent block merkle tree"
    );
    // Reconstruct reward and fee state

    let (state, leaf) = reconstruct_state(
        &instance,
        &ds,
        &mut tx,
        from_height,
        to_view,
        &fee_accounts,
        &reward_accounts,
    )
    .await
    .unwrap();

    assert!(
        state
            .block_merkle_tree
            .lookup(from_height)
            .expect_ok()
            .is_ok(),
        "inconsistent block merkle tree"
    );
    assert_eq!(leaf.view_number(), to_view);

    for account in &reward_accounts {
        state
            .reward_merkle_tree_v2
            .lookup(account)
            .expect_ok()
            .unwrap();
    }

    for account in &fee_accounts {
        state.fee_merkle_tree.lookup(account).expect_ok().unwrap();
    }

    Ok(())
}

/// Test that `fetch_leaf` returns a leaf with exactly the requested block height.
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_fetch_leaf_returns_exact_height() -> anyhow::Result<()> {
    const EPOCH_HEIGHT: u64 = 10;
    const NUM_NODES: usize = 5;
    const TARGET_HEIGHT: u64 = EPOCH_HEIGHT * 3 + 2;

    let network_config = TestConfigBuilder::default()
        .epoch_height(EPOCH_HEIGHT)
        .build();

    let port = reserve_tcp_port().expect("No ports free for query service");

    let (storage, persistence) = node_storage::<SqlDataSource, NUM_NODES>().await;

    let catchup_peers = std::array::from_fn(|_| {
        StatePeers::<StaticVersion<0, 1>>::from_urls(
            vec![format!("http://localhost:{port}").parse().unwrap()],
            Default::default(),
            Duration::from_secs(2),
            &NoMetrics,
        )
    });

    let config = TestNetworkConfigBuilder::with_num_nodes()
        .api_config(SqlDataSource::options(
            &storage[0],
            Options::with_port(port),
        ))
        .network_config(network_config)
        .persistences(persistence)
        .catchups(catchup_peers)
        .build();

    let network = TestNetwork::new(config, TEST_UPGRADE).await;

    // Wait for chain to advance past our target height
    let height_client: Client<ClientErr, StaticVersion<0, 1>> =
        Client::new(format!("http://localhost:{port}").parse().unwrap());
    wait_until_block_height(&height_client, "node/block-height", TARGET_HEIGHT + 5).await;

    let coordinator = network.server.node_state().coordinator;

    // Use StatePeers to fetch the leaf at the exact target height
    let catchup = StatePeers::<StaticVersion<0, 1>>::from_urls(
        vec![format!("http://localhost:{port}").parse().unwrap()],
        Default::default(),
        Duration::from_secs(5),
        &NoMetrics,
    );

    let leaf = catchup.fetch_leaf(coordinator, TARGET_HEIGHT).await?;

    assert_eq!(
        leaf.height(),
        TARGET_HEIGHT,
        "fetch_leaf must return the leaf at exactly the requested height"
    );

    Ok(())
}

/// Start a network at V5 from genesis, restart ALL nodes just before an epoch transition block
/// (`boundary - 3`), and confirm the chain keeps producing blocks afterward.
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_v5_restart_before_epoch_boundary() {
    const NUM_NODES: usize = 3;
    const EPOCH_HEIGHT: u64 = 10;
    // Rewards start in epoch 4. Restart 3 blocks before the boundary that closes epoch 4, so
    // the restarted network produces the boundary block (`4 * EPOCH_HEIGHT`) itself.
    const RESTART_EPOCH: u64 = 4;
    const RESTART_BOUNDARY: u64 = RESTART_EPOCH * EPOCH_HEIGHT;
    const RESTART_HEIGHT: u64 = RESTART_BOUNDARY - 3;
    // Blocks to produce after the restart before declaring success.
    const BLOCKS_AFTER_RESTART: u64 = 5;

    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");

    // Slow empty-block production so we comfortably stop at the exact target height. On an idle
    // chain the empty-block time is ~`builder_timeout`; raise `next_view_timeout` above it so
    // the slow (but healthy) views aren't treated as failures.
    let network_config = TestConfigBuilder::<NUM_NODES>::default()
        .epoch_height(EPOCH_HEIGHT)
        .builder_timeout(Duration::from_secs(3))
        .next_view_timeout(Duration::from_secs(10))
        .build();

    let (storage, persistence) = node_storage::<SqlDataSource, NUM_NODES>().await;

    let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
        .api_config(SqlDataSource::options(
            &storage[0],
            Options::with_port(port),
        ))
        .persistences(persistence.clone())
        .network_config(network_config)
        .build();

    let mut network = TestNetwork::new(config, TEST_UPGRADE).await;

    // Watch the decide stream and stop as soon as `boundary - 3` is decided. Consuming the
    // stream (rather than polling `decided_leaf`) makes this independent of block timing: we
    // see every decided leaf and stop the network the moment the target height appears, so we
    // can never race past it regardless of how fast blocks are produced.
    {
        let mut events = network.server.event_stream();
        'wait: loop {
            let event = events
                .next()
                .await
                .expect("event stream ended unexpectedly");
            let CoordinatorEvent::NewDecide {
                leaf_infos: leaf_chain,
                ..
            } = event
            else {
                continue;
            };
            // `leaf_chain` is newest-first; once any decided leaf has reached the target
            // height, the chain is at or past it.
            for LeafInfo { leaf, .. } in leaf_chain.iter() {
                if leaf.block_header().height() >= RESTART_HEIGHT {
                    break 'wait;
                }
            }
        }
    }

    let restart_height = network.server.decided_leaf().await.height();
    tracing::info!(
        restart_height,
        restart_epoch = RESTART_EPOCH,
        restart_boundary = RESTART_BOUNDARY,
        "restarting all nodes 3 blocks before an epoch boundary"
    );

    // Clone the TestConfig before dropping the network so the anvil/L1/contracts stay alive.
    let saved_cfg = network.cfg.clone();
    // The genesis states carry the deployed stake table, so the rebuild deploys no new one.
    let genesis_states: [_; NUM_NODES] = std::array::from_fn(|i| network.genesis_state(i));

    network.stop_consensus().await;
    drop(network);

    // Rebuild reusing the same persistence so nodes resume from stored state.
    let port2 = reserve_tcp_port().expect("OS should have ephemeral ports available");
    let config2 = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
        .api_config(SqlDataSource::options(
            &storage[0],
            Options::with_port(port2),
        ))
        .persistences(persistence)
        .network_config(saved_cfg)
        .states(genesis_states)
        .build();
    let network2 = TestNetwork::new(config2, TEST_UPGRADE).await;

    // The restarted network must keep advancing, including across the next epoch boundary.
    // Require BLOCKS_AFTER_RESTART new decides, using a lack-of-progress watchdog so a
    // healthy-but-slow chain still passes.
    let target_height = restart_height + BLOCKS_AFTER_RESTART;
    let stall_limit = 30; // 30 polls * 2s = 60s without a new decide => stalled
    let mut last_height = network2.server.decided_leaf().await.height();
    let mut stalled_polls = 0;
    while last_height < target_height {
        sleep(Duration::from_secs(2)).await;
        let height = network2.server.decided_leaf().await.height();
        if height > last_height {
            last_height = height;
            stalled_polls = 0;
        } else {
            stalled_polls += 1;
            if stalled_polls >= stall_limit {
                panic!(
                    "chain stalled after restart 3 blocks before boundary {RESTART_BOUNDARY}: no \
                     new decide for 60s at height {last_height}, unable to produce/cross the \
                     epoch boundary (target height was {target_height})."
                );
            }
        }
    }
}
