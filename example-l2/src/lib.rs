use clap::Parser;
use ethers::types::Address;
use sequencer::{Vm, VmId};
use surf_disco::Url;
use transaction::SignedTransaction;

pub mod api;
pub mod error;
pub mod executor;
mod prover;
pub mod state;
pub mod transaction;
pub mod utils;

// The VmID helps Rollups find their transactions in the sequenced block.
pub const VM_ID: u64 = 1;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// Port where the Rollup API will be served
    #[clap(short, long, env = "ESPRESSO_DEMO_ROLLUP_PORT", default_value = "8082")]
    pub api_port: u16,

    /// URL of a HotShot sequencer node.
    #[clap(long, env = "ESPRESSO_SEQUENCER_URL")]
    pub sequencer_url: Url,

    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(long, env = "ESPRESSO_DEMO_L1_PROVIDER")]
    pub l1_provider: Url,

    /// Address of HotShot contract on layer 1.
    #[clap(long, env = "ESPRESSO_DEMO_HOTSHOT_ADDRESS")]
    pub hotshot_address: Address,

    /// Address of Rollup contract on layer 1.
    #[clap(long, env = "ESPRESSO_DEMO_ROLLUP_ADDRESS")]
    pub rollup_address: Address,

    /// Mnemonic phrase for the rollup wallet.
    ///
    /// This is the wallet that will be used to send batch proofs of transaction validity to the rollup
    /// contract. It must be funded with ETH on the layer 1.
    #[clap(long, env = "ESPRESSO_DEMO_ROLLUP_MNEMONIC")]
    pub rollup_mnemonic: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RollupVM;

impl Vm for RollupVM {
    type Transaction = SignedTransaction;

    fn id(&self) -> VmId {
        VM_ID.into()
    }
}
