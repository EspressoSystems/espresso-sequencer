use contract_bindings::TestHermezContracts;
use std::{path::Path, process::Command, time::Duration};

async fn wait_for_http(url: &str, interval: Duration, max_retries: usize) -> Result<(), ()> {
    for _ in 0..(max_retries + 1) {
        let res = surf::get(url).await;
        if res.is_ok() {
            tracing::debug!("Connected to {url}");
            return Ok(());
        }
        tracing::debug!("Waiting for {url}, retrying in {interval:?}");
        std::thread::sleep(interval);
    }
    panic!("Url {url:?} not available.")
}

/// A zkevm-node inside docker compose with custom contracts
#[derive(Debug, Clone)]
pub struct ZkEvmNode {
    l1: TestHermezContracts,
    project_name: String,
}

impl ZkEvmNode {
    pub fn l1(&self) -> &TestHermezContracts {
        &self.l1
    }

    pub fn project_name(&self) -> &String {
        &self.project_name
    }

    fn compose_cmd_prefix(project_name: &str) -> Command {
        let mut cmd = Command::new("docker");
        let work_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("zkevm-node/test");
        cmd.current_dir(work_dir)
            .arg("compose")
            .arg("--project-name")
            .arg(project_name)
            .arg("-f")
            .arg("permissionless-docker-compose.yml")
            .arg("--env-file")
            .arg("../../.env");
        cmd
    }

    /// Start the L1, deploy contracts, start the L2
    ///
    /// The `project_name` must be unique for each test. At the moment however
    /// there will be port conflicts if more than one `ZkEvmNode`s are started
    /// at the same time.
    pub async fn start(project_name: String) -> Self {
        // Start L1
        Self::compose_cmd_prefix(&project_name)
            .arg("up")
            .arg("zkevm-mock-l1-network")
            .arg("-V")
            .arg("--force-recreate")
            .arg("--abort-on-container-exit")
            .spawn()
            .expect("Failed to start L1 docker container");

        println!("Waiting for L1 to start ...");

        // TODO: L1 port should be configurable
        wait_for_http("http://localhost:8545", Duration::from_millis(100), 10)
            .await
            .unwrap();

        let l1 = TestHermezContracts::deploy().await;

        // Start zkevm-node
        Self::compose_cmd_prefix(&project_name)
            .env(
                "ZKEVM_NODE_ETHERMAN_POEADDR",
                format!("{:?}", l1.rollup.address()),
            )
            .env(
                "ZKEVM_NODE_ETHERMAN_MATICADDR",
                format!("{:?}", l1.matic.address()),
            )
            .env(
                "ZKEVM_NODE_ETHERMAN_GLOBALEXITROOTMANAGERADDR",
                format!("{:?}", l1.global_exit_root.address()),
            )
            .arg("up")
            .arg("zkevm-prover")
            .arg("zkevm-aggregator")
            .arg("zkevm-pool-db")
            .arg("zkevm-state-db")
            .arg("zkevm-permissionless-node")
            .arg("-V")
            .arg("--force-recreate")
            .arg("--abort-on-container-exit")
            .spawn()
            .expect("Failed to start zkevm-node compose environment");

        // TODO: L2 port should be configurable
        wait_for_http("http://localhost:8126", Duration::from_secs(1), 20)
            .await
            .expect("Failed to start zkevm-node");

        Self { project_name, l1 }
    }

    fn stop(&self) -> &Self {
        Self::compose_cmd_prefix(&self.project_name)
            .arg("down")
            .arg("-v")
            .arg("--remove-orphans")
            .spawn()
            .expect("Failed to run docker compose down");
        self
    }
}

impl Drop for ZkEvmNode {
    fn drop(&mut self) {
        self.stop();
    }
}
