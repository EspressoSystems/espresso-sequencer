use clap::Parser;
use surf_disco::Url;
use zkevm::ZkEvm;

pub mod json_rpc;
pub mod query_service;

#[derive(Parser)]
pub struct Options {
    /// URL of a HotShot sequencer node.
    #[clap(long, env = "ESPRESSO_SEQUENCER_URL")]
    pub sequencer_url: Url,

    /// Chain ID for layer 2 EVM.
    ///
    /// This will be used as the VM ID for layer 2 EVM transactions within the HotShot sequencer.
    #[clap(long, env = "ESPRESSO_ZKEVM_L2_CHAIN_ID", default_value = "1001")]
    pub l2_chain_id: u64,
    /// Port on which to serve the JSON-RPC API.
    #[clap(
        short,
        long,
        env = "ESPRESSO_ZKEVM_ADAPTOR_RPC_PORT",
        default_value = "8545"
    )]
    pub rpc_port: u16,

    /// Port on which to serve the Hermez query API adaptor.
    #[clap(
        short,
        long,
        env = "ESPRESSO_ZKEVM_ADAPTOR_QUERY_PORT",
        default_value = "50100"
    )]
    pub query_port: u16,
}

impl Options {
    pub fn zkevm(&self) -> ZkEvm {
        ZkEvm {
            chain_id: self.l2_chain_id,
        }
    }
}

mod hermez;

#[cfg(any(test, feature = "testing"))]
pub use hermez::*;

mod demo;
#[cfg(any(test, feature = "testing"))]
pub use demo::*;
