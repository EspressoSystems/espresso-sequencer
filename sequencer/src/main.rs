#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    console_subscriber::init();
    sequencer::main().await
}
