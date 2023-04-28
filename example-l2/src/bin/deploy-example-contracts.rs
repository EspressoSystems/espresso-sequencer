use clap::Parser;
use example_l2::utils::deploy_example_contracts;
use surf_disco::Url;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(long, env = "ESPRESSO_DEMO_L1_PROVIDER")]
    pub l1_provider: Url,
}

#[async_std::main]
async fn main() {
    let opt = Options::parse();
    deploy_example_contracts(&opt.l1_provider).await;
}
