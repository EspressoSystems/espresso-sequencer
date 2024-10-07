use crate::common::TestConfig;
use anyhow::Result;
use async_std::task::sleep;
use std::time::{Duration, Instant};

#[async_std::test]
async fn test_smoke() -> Result<()> {
    let start = Instant::now();
    dotenvy::dotenv()?;

    let testing = TestConfig::new().await.unwrap();

    println!("Waiting on readiness");
    let _ = testing.readiness().await?;

    let mut initial = testing.test_state().await;
    let initial_block_height = initial.block_height.unwrap();
    println!("Initial State:{}", initial);

    let mut i = 1;
    loop {
        sleep(Duration::from_secs(1)).await;

        let new = testing.test_state().await;
        println!("New State:{}", new);

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
        if new.block_height.unwrap() >= testing.expected_block_height() + testing.initial_height {
            println!("Reached {} block(s)!", testing.expected_block_height());
            break;
        }

        if i % 5 == 0 {
            if new <= initial {
                panic!("Chain state not incrementing");
            }

            if new.txn_count <= initial.txn_count {
                panic!("Transactions not incrementing");
            }
            initial = new;
        }
        i += 1;
    }
    Ok(())
}
