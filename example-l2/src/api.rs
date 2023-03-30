use std::sync::Arc;

use async_std::sync::RwLock;
use ethers::signers::{LocalWallet, Signer};
use futures::FutureExt;
use tide_disco::{Api, App};

use crate::{error::RollupError, state::State};

pub async fn serve() {
    let mut rng = rand::thread_rng();
    type StateType = Arc<RwLock<State>>;
    // TODO: load this from memory so that a CLI can sign transactions
    let genesis_wallet = LocalWallet::new(&mut rng);
    println!("Genesis Wallet Address: {}", genesis_wallet.address());
    // let str = genesis_wallet.address().to_string();
    // let recovered = str.parse::<Address>().unwrap();
    // assert_eq!(genesis_wallet.address(), recovered);
    let seed_data = [(genesis_wallet.address(), 99999)];
    let state = Arc::new(RwLock::new(State::from_initial_balances(seed_data)));
    let mut app = App::<StateType, RollupError>::with_state(state);
    let toml = toml::from_str::<toml::Value>(include_str!("api.toml")).unwrap();
    let mut api = Api::<StateType, RollupError>::new(toml).unwrap();

    api.post("submit", |_, _| {
        async move {
            // Todo: Forward transaction to the sequencer
            Ok(())
        }
        .boxed()
    })
    .unwrap();

    api.get("balance", |req, _state| {
        async move {
            let address = req.param("address").unwrap().as_string().unwrap();
            println!("{}", address);
            Ok(())
        }
        .boxed()
    })
    .unwrap();
    app.register_module("rollup", api).unwrap();
    let res = app.serve("0.0.0.0:8080".to_string()).await;
    res.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[async_std::test]
    async fn smoke_test() {
        serve().await;
    }
}
