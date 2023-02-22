#![cfg(any(test, feature = "testing"))]
use clap::Parser;
use hermez_adaptor::DemoZkEvmNode;
use scopeguard::defer;

#[derive(Parser)]
struct Options {
    /// Whether to run in background
    #[clap(short, long, action)]
    detach: bool,
}

#[async_std::main]
async fn main() {
    let opt = Options::parse();
    let node = DemoZkEvmNode::start().await;

    if !opt.detach {
        defer! {node.stop();}
        loop {
            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
