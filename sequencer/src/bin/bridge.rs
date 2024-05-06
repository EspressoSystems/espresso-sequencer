use anyhow::{bail, ensure, Context};
use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::{sync::Arc, task::sleep};
use clap::Parser;
use contract_bindings::fee_contract::FeeContract;
use es_version::SequencerVersion;
use ethers::{
    middleware::{Middleware, SignerMiddleware},
    providers::Provider,
    types::{Address, BlockId, U256},
};
use futures::stream::StreamExt;
use jf_merkle_tree::{
    prelude::{MerkleProof, Sha3Node},
    MerkleTreeScheme,
};
use sequencer::{
    eth_signature_key::EthKeyPair,
    state::{FeeAccount, FeeAmount, FeeMerkleTree},
    Header,
};
use std::time::Duration;
use surf_disco::{error::ClientError, Url};

type EspressoClient = surf_disco::Client<ClientError, SequencerVersion>;

type FeeMerkleProof = MerkleProof<FeeAmount, FeeAccount, Sha3Node, { FeeMerkleTree::ARITY }>;

/// Command-line utility for working with the Espresso bridge.
#[derive(Debug, Parser)]
enum Command {
    Deposit(Deposit),
    Balance(Balance),
    L1Balance(L1Balance),
}

/// Deposit ETH from the L1 into Espresso.
#[derive(Debug, Parser)]
struct Deposit {
    /// L1 JSON-RPC provider.
    #[clap(short, long, env = "L1_PROVIDER")]
    rpc_url: Url,

    /// Espresso query service provider.
    ///
    /// This must point to an Espresso node running the node, availability, and Merklized state
    /// APIs.
    #[clap(short, long, env = "ESPRESSO_PROVIDER")]
    espresso_provider: Url,

    /// The address of the Espresso fee contract on the L1.
    #[clap(short, long, env = "CONTRACT_ADDRESS")]
    contract_address: Address,

    /// Mnemonic to generate the account from which to deposit.
    #[clap(short, long, env = "MNEMONIC")]
    mnemonic: String,

    /// Account index when deriving an account from MNEMONIC.
    #[clap(short = 'i', long, env = "ACCOUNT_INDEX", default_value = "0")]
    account_index: u32,

    /// Amount of WEI to deposit.
    // Note: we use u64 because U256 parses in hex, which is annoying. We can easily convert to U256
    // after parsing.
    #[clap(short, long, env = "AMOUNT")]
    amount: u64,

    /// Number of confirmations to wait for before considering an L1 transaction mined.
    #[clap(long, env = "CONFIRMATIONS", default_value = "6")]
    confirmations: usize,
}

/// Check the balance (in ETH) of an Espresso account.
#[derive(Debug, Parser)]
struct Balance {
    /// Espresso query service provider.
    ///
    /// This must point to an Espresso node running the node and Merklized state APIs.
    #[clap(short, long, env = "ESPRESSO_PROVIDER")]
    espresso_provider: Url,

    /// Account to check.
    #[clap(short, long, env = "ADDRESS", required_unless_present = "mnemonic")]
    address: Option<Address>,

    /// Mnemonic to generate the account to check.
    #[clap(short, long, env = "MNEMONIC", conflicts_with = "address")]
    mnemonic: Option<String>,

    /// Account index when deriving an account from MNEMONIC.
    #[clap(
        short = 'i',
        long,
        env = "ACCOUNT_INDEX",
        default_value = "0",
        conflicts_with = "address"
    )]
    account_index: u32,

    /// Espresso block number at which to check (default: latest).
    #[clap(short, long, env = "BLOCK")]
    block: Option<u64>,
}

/// Check the balance (in ETH) of an L1 account.
#[derive(Debug, Parser)]
struct L1Balance {
    /// L1 JSON-RPC provider.
    #[clap(short, long, env = "L1_PROVIDER")]
    rpc_url: Url,

    /// Account to check.
    #[clap(short, long, env = "ADDRESS", required_unless_present = "mnemonic")]
    address: Option<Address>,

    /// Mnemonic to generate the account to check.
    #[clap(short, long, env = "MNEMONIC", conflicts_with = "address")]
    mnemonic: Option<String>,

    /// Account index when deriving an account from MNEMONIC.
    #[clap(
        short = 'i',
        long,
        env = "ACCOUNT_INDEX",
        default_value = "0",
        conflicts_with = "address"
    )]
    account_index: u32,

    /// L1 block number at which to check (default: latest).
    #[clap(short, long, env = "BLOCK")]
    block: Option<u64>,
}

async fn deposit(opt: Deposit) -> anyhow::Result<()> {
    // Derive the account to deposit from.
    let key_pair = EthKeyPair::from_mnemonic(opt.mnemonic, opt.account_index)?;

    // Connect to L1.
    let rpc = Provider::try_from(opt.rpc_url.to_string())?;
    let signer = key_pair.signer();
    let l1 = Arc::new(SignerMiddleware::new_with_provider_chain(rpc, signer).await?);
    let contract = FeeContract::new(opt.contract_address, l1.clone());

    // Connect to Espresso.
    let espresso = EspressoClient::new(opt.espresso_provider);

    // Validate deposit.
    let amount = U256::from(opt.amount);
    let min_deposit = contract.min_deposit_amount().call().await?;
    let max_deposit = contract.max_deposit_amount().call().await?;
    ensure!(
        amount >= min_deposit,
        "amount is too small (minimum deposit: {min_deposit})",
    );
    ensure!(
        amount <= max_deposit,
        "amount is too large (maximum deposit: {max_deposit})",
    );

    // Record the initial balance on Espresso.
    let initial_balance = get_espresso_balance(&espresso, l1.address(), None).await?;
    tracing::debug!(%initial_balance, "initial balance");

    // Send the deposit transaction.
    tracing::info!(address = %l1.address(), %amount, "sending deposit transaction");
    let call = contract.deposit(l1.address()).value(amount);
    let tx = call.send().await.context("sending deposit transaction")?;
    tracing::info!(hash = %tx.tx_hash(), "deposit transaction sent to L1");

    // Wait for the transaction to finalize on L1.
    let receipt = tx
        .confirmations(opt.confirmations)
        .await
        .context("waiting for deposit transaction")?
        .context("deposit transaction not mined")?;
    let l1_block = receipt
        .block_number
        .context("deposit transaction not mined")?
        .as_u64();
    ensure!(
        receipt.status == Some(1.into()),
        "deposit transaction reverted"
    );
    tracing::info!(l1_block, "deposit mined on L1");

    // Wait for Espresso to catch up to the L1.
    let espresso_height = espresso
        .get::<u64>("node/block-height")
        .send()
        .await
        .context("getting Espresso block height")?;
    let mut headers = espresso
        .socket(&format!("availability/stream/headers/{espresso_height}"))
        .subscribe()
        .await
        .context("subscribing to Espresso headers")?;
    let espresso_block = loop {
        let header: Header = match headers.next().await.context("header stream ended")? {
            Ok(header) => header,
            Err(err) => {
                tracing::warn!("error in header stream: {err:#}");
                continue;
            }
        };
        let Some(l1_finalized) = header.l1_finalized else {
            continue;
        };
        if l1_finalized.number >= l1_block {
            tracing::info!(block = header.height, "deposit finalized on Espresso");
            break header.height;
        } else {
            tracing::debug!(
                block = header.height,
                l1_block,
                ?l1_finalized,
                "waiting for deposit on Espresso"
            )
        }
    };

    // Confirm that the Espresso balance has increased.
    let final_balance = get_espresso_balance(&espresso, l1.address(), Some(espresso_block)).await?;
    if final_balance >= initial_balance + amount.into() {
        tracing::info!(%final_balance, "deposit successful");
    } else {
        // The balance didn't increase as much as expected. This doesn't necessarily mean the
        // deposit failed: there could have been a race condition where the balance on Espresso was
        // altered by some other operation at the same time, but we should at least let the user
        // know about it.
        tracing::warn!(%initial_balance, %final_balance, "Espresso balance did not increase as expected");
    }

    Ok(())
}

async fn balance(opt: Balance) -> anyhow::Result<()> {
    // Derive the address to look up.
    let address = if let Some(address) = opt.address {
        address
    } else if let Some(mnemonic) = opt.mnemonic {
        EthKeyPair::from_mnemonic(mnemonic, opt.account_index)?.address()
    } else {
        bail!("address or mnemonic must be provided");
    };

    let espresso = EspressoClient::new(opt.espresso_provider);
    let balance = get_espresso_balance(&espresso, address, opt.block).await?;

    // Output the balance on regular standard out, rather than as a log message, to make scripting
    // easier.
    println!("{balance}");

    Ok(())
}

async fn l1_balance(opt: L1Balance) -> anyhow::Result<()> {
    // Derive the address to look up.
    let address = if let Some(address) = opt.address {
        address
    } else if let Some(mnemonic) = opt.mnemonic {
        EthKeyPair::from_mnemonic(mnemonic, opt.account_index)?.address()
    } else {
        bail!("address or mnemonic must be provided");
    };

    let l1 = Provider::try_from(opt.rpc_url.to_string())?;

    let block = opt.block.map(BlockId::from);
    tracing::debug!(%address, ?block, "fetching L1 balance");
    let balance = l1
        .get_balance(address, block)
        .await
        .context("getting account balance")?;

    // Output the balance on regular standard out, rather than as a log message, to make scripting
    // easier.
    println!("{balance}");

    Ok(())
}

async fn get_espresso_balance(
    espresso: &EspressoClient,
    address: Address,
    block: Option<u64>,
) -> anyhow::Result<FeeAmount> {
    // Get the block height to query at, defaulting to the latest block.
    let block = if let Some(block) = block {
        block
    } else {
        espresso
            .get::<u64>("node/block-height")
            .send()
            .await
            .context("getting block height")?
            - 1
    };

    // Download the Merkle path for this fee account at the specified block height. Transient errors
    // are possible (for example, if we are fetching from the latest block, the block height might
    // get incremented slightly before the state becomes available) so retry a few times.
    let mut retry = 0;
    let max_retries = 5;
    let proof = loop {
        tracing::debug!(%address, block, retry, "fetching Espresso balance");
        match espresso
            .get::<FeeMerkleProof>(&format!("fee-state/{block}/{address:#x}"))
            .send()
            .await
        {
            Ok(proof) => break proof,
            Err(err) => {
                tracing::warn!("error getting account balance: {err:#}");
                retry += 1;

                if retry == max_retries {
                    return Err(err).context("getting account balance");
                } else {
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    };

    // If the element in the Merkle path is missing -- there is no account with this address -- the
    // balance is defined to be 0.
    let balance = proof.elem().copied().unwrap_or(0.into());
    Ok(balance)
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();
    setup_backtrace();

    match Command::parse() {
        Command::Deposit(opt) => deposit(opt).await,
        Command::Balance(opt) => balance(opt).await,
        Command::L1Balance(opt) => l1_balance(opt).await,
    }
}
