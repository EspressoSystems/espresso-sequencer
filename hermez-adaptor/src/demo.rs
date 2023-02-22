#![cfg(any(test, feature = "testing"))]
use crate::{wait_for_rpc, Layer1Backend, ZkEvmEnv, ZkEvmNode};
use contract_bindings::TestHermezContracts;
use std::{collections::HashMap, env, time::Duration};

/// A zkevm-node inside docker compose with custom contracts
#[derive(Debug, Clone)]
pub struct DemoZkEvmNode {
    env: ZkEvmEnv,
    l1: TestHermezContracts,
    project_name: String,
    layer1_backend: Layer1Backend,
}

impl DemoZkEvmNode {
    pub fn env(&self) -> &ZkEvmEnv {
        &self.env
    }

    pub fn l1(&self) -> &TestHermezContracts {
        &self.l1
    }

    pub fn project_name(&self) -> &String {
        &self.project_name
    }

    pub fn layer1_backend(&self) -> &Layer1Backend {
        &self.layer1_backend
    }

    /// Start the L1, deploy contracts, start the L2
    pub async fn start() -> Self {
        let env = ZkEvmEnv::default();
        let project_name = "demo".to_string();
        let layer1_backend = Layer1Backend::Geth;

        tracing::info!("Starting ZkEvmNode with env: {:?}", env);
        tracing::info!(
            "Compose prefix: {:?}",
            ZkEvmNode::compose_cmd_prefix(&env, &project_name, &layer1_backend)
        );

        // Start L1
        ZkEvmNode::compose_cmd_prefix(&env, &project_name, &layer1_backend)
            .arg("up")
            .arg("zkevm-mock-l1-network")
            .arg("-V")
            .arg("--force-recreate")
            .arg("--abort-on-container-exit")
            .spawn()
            .expect("Failed to start L1 docker container");

        tracing::info!("Waiting for L1 to start ...");

        wait_for_rpc(&env.l1_provider(), Duration::from_millis(200), 100)
            .await
            .unwrap();

        let dotenv = load_dotenv();
        let rollup_address = dotenv["ESPRESSO_ZKEVM_ROLLUP_ADDRESS"].parse().unwrap();
        let bridge_address = dotenv["ESPRESSO_ZKEVM_BRIDGE_ADDRESS"].parse().unwrap();
        let global_exit_root_address = dotenv["ESPRESSO_ZKEVM_GER_ADDRESS"].parse().unwrap();
        let verifier_address = dotenv["ESPRESSO_ZKEVM_VERIFIER_ADDRESS"].parse().unwrap();
        let matic_address = dotenv["ESPRESSO_ZKEVM_MATIC_ADDRESS"].parse().unwrap();

        let l1 = TestHermezContracts::connect(
            &env.l1_provider(),
            rollup_address,
            bridge_address,
            global_exit_root_address,
            verifier_address,
            matic_address,
        )
        .await;

        // Start zkevm-node
        ZkEvmNode::compose_cmd_prefix(&env, &project_name, &layer1_backend)
            .env(
                "ESPRESSO_ZKEVM_GENBLOCKNUMBER",
                l1.gen_block_number.to_string(),
            )
            .arg("up")
            .arg("zkevm-prover")
            .arg("zkevm-aggregator")
            .arg("zkevm-state-db")
            .arg("zkevm-permissionless-node")
            .arg("zkevm-eth-tx-manager")
            .arg("-V")
            .arg("--force-recreate")
            .arg("--abort-on-container-exit")
            .spawn()
            .expect("Failed to start zkevm-node compose environment");

        wait_for_rpc(&env.l2_provider(), Duration::from_secs(1), 100)
            .await
            .expect("Failed to start zkevm-node");

        Self {
            env,
            project_name,
            l1,
            layer1_backend,
        }
    }

    pub fn stop(&self) -> &Self {
        ZkEvmNode::compose_cmd_prefix(self.env(), self.project_name(), self.layer1_backend())
            .arg("down")
            .arg("-v")
            .arg("--remove-orphans")
            .spawn()
            .expect("Failed to run docker compose down");
        self
    }
}

fn load_dotenv() -> HashMap<String, String> {
    let old_vars: HashMap<_, _> = env::vars().collect();

    // dotenv only supports loading the vars into the environment.
    dotenv::dotenv().ok();
    let new_vars: HashMap<_, _> = dotenv::vars().collect();

    // Rebuild the old environment.
    for key in new_vars.keys() {
        match old_vars.get(key) {
            // If the key was in the old environment, set it back.
            Some(value) => env::set_var(key, value),
            // If the key was not in the old environment, remove it.
            None => env::remove_var(key),
        }
    }
    new_vars
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_dotenv_keeps_env() {
        let old_vars: HashMap<_, _> = env::vars().collect();
        load_dotenv();
        let new_vars: HashMap<_, _> = env::vars().collect();
        assert_eq!(new_vars, old_vars);
    }
}
