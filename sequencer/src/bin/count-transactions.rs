//! Utility program to count transactions sequenced by HotShot.

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::task::sleep;
use clap::Parser;
use committable::Committable;
use es_version::SequencerVersion;
use espresso_types::SeqTypes;
use futures::future::join_all;
use hotshot_query_service::availability::BlockQueryData;
use std::cmp::max;
use std::collections::HashSet;
use std::time::Duration;
use surf_disco::Url;

/// Utility program to count transactions sequenced by HotShot.
#[derive(Clone, Debug, Parser)]
struct Options {
    /// Start counting from block FROM.
    #[clap(long, name = "FROM")]
    from: Option<u64>,

    /// Stop counting at block TO.
    #[clap(long, name = "TO")]
    to: Option<u64>,

    /// Number of parallel tasks to run.
    #[clap(short, long, default_value = "1")]
    jobs: usize,

    /// URL of the HotShot query service.
    url: Url,
}

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();
    let client = surf_disco::Client::<hotshot_query_service::Error, SequencerVersion>::new(opt.url);
    client.connect(None).await;

    let block_height: u64 = client
        .get("status/latest_block_height")
        .send()
        .await
        .unwrap();
    let from = opt.from.unwrap_or(0);
    let to = max(opt.to.unwrap_or(block_height), from);

    let (totals, uniques): (Vec<_>, Vec<_>) = join_all((0..opt.jobs).map(|i| {
        let client = client.clone();
        async move {
            let num_blocks = (to - from) / (opt.jobs as u64);
            let offset = i as u64 * num_blocks;
            let from = from + offset;
            let to = if i + 1 == opt.jobs {
                to
            } else {
                from + num_blocks
            };
            tracing::info!("task {i} counting blocks {from}-{to}");

            let mut total = 0;
            let mut unique = HashSet::new();
            for height in from..to {
                tracing::info!("task {i} processing block {height}/{to}");
                let block: BlockQueryData<SeqTypes> = loop {
                    match client
                        .get(&format!("availability/block/{height}"))
                        .send()
                        .await
                    {
                        Ok(block) => break block,
                        Err(err) => {
                            tracing::error!("task {i} error fetching block {height}: {err}");

                            // Back off a bit and then retry.
                            sleep(Duration::from_millis(100)).await;
                        }
                    }
                };
                for (_, txn) in block.enumerate() {
                    total += 1;
                    unique.insert(txn.commit());
                }
            }
            (total, unique.len())
        }
    }))
    .await
    .into_iter()
    .unzip();

    let total: usize = totals.into_iter().sum();
    let unique: usize = uniques.into_iter().sum();

    println!("For blocks {from}-{to}:");
    println!("Total transactions: {total}");
    println!("Unique: {unique}");
}
