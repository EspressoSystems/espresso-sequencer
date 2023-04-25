use ark_serialize::CanonicalSerialize;
use async_std::task::sleep;
use commit::{Commitment, Committable};
use ethers::{
    abi::Detokenize,
    contract::builders::ContractCall,
    prelude::*,
    providers::Middleware as _,
    providers::{Http, Provider},
    types::U256,
};
use std::process::{Child, Command};
use std::time::Duration;
use url::Url;

pub type Middleware = NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>;

#[derive(Clone, Debug, Default)]
pub struct AnvilOptions {
    block_time: Option<Duration>,
    port: Option<u16>,
}

impl AnvilOptions {
    pub fn block_time(mut self, time: Duration) -> Self {
        self.block_time = Some(time);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub async fn spawn(self) -> Anvil {
        let port = self
            .port
            .unwrap_or_else(|| portpicker::pick_unused_port().unwrap());

        let mut command = "anvil --silent --host 0.0.0.0".to_string();
        if let Some(block_time) = self.block_time {
            command = format!("{command} -b {}", block_time.as_secs());
        }

        tracing::info!("Starting Anvil: {command}");
        let child = Command::new("docker")
            .arg("run")
            .arg("-p")
            .arg(format!("{port}:8545"))
            .arg("ghcr.io/foundry-rs/foundry:latest")
            // Ideally the stdout would be captured, in tests but I could not
            // get this to work. Pass `--silent` to avoid spamming the test
            // output.
            .arg(&command)
            .spawn()
            .unwrap();

        let url = Url::parse(&format!("http://localhost:{port}")).unwrap();
        wait_for_rpc(&url, Duration::from_millis(200), 20)
            .await
            .unwrap();

        Anvil { child, url }
    }
}

#[derive(Debug)]
pub struct Anvil {
    child: Child,
    url: Url,
}

impl Anvil {
    pub fn url(&self) -> Url {
        self.url.clone()
    }
}

impl Drop for Anvil {
    fn drop(&mut self) {
        self.child.kill().unwrap();
    }
}

pub async fn wait_for_http(
    url: &Url,
    interval: Duration,
    max_retries: usize,
) -> Result<usize, String> {
    for i in 0..(max_retries + 1) {
        let res = surf::get(url).await;
        if res.is_ok() {
            tracing::debug!("Connected to {url}");
            return Ok(i);
        }
        tracing::debug!("Waiting for {url}, retrying in {interval:?}");
        sleep(interval).await;
    }
    Err(format!("Url {url:?} not available."))
}

pub async fn wait_for_rpc(
    url: &Url,
    interval: Duration,
    max_retries: usize,
) -> Result<usize, String> {
    let retries = wait_for_http(url, interval, max_retries).await?;
    let client = Provider::new(Http::new(url.clone()));
    for i in retries..(max_retries + 1) {
        if client.get_block_number().await.is_ok() {
            tracing::debug!("JSON-RPC ready at {url}");
            return Ok(i);
        }
        tracing::debug!("Waiting for JSON-RPC at {url}, retrying in {interval:?}");
        sleep(interval).await;
    }
    Err(format!("No JSON-RPC at {url}"))
}

pub fn commitment_to_u256<T: Committable>(comm: Commitment<T>) -> U256 {
    let mut buf = vec![];
    comm.serialize(&mut buf).unwrap();
    let state_comm: [u8; 32] = buf.try_into().unwrap();
    U256::from_little_endian(&state_comm)
}

pub async fn contract_send<T: Detokenize>(
    call: ContractCall<Middleware, T>,
) -> Option<(TransactionReceipt, u64)> {
    let pending = match call.send().await {
        Ok(pending) => pending,
        Err(err) => {
            tracing::error!("error sending transaction: {}", err);
            return None;
        }
    };
    let receipt = match pending.await {
        Ok(Some(receipt)) => receipt,
        Ok(None) => {
            tracing::error!("transaction not mined");
            return None;
        }
        Err(err) => {
            tracing::error!("error waiting for transaction to be mined: {}", err);
            return None;
        }
    };
    if receipt.status != Some(1.into()) {
        tracing::error!("transaction reverted");
        return None;
    }

    // If a transaction is mined and we get a receipt for it, the block number should _always_ be
    // set. If it is not, something has gone horribly wrong with the RPC.
    let block_number = receipt
        .block_number
        .expect("transaction mined but block number not set");
    Some((receipt, block_number.as_u64()))
}
