use std::{path::Path, time::Duration};

use crate::common::TestConfig;
use anyhow::Result;
use client::SequencerClient;
use dotenvy::var;
use espresso_types::{EpochVersion, FeeVersion, MarketplaceVersion};
use futures::{future::join_all, StreamExt};
use sequencer_utils::stake_table::{update_stake_table, PermissionedStakeTableUpdate};
use tokio::time::sleep;
use vbs::version::{StaticVersionType, Version};

const SEQUENCER_BLOCKS_TIMEOUT: u64 = 200;

#[tokio::test(flavor = "multi_thread")]
async fn test_upgrade() -> Result<()> {
    dotenvy::dotenv()?;

    let testing = TestConfig::new().await.unwrap();

    let (base, upgrade) = match testing.sequencer_version {
        3 => (FeeVersion::version(), EpochVersion::version()),
        version if version > 3 => (FeeVersion::version(), MarketplaceVersion::version()),
        _ => panic!("Invalid sequencer version provided for upgrade test."),
    };

    println!("Waiting on readiness");
    let _ = testing.readiness().await?;

    let initial = testing.test_state().await;
    println!("Initial State:{}", initial);

    let clients = testing.sequencer_clients;

    let height = test_header_version(clients.clone(), base, upgrade).await?;
    // check that atleast 50 blocks are produced after the upgrade
    test_blocks_production(clients.clone(), height, 50).await?;

    if upgrade == EpochVersion::version() {
        test_stake_table_update(clients).await?;
    }

    // TODO assert transactions are incrementing
    Ok(())
}

async fn test_header_version(
    clients: Vec<SequencerClient>,
    base: Version,
    upgrade: Version,
) -> Result<u64> {
    // Test is limited to those sequencers with correct modules
    // enabled. It would be less fragile if we could discover them.
    let subscriptions = join_all(clients.iter().map(|c| c.subscribe_headers(0)))
        .await
        .into_iter()
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut stream = futures::stream::iter(subscriptions).flatten_unordered(None);
    let mut height = 0;
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
            assert_eq!(header.version(), base)
        }

        if header.version() == upgrade {
            println!("header version matched! height={:?}", header.height());
            height = header.height();
            break;
        }

        if header.height() > SEQUENCER_BLOCKS_TIMEOUT {
            panic!("Exceeded maximum block height. Upgrade should have finished by now :(");
        }
    }

    Ok(height)
}

async fn test_blocks_production(clients: Vec<SequencerClient>, from: u64, num: u64) -> Result<()> {
    let subscriptions = join_all(clients.iter().map(|c| c.subscribe_blocks(from)))
        .await
        .into_iter()
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut num_blocks = 0;

    for mut node in subscriptions {
        while let Some(block) = node.next().await {
            let _block = block.unwrap();
            num_blocks += 1;
            if num_blocks == num {
                break;
            }
        }

        num_blocks = 0;
    }

    Ok(())
}

async fn test_stake_table_update(clients: Vec<SequencerClient>) -> Result<()> {
    /*
            EPOCH V3
    */

    let rpc_url = var("ESPRESSO_SEQUENCER_L1_PROVIDER")?;
    let account_index = var("ESPRESSO_DEPLOYER_ACCOUNT_INDEX")?;
    let contract_address = var("ESPRESSO_SEQUENCER_PERMISSIONED_STAKE_TABLE_ADDRESS")?;
    let initial_stake_table_path = var("ESPRESSO_SEQUENCER_INITIAL_PERMISSIONED_STAKE_TABLE_PATH")?;

    let permissioned_stake_table =
        PermissionedStakeTableUpdate::from_toml_file(Path::new(&initial_stake_table_path))?;

    // initial stake table has 5 new stakers

    let new_stakers = permissioned_stake_table.new_stakers;
    //lets remove one
    let staker_removed = new_stakers[0].clone();

    let st_with_one_removed =
        PermissionedStakeTableUpdate::new(vec![staker_removed.clone()], vec![]);
    let client = clients[0].clone();

    let epoch_before_update = client.current_epoch().await?;

    update_stake_table(
        rpc_url.parse()?,
        Duration::from_secs(7),
        "test test test test test test test test test test test junk".to_string(),
        account_index.parse()?,
        contract_address.parse()?,
        st_with_one_removed,
    )
    .await?;

    loop {
        sleep(Duration::from_secs(10)).await;
        let epoch = clients[0].current_epoch().await?;

        if epoch > epoch_before_update {
            let stake_table = client.stake_table(epoch).await?;
            assert_eq!(stake_table.len(), 4);

            assert!(
                stake_table
                    .iter()
                    .all(|st| st.stake_key != staker_removed.stake_table_key),
                "Entry for {} already exists in the stake table",
                staker_removed.stake_table_key
            );

            break;
        }
    }
    // TODO: randomize this test

    Ok(())
}
