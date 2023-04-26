use std::time::Duration;

use contract_bindings::{example_rollup::ExampleRollup, HotShot, TestClients};
use ethers::{
    prelude::{k256::ecdsa::SigningKey, *},
    providers::Provider,
};
use surf_disco::Url;
type HotShotContract = HotShot<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;
type ExampleRollupContract = ExampleRollup<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

pub async fn deploy_example_contracts(url: &Url) -> (HotShotContract, ExampleRollupContract) {
    let mut provider = Provider::try_from(url.to_string()).unwrap();
    provider.set_interval(Duration::from_millis(10));
    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    let clients = TestClients::new(&provider, chain_id);
    let hotshot = HotShot::deploy(clients.trusted_sequencer.clone(), ())
        .unwrap()
        .send()
        .await
        .unwrap();

    let rollup = ExampleRollup::deploy(clients.deployer.clone(), hotshot.address())
        .unwrap()
        .send()
        .await
        .unwrap();

    (hotshot, rollup)
}
