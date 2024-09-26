use anyhow::{anyhow, Result};
use async_std::{future, task::sleep};
use client::SequencerClient;
use espresso_types::FeeAmount;
use ethers::prelude::*;
use futures::future::join_all;
use reqwest::blocking;
use std::{
    fmt,
    str::FromStr,
    time::{Duration, Instant},
};
use surf_disco::Url;

const L1_PROVIDER_RETRY_INTERVAL: Duration = Duration::from_secs(1);
// TODO add to .env or derive
const BUILDER_ADDRESS: &str = "0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f";
// TODO add to .env
const RECIPIENT_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
/// Duration in seconds to wait before declaring the chain deceased.
const SEQUENCER_BLOCKS_TIMEOUT: u64 = 120;

async fn wait_for_service(url: &str, interval: u64, timeout: u64) -> Result<String> {
    future::timeout(Duration::from_secs(timeout), async {
        loop {
            if let Ok(body) = blocking::get(format!("{url}/healthcheck")) {
                return body.text().map_err(|e| {
                    anyhow!(
                        "Wait for service, could not decode response: ({}) {}",
                        url,
                        e
                    )
                });
            } else {
                sleep(Duration::from_millis(interval)).await;
            }
        }
    })
    .await
    .map_err(|e| anyhow!("Wait for service, timeout: ({}) {}", url, e))?
}

#[derive(Clone, Debug)]
struct TestConfig {
    load_generator_url: String,
    provider: Provider<Http>,
    espresso: SequencerClient,
    builder_address: Address,
    recipient_address: Address,
    light_client_address: Address,
    prover_url: String,
    builder_url: String,
    expected_block_height: u64,
    timeout: f64,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct TestState {
    block_height: Option<u64>,
    txn_count: u64,
    builder_balance: FeeAmount,
    recipient_balance: FeeAmount,
    light_client_update: u64,
}

impl fmt::Display for TestState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = format!(
            "
        block_height: {}
        transactions: {}
        builder_balance: {}
        recipient_balance: {}
        light_client_updated: {}
",
            self.block_height.unwrap(),
            self.txn_count,
            self.builder_balance,
            self.recipient_balance,
            self.light_client_update
        );

        write!(f, "{}", output)
    }
}

fn url_from_port(port: String) -> Result<String> {
    Ok(format!(
        "{}://{}:{}",
        dotenvy::var("INTEGRATION_TEST_PROTO")?,
        dotenvy::var("INTEGRATION_TEST_HOST")?,
        port
    ))
}

impl TestConfig {
    async fn new() -> Result<Self> {
        let mut load_generator_url =
            url_from_port(dotenvy::var("ESPRESSO_SUBMIT_TRANSACTIONS_PRIVATE_PORT")?)?;

        if dotenvy::var("MARKETPLACE_SMOKE_TEST")
            .map_err(|e| anyhow!(e))
            .and_then(|var| var.parse::<bool>().map_err(|e| anyhow!(e)))
            .ok()
            .is_some_and(|val| val)
        {
            load_generator_url = url_from_port(dotenvy::var(
                "ESPRESSO_SUBMIT_TRANSACTIONS_PRIVATE_FALLBACK_PORT",
            )?)?;
        }

        let l1_provider_url = url_from_port(dotenvy::var("ESPRESSO_SEQUENCER_L1_PORT")?)?;
        let sequencer_api_url = url_from_port(dotenvy::var("ESPRESSO_SEQUENCER1_API_PORT")?)?;
        let builder_url = url_from_port(dotenvy::var("ESPRESSO_BUILDER_SERVER_PORT")?)?;
        let prover_url = url_from_port(dotenvy::var("ESPRESSO_PROVER_SERVICE_PORT")?)?;

        let light_client_proxy_address =
            dotenvy::var("ESPRESSO_SEQUENCER_LIGHT_CLIENT_PROXY_ADDRESS")?;

        // Number of blocks to wait before deeming the test successful
        let expected_block_height = dotenvy::var("INTEGRATION_TEST_EXPECTED_BLOCK_HEIGHT")?
            .parse::<u64>()
            .unwrap();

        // Set the time out. Give a little more leeway when we have a
        // large `expected_block_height`.
        let timeout = expected_block_height as f64
            + SEQUENCER_BLOCKS_TIMEOUT as f64
            + (expected_block_height as f64).sqrt();

        Ok(Self {
            load_generator_url,
            provider: Provider::<Http>::try_from(l1_provider_url)?
                .interval(L1_PROVIDER_RETRY_INTERVAL),
            espresso: SequencerClient::new(Url::from_str(&sequencer_api_url).unwrap()),
            light_client_address: light_client_proxy_address.parse::<Address>().unwrap(),
            builder_address: BUILDER_ADDRESS.parse::<Address>().unwrap(),
            recipient_address: RECIPIENT_ADDRESS.parse::<Address>().unwrap(),
            builder_url,
            prover_url,
            expected_block_height,
            timeout,
        })
    }

    /// Number of blocks to wait before deeming the test successful
    fn expected_block_height(&self) -> u64 {
        self.expected_block_height
    }
    /// Get the latest block where we see a light client update
    async fn latest_light_client_update(&self) -> u64 {
        let filter = Filter::new()
            .from_block(BlockNumber::Earliest)
            .address(self.light_client_address);
        // block number for latest light client update
        let latest_light_client_update = self
            .provider
            .get_logs(&filter)
            .await
            .unwrap()
            .last()
            .unwrap()
            .block_number;

        latest_light_client_update.unwrap().as_u64()
    }

    /// Return current state  of the test
    async fn test_state(&self) -> TestState {
        let block_height = self.espresso.get_height().await.ok();
        let txn_count = self.espresso.get_transaction_count().await.unwrap();

        let builder_balance = self
            .espresso
            .get_espresso_balance(self.builder_address, block_height)
            .await
            .unwrap();
        let recipient_balance = self
            .espresso
            .get_espresso_balance(self.recipient_address, block_height)
            .await
            .unwrap();

        let light_client_update = self.latest_light_client_update().await;

        TestState {
            block_height,
            txn_count,
            builder_balance,
            recipient_balance,
            light_client_update,
        }
    }

    /// Check if services are healthy
    async fn readiness(&self) -> Result<Vec<String>> {
        join_all(vec![
            wait_for_service(&self.load_generator_url, 1000, 600),
            wait_for_service(&self.builder_url, 1000, 200),
            wait_for_service(&self.prover_url, 1000, 300),
        ])
        .await
        .into_iter()
        .collect::<Result<Vec<String>>>()
    }
}

#[async_std::test]
async fn test_smoke() -> Result<()> {
    let start = Instant::now();
    dotenvy::dotenv()?;

    let testing = TestConfig::new().await.unwrap();
    let _ = testing.readiness().await?;

    let initial = testing.test_state().await;
    println!("Initial State:{}", initial);

    loop {
        sleep(Duration::from_secs(1)).await;

        let new = testing.test_state().await;

        println!("New State:{}", new);
        if new <= initial {
            panic!("Chain state not incrementing");
        }

        if initial.builder_balance + initial.recipient_balance
            != new.builder_balance + new.recipient_balance
        {
            panic!("Balance not conserved");
        }

        // Timeout if tests take too long.
        if start.elapsed().as_secs() as f64 > testing.timeout {
            panic!("Timeout waiting for block height, transaction count, and light client updates to increase.");
        }

        // test that we progress EXPECTED_BLOCK_HEIGHT blocks from where we started
        if new.block_height.unwrap()
            >= testing.expected_block_height() + initial.block_height.unwrap()
        {
            println!("Reached {} block(s)!", testing.expected_block_height());
            break;
        }
    }
    Ok(())
}
