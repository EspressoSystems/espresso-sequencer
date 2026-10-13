//! The leader's block build (`request_block` in crates/hotshot/new-protocol/src/block.rs), timed
//! step by step.
//!
//! `request_block` runs `from_transactions` on the cached transactions, then
//! `hotshot_new_protocol::block::block_commitments`, the function the leader calls. A tracing
//! layer records each step's start, completion and CPU time; when `CRITERION_HOME` is set they are
//! written to `steps.json` next to the benchmark's criterion output. Step samples start with
//! criterion's warm-up iterations; scripts/bench-block-build keeps one per measured iteration,
//! which assumes every recorded span opens exactly once per iteration.
//!
//! Inputs mirror the network-bench load generator: random 1 MiB transactions round-robin over
//! 16 namespaces in 30 to 70 MiB blocks, protocol version 0.6 (AvidmGf2), 100 nodes. Real stakes
//! are large, so the VID total weight is the `approximate_weights` result for 100 equal stakes
//! (1100).
//!
//! To compare two git refs, run `just bench-block-build BASE HEAD` (scripts/bench-block-build).

use std::{
    collections::BTreeMap,
    env, fs, mem,
    path::PathBuf,
    sync::Mutex,
    time::{Duration, Instant},
};

use criterion::{BenchmarkId, Criterion, SamplingMode};
use espresso_types::{ChainConfig, NamespaceId, NsTable, Payload, SeqTypes, Transaction};
use hotshot_new_protocol::block::{BlockCommitments, block_commitments};
use hotshot_types::{consensus::PayloadWithMetadata, data::vid_disperse::VID_TARGET_TOTAL_STAKE};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use tracing::{Level, Subscriber, span};
use tracing_subscriber::{
    Layer,
    filter::Targets,
    layer::{Context, SubscriberExt},
    registry::LookupSpan,
};
use versions::NEW_PROTOCOL_VERSION;

const MIB: usize = 1 << 20;
const NAMESPACES: u64 = 16;
const NODES: usize = 100;
const BLOCK_MIB: [usize; 5] = [30, 40, 50, 60, 70];
/// `approximate_weights` total for `NODES` equal stakes above `VID_TARGET_TOTAL_STAKE`.
const VID_WEIGHT: usize = NODES * (VID_TARGET_TOTAL_STAKE as usize / NODES + 1);

fn bench_block_build(c: &mut Criterion) {
    install_step_timer();
    let threads = rayon::current_num_threads();
    let mut group = c.benchmark_group("block_build");
    group
        .sampling_mode(SamplingMode::Flat)
        .sample_size(10)
        .warm_up_time(Duration::from_millis(200))
        .measurement_time(Duration::from_secs(1));
    for block_mib in BLOCK_MIB {
        let label = format!("{block_mib}MiB_t{threads}_n{NODES}");
        let txs = transactions(block_mib);
        group.bench_function(BenchmarkId::new("request_block", &label), |b| {
            b.iter_with_large_drop(|| request_block(&txs))
        });
        write_steps(&format!("block_build/request_block/{label}"));
    }
    group.finish();
}

/// The leader's build task after the transactions are taken; outputs are returned so their drop
/// is untimed.
fn request_block(
    txs: &[Transaction],
) -> (PayloadWithMetadata<SeqTypes>, BlockCommitments<SeqTypes>) {
    let _span = tracing::debug_span!("request_block").entered();
    let (payload, metadata): (Payload, NsTable) =
        Payload::from_transactions_sync(txs, chain_config()).expect("payload construction");
    let payload = PayloadWithMetadata { payload, metadata };
    let commitments = block_commitments(&payload, VID_WEIGHT, NEW_PROTOCOL_VERSION);
    (payload, commitments)
}

fn transactions(block_mib: usize) -> Vec<Transaction> {
    let mut rng = ChaCha20Rng::seed_from_u64(block_mib as u64);
    (0..block_mib)
        .map(|i| {
            let mut body = vec![0u8; MIB];
            rng.fill_bytes(&mut body);
            Transaction::new(NamespaceId::from(10_000 + i as u64 % NAMESPACES), body)
        })
        .collect()
}

fn chain_config() -> ChainConfig {
    ChainConfig {
        max_block_size: (1u64 << 30).into(),
        ..Default::default()
    }
}

/// `[start, completion - start, cpu]` in ms per step and iteration, by step name; start is
/// relative to the enclosing `request_block` span.
static STEPS: Mutex<BTreeMap<&'static str, Vec<[f64; 3]>>> = Mutex::new(BTreeMap::new());

/// The running `request_block` iteration. Steps on rayon threads have no parent span, and
/// iterations run one at a time, so this is their common state.
static ITERATION: Mutex<Option<Iteration>> = Mutex::new(None);

/// Span names of the transaction hashing step, before and after the blake3 digest switch.
const TX_STEPS: [&str; 2] = ["transaction_commitments", "transaction_digests"];

/// Wall and CPU readings at a span's most recent enter.
struct Entered {
    at: Instant,
    thread_cpu: f64,
    /// Only read for `request_block` and `block_commitments`, which no rayon worker enters.
    process_cpu: f64,
}

/// A step span that has exited, in ms relative to the iteration start.
struct Step {
    name: &'static str,
    start: f64,
    end: f64,
    cpu: f64,
}

struct Iteration {
    origin: Instant,
    steps: Vec<Step>,
    /// Sum of `tx_leaf` thread CPU and the latest `tx_leaf` exit.
    leaf_cpu: f64,
    leaves_end: f64,
    commitments_process_cpu: f64,
}

/// Times each step by its own work. A rayon worker that waits inside a step span steals unrelated
/// jobs, so a step's span can last as long as the VID work. The `tx_leaf` spans make no rayon
/// calls: the hashing step completes at the last leaf exit and costs the leaves' summed thread
/// CPU. The VID step's CPU is what the process spent in `block_commitments` beyond the other steps.
struct StepTimer;

impl<S: Subscriber + for<'a> LookupSpan<'a>> Layer<S> for StepTimer {
    fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
        let at = Instant::now();
        let span = ctx.span(id).expect("entered span is registered");
        let wide = matches!(span.name(), "request_block" | "block_commitments");
        let entered = Entered {
            at,
            thread_cpu: cpu_ms(libc::CLOCK_THREAD_CPUTIME_ID),
            process_cpu: if wide {
                cpu_ms(libc::CLOCK_PROCESS_CPUTIME_ID)
            } else {
                0.0
            },
        };
        if span.name() == "request_block" {
            *ITERATION.lock().expect("iteration lock") = Some(Iteration {
                origin: at,
                steps: Vec::new(),
                leaf_cpu: 0.0,
                leaves_end: 0.0,
                commitments_process_cpu: 0.0,
            });
        }
        span.extensions_mut().replace(entered);
    }

    fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>) {
        let end = Instant::now();
        let thread_cpu = cpu_ms(libc::CLOCK_THREAD_CPUTIME_ID);
        let span = ctx.span(id).expect("exited span is registered");
        let name = span.name();
        let (start, start_cpu, start_process) = {
            let ext = span.extensions();
            let entered = ext.get::<Entered>().expect("recorded in on_enter");
            (entered.at, entered.thread_cpu, entered.process_cpu)
        };
        let mut guard = ITERATION.lock().expect("iteration lock");
        let iteration = guard.as_mut().expect("spans run inside request_block");
        let origin = iteration.origin;
        let rel = |t: Instant| ms(t - origin);
        let cpu = thread_cpu - start_cpu;
        match name {
            "tx_leaf" => {
                iteration.leaf_cpu += cpu;
                iteration.leaves_end = iteration.leaves_end.max(rel(end));
            },
            "request_block" | "block_commitments" => {
                let process = cpu_ms(libc::CLOCK_PROCESS_CPUTIME_ID) - start_process;
                if name == "block_commitments" {
                    iteration.commitments_process_cpu = process;
                }
                iteration.steps.push(Step {
                    name,
                    start: rel(start),
                    end: rel(end),
                    cpu: process,
                });
                if name == "request_block" {
                    record(guard.take().expect("iteration present"));
                }
            },
            _ => iteration.steps.push(Step {
                name,
                start: rel(start),
                end: rel(end),
                cpu,
            }),
        }
    }
}

/// Pushes one row per step of a finished iteration into [`STEPS`].
fn record(iteration: Iteration) {
    let cpu_of = |name: &str| {
        iteration
            .steps
            .iter()
            .find(|s| s.name == name)
            .map_or(0.0, |s| s.cpu)
    };
    let vid_cpu = iteration.commitments_process_cpu
        - iteration.leaf_cpu
        - cpu_of("builder_commitment")
        - cpu_of("encode");
    let mut steps = STEPS.lock().expect("steps lock");
    for step in &iteration.steps {
        let (end, cpu) = match step.name {
            name if TX_STEPS.contains(&name) => (iteration.leaves_end, iteration.leaf_cpu),
            "vid_commitment" => (step.end, vid_cpu),
            _ => (step.end, step.cpu),
        };
        steps
            .entry(step.name)
            .or_default()
            .push([step.start, end - step.start, cpu]);
    }
}

fn cpu_ms(clock: libc::clockid_t) -> f64 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: `ts` is a valid timespec for clock_gettime to write.
    let rc = unsafe { libc::clock_gettime(clock, &mut ts) };
    assert_eq!(rc, 0, "clock_gettime failed");
    ts.tv_sec as f64 * 1e3 + ts.tv_nsec as f64 / 1e6
}

fn install_step_timer() {
    let targets = Targets::new()
        .with_target("block_build", Level::DEBUG)
        .with_target("hotshot_new_protocol::block", Level::DEBUG)
        .with_target("espresso_types", Level::DEBUG);
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(StepTimer.with_filter(targets)),
    )
    .expect("no other subscriber");
}

/// Writes the steps recorded since the last call to `$CRITERION_HOME/<id>/steps.json`.
fn write_steps(id: &str) {
    let steps = mem::take(&mut *STEPS.lock().expect("steps lock"));
    // Benchmarks skipped by the command-line filter record nothing.
    let Some(home) = env::var_os("CRITERION_HOME").filter(|_| !steps.is_empty()) else {
        return;
    };
    let path = PathBuf::from(home).join(id).join("steps.json");
    fs::create_dir_all(path.parent().expect("steps path has a parent")).expect("create steps dir");
    fs::write(&path, serde_json::to_vec(&steps).expect("serialize steps"))
        .expect("write steps.json");
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1e3
}

criterion::criterion_group!(benches, bench_block_build);
criterion::criterion_main!(benches);
