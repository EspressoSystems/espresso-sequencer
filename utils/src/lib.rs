use async_std::task::sleep;
use ethers::providers::{Http, Middleware, Provider};
use std::process::{Child, Command};
use std::time::Duration;
use url::Url;

#[derive(Debug)]
pub struct Anvil {
    child: Child,
    url: Url,
}

impl Anvil {
    pub async fn spawn(port: Option<u16>) -> Self {
        let port = port.unwrap_or_else(|| portpicker::pick_unused_port().unwrap());

        let child = Command::new("docker")
            .arg("run")
            .arg("-p")
            .arg(format!("{port}:8545"))
            .arg("ghcr.io/foundry-rs/foundry:latest")
            // Ideally the stdout would be captured, in tests but I could not
            // get this to work. Pass `--silent` to avoid spamming the test
            // output.
            .arg("anvil --silent --host 0.0.0.0")
            .spawn()
            .unwrap();

        let url = Url::parse(&format!("http://localhost:{port}")).unwrap();
        wait_for_rpc(&url, Duration::from_millis(200), 20)
            .await
            .unwrap();

        Self { child, url }
    }

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
