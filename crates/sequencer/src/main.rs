#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    sequencer::main().await
}
