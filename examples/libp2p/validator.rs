//! A validator using libp2p
use std::{net::SocketAddr, str::FromStr};

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use hotshot_example_types::state_types::TestTypes;
use hotshot_orchestrator::client::ValidatorArgs;
use local_ip_address::local_ip;
use tracing::{debug, instrument};

use crate::types::{DaNetwork, NodeImpl, QuorumNetwork, ThisRun};

/// types used for this example
pub mod types;

/// general infra used for this example
#[path = "../infra/mod.rs"]
pub mod infra;

#[cfg_attr(async_executor_impl = "tokio", tokio::main(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
#[instrument]
async fn main() {
    setup_logging();
    setup_backtrace();
    let mut args = ValidatorArgs::parse();

    // If we did not set the advertise address, use our local IP and port 8000
    let local_ip = local_ip().expect("failed to get local IP");
    args.advertise_address = Some(
        args.advertise_address.unwrap_or(
            SocketAddr::from_str(&format!("{local_ip}:8000"))
                .expect("failed to convert local IP to socket address"),
        ),
    );

    debug!("connecting to orchestrator at {:?}", args.url);
    infra::main_entry_point::<TestTypes, DaNetwork, QuorumNetwork, NodeImpl, ThisRun>(args).await;
}
