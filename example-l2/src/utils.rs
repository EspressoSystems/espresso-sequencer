use std::time::Duration;

use crate::state::State;
use commit::Commitment;
use contract_bindings::{example_rollup::ExampleRollup, HotShot, TestClients};
use ethers::{
    prelude::{k256::ecdsa::SigningKey, *},
    providers::Provider,
};
use sequencer_utils::commitment_to_u256;
use surf_disco::Url;
type HotShotContract = HotShot<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;
type ExampleRollupContract = ExampleRollup<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

pub async fn deploy_example_contracts(
    url: &Url,
    initial_state: Commitment<State>,
) -> (HotShotContract, ExampleRollupContract, TestClients) {
    let mut provider = Provider::try_from(url.to_string()).unwrap();
    provider.set_interval(Duration::from_millis(10));
    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    let clients = TestClients::new(&provider, chain_id);
    let hotshot_contract = HotShot::deploy(clients.deployer.provider.clone(), ())
        .unwrap()
        .send()
        .await
        .unwrap();
    let rollup_contract = ExampleRollup::deploy(
        clients.deployer.provider.clone(),
        (
            hotshot_contract.address(),
            commitment_to_u256(initial_state),
        ),
    )
    .unwrap()
    .send()
    .await
    .unwrap();

    (hotshot_contract, rollup_contract, clients)
}
