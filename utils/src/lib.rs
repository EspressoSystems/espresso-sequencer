use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use async_std::{sync::Arc, task::sleep};
use commit::{Commitment, Committable};
use ethers::{
    abi::Detokenize,
    contract::builders::ContractCall,
    prelude::*,
    providers::Middleware as _,
    providers::{Http, Provider},
    signers::coins_bip39::English,
    types::U256,
};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::Duration;
use tempfile::TempDir;
use url::Url;

pub type Middleware = NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>;

#[derive(Clone, Debug, Default)]
pub struct AnvilOptions {
    block_time: Option<Duration>,
    port: Option<u16>,
    load_state: Option<PathBuf>,
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

    pub fn load_state(mut self, path: PathBuf) -> Self {
        self.load_state = Some(path);
        self
    }

    pub async fn spawn(self) -> Anvil {
        let state_dir = TempDir::new().unwrap();
        let (child, url) = Anvil::spawn_server(self, state_dir.path()).await;
        Anvil {
            child,
            url,
            state_dir,
        }
    }
}

#[derive(Debug)]
pub struct Anvil {
    child: Child,
    url: Url,
    state_dir: TempDir,
}

impl Anvil {
    async fn spawn_server(opt: AnvilOptions, state_dir: &Path) -> (Child, Url) {
        let port = opt
            .port
            .unwrap_or_else(|| portpicker::pick_unused_port().unwrap());

        let mut command = Command::new("anvil");
        command.args([
            "--silent",
            "--port",
            &port.to_string(),
            "--dump-state",
            &state_dir.display().to_string(),
        ]);

        if let Some(block_time) = opt.block_time {
            command.args(["-b", &block_time.as_secs().to_string()]);
        }
        if let Some(load_state) = opt.load_state {
            command.args(["--load-state", &load_state.display().to_string()]);
        }

        tracing::info!("Starting Anvil: {:?}", &command);

        let child = command.spawn().unwrap();

        let url = Url::parse(&format!("http://localhost:{port}")).unwrap();
        wait_for_rpc(&url, Duration::from_millis(200), 20)
            .await
            .unwrap();

        (child, url)
    }

    pub fn url(&self) -> Url {
        self.url.clone()
    }

    pub fn provider(&self) -> Provider<Http> {
        Provider::try_from(self.url().to_string()).unwrap()
    }

    fn shutdown_gracefully(&self) {
        Command::new("kill")
            .args(["-s", "INT", &self.child.id().to_string()])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    /// Restart the server, possibly with different options.
    pub async fn restart(&mut self, mut opt: AnvilOptions) {
        // Stop the server and wait for it to dump its state.
        self.shutdown_gracefully();

        // If `opt` does not explicitly override the URL, use the current one.
        if opt.port.is_none() {
            opt.port = self.url.port();
        }

        // Load state from the file where we just dumped state.
        opt = opt.load_state(self.state_dir.path().join("state.json"));

        // Restart the server with the new options, loading state from disk.
        let (child, url) = Self::spawn_server(opt, self.state_dir.path()).await;
        self.child = child;
        self.url = url;
    }
}

impl Drop for Anvil {
    fn drop(&mut self) {
        self.shutdown_gracefully()
    }
}

pub async fn connect_rpc(
    provider: &Url,
    mnemonic: &str,
    index: u32,
    chain_id: Option<u64>,
) -> Option<Arc<Middleware>> {
    let provider = match Provider::try_from(provider.to_string()) {
        Ok(provider) => provider,
        Err(err) => {
            tracing::error!("error connecting to RPC {}: {}", provider, err);
            return None;
        }
    };
    let chain_id = match chain_id {
        Some(id) => id,
        None => match provider.get_chainid().await {
            Ok(id) => id.as_u64(),
            Err(err) => {
                tracing::error!("error getting chain ID: {}", err);
                return None;
            }
        },
    };
    let mnemonic = match MnemonicBuilder::<English>::default()
        .phrase(mnemonic)
        .index(index)
    {
        Ok(mnemonic) => mnemonic,
        Err(err) => {
            tracing::error!("error building walletE: {}", err);
            return None;
        }
    };
    let wallet = match mnemonic.build() {
        Ok(wallet) => wallet,
        Err(err) => {
            tracing::error!("error opening wallet: {}", err);
            return None;
        }
    };
    let wallet = wallet.with_chain_id(chain_id);
    let address = wallet.address();
    Some(Arc::new(NonceManagerMiddleware::new(
        SignerMiddleware::new(provider, wallet),
        address,
    )))
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

pub fn u256_to_commitment<T: Committable>(comm: U256) -> Result<Commitment<T>, SerializationError> {
    let mut commit_bytes = [0; 32];
    comm.to_little_endian(&mut commit_bytes);
    Commitment::deserialize(&*commit_bytes.to_vec())
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

#[cfg(test)]
mod test {
    use super::*;
    use commit::RawCommitmentBuilder;

    struct TestCommittable;

    impl Committable for TestCommittable {
        fn commit(&self) -> Commitment<Self> {
            RawCommitmentBuilder::new("TestCommittable").finalize()
        }
    }

    #[test]
    fn test_commitment_to_u256_round_trip() {
        assert_eq!(
            TestCommittable.commit(),
            u256_to_commitment(commitment_to_u256(TestCommittable.commit())).unwrap()
        );
    }
}
