use std::time::Duration;

use clap::{Args, Parser, Subcommand};
use ethers::{
    prelude::k256::ecdsa::SigningKey,
    signers::{LocalWallet, Signer, Wallet},
    types::Address,
};
use example_l2::{
    seed::SeedIdentity,
    state::{Amount, Nonce},
    transaction::{SignedTransaction, Transaction},
};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use surf_disco::Client;
use tide_disco::{error::ServerError, Url};

type RollupClient = Client<ServerError>;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// Url of the Rollup client
    #[clap(short, long, default_value = "http://localhost:8082")]
    pub rollup_url: Url,

    #[command(subcommand)]
    pub command: ExampleRollupCommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ExampleRollupCommand {
    Transfer(Transfer),
    CheckBalance(CheckBalance),
}

#[derive(Args, Clone, Debug)]
pub struct Transfer {
    pub sender: SeedIdentity,
    pub receiver: SeedIdentity,
    pub amount: Amount,
}

#[derive(Args, Clone, Debug)]
pub struct CheckBalance {
    pub identity: SeedIdentity,
}

fn get_wallet_from_identity(identity: &SeedIdentity) -> Wallet<SigningKey> {
    LocalWallet::new(&mut ChaChaRng::seed_from_u64(*identity as u64))
}

async fn transfer(transfer: &Transfer, client: &RollupClient) {
    let sender = get_wallet_from_identity(&transfer.sender);
    let receiver = get_wallet_from_identity(&transfer.receiver);
    let amount = transfer.amount;
    let nonce = get_nonce(&sender.address(), client).await + 1;
    let transaction = Transaction {
        amount,
        destination: receiver.address(),
        nonce,
    };
    let signed_transaction = SignedTransaction::new(transaction, &sender).await;

    println!(
        "Submitting Transaction to Rollup API: Transferring {} tokens from {} to {}",
        amount,
        sender.address(),
        receiver.address(),
    );

    client
        .post::<()>("rollup/submit")
        .body_json(&signed_transaction)
        .expect("Error setting the response body")
        .send()
        .await
        .expect("Error sending the transfer transaction")
}

async fn get_nonce(address: &Address, client: &RollupClient) -> Nonce {
    client
        .get::<u64>(&format!("rollup/nonce/{:?}", address))
        .send()
        .await
        .expect("Error sending the get nonce request")
}

async fn check_balance(check_balance: &CheckBalance, client: &RollupClient) {
    let address = get_wallet_from_identity(&check_balance.identity).address();
    let balance = client
        .get::<u64>(&format!("rollup/balance/{:?}", address))
        .send()
        .await
        .expect("Error sending the check balance request");

    println!("Balance of {:?}: {}", address, balance)
}

#[async_std::main]
async fn main() {
    let Options {
        rollup_url,
        command,
    } = Options::parse();
    let client: RollupClient = Client::new(rollup_url);
    let connected = client.connect(Some(Duration::from_secs(2))).await;
    if !connected {
        println!("Could not connect to the Rollup Client. Ensure that the client is running and that the supplied port is correct.");
        return;
    }

    match command {
        ExampleRollupCommand::Transfer(transfer_cmd) => transfer(&transfer_cmd, &client).await,
        ExampleRollupCommand::CheckBalance(check_balance_cmd) => {
            check_balance(&check_balance_cmd, &client).await;
        }
    };
}
