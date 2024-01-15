//! Utility program to submit random transactions to an Espresso Sequencer.

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::task::{sleep, spawn};
use bytesize::ByteSize;
use clap::Parser;
use commit::{Commitment, Committable};
use derive_more::From;
use futures::{
    channel::mpsc::{self, Sender},
    sink::SinkExt,
    stream::StreamExt,
};
use hotshot_query_service::{availability::BlockQueryData, Error};
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use sequencer::{options::parse_duration, SeqTypes, Transaction};
use snafu::Snafu;
use std::{collections::HashSet, time::Duration};
use surf_disco::{Client, Url};

/// Submit random transactions to an Espresso Sequencer.
#[derive(Clone, Debug, Parser)]
struct Options {
    /// Minimum size of transaction to submit.
    #[clap(long, default_value = "1", value_parser = parse_size)]
    min_size: usize,

    /// Maximum size of transaction to submit.
    #[clap(long, default_value = "1kb", value_parser = parse_size)]
    max_size: usize,

    /// Minimum namespace ID to submit to.
    #[clap(long, default_value = "10000")]
    min_namespace: u64,

    /// Maximum namespace ID to submit to.
    #[clap(long, default_value = "10010")]
    max_namespace: u64,

    /// Optional delay between submitting transactions.
    ///
    /// Can be used to moderate the rate of submission.
    #[clap(long, value_parser = parse_duration)]
    delay: Option<Duration>,

    /// Maximum number of unprocessed transaction submissions.
    ///
    /// This can be used to apply backpressure so that the tasks submitting transactions do not get
    /// too far ahead of the task processing results.
    #[clap(long, default_value = "1000")]
    channel_bound: usize,

    /// Seed for reproducible randomness.
    #[clap(long)]
    seed: Option<u64>,

    /// Number of parallel tasks to run.
    #[clap(short, long, default_value = "1")]
    jobs: usize,

    /// URL of the query service.
    url: Url,
}

#[derive(Clone, Debug, From, Snafu)]
struct ParseSizeError {
    msg: String,
}

fn parse_size(s: &str) -> Result<usize, ParseSizeError> {
    Ok(s.parse::<ByteSize>()?.0 as usize)
}

#[async_std::main]
async fn main() {
    setup_backtrace();
    setup_logging();

    let opt = Options::parse();
    let (sender, mut receiver) = mpsc::channel(opt.channel_bound);

    let seed = opt.seed.unwrap_or_else(random_seed);
    tracing::info!("PRNG seed: {seed}");
    let mut rng = ChaChaRng::seed_from_u64(seed);

    // Subscribe to block stream so we can check that our transactions are getting sequenced.
    let client = Client::<Error>::new(opt.url.clone());
    let block_height: usize = client
        .get("status/latest_block_height")
        .send()
        .await
        .unwrap();
    let mut blocks = client
        .socket(&format!("availability/stream/blocks/{}", block_height - 1))
        .subscribe()
        .await
        .unwrap();
    tracing::info!("listening for blocks starting at {block_height}");

    // Spawn tasks to submit transactions.
    for _ in 0..opt.jobs {
        spawn(submit_transactions(
            opt.clone(),
            sender.clone(),
            ChaChaRng::from_rng(&mut rng).unwrap(),
        ));
    }

    // Keep track of the results.
    let mut pending = HashSet::new();
    while let Some(block) = blocks.next().await {
        let block: BlockQueryData<SeqTypes> = match block {
            Ok(block) => block,
            Err(err) => {
                tracing::warn!("error getting block: {err}");
                continue;
            }
        };
        tracing::info!("got block {}", block.height());

        // Get all transactions which were submitted before this block.
        while let Ok(Some(tx)) = receiver.try_next() {
            pending.insert(tx);
        }

        // Clear pending transactions from the block.
        for (_, tx) in block.enumerate() {
            if pending.remove(&tx.commit()) {
                tracing::debug!("got transaction {}", tx.commit());
            }
        }

        tracing::info!("{} transactions still pending", pending.len());
    }
    tracing::info!(
        "block stream ended with {} transactions still pending",
        pending.len()
    );
}

async fn submit_transactions(
    opt: Options,
    mut sender: Sender<Commitment<Transaction>>,
    mut rng: ChaChaRng,
) {
    let client = Client::<Error>::new(opt.url.clone());
    loop {
        let tx = random_transaction(&opt, &mut rng);
        let hash = tx.commit();
        tracing::debug!(
            "submitting transaction {hash} for namespace {} of size {}",
            tx.vm(),
            tx.payload().len()
        );
        if let Err(err) = client
            .post::<()>("submit/submit")
            .body_binary(&tx)
            .unwrap()
            .send()
            .await
        {
            tracing::error!("failed to submit transaction: {err}");
        }
        sender.send(hash).await.ok();

        if let Some(delay) = opt.delay {
            sleep(delay).await;
        }
    }
}

fn random_transaction(opt: &Options, rng: &mut ChaChaRng) -> Transaction {
    let vm = rng.gen_range(opt.min_namespace..=opt.max_namespace);

    let len = rng.gen_range(opt.min_size..=opt.max_size);
    let mut payload = vec![0; len];
    rng.fill_bytes(&mut payload);

    Transaction::new(vm.into(), payload)
}

fn random_seed() -> u64 {
    ChaChaRng::from_entropy().next_u64()
}
