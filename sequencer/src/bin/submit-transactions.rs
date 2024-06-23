//! Utility program to submit random transactions to an Espresso Sequencer.

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::task::{sleep, spawn};
use clap::Parser;
use committable::{Commitment, Committable};
use es_version::{SequencerVersion, SEQUENCER_VERSION};
use futures::{
    channel::mpsc::{self, Sender},
    sink::SinkExt,
    stream::StreamExt,
};
use hotshot_query_service::{availability::BlockQueryData, types::HeightIndexed, Error};
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use rand_distr::Distribution;
use sequencer::{
    options::{parse_duration, parse_size},
    SeqTypes, Transaction,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use surf_disco::{Client, Url};
use tide_disco::{error::ServerError, App};
use vbs::version::StaticVersionType;

/// Submit random transactions to an Espresso Sequencer.
#[derive(Clone, Debug, Parser)]
struct Options {
    /// Minimum size of transaction to submit.
    ///
    /// The size of each transaction will be chosen uniformly between MIN_SIZE and MAX_SIZE.
    #[clap(long, name = "MIN_SIZE", default_value = "1", value_parser = parse_size, env = "ESPRESSO_SUBMIT_TRANSACTIONS_MIN_SIZE")]
    min_size: u64,

    /// Maximum size of transaction to submit.
    ///
    /// The size of each transaction will be chosen uniformly between MIN_SIZE and MAX_SIZE.
    #[clap(long, name = "MAX_SIZE", default_value = "1kb", value_parser = parse_size, env = "ESPRESSO_SUBMIT_TRANSACTIONS_MAX_SIZE")]
    max_size: u64,

    /// Minimum size of batch of transactions to submit.
    ///
    /// Batches will be a random count between MIN_BATCH_SIZE and MAX_BATCH_SIZE, with a falling distribution favoring smaller batches.
    /// This is by selecting a random size S on each iteration I since last batch, and collecting a batch whenever that S <= I.
    #[clap(long, name = "MIN_BATCH_SIZE", default_value = "1", value_parser = parse_size, env = "ESPRESSO_SUBMIT_TRANSACTIONS_MIN_BATCH_SIZE")]
    min_batch_size: u64,

    /// Maximum size of batch of transactions to submit.
    ///
    /// Batches will be a random count between MIN_BATCH_SIZE and MAX_BATCH_SIZE, with a falling distribution favoring smaller batches.
    #[clap(long, name = "MAX_BATCH_SIZE", default_value = "20", value_parser = parse_size, env = "ESPRESSO_SUBMIT_TRANSACTIONS_MAX_BATCH_SIZE")]
    max_batch_size: u64,

    /// Minimum namespace ID to submit to.
    #[clap(
        long,
        default_value = "10000",
        env = "ESPRESSO_SUBMIT_TRANSACTIONS_MIN_NAMESPACE"
    )]
    min_namespace: u32,

    /// Maximum namespace ID to submit to.
    #[clap(
        long,
        default_value = "10010",
        env = "ESPRESSO_SUBMIT_TRANSACTIONS_MAX_NAMESPACE"
    )]
    max_namespace: u32,

    /// Mean delay between submitting transactions.
    ///
    /// The delay after each transaction will be sampled from an exponential distribution with mean
    /// DELAY.
    #[clap(long, name = "DELAY", value_parser = parse_duration, default_value = "30s", env = "ESPRESSO_SUBMIT_TRANSACTIONS_DELAY")]
    delay: Duration,

    /// Maximum number of unprocessed transaction submissions.
    ///
    /// This can be used to apply backpressure so that the tasks submitting transactions do not get
    /// too far ahead of the task processing results.
    #[clap(
        long,
        default_value = "1000",
        env = "ESPRESSO_SUBMIT_TRANSACTIONS_CHANNEL_BOUND"
    )]
    channel_bound: usize,

    /// Seed for reproducible randomness.
    #[clap(long, env = "ESPRESSO_SUBMIT_TRANSACTIONS_SEED")]
    seed: Option<u64>,

    /// Number of parallel tasks to run.
    #[clap(
        short,
        long,
        default_value = "1",
        env = "ESPRESSO_SUBMIT_TRANSACTIONS_JOBS"
    )]
    jobs: usize,

    /// Number of accumulated pending transactions which should trigger a warning.
    #[clap(
        long,
        default_value = "10",
        env = "ESPRESSO_SUBMIT_TRANSACTIONS_PENDING_TRANSACTIONS_WARNING_THRESHOLD"
    )]
    pending_transactions_warning_threshold: usize,

    /// Duration after which we should warn about a pending transaction.
    #[clap(long, value_parser = parse_duration, default_value = "30s", env = "ESPRESSO_SUBMIT_TRANSACTIONS_SLOW_TRANSACTION_WARNING_THRESHOLD")]
    slow_transaction_warning_threshold: Duration,

    /// Enable an HTTP server with a healthcheck endpoint on this port.
    #[clap(short, long, env = "ESPRESSO_SUBMIT_TRANSACTIONS_PORT")]
    port: Option<u16>,

    /// Alternative URL to submit transactions to, if not the query service URL.
    #[clap(long, env = "ESPRESSO_SUBMIT_TRANSACTIONS_SUBMIT_URL")]
    submit_url: Option<Url>,

    /// URL of the query service.
    #[clap(env = "ESPRESSO_SEQUENCER_URL")]
    url: Url,
}

impl Options {
    fn submit_url(&self) -> Url {
        self.submit_url
            .clone()
            .unwrap_or_else(|| self.url.join("submit").unwrap())
    }
    fn use_public_mempool(&self) -> bool {
        self.submit_url.is_none()
    }
}

#[async_std::main]
async fn main() {
    setup_backtrace();
    setup_logging();

    let opt = Options::parse();
    tracing::warn!("starting load generator for sequencer {}", opt.url);

    let (sender, mut receiver) = mpsc::channel(opt.channel_bound);

    let seed = opt.seed.unwrap_or_else(random_seed);
    tracing::info!("PRNG seed: {seed}");
    let mut rng = ChaChaRng::seed_from_u64(seed);

    // Subscribe to block stream so we can check that our transactions are getting sequenced.
    let client = Client::<Error, SequencerVersion>::new(opt.url.clone());
    let block_height: usize = client.get("status/block-height").send().await.unwrap();
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
            SEQUENCER_VERSION,
        ));
    }

    // Start healthcheck endpoint once tasks are running.
    if let Some(port) = opt.port {
        spawn(server(port, SEQUENCER_VERSION));
    }

    // Keep track of the results.
    let mut pending = HashMap::new();
    let mut total_latency = Duration::default();
    let mut total_transactions = 0;
    while let Some(block) = blocks.next().await {
        let block: BlockQueryData<SeqTypes> = match block {
            Ok(block) => block,
            Err(err) => {
                tracing::warn!("error getting block: {err}");
                continue;
            }
        };
        let received_at = Instant::now();
        tracing::debug!("got block {}", block.height());

        // Get all transactions which were submitted before this block.
        while let Ok(Some(tx)) = receiver.try_next() {
            pending.insert(tx.hash, tx.submitted_at);
        }

        // Clear pending transactions from the block.
        for (_, tx) in block.enumerate() {
            if let Some(submitted_at) = pending.remove(&tx.commit()) {
                let latency = received_at - submitted_at;
                tracing::info!(
                    "got transaction {} in block {}, latency {latency:?}",
                    tx.commit(),
                    block.height()
                );
                total_latency += latency;
                total_transactions += 1;
                tracing::info!("average latency: {:?}", total_latency / total_transactions);
            }
        }

        // If a lot of transactions are pending, it might indicate the sequencer is struggling to
        // finalize them. We should warn about this.
        if pending.len() >= opt.pending_transactions_warning_threshold {
            tracing::warn!(
                "transactions are not being finalized or being finalized too slowly, {} pending",
                pending.len()
            );
        } else {
            tracing::debug!("{} transactions still pending", pending.len());

            // Even if we are not accumulating transactions, it is still possible that some
            // individual transactions are not being finalized. Warn about any transaction which has
            // been pending for too long.
            for (tx, submitted_at) in &pending {
                let duration = received_at - *submitted_at;
                if duration >= opt.slow_transaction_warning_threshold {
                    tracing::warn!("transaction {tx} has been pending for {duration:?}");
                }
            }
        }
    }
    tracing::info!(
        "block stream ended with {} transactions still pending",
        pending.len()
    );
}

struct SubmittedTransaction {
    hash: Commitment<Transaction>,
    submitted_at: Instant,
}

async fn submit_transactions<Ver: StaticVersionType>(
    opt: Options,
    mut sender: Sender<SubmittedTransaction>,
    mut rng: ChaChaRng,
    _: Ver,
) {
    let url = opt.submit_url();
    tracing::info!(%url, "starting load generator task");
    let client = Client::<Error, Ver>::new(url);

    // Create an exponential distribution for sampling delay times. The distribution should have
    // mean `opt.delay`, or parameter `\lambda = 1 / opt.delay`.
    let delay_distr = rand_distr::Exp::<f64>::new(1f64 / opt.delay.as_millis() as f64).unwrap();

    let mut txns = Vec::new();
    let mut hashes = Vec::new();
    loop {
        let tx = random_transaction(&opt, &mut rng);
        let hash = tx.commit();
        tracing::info!(
            "Adding transaction {hash} for namespace {} of size {}",
            tx.namespace(),
            tx.payload().len()
        );
        txns.push(tx);
        hashes.push(hash);

        let randomized_batch_size = if opt.use_public_mempool() {
            1
        } else {
            rng.gen_range(opt.min_batch_size..=opt.max_batch_size)
        };
        let txns_batch_count = txns.len() as u64;
        if randomized_batch_size <= txns_batch_count {
            if let Err(err) = if txns_batch_count == 1 {
                // occasionally test the 'submit' endpoint, just for coverage
                client
                    .post::<()>("submit")
                    .body_binary(&txns[0])
                    .unwrap()
                    .send()
                    .await
            } else {
                client
                    .post::<()>("batch")
                    .body_binary(&txns)
                    .unwrap()
                    .send()
                    .await
            } {
                tracing::error!("failed to submit batch of {txns_batch_count} transactions: {err}");
            } else {
                println!("submitted batch of {txns_batch_count} transactions");
                tracing::info!("submitted batch of {txns_batch_count} transactions");
                let submitted_at = Instant::now();
                for hash in hashes.iter() {
                    sender
                        .send(SubmittedTransaction {
                            hash: *hash,
                            submitted_at,
                        })
                        .await
                        .ok();
                }
            }
            txns.clear();
            hashes.clear();

            let delay = Duration::from_millis(delay_distr.sample(&mut rng) as u64);
            tracing::info!("sleeping for {delay:?}");
            sleep(delay).await;
        }
    }
}

async fn server<Ver: StaticVersionType + 'static>(port: u16, bind_version: Ver) {
    if let Err(err) = App::<(), ServerError>::with_state(())
        .serve(format!("0.0.0.0:{port}"), bind_version)
        .await
    {
        tracing::error!("web server exited: {err}");
    }
}

fn random_transaction(opt: &Options, rng: &mut ChaChaRng) -> Transaction {
    // TODO instead use NamespaceId::random, but that does not allow us to
    // enforce `gen_range(opt.min_namespace..=opt.max_namespace)`
    let namespace = rng.gen_range(opt.min_namespace..=opt.max_namespace);

    let len = rng.gen_range(opt.min_size..=opt.max_size);
    let mut payload = vec![0; len as usize];
    rng.fill_bytes(&mut payload);

    Transaction::new(namespace.into(), payload)
}

fn random_seed() -> u64 {
    ChaChaRng::from_entropy().next_u64()
}
