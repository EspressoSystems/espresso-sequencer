#![cfg(any(test, feature = "testing"))]
use clap::Parser;
use hermez_adaptor::{DemoZkEvmNode, Layer1Backend};

#[derive(Parser)]
struct Options {
    /// Choice of Layer 1 backend
    #[clap(long, env = "ESPRESSO_ZKEVM_L1_PROVIDER", default_value = "Geth")]
    backend: Layer1Backend,
}

#[async_std::main]
async fn main() {
    let _opt = Options::parse();

    let _node = DemoZkEvmNode::start().await;
}
