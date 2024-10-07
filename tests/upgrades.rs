use crate::common::TestConfig;
use anyhow::Result;
use espresso_types::{FeeVersion, MarketplaceVersion, V0_1};
use futures::{stream, StreamExt};
use vbs::version::StaticVersionType;

const SEQUENCER_BLOCKS_TIMEOUT: u64 = 120;

#[async_std::test]
async fn test_upgrade() -> Result<()> {
    dotenvy::dotenv()?;

    let testing = TestConfig::new().await.unwrap();

    let versions = if testing.sequencer_version >= 3 {
        (FeeVersion::version(), MarketplaceVersion::version())
    } else {
        (V0_1::version(), FeeVersion::version())
    };

    println!("Waiting on readiness");
    let _ = testing.readiness().await?;

    let initial = testing.test_state().await;
    println!("Initial State:{}", initial);

    let clients = testing.sequencer_clients;

    // Test is limited to those sequencers with correct modules
    // enabled. It would be less fragile if we could discover them.
    let subscriptions = vec![
        clients[0].subscribe_headers(0).await?,
        clients[1].subscribe_headers(0).await?,
    ];
    let subscriptions_size = subscriptions.len();
    let mut streams = stream::iter(subscriptions)
        .flatten_unordered(usize::try_from(SEQUENCER_BLOCKS_TIMEOUT).ok());

    let mut upgraded_nodes: usize = 0;
    while let Some(header) = streams.next().await {
        let header = header.unwrap();

        // TODO is it possible to discover the view at which upgrade should be finished?
        // First few views should be `FeeVersion`.
        if header.height() <= 5 {
            assert_eq!(header.version(), versions.0)
        }

        // Track how many nodes have been upgraded
        if header.version() == versions.1 {
            upgraded_nodes += 1;
        }

        if upgraded_nodes == subscriptions_size {
            println!("Upgrade succeeded @ height {}!", header.height());
            break;
        }

        if header.height() > SEQUENCER_BLOCKS_TIMEOUT {
            panic!("Exceeded maximum block height. Upgrade should have finished by now :(");
        }
    }
    Ok(())
}
