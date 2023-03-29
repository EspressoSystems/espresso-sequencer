use clap::Parser;
use ethers::prelude::*;
use surf_disco::Url;
use zkevm::ZkEvm;

pub mod json_rpc;
pub mod query_service;
pub mod sequencer;

#[derive(Parser)]
pub struct Options {
    /// URL of a HotShot sequencer node.
    #[clap(long, env = "ESPRESSO_SEQUENCER_URL")]
    pub sequencer_url: Url,

    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(long, env = "ESPRESSO_ZKEVM_L1_PROVIDER")]
    pub l1_provider: Url,

    /// Chain ID for layer 1 Ethereum.
    ///
    /// This can be specified explicitly as a sanity check. No transactions will be executed if the
    /// RPC specified by `l1_provider` has a different chain ID. If not specified, the chain ID from
    /// the RPC will be used.
    #[clap(long, env = "ESPRESSO_ZKEVM_L1_CHAIN_ID")]
    pub l1_chain_id: Option<u64>,

    /// Chain ID for layer 2 EVM.
    ///
    /// This will be used as the VM ID for layer 2 EVM transactions within the HotShot sequencer.
    #[clap(long, env = "ESPRESSO_ZKEVM_L2_CHAIN_ID", default_value = "1001")]
    pub l2_chain_id: u64,

    /// Address of HotShot contract on layer 1.
    #[clap(long, env = "ESPRESSO_ZKEVM_HOTSHOT_ADDRESS")]
    pub hotshot_address: Address,

    /// Mnemonic phrase for the sequencer wallet.
    ///
    /// This is the wallet that will be used to send blocks sequenced by HotShot to the rollup
    /// contract. It must be funded with ETH and MATIC on layer 1.
    #[clap(long, env = "ESPRESSO_ZKEVM_SEQUENCER_MNEMONIC")]
    pub sequencer_mnemonic: String,

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
