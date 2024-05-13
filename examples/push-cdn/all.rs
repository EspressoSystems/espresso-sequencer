//! A example program using the Push CDN
/// The types we're importing
pub mod types;

use async_compatibility_layer::art::async_spawn;
use cdn_broker::{reexports::crypto::signature::KeyPair, Broker};
use cdn_marshal::Marshal;
use hotshot::{
    traits::implementations::{TestingDef, WrappedSignatureKey},
    types::SignatureKey,
};
use hotshot_example_types::state_types::TestTypes;
use hotshot_orchestrator::client::ValidatorArgs;
use hotshot_types::traits::node_implementation::NodeType;

use crate::{
    infra::{read_orchestrator_init_config, run_orchestrator, OrchestratorArgs},
    types::{DaNetwork, NodeImpl, QuorumNetwork, ThisRun},
};

/// The infra implementation
#[path = "../infra/mod.rs"]
pub mod infra;

use tracing::error;

#[cfg_attr(async_executor_impl = "tokio", tokio::main)]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
async fn main() {
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    setup_logging();
    setup_backtrace();

    // use configfile args
    let (config, orchestrator_url) = read_orchestrator_init_config::<TestTypes>();

    // Start the orhcestrator
    async_spawn(run_orchestrator::<TestTypes>(OrchestratorArgs {
        url: orchestrator_url.clone(),
        config: config.clone(),
    }));

    // The configuration we are using for this example is 2 brokers & 1 marshal

    // A keypair shared between brokers
    let (broker_public_key, broker_private_key) =
        <TestTypes as NodeType>::SignatureKey::generated_from_seed_indexed([0u8; 32], 1337);

    // The broker (peer) discovery endpoint shall be a local SQLite file
    let discovery_endpoint = "test.sqlite".to_string();

    // 2 brokers
    for _ in 0..2 {
        // Get the ports to bind to
        let private_port = portpicker::pick_unused_port().expect("could not find an open port");
        let public_port = portpicker::pick_unused_port().expect("could not find an open port");

        // Extrapolate addresses
        let private_address = format!("127.0.0.1:{private_port}");
        let public_address = format!("127.0.0.1:{public_port}");

        let config: cdn_broker::Config<TestingDef<TestTypes>> = cdn_broker::Config {
            discovery_endpoint: discovery_endpoint.clone(),
            public_advertise_endpoint: public_address.clone(),
            public_bind_endpoint: public_address,
            private_advertise_endpoint: private_address.clone(),
            private_bind_endpoint: private_address,

            keypair: KeyPair {
                public_key: WrappedSignatureKey(broker_public_key),
                private_key: broker_private_key.clone(),
            },

            metrics_bind_endpoint: None,
            ca_cert_path: None,
            ca_key_path: None,
        };

        // Create and spawn the broker
        async_spawn(async move {
            let broker: Broker<TestingDef<TestTypes>> =
                Broker::new(config).await.expect("broker failed to start");

            // Error if we stopped unexpectedly
            if let Err(err) = broker.start().await {
                error!("broker stopped: {err}");
            }
        });
    }

    // Get the port to use for the marshal
    let marshal_port = 9000;

    // Configure the marshal
    let marshal_endpoint = format!("127.0.0.1:{marshal_port}");
    let marshal_config = cdn_marshal::Config {
        bind_endpoint: marshal_endpoint.clone(),
        discovery_endpoint,
        metrics_bind_endpoint: None,
        ca_cert_path: None,
        ca_key_path: None,
    };

    // Spawn the marshal
    async_spawn(async move {
        let marshal: Marshal<TestingDef<TestTypes>> = Marshal::new(marshal_config)
            .await
            .expect("failed to spawn marshal");

        // Error if we stopped unexpectedly
        if let Err(err) = marshal.start().await {
            error!("broker stopped: {err}");
        }
    });

    // Start the proper number of nodes
    let mut nodes = Vec::new();
    for _ in 0..(config.config.num_nodes_with_stake.get()) {
        let orchestrator_url = orchestrator_url.clone();
        let node = async_spawn(async move {
            infra::main_entry_point::<TestTypes, DaNetwork, QuorumNetwork, NodeImpl, ThisRun>(
                ValidatorArgs {
                    url: orchestrator_url,
                    advertise_address: None,
                    network_config_file: None,
                },
            )
            .await;
        });
        nodes.push(node);
    }
    let _result = futures::future::join_all(nodes).await;
}
