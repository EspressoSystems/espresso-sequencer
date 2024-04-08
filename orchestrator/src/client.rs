use std::{net::SocketAddr, time::Duration};

use crate::{config::NetworkConfig, OrchestratorVersion};
use async_compatibility_layer::art::async_sleep;
use clap::Parser;
use futures::{Future, FutureExt};

use hotshot_types::{
    constants::Version01,
    traits::{election::ElectionConfig, signature_key::SignatureKey},
    PeerConfig,
};
use libp2p::{Multiaddr, PeerId};
use surf_disco::{error::ClientError, Client};
use tide_disco::Url;
use vbs::BinarySerializer;
/// Holds the client connection to the orchestrator
pub struct OrchestratorClient {
    /// the client
    client: surf_disco::Client<ClientError, OrchestratorVersion>,
}

/// Struct describing a benchmark result
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BenchResults {
    /// The average latency of the transactions
    pub avg_latency_in_sec: i64,
    /// The number of transactions that were latency measured
    pub num_latency: i64,
    /// The minimum latency of the transactions
    pub minimum_latency_in_sec: i64,
    /// The maximum latency of the transactions
    pub maximum_latency_in_sec: i64,
    /// The throughput of the consensus protocol = number of transactions committed per second * transaction size in bytes
    pub throughput_bytes_per_sec: u64,
    /// The number of transactions committed during benchmarking
    pub total_transactions_committed: u64,
    /// The size of each transaction in bytes
    pub transaction_size_in_bytes: u64,
    /// The total time elapsed for benchmarking
    pub total_time_elapsed_in_sec: u64,
    /// The total number of views during benchmarking
    pub total_num_views: usize,
    /// The number of failed views during benchmarking
    pub failed_num_views: usize,
}

impl BenchResults {
    /// printout the results of one example run
    pub fn printout(&self) {
        println!("=====================");
        println!("Benchmark results:");
        println!(
            "Average latency: {} seconds, Minimum latency: {} seconds, Maximum latency: {} seconds",
            self.avg_latency_in_sec, self.minimum_latency_in_sec, self.maximum_latency_in_sec
        );
        println!("Throughput: {} bytes/sec", self.throughput_bytes_per_sec);
        println!(
            "Total transactions committed: {}",
            self.total_transactions_committed
        );
        println!(
            "Total number of views: {}, Failed number of views: {}",
            self.total_num_views, self.failed_num_views
        );
        println!("=====================");
    }
}

/// Struct describing a benchmark result needed for download, also include the config
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BenchResultsDownloadConfig {
    // Config starting here
    /// The commit this benchmark was run on
    pub commit_sha: String,
    /// Total number of nodes
    pub total_nodes: usize,
    /// The size of the da committee
    pub da_committee_size: usize,
    /// Number of transactions submitted per round
    pub transactions_per_round: usize,
    /// The size of each transaction in bytes
    pub transaction_size: u64,
    /// The number of rounds
    pub rounds: usize,
    /// The type of leader election used
    pub leader_election_type: String,

    // Results starting here
    /// The average latency of the transactions
    pub avg_latency_in_sec: i64,
    /// The minimum latency of the transactions
    pub minimum_latency_in_sec: i64,
    /// The maximum latency of the transactions
    pub maximum_latency_in_sec: i64,
    /// The throughput of the consensus protocol = number of transactions committed per second * transaction size in bytes
    pub throughput_bytes_per_sec: u64,
    /// The number of transactions committed during benchmarking
    pub total_transactions_committed: u64,
    /// The total time elapsed for benchmarking
    pub total_time_elapsed_in_sec: u64,
    /// The total number of views during benchmarking
    pub total_num_views: usize,
    /// The number of failed views during benchmarking
    pub failed_num_views: usize,
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
    /// The optional advertise address to use for Libp2p
    pub advertise_address: Option<SocketAddr>,
    /// An optional network config file to save to/load from
    /// Allows for rejoining the network on a complete state loss
    #[arg(short, long)]
    pub network_config_file: Option<String>,
}

/// arguments to run multiple validators
#[derive(Parser, Debug, Clone)]
pub struct MultiValidatorArgs {
    /// Number of validators to run
    pub num_nodes: u16,
    /// The address the orchestrator runs on
    pub url: Url,
    /// The optional advertise address to use for Libp2p
    pub advertise_address: Option<SocketAddr>,
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
    #[must_use]
    pub fn from_multi_args(multi_args: MultiValidatorArgs, node_index: u16) -> Self {
        Self {
            url: multi_args.url,
            advertise_address: multi_args.advertise_address,
            network_config_file: multi_args
                .network_config_file
                .map(|s| format!("{s}-{node_index}")),
        }
    }
}

impl OrchestratorClient {
    /// Creates the client that will connect to the orchestrator
    #[must_use]
    pub fn new(args: ValidatorArgs) -> Self {
        let client = surf_disco::Client::<ClientError, OrchestratorVersion>::new(args.url);
        // TODO ED: Add healthcheck wait here
        OrchestratorClient { client }
    }

    /// Get the config from the orchestrator.
    /// If the identity is provided, register the identity with the orchestrator.
    /// If not, just retrieving the config (for passive observers)
    ///
    /// # Panics
    /// if unable to convert the node index from usize into u64
    /// (only applicable on 32 bit systems)
    ///
    /// # Errors
    /// If we were unable to serialize the Libp2p data
    #[allow(clippy::type_complexity)]
    pub async fn get_config_without_peer<K: SignatureKey, E: ElectionConfig>(
        &self,
        libp2p_address: Option<SocketAddr>,
        libp2p_public_key: Option<PeerId>,
    ) -> anyhow::Result<NetworkConfig<K, E>> {
        // Get the (possible) Libp2p advertise address from our args
        let libp2p_address = libp2p_address.map(|f| {
            Multiaddr::try_from(format!(
                "/{}/{}/udp/{}/quic-v1",
                if f.is_ipv4() { "ip4" } else { "ip6" },
                f.ip(),
                f.port()
            ))
            .expect("failed to create multiaddress")
        });

        // Serialize our (possible) libp2p-specific data
        let request_body =
            vbs::Serializer::<Version01>::serialize(&(libp2p_address, libp2p_public_key))?;

        let identity = |client: Client<ClientError, OrchestratorVersion>| {
            // We need to clone here to move it into the closure
            let request_body = request_body.clone();
            async move {
                let node_index: Result<u16, ClientError> = client
                    .post("api/identity")
                    .body_binary(&request_body)
                    .expect("failed to set request body")
                    .send()
                    .await;

                node_index
            }
            .boxed()
        };
        let node_index = self.wait_for_fn_from_orchestrator(identity).await;

        // get the corresponding config
        let f = |client: Client<ClientError, OrchestratorVersion>| {
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
        config.node_index = From::<u16>::from(node_index);

        Ok(config)
    }

    /// Post to the orchestrator and get the latest `node_index`
    /// Then return it for the init validator config
    /// # Panics
    /// if unable to post
    pub async fn get_node_index_for_init_validator_config(&self) -> u16 {
        let cur_node_index = |client: Client<ClientError, OrchestratorVersion>| {
            async move {
                let cur_node_index: Result<u16, ClientError> =
                    client.post("api/get_tmp_node_index").send().await;
                cur_node_index
            }
            .boxed()
        };
        self.wait_for_fn_from_orchestrator(cur_node_index).await
    }

    /// Requests the configuration from the orchestrator with the stipulation that
    /// a successful call requires all nodes to be registered.
    ///
    /// Does not fail, retries internally until success.
    pub async fn get_config_after_collection<K: SignatureKey, E: ElectionConfig>(
        &self,
    ) -> NetworkConfig<K, E> {
        // Define the request for post-register configurations
        let get_config_after_collection = |client: Client<ClientError, OrchestratorVersion>| {
            async move {
                client
                    .get("api/get_config_after_peer_collected")
                    .send()
                    .await
            }
            .boxed()
        };

        // Loop until successful
        self.wait_for_fn_from_orchestrator(get_config_after_collection)
            .await
    }

    /// Sends my public key to the orchestrator so that it can collect all public keys
    /// And get the updated config
    /// Blocks until the orchestrator collects all peer's public keys/configs
    /// # Panics
    /// if unable to post
    pub async fn post_and_wait_all_public_keys<K: SignatureKey, E: ElectionConfig>(
        &self,
        node_index: u64,
        my_pub_key: PeerConfig<K>,
    ) -> NetworkConfig<K, E> {
        // send my public key
        let _send_pubkey_ready_f: Result<(), ClientError> = self
            .client
            .post(&format!("api/pubkey/{node_index}"))
            .body_binary(&PeerConfig::<K>::to_bytes(&my_pub_key))
            .unwrap()
            .send()
            .await;

        // wait for all nodes' public keys
        let wait_for_all_nodes_pub_key = |client: Client<ClientError, OrchestratorVersion>| {
            async move { client.get("api/peer_pub_ready").send().await }.boxed()
        };
        self.wait_for_fn_from_orchestrator::<_, _, ()>(wait_for_all_nodes_pub_key)
            .await;

        self.get_config_after_collection().await
    }

    /// Tells the orchestrator this validator is ready to start
    /// Blocks until the orchestrator indicates all nodes are ready to start
    /// # Panics
    /// Panics if unable to post.
    pub async fn wait_for_all_nodes_ready(&self, node_index: u64) -> bool {
        let send_ready_f = |client: Client<ClientError, OrchestratorVersion>| {
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

        let wait_for_all_nodes_ready_f = |client: Client<ClientError, OrchestratorVersion>| {
            async move { client.get("api/start").send().await }.boxed()
        };
        self.wait_for_fn_from_orchestrator(wait_for_all_nodes_ready_f)
            .await
    }

    /// Sends the benchmark metrics to the orchestrator
    /// # Panics
    /// Panics if unable to post
    pub async fn post_bench_results(&self, bench_results: BenchResults) {
        let _send_metrics_f: Result<(), ClientError> = self
            .client
            .post("api/results")
            .body_json(&bench_results)
            .unwrap()
            .send()
            .await;
    }

    /// Generic function that waits for the orchestrator to return a non-error
    /// Returns whatever type the given function returns
    async fn wait_for_fn_from_orchestrator<F, Fut, GEN>(&self, f: F) -> GEN
    where
        F: Fn(Client<ClientError, OrchestratorVersion>) -> Fut,
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
