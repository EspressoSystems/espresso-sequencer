#![cfg(any(test, feature = "testing"))]
use hermez_adaptor::DemoZkEvmNode;

#[async_std::main]
async fn main() {
    let _node = DemoZkEvmNode::start().await;
}
