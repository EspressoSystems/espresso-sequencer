#![cfg(any(test, feature = "testing"))]

use contract_bindings::TestHermezContracts;
use portpicker::pick_unused_port;
use std::{
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};
use surf_disco::Url;

async fn wait_for_http(
    url: impl AsRef<str>,
    interval: Duration,
    max_retries: usize,
) -> Result<(), ()> {
    let url = url.as_ref();
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

#[derive(Clone, Debug)]
pub struct ZkEvmEnv {
    cdn_server_port: u16,
    sequencer_api_port: u16,
    sequencer_storage_path: PathBuf,
    l1_port: u16,
    l2_port: u16,
    l1_chain_id: Option<u64>,
    l2_chain_id: Option<u64>,
    sequencer_mnemonic: String,
    adaptor_port: u16,
}

impl ZkEvmEnv {
    pub fn random() -> Self {
        let cdn_server_port = pick_unused_port().unwrap();
        let sequencer_api_port = pick_unused_port().unwrap();
        let l1_port = pick_unused_port().unwrap();
        let l2_port = pick_unused_port().unwrap();
        let adaptor_port = pick_unused_port().unwrap();

        // Use default values for things that are deterministic or internal to a docker-compose
        // service.
        let sequencer_storage_path = "/store/sequencer".into();
        let sequencer_mnemonic =
            "test test test test test test test test test test test junk".into();
        let l1_chain_id = None;
        let l2_chain_id = None;

        Self {
            cdn_server_port,
            sequencer_api_port,
            l1_port,
            l2_port,
            l1_chain_id,
            l2_chain_id,
            adaptor_port,
            sequencer_storage_path,
            sequencer_mnemonic,
        }
    }

    pub fn cmd(&self, command: &str) -> Command {
        let mut cmd = Command::new(command);
        cmd.env("ESPRESSO_CDN_SERVER_PORT", self.cdn_server_port.to_string())
            .env(
                "ESPRESSO_SEQUENCER_API_PORT",
                self.sequencer_api_port.to_string(),
            )
            .env(
                "ESPRESSO_SEQUENCER_STORAGE_PATH",
                self.sequencer_storage_path.as_os_str(),
            )
            .env("ESPRESSO_ZKEVM_L1_PORT", self.l1_port.to_string())
            .env("ESPRESSO_ZKEVM_L1_PROVIDER", self.l1_provider().as_ref())
            .env("ESPRESSO_ZKEVM_L2_PORT", self.l2_port.to_string())
            .env("ESPRESSO_ZKEVM_L2_PROVIDER", self.l2_provider().as_ref())
            .env(
                "ESPRESSO_ZKEVM_SEQUENCER_MNEMONIC",
                &self.sequencer_mnemonic,
            )
            .env("ESPRESSO_ZKEVM_ADAPTOR_PORT", self.adaptor_port.to_string());
        if let Some(id) = self.l1_chain_id {
            cmd.env("ESPRESSO_ZKEVM_L1_CHAIN_ID", id.to_string());
        }
        if let Some(id) = self.l2_chain_id {
            cmd.env("ESPRESSO_ZKEVM_L2_CHAIN_ID", id.to_string());
        }
        cmd
    }

    pub fn l1_provider(&self) -> Url {
        format!("http://localhost:{}", self.l1_port)
            .parse()
            .unwrap()
    }

    pub fn l1_chain_id(&self) -> Option<u64> {
        self.l1_chain_id
    }

    pub fn l2_provider(&self) -> Url {
        format!("http://localhost:{}", self.l2_port)
            .parse()
            .unwrap()
    }

    pub fn l2_chain_id(&self) -> Option<u64> {
        self.l2_chain_id
    }

    pub fn funded_mnemonic(&self) -> &str {
        &self.sequencer_mnemonic
    }
}

/// A zkevm-node inside docker compose with custom contracts
#[derive(Debug, Clone)]
pub struct ZkEvmNode {
    env: ZkEvmEnv,
    l1: TestHermezContracts,
    project_name: String,
}

impl ZkEvmNode {
    pub fn env(&self) -> &ZkEvmEnv {
        &self.env
    }

    pub fn l1(&self) -> &TestHermezContracts {
        &self.l1
    }

    pub fn project_name(&self) -> &String {
        &self.project_name
    }

    fn compose_cmd_prefix(env: &ZkEvmEnv, project_name: &str) -> Command {
        let mut cmd = env.cmd("docker");
        let work_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("zkevm-node/test");
        cmd.current_dir(work_dir)
            .arg("compose")
            .arg("--project-name")
            .arg(project_name)
            .arg("-f")
            .arg("permissionless-docker-compose.yml");
        cmd
    }

    /// Start the L1, deploy contracts, start the L2
    ///
    /// The `project_name` must be unique for each test. At the moment however
    /// there will be port conflicts if more than one `ZkEvmNode`s are started
    /// at the same time.
    pub async fn start(project_name: String) -> Self {
        let env = ZkEvmEnv::random();
        tracing::info!("Starting ZkEvmNode with env: {:?}", env);
        tracing::info!(
            "Compose prefix: {:?}",
            Self::compose_cmd_prefix(&env, &project_name)
        );

        // Pull Docker images before doing anything. This can take a long time, and if we do it
        // later it might cause some `wait_for_http` calls to time out.
        let status = Self::compose_cmd_prefix(&env, &project_name)
            .arg("pull")
            .spawn()
            .expect("Failed to pull Docker images")
            .wait()
            .expect("Error waiting for Docker pull command");
        if !status.success() {
            panic!("Failed to pull Docker images ({status})");
        }

        // Start L1
        Self::compose_cmd_prefix(&env, &project_name)
            .arg("up")
            .arg("zkevm-mock-l1-network")
            .arg("-V")
            .arg("--force-recreate")
            .arg("--abort-on-container-exit")
            .spawn()
            .expect("Failed to start L1 docker container");

        println!("Waiting for L1 to start ...");

        // TODO: L1 port should be configurable
        wait_for_http(env.l1_provider(), Duration::from_millis(100), 30)
            .await
            .unwrap();

        // Use a dummy URL for the trusted sequencer since we're not running one anyways.
        let l1 = TestHermezContracts::deploy(&env.l1_provider(), "http://dummy:1234").await;

        // Start zkevm-node
        Self::compose_cmd_prefix(&env, &project_name)
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
        wait_for_http(env.l2_provider(), Duration::from_secs(1), 20)
            .await
            .expect("Failed to start zkevm-node");

        Self {
            env,
            project_name,
            l1,
        }
    }

    fn stop(&self) -> &Self {
        Self::compose_cmd_prefix(&self.env, &self.project_name)
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
