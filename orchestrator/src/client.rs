use std::{net::IpAddr, time::Duration};

use crate::config::NetworkConfig;
use async_compatibility_layer::art::async_sleep;
use clap::Parser;
use futures::{Future, FutureExt};

use hotshot_types::traits::node_implementation::NodeType;
use surf_disco::{error::ClientError, Client};

/// Holds the client connection to the orchestrator
pub struct OrchestratorClient {
    client: surf_disco::Client<ClientError>,
}

// VALIDATOR

#[derive(Parser, Debug, Clone)]
#[command(
    name = "Multi-machine consensus",
    about = "Simulates consensus among multiple machines"
)]
/// Arguments passed to the validator
pub struct ValidatorArgs {
    /// The address the orchestrator runs on
    pub host: String,
    /// The port the orchestrator runs on
    pub port: u16,
    /// This node's public IP address, for libp2p
    /// If no IP address is passed in, it will default to 127.0.0.1
    pub public_ip: Option<IpAddr>,
}

impl OrchestratorClient {
    /// Creates the client that connects to the orchestrator
    pub async fn connect_to_orchestrator(args: ValidatorArgs) -> Self {
        let base_url = format!("{0}:{1}", args.host, args.port);
        let base_url = format!("http://{base_url}").parse().unwrap();
        let client = surf_disco::Client::<ClientError>::new(base_url);
        // TODO ED: Add healthcheck wait here
        OrchestratorClient { client }
    }

    /// Sends an identify message to the server
    /// Returns this validator's node_index in the network
    pub async fn identify_with_orchestrator(&self, identity: String) -> u16 {
        let identity = identity.as_str();
        let f = |client: Client<ClientError>| {
            async move {
                let node_index: Result<u16, ClientError> = client
                    .post(&format!("api/identity/{identity}"))
                    .send()
                    .await;
                node_index
            }
            .boxed()
        };
        self.wait_for_fn_from_orchestrator(f).await
    }

    /// Returns the run configuration from the orchestrator
    /// Will block until the configuration is returned
    #[allow(clippy::type_complexity)]

    pub async fn get_config_from_orchestrator<TYPES: NodeType>(
        &self,
        node_index: u16,
    ) -> NetworkConfig<
        TYPES::SignatureKey,
        TYPES::ElectionConfigType,
    > {
        let f = |client: Client<ClientError>| {
            async move {
                let config: Result<
                    NetworkConfig<
                        TYPES::SignatureKey,
                        TYPES::ElectionConfigType,
                    >,
                    ClientError,
                > = client
                    .post(&format!("api/config/{node_index}"))
                    .send()
                    .await;
                config
            }
            .boxed()
        };
        self.wait_for_fn_from_orchestrator(f).await
    }

    /// Tells the orchestrator this validator is ready to start
    /// Blocks until the orchestrator indicates all nodes are ready to start
    pub async fn wait_for_all_nodes_ready(&self, node_index: u64) -> bool {
        let send_ready_f = |client: Client<ClientError>| {
            async move {
                let result: Result<_, ClientError> = client
                    .post("api/ready")
                    .body_json(&node_index)
                    .unwrap()
                    .send()
                    .await;
                result
            }
            .boxed()
        };
        self.wait_for_fn_from_orchestrator::<_, _, ()>(send_ready_f)
            .await;

        let wait_for_all_nodes_ready_f = |client: Client<ClientError>| {
            async move { client.get("api/start").send().await }.boxed()
        };
        self.wait_for_fn_from_orchestrator(wait_for_all_nodes_ready_f)
            .await
    }

    /// Generic function that waits for the orchestrator to return a non-error
    /// Returns whatever type the given function returns
    async fn wait_for_fn_from_orchestrator<F, Fut, GEN>(&self, f: F) -> GEN
    where
        F: Fn(Client<ClientError>) -> Fut,
        Fut: Future<Output = Result<GEN, ClientError>>,
    {
        loop {
            let client = self.client.clone();
            let res = f(client).await;
            match res {
                Ok(x) => break x,
                Err(_x) => {
                    async_sleep(Duration::from_millis(250)).await;
                }
            }
        }
    }
}
