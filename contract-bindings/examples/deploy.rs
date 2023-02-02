use std::{path::Path, process::Command};

use contract_bindings::TestHermezSystem;

#[async_std::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Start L1
    Command::new("docker")
        .current_dir(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("zkevm-node/test"),
        )
        .arg("compose")
        .arg("-f")
        .arg("permissionless-docker-compose.yml")
        .arg("--env-file")
        .arg("../../.env")
        .arg("up")
        .arg("zkevm-mock-l1-network")
        .arg("-V")
        .arg("--force-recreate")
        .arg("--abort-on-container-exit")
        .spawn()
        .expect("Failed to start L1 docker container");

    println!("Waiting for L1 to start ...");
    std::thread::sleep(std::time::Duration::from_secs(2));

    let system = TestHermezSystem::deploy().await;
    let TestHermezSystem {
        // provider,
        rollup,
        // clients,
        matic,
        global_exit_root,
        ..
    } = system.clone();

    // // Try to sequence a batch
    // let l2_tx_data = hex::decode("1234").unwrap();

    // let current_timestamp = provider
    //     .get_block(BlockNumber::Latest)
    //     .await
    //     .unwrap()
    //     .unwrap()
    //     .timestamp;

    // let batch = BatchData {
    //     transactions: l2_tx_data.into(),
    //     global_exit_root: [0u8; 32],
    //     timestamp: current_timestamp.as_u64(),
    //     min_forced_timestamp: 0u64,
    // };

    // // Send a batch
    // let rollup_trusted: PolygonZkEVM<_> = rollup.connect(clients.trusted_sequencer).into();
    // let receipt = rollup_trusted
    //     .sequence_batches(vec![batch])
    //     .send()
    //     .await
    //     .unwrap()
    //     .await
    //     .unwrap();

    // assert_eq!(receipt.unwrap().status, Some(1u64.into()));

    // Start zkevm-node
    let mut cmd = Command::new("docker");

    cmd.env(
        "ZKEVM_NODE_ETHERMAN_POEADDR",
        format!("{:?}", rollup.address()),
    )
    .env(
        "ZKEVM_NODE_ETHERMAN_MATICADDR",
        format!("{:?}", matic.address()),
    )
    .env(
        "ZKEVM_NODE_ETHERMAN_GLOBALEXITROOTMANAGERADDR",
        format!("{:?}", global_exit_root.address()),
    )
    .current_dir(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("zkevm-node/test"),
    )
    .arg("compose")
    .arg("-f")
    .arg("permissionless-docker-compose.yml")
    .arg("--env-file")
    .arg("../../.env")
    .arg("up")
    .arg("zkevm-prover")
    .arg("zkevm-aggregator")
    .arg("zkevm-pool-db")
    .arg("zkevm-state-db")
    .arg("zkevm-permissionless-node")
    .arg("-V")
    .arg("--force-recreate")
    .arg("--abort-on-container-exit");

    for (k, v) in cmd.get_envs() {
        println!("ENV {k:?}: {v:?}");
    }

    println!("Starting zkevm-node {cmd:?}");

    let _handle = cmd.spawn().expect("Failed to start docker");

    loop {
        system.mine_block().await;
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    // handle.wait().unwrap();

    // let stdout = cmd.stdout.as_mut().unwrap();
    // let stdout_reader = BufReader::new(stdout);
    // let stdout_lines = stdout_reader.lines();

    // for line in stdout_lines {
    //     println!("Read: {:?}", line);
    // }
    //
    // system.mine_blocks(10).await;

    // println!("Done!")
}
