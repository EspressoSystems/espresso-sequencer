use async_std::sync::RwLock;
use clap::Parser;
use ethers::abi::Address;
use futures::FutureExt;
use sequencer::Transaction;
use sequencer::VmTransaction;
use std::io;
use std::sync::Arc;
use surf_disco::{error::ClientError, Url};
use tide_disco::{error::ServerError, Api, App};

use crate::{state::State, transaction::SignedTransaction};

// The VmID helps Rollups find their transactions in the sequenced block.
pub const VM_ID: u64 = 1;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// Port where the Rollup API will be served
    #[clap(short, long, env = "ESPRESSO_MOCK_ROLLUP_PORT", default_value = "8080")]
    pub api_port: u16,
    /// URL of a HotShot sequencer node.
    #[clap(long, env = "ESPRESSO_SEQUENCER_URL")]
    pub sequencer_url: Url,
}

async fn submit_transaction(
    submit_url: Url,
    transaction: SignedTransaction,
) -> Result<(), ServerError> {
    let raw_tx = transaction.encode();
    let txn = Transaction::new(VM_ID.into(), raw_tx.to_vec());
    let client = surf_disco::Client::<ClientError>::new(submit_url);
    client
        .post::<()>("submit/submit")
        .body_json(&txn)?
        .send()
        .await?;
    Ok(())
}

pub async fn serve(options: Options, state: Arc<RwLock<State>>) -> io::Result<()> {
    type StateType = Arc<RwLock<State>>;
    let error_mapper = |err| io::Error::new(io::ErrorKind::Other, err);
    let Options {
        api_port,
        sequencer_url,
    } = options;
    let mut app = App::<StateType, ServerError>::with_state(state);
    let toml = toml::from_str::<toml::Value>(include_str!("api.toml"))
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    let mut api = Api::<StateType, ServerError>::new(toml).map_err(error_mapper)?;

    api.post("submit", move |req, _| {
        let url = sequencer_url.clone();
        async move {
            let transaction = req
                .body_auto::<SignedTransaction>().
            map_err(|_| ServerError {
                status: tide_disco::StatusCode::BadRequest,
                message: "Malformed transaction. Ensure that the transaction is a JSON serialized SignedTransaction".into()
            })?;
            submit_transaction(url, transaction).await
        }
        .boxed()
    })
    .map_err(error_mapper)?;

    api.get("balance", |req, state| {
        async move {
            let address_str = req.string_param("address")?;
            let address = address_str.parse::<Address>().
            map_err(|_| ServerError {
                status: tide_disco::StatusCode::BadRequest,
                message: "Malformed address. Ensure that the address is valid hex encoded Ethereum address.".into()
            })?;
            let balance = state.get_balance(&address);
            Ok(balance)
        }
        .boxed()
    })
    .map_err(error_mapper)?;

    app.register_module("rollup", api)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    app.serve(format!("0.0.0.0:{}", api_port)).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;
    use async_std::task::spawn;
    use ethers::signers::{LocalWallet, Signer};
    use futures::future::ready;
    use hotshot_query_service::data_source::QueryData;
    use portpicker::pick_unused_port;
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use sequencer::{
        api::SequencerNode, testing::wait_for_decide_on_handle, Transaction as SeqTransaction,
    };
    use std::path::Path;
    use surf_disco::Client;
    use tempfile::TempDir;

    const GENESIS_BALANCE: u64 = 9999;

    #[async_std::test]
    async fn query_test() {
        let mut rng = rand::thread_rng();
        let genesis_wallet = LocalWallet::new(&mut rng);
        let genesis_address = genesis_wallet.address();
        let state = Arc::new(RwLock::new(State::from_initial_balances([(
            genesis_address,
            GENESIS_BALANCE,
        )])));
        let port = pick_unused_port().expect("No ports free");
        let api_url: Url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(api_url.clone());
        let options = Options {
            api_port: port,
            sequencer_url: api_url,
        };

        spawn(serve(options, state));

        client.connect(None).await;

        // Fetch genesis block balance
        let balance = client
            .get::<u64>(&format!("rollup/balance/{:?}", genesis_address))
            .send()
            .await
            .unwrap();

        assert_eq!(balance, GENESIS_BALANCE);
    }

    #[async_std::test]
    async fn submit_test() {
        // Start a sequencer network.
        let sequencer_port = pick_unused_port().unwrap();
        let nodes = sequencer::testing::init_hotshot_handles().await;
        let api_node = nodes[0].clone();
        let tmp_dir = TempDir::new().unwrap();
        let storage_path: &Path = &(tmp_dir.path().join("tmp_storage"));
        let init_handle = Box::new(move |_| (ready((api_node, 0)).boxed()));
        let query_data = QueryData::create(storage_path, ()).unwrap();
        let SequencerNode { handle, .. } =
            sequencer::api::serve(query_data, init_handle, sequencer_port)
                .await
                .unwrap();
        for node in &nodes {
            node.start().await;
        }

        // Start the Rollup API
        let api_port = pick_unused_port().unwrap();
        let sequencer_url = format!("http://localhost:{sequencer_port}")
            .parse()
            .unwrap();
        let genesis_wallet = LocalWallet::new(&mut ChaChaRng::seed_from_u64(0));
        let genesis_address = genesis_wallet.address();
        let state = Arc::new(RwLock::new(State::from_initial_balances([(
            genesis_address,
            GENESIS_BALANCE,
        )])));
        let options = Options {
            api_port,
            sequencer_url,
        };
        spawn(async move { serve(options, state).await });

        // Create a transaction
        let transaction = Transaction {
            amount: 100,
            destination: genesis_address,
            nonce: 1,
        };
        let signed_transaction = SignedTransaction::new(transaction, &genesis_wallet).await;

        // Submit the transaction
        let api_url = format!("http://localhost:{api_port}").parse().unwrap();
        let api_client: Client<ServerError> = Client::new(api_url);
        api_client.connect(None).await;
        api_client
            .post::<()>("rollup/submit")
            .body_json(&signed_transaction)
            .unwrap()
            .send()
            .await
            .unwrap();

        // Wait for a Decide event containing transaction matching the one we sent
        let raw_tx = signed_transaction.encode();
        let txn = SeqTransaction::new(VM_ID.into(), raw_tx.to_vec());
        wait_for_decide_on_handle(
            handle.clone(),
            sequencer::transaction::SequencerTransaction::Wrapped(txn),
        )
        .await
        .unwrap()
    }
}
