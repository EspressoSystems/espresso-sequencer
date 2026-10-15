use super::*;

/// Typed light client errors reach the HTTP client with their own status, not as a 500.
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_light_client_proof_errors_keep_their_status() {
    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    let storage = SqlDataSource::create_storage().await;
    let config = TestNetworkConfigBuilder::default()
        .api_config(
            SqlDataSource::options(&storage, Options::with_port(port))
                .light_client(Default::default()),
        )
        .network_config(TestConfigBuilder::default().build())
        .build();
    let _network = TestNetwork::new(config, TEST_UPGRADE).await;

    let client: Client<ClientErr, StaticVersion<0, 1>> =
        Client::new(format!("http://localhost:{port}").parse().unwrap());
    assert!(client.connect(Some(Duration::from_secs(60))).await);
    tokio::time::timeout(
        Duration::from_secs(120),
        wait_until_block_height(&client, "status/block-height", 2),
    )
    .await
    .expect("network did not reach block height 2");

    for (path, status) in [
        // A finalized height must be past the requested leaf.
        ("leaf/1/1", reqwest::StatusCode::BAD_REQUEST),
        // A header proof's root must be past the requested header.
        ("header/1/1", reqwest::StatusCode::BAD_REQUEST),
        // This 404 comes from the header proof helper, exercising `lc_error`.
        ("header/1000001/1000000", reqwest::StatusCode::NOT_FOUND),
    ] {
        let res = reqwest::get(format!("http://localhost:{port}/v1/light-client/{path}"))
            .await
            .unwrap();
        assert_eq!(res.status(), status, "{path}");
    }
}

const FOLLOW_TIMEOUT: Duration = Duration::from_secs(240);

/// Node 0 is the follower's only upstream. The query service only stores new heights through
/// the follow loop, so the follower's archive reaching the validators' tip proves it follows
/// live; then the endpoints a staked query node serves are compared with node 0's.
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_light_client_follower_query_node() -> anyhow::Result<()> {
    const NUM_NODES: usize = 5;
    const EPOCH_HEIGHT: u64 = 10;
    const EPOCHS_TO_FOLLOW: u64 = 4;
    const NEW_PROTOCOL: Upgrade = Upgrade::trivial(NEW_PROTOCOL_VERSION);

    let network_config = TestConfigBuilder::default()
        .epoch_height(EPOCH_HEIGHT)
        .epoch_start_block(0)
        .build();

    let api_port = reserve_tcp_port().expect("No ports free for query service");
    let api_url: Url = format!("http://localhost:{api_port}").parse()?;

    let storage = join_all((0..NUM_NODES).map(|_| SqlDataSource::create_storage())).await;
    let persistence: [_; NUM_NODES] = storage
        .iter()
        .map(<SqlDataSource as TestableSequencerDataSource>::persistence_options)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let follower_storage = SqlDataSource::create_storage().await;

    let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
        .api_config(
            Options::with_port(api_port)
                .catchup(Default::default())
                .light_client(Default::default())
                .config(Default::default())
                .submit(Default::default())
                .query_sql(Query::default(), tmp_options(&storage[0])),
        )
        .network_config(network_config)
        .persistences(persistence)
        .catchups(std::array::from_fn(|_| {
            StatePeers::<SequencerApiVersion>::from_urls(
                vec![api_url.clone()],
                Default::default(),
                Duration::from_secs(2),
                &NoMetrics,
            )
        }))
        .pos_hook(
            DelegationConfig::MultipleDelegators,
            StakeTableContractVersion::V3,
            NEW_PROTOCOL,
        )
        .await?
        .build();

    let genesis_state = config.states()[0].clone();
    let network = TestNetwork::new(config, NEW_PROTOCOL).await;
    let api_client: Client<ClientErr, SequencerApiVersion> = Client::new(api_url.clone());
    assert!(
        api_client.connect(Some(Duration::from_secs(60))).await,
        "node 0 query API did not come up"
    );

    // Let the validators decide the first epoch before the follower bootstraps from them.
    let mut events = network.server.event_stream();
    wait_for_epochs(&mut events, EPOCH_HEIGHT, 1).await;

    let genesis = follower_genesis(&network.cfg, &genesis_state, NEW_PROTOCOL);
    let params = FollowerParams {
        upstreams: vec![api_url.clone()],
        config_peers: None,
        catchup: CatchupParams {
            state_peers: vec![api_url.clone()],
            backoff: Default::default(),
            base_timeout: Duration::from_secs(2),
            local_timeout: Duration::from_secs(5),
        },
        bootstrap_epoch_catchup_timeout: Duration::from_secs(30),
        options: FollowerOptions {
            poll_interval: Duration::from_millis(500),
            max_blocks_per_poll: 10,
            light_client: Default::default(),
            light_client_db: Default::default(),
            light_client_genesis: None,
        },
    };
    let l1_params = || L1Params {
        urls: vec![network.cfg.l1_url()],
        options: network.cfg.l1_opt(),
    };
    let (mut follower, follower_client) = start_follower(
        &follower_storage,
        genesis.clone(),
        params.clone(),
        l1_params(),
    )
    .await?;

    wait_for_epochs(&mut events, EPOCH_HEIGHT, EPOCHS_TO_FOLLOW).await;
    let tip = network.server.decided_leaf().await.height();
    assert_follows_to(&follower, &follower_client, tip).await?;

    let finalized_height = latest_cert2_height(&api_client, tip)
        .await
        .context("no cert2 stored on a new protocol chain")?;
    assert_same_chain(&api_client, &follower_client, finalized_height).await?;
    assert_same_epoch_state(
        &api_client,
        &follower_client,
        epoch_from_block_number(tip, EPOCH_HEIGHT),
    )
    .await?;
    assert_same_merklized_state(
        &api_client,
        &follower_client,
        finalized_height,
        TestConfig::<NUM_NODES>::builder_key().fee_account(),
        genesis_state.chain_config.commit(),
    )
    .await?;
    let rewarded: Vec<RewardAccountV2> = network
        .server
        .decided_state()
        .await
        .unwrap()
        .reward_merkle_tree_v2
        .iter()
        .map(|(account, _)| *account)
        .collect();
    assert!(
        !rewarded.is_empty(),
        "no rewards distributed after {EPOCHS_TO_FOLLOW} epochs"
    );
    assert_same_rewards(&api_client, &follower_client, tip, &rewarded).await?;

    let txn = Transaction::new(NamespaceId::from(1_u32), vec![1, 2, 3]);
    let hash: Commitment<Transaction> = follower_client
        .post("submit/submit")
        .body_binary(&txn)?
        .send()
        .await
        .context("follower did not forward the transaction")?;
    assert_eq!(hash, txn.commit());
    wait_for_decide_on_handle(&mut events, &txn).await;

    assert_same_response(&api_client, &follower_client, "config/hotshot").await?;
    follower_client
        .get::<LeafProof>(&format!("light-client/leaf/{finalized_height}"))
        .send()
        .await
        .context("follower does not serve light client proofs")?;
    assert_validator_only_routes_not_found(&follower_client, finalized_height).await;

    // Only one node may write to the database, so the first follower stops before the second
    // starts.
    follower.shut_down().await;
    drop(follower);
    let (_follower, follower_client) =
        start_follower(&follower_storage, genesis, params, l1_params()).await?;
    wait_for_epochs(&mut events, EPOCH_HEIGHT, EPOCHS_TO_FOLLOW + 1).await;
    let tip = network.server.decided_leaf().await.height();
    timeout(
        FOLLOW_TIMEOUT,
        wait_until_block_height(&follower_client, "node/block-height", tip + 1),
    )
    .await
    .context("restarted follower did not catch up with the validators")?;

    Ok(())
}

/// Mirrors what `TestConfig::init_node` gives its validators.
fn follower_genesis<const N: usize>(
    cfg: &TestConfig<N>,
    genesis_state: &ValidatedState,
    upgrade: Upgrade,
) -> NodeGenesis {
    let hotshot = cfg.hotshot_config();
    let chain_config = genesis_state
        .chain_config
        .resolve()
        .expect("test states carry a full chain config");
    NodeGenesis {
        base_version: upgrade.base,
        upgrade_version: upgrade.target,
        genesis_version: upgrade.base,
        epoch_height: Some(hotshot.epoch_height),
        drb_difficulty: Some(hotshot.drb_difficulty),
        drb_upgrade_difficulty: Some(hotshot.drb_upgrade_difficulty),
        epoch_start_block: Some(hotshot.epoch_start_block),
        stake_table_capacity: Some(hotshot.stake_table_capacity),
        chain_config,
        stake_table: StakeTableConfig {
            capacity: hotshot.stake_table_capacity,
        },
        accounts: [(
            TestConfig::<N>::builder_key().fee_account(),
            U256::MAX.into(),
        )]
        .into_iter()
        .collect(),
        l1_finalized: L1Finalized::Number { number: 0 },
        header: GenesisHeader {
            chain_config,
            ..Default::default()
        },
        upgrades: cfg.upgrades(),
        da_committees: None,
    }
}

async fn start_follower(
    storage: &<SqlDataSource as TestableSequencerDataSource>::Storage,
    genesis: NodeGenesis,
    params: FollowerParams,
    l1_params: L1Params,
) -> anyhow::Result<(
    FollowerContext<persistence::sql::Persistence>,
    Client<ClientErr, SequencerApiVersion>,
)> {
    let port = reserve_tcp_port().expect("No ports free for query service");
    let mut db = tmp_options(storage);
    let persistence = db.create().await?;
    let handle = Options::with_port(port)
        .query_sql(
            Query {
                peers: params.upstreams.clone(),
                ..Default::default()
            },
            db,
        )
        .catchup(Default::default())
        .light_client(Default::default())
        .config(Default::default())
        .submit(Default::default())
        .serve(move |metrics, sink, _| {
            async move {
                init_follower_node(genesis, params, metrics, persistence, l1_params, sink).await
            }
            .boxed()
        })
        .await
        .context("follower should start")?;
    let follower = FollowerContext::new(handle);

    let client: Client<ClientErr, SequencerApiVersion> =
        Client::new(format!("http://localhost:{port}").parse()?);
    ensure!(
        client.connect(Some(Duration::from_secs(60))).await,
        "follower query API did not come up"
    );
    Ok((follower, client))
}

async fn assert_follows_to(
    follower: &FollowerContext<persistence::sql::Persistence>,
    follower_client: &Client<ClientErr, SequencerApiVersion>,
    tip: u64,
) -> anyhow::Result<()> {
    timeout(FOLLOW_TIMEOUT, async {
        while follower.decided_leaf().await.height() < tip {
            sleep(Duration::from_secs(1)).await;
        }
    })
    .await
    .context("follower did not decide up to the validators' tip")?;
    // `node/block-height` counts blocks, so `tip` is stored at `tip + 1`.
    timeout(
        FOLLOW_TIMEOUT,
        wait_until_block_height(follower_client, "node/block-height", tip + 1),
    )
    .await
    .context("follower's archive did not reach the validators' tip")
}

/// Cert2s are only stored at the heights they finalize.
async fn latest_cert2_height(
    client: &Client<ClientErr, SequencerApiVersion>,
    tip: u64,
) -> Option<u64> {
    for height in (1..tip).rev() {
        if client
            .get::<Certificate2<SeqTypes>>(&format!("availability/cert2/{height}"))
            .send()
            .await
            .is_ok()
        {
            return Some(height);
        }
    }
    None
}

async fn assert_same_chain(
    api_client: &Client<ClientErr, SequencerApiVersion>,
    follower_client: &Client<ClientErr, SequencerApiVersion>,
    height: u64,
) -> anyhow::Result<()> {
    for path in [
        format!("availability/leaf/{height}"),
        format!("availability/header/{height}"),
        format!("availability/block/{height}"),
        format!("availability/vid/common/{height}"),
        format!("availability/cert2/{height}"),
        format!("catchup/{height}/cert2"),
    ] {
        assert_same_response(api_client, follower_client, &path).await?;
    }
    Ok(())
}

async fn assert_same_epoch_state(
    api_client: &Client<ClientErr, SequencerApiVersion>,
    follower_client: &Client<ClientErr, SequencerApiVersion>,
    current_epoch: u64,
) -> anyhow::Result<()> {
    for epoch in 1..=current_epoch {
        for path in [
            format!("node/stake-table/{epoch}"),
            format!("node/validators/{epoch}"),
            format!("node/block-reward/epoch/{epoch}"),
        ] {
            assert_same_response(api_client, follower_client, &path).await?;
        }
        let state_cert = format!("availability/state-cert-v2/{epoch}");
        if api_client
            .get::<serde_json::Value>(&state_cert)
            .send()
            .await
            .is_ok()
        {
            assert_same_response(api_client, follower_client, &state_cert).await?;
        }
    }
    Ok(())
}

async fn assert_same_merklized_state(
    api_client: &Client<ClientErr, SequencerApiVersion>,
    follower_client: &Client<ClientErr, SequencerApiVersion>,
    height: u64,
    account: FeeAccount,
    chain_config: Commitment<ChainConfig>,
) -> anyhow::Result<()> {
    for client in [api_client, follower_client] {
        timeout(
            FOLLOW_TIMEOUT,
            wait_until_block_height(client, "fee-state/block-height", height + 1),
        )
        .await
        .context("merklized state did not reach the finalized height")?;
    }
    let leaf: LeafQueryData<SeqTypes> = api_client
        .get(&format!("availability/leaf/{height}"))
        .send()
        .await?;
    let view = leaf.leaf().view_number();
    for path in [
        format!("fee-state/{height}/{account}"),
        format!("catchup/{height}/{view}/account/{account}"),
        format!("catchup/{height}/{view}/blocks"),
        format!("catchup/chain-config/{chain_config}"),
    ] {
        assert_same_response(api_client, follower_client, &path).await?;
    }
    Ok(())
}

async fn assert_same_rewards(
    api_client: &Client<ClientErr, SequencerApiVersion>,
    follower_client: &Client<ClientErr, SequencerApiVersion>,
    tip: u64,
    accounts: &[RewardAccountV2],
) -> anyhow::Result<()> {
    for client in [api_client, follower_client] {
        timeout(
            FOLLOW_TIMEOUT,
            wait_until_block_height(client, "reward-state-v2/block-height", tip + 1),
        )
        .await
        .context("reward state did not reach the tip")?;
    }
    for account in accounts {
        assert_same_response(
            api_client,
            follower_client,
            &format!("reward-state-v2/reward-claim-input/{tip}/{account}"),
        )
        .await?;
    }
    Ok(())
}

async fn assert_validator_only_routes_not_found(
    follower_client: &Client<ClientErr, SequencerApiVersion>,
    height: u64,
) {
    for path in [
        format!("state-signature/block/{height}"),
        "status/keys".to_string(),
    ] {
        let err = follower_client
            .get::<serde_json::Value>(&path)
            .send()
            .await
            .unwrap_err();
        assert_matches!(err, ClientErr { status, .. } if status == StatusCode::NOT_FOUND, "{path}");
    }
}

/// A path both nodes fail on proves nothing about the follower, so node 0 has to succeed.
async fn assert_same_response(
    expected: &Client<ClientErr, SequencerApiVersion>,
    actual: &Client<ClientErr, SequencerApiVersion>,
    path: &str,
) -> anyhow::Result<()> {
    let expected: serde_json::Value = expected
        .get(path)
        .send()
        .await
        .with_context(|| format!("{path}: node 0 did not answer"))?;
    let actual = actual
        .get::<serde_json::Value>(path)
        .send()
        .await
        .map_err(|err| err.status);
    ensure!(
        actual.as_ref() == Ok(&expected),
        "{path}: follower answered {actual:?}, node 0 answered {expected:?}"
    );
    Ok(())
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
pub(crate) async fn test_state_cert_query() {
    const TEST_EPOCH_HEIGHT: u64 = 10;
    const TEST_EPOCHS: u64 = 5;

    let network_config = TestConfigBuilder::default()
        .epoch_height(TEST_EPOCH_HEIGHT)
        .build();

    let api_port = reserve_tcp_port().expect("OS should have ephemeral ports available");

    tracing::info!("API PORT = {api_port}");
    const NUM_NODES: usize = 2;

    let (storage, persistence) = node_storage::<SqlDataSource, NUM_NODES>().await;

    let config = TestNetworkConfigBuilder::with_num_nodes()
        .api_config(SqlDataSource::options(
            &storage[0],
            Options::with_port(api_port).catchup(Default::default()),
        ))
        .network_config(network_config)
        .persistences(persistence.clone())
        .build();

    let network = TestNetwork::new(config, TEST_UPGRADE).await;
    let mut events = network.server.event_stream();

    // Wait until 5 epochs have passed.
    loop {
        let event = events.next().await.unwrap();
        tracing::info!("Received event from handle: {event:?}");

        if let CoordinatorEvent::NewDecide {
            leaf_infos: leaf_chain,
            ..
        } = event
        {
            println!(
                "Decide event received: {:?}",
                leaf_chain.first().unwrap().leaf.height()
            );
            if let Some(first_leaf) = leaf_chain.first() {
                let height = first_leaf.leaf.height();
                tracing::info!("Decide event received at height: {height}");

                if height >= TEST_EPOCHS * TEST_EPOCH_HEIGHT {
                    break;
                }
            }
        }
    }

    // Connect client.
    let client: Client<ClientErr, StaticVersion<0, 1>> =
        Client::new(format!("http://localhost:{api_port}").parse().unwrap());
    client.connect(Some(Duration::from_secs(10))).await;

    assert_state_certs(&client, 3..=TEST_EPOCHS, TEST_EPOCH_HEIGHT).await;
}

/// Light client provers and catching-up nodes still fetch the state certificates of
/// mainnet's epochs before 0.6, whose blocks only legacy consensus produced.
#[rstest]
#[case::v3("v3")]
#[case::v4("v4")]
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_legacy_state_certs(#[case] chain: &str) -> anyhow::Result<()> {
    let chain = LegacyChain::load(chain)?;
    let replay = chain.replay().await?;
    let (last_epoch, _) = chain.state_certs.last().unwrap();
    assert_state_certs(&replay.client, 3..=*last_epoch, chain.epoch_height()).await;

    let peers = StatePeers::<SequencerApiVersion>::from_urls(
        vec![replay.url.clone()],
        Default::default(),
        Duration::from_secs(2),
        &NoMetrics,
    );
    for (epoch, cert) in &chain.state_certs {
        ensure!(
            peers.try_fetch_state_cert(0, *epoch).await? == *cert,
            "catchup of the epoch {epoch} state certificate"
        );
    }
    Ok(())
}

/// Checks the v1 and v2 state certificates of `epochs` agree, and that each certifies its
/// epoch root block.
async fn assert_state_certs(
    client: &Client<ClientErr, StaticVersion<0, 1>>,
    epochs: std::ops::RangeInclusive<u64>,
    epoch_height: u64,
) {
    for i in epochs {
        let state_query_data_v2 = client
            .get::<StateCertQueryDataV2<SeqTypes>>(&format!("availability/state-cert-v2/{i}"))
            .send()
            .await
            .unwrap();
        let state_cert_v2 = state_query_data_v2.0.clone();
        tracing::info!("state_cert_v2: {state_cert_v2:?}");
        assert_eq!(state_cert_v2.epoch.u64(), i);
        assert_eq!(
            state_cert_v2.light_client_state.block_height,
            i * epoch_height - 5
        );
        let block_height = state_cert_v2.light_client_state.block_height;

        let header: Header = client
            .get(&format!("availability/header/{block_height}"))
            .send()
            .await
            .unwrap();

        // Headers commit to an auth root from 0.4 on.
        if header.version() >= DRB_AND_HEADER_UPGRADE_VERSION {
            let auth_root = state_cert_v2.auth_root;
            let header_auth_root = header.auth_root().unwrap();
            if auth_root.is_zero() || header_auth_root.is_zero() {
                panic!("auth root shouldn't be zero");
            }

            assert_eq!(auth_root, header_auth_root, "auth root mismatch");
        }

        let state_query_data_v1 = client
            .get::<StateCertQueryDataV1<SeqTypes>>(&format!("availability/state-cert/{i}"))
            .send()
            .await
            .unwrap();

        let state_cert_v1 = state_query_data_v1.0.clone();
        tracing::info!("state_cert_v1: {state_cert_v1:?}");
        assert_eq!(state_query_data_v1, state_query_data_v2.into());
    }
}

/// Test state certificate catchup functionality by simulating a node that falls behind and needs
/// to catch up. This test starts a 5-node network with epoch height 10, waits for 3 epochs to
/// pass, then removes and restarts node 0 with a fresh storage. The
/// restarted node catches up for the missing state certificates.
#[test_log::test(tokio::test(flavor = "multi_thread"))]
pub(crate) async fn test_state_cert_catchup() {
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

    let mut events = network.peers[2].event_stream();
    // Wait until at least 5 epochs have passed
    wait_for_epochs(&mut events, EPOCH_HEIGHT, 3).await;

    // Remove peer 0 and restart it with the query module enabled.
    // Adding an additional node to the test network is not straight forward,
    // as the keys have already been initialized in the config above.
    // So, we remove this node and re-add it using the same index.
    // Await the shutdown so the node's cliquenet listener releases its
    // port before the replacement node binds the same address.
    network.peers.remove(0).shut_down().await;

    let new_storage: hotshot_query_service::data_source::sql::testing::TmpDb =
        SqlDataSource::create_storage().await;
    let new_persistence: persistence::sql::Options =
        <SqlDataSource as TestableSequencerDataSource>::persistence_options(&new_storage);

    let node_0_port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    tracing::info!("node_0_port {node_0_port}");
    let opt = Options::with_port(node_0_port).query_sql(
        Query {
            peers: vec![format!("http://localhost:{api_port}").parse().unwrap()],
            ..Query::test()
        },
        tmp_options(&new_storage),
    );
    let node_0 = opt
        .clone()
        .serve(|metrics, consumer, storage| {
            let cfg = network.cfg.clone();
            let new_persistence = new_persistence.clone();
            let state = state.clone();
            async move {
                Ok(cfg
                    .init_node(
                        1,
                        state,
                        new_persistence.clone(),
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

    let mut events = node_0.event_stream();
    // Wait until at least 5 epochs have passed
    wait_for_epochs(&mut events, EPOCH_HEIGHT, 5).await;

    let client: Client<ClientErr, StaticVersion<0, 1>> =
        Client::new(format!("http://localhost:{node_0_port}").parse().unwrap());
    client.connect(Some(Duration::from_secs(60))).await;

    for epoch in 3..=5 {
        let state_cert = client
            .get::<StateCertQueryDataV2<SeqTypes>>(&format!("availability/state-cert-v2/{epoch}"))
            .send()
            .await
            .unwrap();
        assert_eq!(state_cert.0.epoch.u64(), epoch);
    }
}

/// Verify the light client leaf, header, and payload proofs at each height
/// against the ground truth captured from the availability streams.
///
/// Only checks that proofs verify, not which `FinalityProof` variant they
/// use
async fn check_light_client_proofs(
    client: &Client<ClientErr, StaticVersion<0, 1>>,
    actual_leaves: &[LeafQueryData<SeqTypes>],
    actual_blocks: &[BlockQueryData<SeqTypes>],
    heights: impl IntoIterator<Item = u64>,
    epoch_height: u64,
) {
    let quorum = EpochChangeQuorum::new(epoch_height);
    for i in heights {
        let leaf = &actual_leaves[i as usize];
        let block = &actual_blocks[i as usize];
        let version = leaf.header().version();
        tracing::info!(i, %version, "check light client proofs");

        // Get the same leaf proof by various IDs.
        let proofs = try_join_all(
            [
                format!("light-client/leaf/{i}"),
                format!("light-client/leaf/hash/{}", leaf.hash()),
                format!("light-client/leaf/block-hash/{}", leaf.block_hash()),
            ]
            .into_iter()
            .map(|path| async move {
                tracing::info!(i, path, "fetch leaf proof");
                let proof = client.get::<LeafProof>(&path).send().await?;
                Ok::<_, anyhow::Error>((path, proof))
            }),
        )
        .await
        .unwrap();

        // Check proofs against the expected leaf.
        for (path, proof) in proofs {
            tracing::info!(i, path, "check leaf proof");
            assert_eq!(
                proof
                    .verify(LeafProofHint::Quorum(&quorum))
                    .await
                    .unwrap_or_else(|err| panic!("{path}: proof failed to verify: {err:#}")),
                *leaf
            );
        }

        // Get the corresponding header.
        let root_height = i + 1;
        let root = actual_leaves[root_height as usize].header();
        let proofs = try_join_all(
            [
                format!("light-client/header/{root_height}/{i}"),
                format!(
                    "light-client/header/{root_height}/hash/{}",
                    leaf.block_hash()
                ),
            ]
            .into_iter()
            .map(|path| async move {
                tracing::info!(i, path, "fetch header proof");
                let proof = client.get::<HeaderProof>(&path).send().await?;
                Ok::<_, anyhow::Error>((path, proof))
            }),
        )
        .await
        .unwrap();
        for (path, proof) in proofs {
            tracing::info!(i, path, "check header proof");
            assert_eq!(
                proof.verify_ref(root.block_merkle_tree_root()).unwrap(),
                leaf.header()
            );
        }

        // Get the corresponding payload.
        let proof = client
            .get::<PayloadProof>(&format!("light-client/payload/{i}"))
            .send()
            .await
            .unwrap();
        assert_eq!(proof.verify(leaf.header()).unwrap(), *block.payload());
    }
}

/// Check the light client stake table endpoint: replaying `first_epoch + 2`
/// reproduces the validator set loaded from storage, and an earlier epoch
/// is a `BAD_REQUEST`.
async fn check_light_client_stake_table(
    client: &Client<ClientErr, StaticVersion<0, 1>>,
    storage: &impl SequencerPersistence,
    first_epoch: EpochNumber,
) {
    let events: Vec<StakeTableEvent> = client
        .get(&format!("light-client/stake-table/{}", first_epoch + 2))
        .send()
        .await
        .unwrap();
    let mut state_from_events = StakeTableState::default();
    for event in events {
        state_from_events.apply_event(event).unwrap().unwrap();
    }
    assert_eq!(
        state_from_events.into_validators(),
        storage
            .load_all_validators(first_epoch + 2, 0, 1_000_000)
            .await
            .unwrap()
            .into_iter()
            .map(|v| (v.account, v))
            .collect::<RegisteredValidatorMap>()
    );

    // Querying for a stake table before the first real epoch is an error.
    let err = client
        .get::<Vec<StakeTableEvent>>(&format!("light-client/stake-table/{}", first_epoch + 1))
        .send()
        .await
        .unwrap_err();
    assert_eq!(err.status, StatusCode::BAD_REQUEST);
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_light_client_completeness() {
    // Run through two epoch changes, then check that we are able to get a correct light client
    // proof for every finalized leaf.

    const NUM_NODES: usize = 1;
    const EPOCH_HEIGHT: u64 = 50;

    let port = reserve_tcp_port().expect("OS should have ephemeral ports available");
    let url: Url = format!("http://localhost:{port}").parse().unwrap();

    let test_config = TestConfigBuilder::default()
        .epoch_height(EPOCH_HEIGHT)
        .epoch_start_block(0)
        // No transactions here, so this sets the seconds per block, and the
        // test is bound by block count.
        .builder_timeout(Duration::from_millis(250))
        .build();

    let (storage, persistence) = node_storage::<SqlDataSource, NUM_NODES>().await;

    let config = TestNetworkConfigBuilder::<NUM_NODES, _, _>::with_num_nodes()
        .api_config(
            SqlDataSource::options(&storage[0], Options::with_port(port))
                .light_client(Default::default()),
        )
        .persistences(persistence.clone())
        .catchups(std::array::from_fn(|_| {
            StatePeers::<SequencerApiVersion>::from_urls(
                vec![url.clone()],
                Default::default(),
                Duration::from_secs(2),
                &NoMetrics,
            )
        }))
        .network_config(test_config)
        .build();

    let mut network = TestNetwork::new(config, TEST_UPGRADE).await;
    let client: Client<ClientErr, StaticVersion<0, 1>> = Client::new(url);
    client.connect(None).await;

    // Get a leaf stream so that we can wait for various events. Also keep track of each leaf
    // yielded, which we can use as ground truth later in the test.
    let mut actual_leaves = vec![];
    let mut actual_blocks = vec![];
    let mut leaves = client
        .socket("availability/stream/leaves/0")
        .subscribe::<LeafQueryData<SeqTypes>>()
        .await
        .unwrap()
        .zip(
            client
                .socket("availability/stream/blocks/0")
                .subscribe::<BlockQueryData<SeqTypes>>()
                .await
                .unwrap(),
        )
        .map(|(leaf, block)| {
            let leaf = leaf.unwrap();
            let block = block.unwrap();
            actual_leaves.push(leaf.clone());
            actual_blocks.push(block);
            leaf
        });

    let first_epoch = {
        let leaf: LeafQueryData<SeqTypes> = leaves.next().await.unwrap();
        leaf.leaf().epoch(EPOCH_HEIGHT).unwrap()
    };

    // Wait for two epoch changes (so we get to the first epoch that actually uses the stake
    // table).
    let mut epoch_heights = [0; 2];
    for (i, epoch_height) in epoch_heights.iter_mut().enumerate() {
        let desired_epoch = first_epoch + (i as u64) + 1;
        *epoch_height = loop {
            let leaf = leaves.next().await.unwrap();
            let epoch = leaf.leaf().epoch(EPOCH_HEIGHT).unwrap();
            if epoch > desired_epoch {
                tracing::info!(
                    height = leaf.height(),
                    ?desired_epoch,
                    ?epoch,
                    "changed epoch"
                );
                break leaf.height();
            }
            tracing::info!(
                ?desired_epoch,
                height = leaf.header().height(),
                view = ?leaf.leaf().view_number(),
                "waiting for epoch change"
            );
        };
    }

    // Wait a few more blocks.
    let max_block = epoch_heights[1] + 1;
    loop {
        let leaf = leaves.next().await.unwrap();
        if leaf.height() > max_block {
            break;
        }
        tracing::info!(max_block, height = leaf.height(), "waiting for block");
    }

    // Stop consensus. All the blocks we are going to query have already been produced.
    // Continuing to run consensus would just waste resources while we check stuff.
    network.stop_consensus().await;

    // Check light client. Querying every single block is too slow, so we'll check a few blocks
    // around various critical points:
    let heights =
    // * The first few blocks, including genesis
        (0..=1)
    // * A few blocks just before and after the first epoch change
        .chain(epoch_heights[0]-1..=epoch_heights[0] + 1)
    // * A few blocks just before and after the stake table comes into effect
        .chain(epoch_heights[1]-1..=max_block);

    check_light_client_proofs(
        &client,
        &actual_leaves,
        &actual_blocks,
        heights,
        EPOCH_HEIGHT,
    )
    .await;
    check_light_client_stake_table(&client, &*network.server.persistence(), first_epoch).await;
}

/// Light clients still prove mainnet's leaves from before its 0.3 upgrade, across it and across
/// the first epoch on the stake table, all produced by legacy consensus.
#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn test_legacy_light_client_completeness() -> anyhow::Result<()> {
    let chain = LegacyChain::load("v2-v3")?;
    let replay = chain.replay().await?;
    let epoch_height = chain.epoch_height();
    let leaves: Vec<_> = chain.blocks.iter().map(|b| b.leaf.clone()).collect();
    let blocks: Vec<_> = chain.blocks.iter().map(|b| b.block.clone()).collect();

    let upgraded = leaves
        .iter()
        .find(|leaf| leaf.header().version() >= EPOCH_VERSION)
        .context("the chain upgrades to 0.3")?;
    let upgrade_height = upgraded.height();
    let first_epoch = upgraded.leaf().epoch(epoch_height).unwrap();
    let epoch_change = |epoch: EpochNumber| {
        leaves
            .iter()
            .find(|leaf| leaf.leaf().epoch(epoch_height).is_some_and(|e| e > epoch))
            .map(|leaf| leaf.height())
            .context("the chain changes epoch")
    };
    let epoch_heights = [
        epoch_change(first_epoch + 1)?,
        epoch_change(first_epoch + 2)?,
    ];
    let max_block = epoch_heights[1] + 1;
    ensure!(
        max_block < chain.tip(),
        "a header proof needs the block after it"
    );

    let heights = (0..=1)
        .chain(upgrade_height - 1..=upgrade_height + 1)
        .chain(epoch_heights[0] - 1..=epoch_heights[0] + 1)
        .chain(epoch_heights[1] - 1..=max_block);
    check_light_client_proofs(&replay.client, &leaves, &blocks, heights, epoch_height).await;
    check_light_client_stake_table(&replay.client, &replay.persistence().await?, first_epoch).await;
    Ok(())
}
