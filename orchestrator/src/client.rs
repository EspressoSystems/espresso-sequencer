use std::{net::IpAddr, time::Duration};

use crate::config::NetworkConfig;
use async_compatibility_layer::art::async_sleep;
use clap::Parser;
use futures::{Future, FutureExt};

use hotshot_types::traits::{election::ElectionConfig, signature_key::SignatureKey};
use surf_disco::{error::ClientError, Client};
use tide_disco::Url;

/// Holds the client connection to the orchestrator
pub struct OrchestratorClient {
    client: surf_disco::Client<ClientError>,
    pub identity: String,
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
    pub url: Url,
    /// This node's public IP address, for libp2p
    /// If no IP address is passed in, it will default to 127.0.0.1
    pub public_ip: Option<IpAddr>,
    /// An optional network config file to save to/load from
    /// Allows for rejoining the network on a complete state loss
    #[arg(short, long)]
    pub network_config_file: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct MultiValidatorArgs {
    /// Number of validators to run
    pub num_nodes: u16,
    /// The address the orchestrator runs on
    pub url: Url,
    /// This node's public IP address, for libp2p
    /// If no IP address is passed in, it will default to 127.0.0.1
    pub public_ip: Option<IpAddr>,
    /// An optional network config file to save to/load from
    /// Allows for rejoining the network on a complete state loss
    #[arg(short, long)]
    pub network_config_file: Option<String>,
}

impl ValidatorArgs {
    /// Constructs `ValidatorArgs` from `MultiValidatorArgs` and a node index.
    ///
    /// If `network_config_file` is present in `MultiValidatorArgs`, it appends the node index to it to create a unique file name for each node.
    ///
    /// # Arguments
    ///
    /// * `multi_args` - A `MultiValidatorArgs` instance containing the base arguments for the construction.
    /// * `node_index` - A `u16` representing the index of the node for which the args are being constructed.
    ///
    /// # Returns
    ///
    /// This function returns a new instance of `ValidatorArgs`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // NOTE this is a toy example,
    /// // the user will need to construct a multivalidatorargs since `new` does not exist
    /// # use hotshot_orchestrator::client::MultiValidatorArgs;
    /// let multi_args = MultiValidatorArgs::new();
    /// let node_index = 1;
    /// let instance = Self::from_multi_args(multi_args, node_index);
    /// ```
    pub fn from_multi_args(multi_args: MultiValidatorArgs, node_index: u16) -> Self {
        Self {
            url: multi_args.url,
            public_ip: multi_args.public_ip,
            network_config_file: multi_args
                .network_config_file
                .map(|s| format!("{}-{}", s, node_index)),
        }
    }
}

impl OrchestratorClient {
    /// Creates the client that will connect to the orchestrator
    pub async fn new(args: ValidatorArgs, identity: String) -> Self {
        let client = surf_disco::Client::<ClientError>::new(args.url);
        // TODO ED: Add healthcheck wait here
        OrchestratorClient { client, identity }
    }

    /// Sends an identify message to the orchestrator and attempts to get its config
    /// Returns both the node_index and the run configuration from the orchestrator
    /// Will block until both are returned
    #[allow(clippy::type_complexity)]
    pub async fn get_config<K: SignatureKey, E: ElectionConfig>(
        &self,
        identity: String,
    ) -> NetworkConfig<K, E> {
        // get the node index
        let identity = identity.as_str();
        let identity = |client: Client<ClientError>| {
            async move {
                let node_index: Result<u16, ClientError> = client
                    .post(&format!("api/identity/{identity}"))
                    .send()
                    .await;
                node_index
            }
            .boxed()
        };
        let node_index = self.wait_for_fn_from_orchestrator(identity).await;

        // get the corresponding config
        let f = |client: Client<ClientError>| {
            async move {
                let config: Result<NetworkConfig<K, E>, ClientError> = client
                    .post(&format!("api/config/{node_index}"))
                    .send()
                    .await;
                config
            }
            .boxed()
        };

        let mut config = self.wait_for_fn_from_orchestrator(f).await;

        config.node_index = node_index as u64;

        config
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
