use clap::Parser;
use example_l2::utils::deploy_example_contracts;
use surf_disco::Url;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// Port of the test L1
    #[clap(short, long, env = "ESPRESSO_ZKEVM_L1_PORT", default_value = "8545")]
    pub l1_port: u16,
}

#[async_std::main]
async fn main() {
    let opt = Options::parse();
    let l1_url = Url::parse(&format!("http://localhost:{}", opt.l1_port)).unwrap();
    deploy_example_contracts(&l1_url).await;
}
