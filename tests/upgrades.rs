use crate::common::TestConfig;
use anyhow::Result;
use espresso_types::{FeeVersion, MarketplaceVersion};
use futures::{StreamExt, future::join_all};
use vbs::version::StaticVersionType;

const SEQUENCER_BLOCKS_TIMEOUT: u64 = 200;

#[tokio::test(flavor = "multi_thread")]
async fn test_upgrade() -> Result<()> {
    dotenvy::dotenv()?;

    let testing = TestConfig::new().await.unwrap();

    let versions = if testing.sequencer_version as u16 >= MarketplaceVersion::version().minor {
        (FeeVersion::version(), MarketplaceVersion::version())
    } else {
        panic!("Invalid sequencer version provided for upgrade test.");
    };

    println!("Waiting on readiness");
    let _ = testing.readiness().await?;

    let initial = testing.test_state().await;
    println!("Initial State:{}", initial);

    let clients = testing.sequencer_clients;

    // Test is limited to those sequencers with correct modules
    // enabled. It would be less fragile if we could discover them.
    let subscriptions = join_all(clients.iter().map(|c| c.subscribe_headers(0)))
        .await
        .into_iter()
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut stream = futures::stream::iter(subscriptions).flatten_unordered(None);

    while let Some(header) = stream.next().await {
        let header = header.unwrap();
        println!(
            "block: height={}, version={}",
            header.height(),
            header.version()
        );

        // TODO is it possible to discover the view at which upgrade should be finished?
        // First few views should be `Base` version.
        if header.height() <= 20 {
            assert_eq!(header.version(), versions.0)
        }

        if header.version() == versions.1 {
            println!("header version matched! height={:?}", header.height());
            break;
        }

        if header.height() > SEQUENCER_BLOCKS_TIMEOUT {
            panic!("Exceeded maximum block height. Upgrade should have finished by now :(");
        }
    }

    // TODO assert transactions are incrementing
    Ok(())
}
