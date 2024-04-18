use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use derive_more::From;
use ethers::utils::hex::{self, FromHexError};
use hotshot::traits::election::static_committee::StaticElectionConfig;
use hotshot_orchestrator::config::Libp2pConfig;
use hotshot_orchestrator::{config::NetworkConfig, run_orchestrator};
use sequencer::{options::parse_duration, PubKey};
use snafu::Snafu;
use std::fmt::{self, Display, Formatter};
use std::num::{NonZeroUsize, ParseIntError};
use std::str::FromStr;
use std::time::Duration;
use url::Url;

#[derive(Parser)]
struct Args {
    /// Port to run the server on.
    #[clap(short, long, env = "ESPRESSO_ORCHESTRATOR_PORT")]
    port: u16,

    /// Number of nodes in the network.
    #[clap(short, long, env = "ESPRESSO_ORCHESTRATOR_NUM_NODES")]
    num_nodes: NonZeroUsize,

    /// Duration to wait after all nodes are connected before starting the run.
    #[clap(long, env = "ESPRESSO_ORCHESTRATOR_START_DELAY", default_value = "10s", value_parser = parse_duration)]
    start_delay: Duration,

    /// Minimum time to wait for submitted transactions before proposing a block.
    ///
    /// Increasing this trades off latency for throughput: the rate of new block proposals gets
    /// slower, but each block is proportionally larger. Because of batch verification, larger
    /// blocks should lead to increased throughput.
    ///
    /// `min-propose-time` is set to 1s by default, since minimum block size can be controlled using
    /// `min-transactions`, which is a more intentional, declarative setting. You may still wish to
    /// set a non-zero `min-propose-time` to allow for larger blocks in higher volumes while setting
    /// `min-transactions` to something small to handle low-volume conditions.
    #[arg(
        long,
        env = "ESPRESSO_ORCHESTRATOR_MIN_PROPOSE_TIME",
        default_value = "1s",
        value_parser = parse_duration
    )]
    min_propose_time: Duration,

    /// Maximum time to wait for submitted transactions before proposing a block.
    ///
    /// If a validator has not received `min-transactions` after `min-propose-time`, it will wait up
    /// to `max-propose-time` before giving up and submitting a block with whatever transactions it
    /// does have.
    #[arg(
        long,
        env = "ESPRESSO_ORCHESTRATOR_MAX_PROPOSE_TIME",
        default_value = "30s",
        value_parser = parse_duration
    )]
    max_propose_time: Duration,

    /// Minimum number of transactions to include in a block, if possible.
    ///
    /// After `min-propose-time`, a leader will propose a block as soon as it has at least
    /// `min-transactions`. Note that a block with fewer than `min-transactions` may still be
    /// proposed, if `min-transactions` are not submitted before `max-propose-time`.
    ///
    /// The default is 1, because a non-zero value of `min-transactions` is required in order for
    /// `max-propose-time` to have any effect -- if `min-transactions = 0`, then an empty block will
    /// be proposed each view after `min-propose-time`. Setting `min-transactions` to 1 limits the
    /// number of empty blocks proposed while still allowing a block to be proposed as soon as any
    /// transaction has been received. In a setting where high volume is expected most of the time,
    /// you might set this greater than 1 to encourage larger blocks and better throughput, while
    /// setting `max-propose-time` very large to handle low-volume conditions without affecting
    /// latency in high-volume conditions.
    #[clap(
        long,
        env = "ESPRESSO_ORCHESTRATOR_MIN_TRANSACTIONS",
        default_value = "1"
    )]
    min_transactions: usize,

    /// Base duration for next-view timeout.
    #[arg(
        long,
        env = "ESPRESSO_ORCHESTRATOR_NEXT_VIEW_TIMEOUT",
        default_value = "60s",
        value_parser = parse_duration
    )]
    next_view_timeout: Duration,

    /// The exponential backoff ratio for the next-view timeout.
    #[arg(
        long,
        env = "ESPRESSO_ORCHESTRATOR_TIMEOUT_RATIO",
        default_value = "11:10"
    )]
    timeout_ratio: Ratio,

    /// The delay a leader inserts before starting pre-commit.
    #[arg(
        long,
        env = "ESPRESSO_ORCHESTRATOR_ROUND_START_DELAY",
        default_value = "1ms",
        value_parser = parse_duration
    )]
    round_start_delay: Duration,

    /// Maximum number of transactions in a block.
    ///
    /// TODO replace with maximum byte len:
    /// https://github.com/EspressoSystems/espresso-sequencer/pull/1087#discussion_r1489585926
    #[arg(
        long,
        env = "ESPRESSO_ORCHESTRATOR_MAX_TRANSACTIONS",
        default_value = "1000"
    )]
    max_transactions: NonZeroUsize,

    /// The number of nodes a Libp2p node should try to maintain
    /// a connection with at one time.
    #[arg(long, env = "ESPRESSO_ORCHESTRATOR_LIBP2P_MESH_N", default_value = "4")]
    libp2p_mesh_n: usize,

    /// Seed to use for generating node keys.
    ///
    /// The seed is a 32 byte integer, encoded in hex.
    #[arg(long, env = "ESPRESSO_ORCHESTRATOR_KEYGEN_SEED", default_value = "0x0000000000000000000000000000000000000000000000000000000000000000", value_parser = parse_seed)]
    keygen_seed: [u8; 32],

    /// HotShot builder URL
    #[arg(long, env = "ESPRESSO_ORCHESTRATOR_BUILDER_URL")]
    builder_url: Url,
}

#[derive(Debug, Snafu, From)]
enum ParseSeedError {
    #[snafu(display("seed must be valid hex: {source}"))]
    Hex { source: FromHexError },

    #[snafu(display("wrong length for seed {length} (expected 32)"))]
    WrongLength { length: usize },
}

fn parse_seed(s: &str) -> Result<[u8; 32], ParseSeedError> {
    <[u8; 32]>::try_from(hex::decode(s)?)
        .map_err(|vec| ParseSeedError::WrongLength { length: vec.len() })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Ratio {
    numerator: u64,
    denominator: u64,
}

impl From<Ratio> for (u64, u64) {
    fn from(r: Ratio) -> Self {
        (r.numerator, r.denominator)
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.numerator, self.denominator)
    }
}

#[derive(Debug, Snafu)]
enum ParseRatioError {
    #[snafu(display("numerator and denominator must be separated by :"))]
    MissingDelimiter,
    InvalidNumerator {
        err: ParseIntError,
    },
    InvalidDenominator {
        err: ParseIntError,
    },
}

impl FromStr for Ratio {
    type Err = ParseRatioError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, den) = s.split_once(':').ok_or(ParseRatioError::MissingDelimiter)?;
        Ok(Self {
            numerator: num
                .parse()
                .map_err(|err| ParseRatioError::InvalidNumerator { err })?,
            denominator: den
                .parse()
                .map_err(|err| ParseRatioError::InvalidDenominator { err })?,
        })
    }
}

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();
    let args = Args::parse();
    let mut config = NetworkConfig::<PubKey, StaticElectionConfig> {
        start_delay_seconds: args.start_delay.as_secs(),
        ..Default::default()
    };

    // The Libp2p configuration
    let libp2p_config = Libp2pConfig {
        bootstrap_nodes: Vec::new(),
        node_index: 0,
        bootstrap_mesh_n_high: args.libp2p_mesh_n,
        bootstrap_mesh_n_low: args.libp2p_mesh_n,
        bootstrap_mesh_outbound_min: args.libp2p_mesh_n / 2,
        bootstrap_mesh_n: args.libp2p_mesh_n,
        mesh_n_high: args.libp2p_mesh_n,
        mesh_n_low: args.libp2p_mesh_n,
        mesh_outbound_min: args.libp2p_mesh_n / 2,
        mesh_n: args.libp2p_mesh_n,
        next_view_timeout: config.next_view_timeout,
        propose_min_round_time: config.propose_min_round_time,
        propose_max_round_time: config.propose_max_round_time,
        online_time: 10,
        num_txn_per_round: 0,
        server_mode: false,
    };

    config.config.num_nodes_with_stake = args.num_nodes;
    config.config.num_nodes_without_stake = 0;
    config.config.known_nodes_with_stake = vec![Default::default(); args.num_nodes.get()];
    config.config.known_nodes_without_stake = vec![];
    config.config.max_transactions = args.max_transactions;
    config.config.next_view_timeout = args.next_view_timeout.as_millis() as u64;
    config.libp2p_config = Some(libp2p_config);
    config.config.timeout_ratio = args.timeout_ratio.into();
    config.config.round_start_delay = args.round_start_delay.as_millis() as u64;
    config.config.start_delay = args.start_delay.as_millis() as u64;
    config.config.propose_min_round_time = args.min_propose_time;
    config.config.propose_max_round_time = args.max_propose_time;
    config.config.da_staked_committee_size = args.num_nodes.get();
    config.config.da_non_staked_committee_size = 0;
    config.config.min_transactions = args.min_transactions;
    config.config.builder_url = args.builder_url;

    run_orchestrator(
        config,
        format!("http://0.0.0.0:{}", args.port).parse().unwrap(),
    )
    .await
    .unwrap();
}
