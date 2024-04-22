use crate::{api, persistence};
use anyhow::{bail, Context};
use bytesize::ByteSize;
use clap::{error::ErrorKind, Args, FromArgMatches, Parser};
use cld::ClDuration;
use derive_more::From;
use ethers::types::{Address, U256};
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;
use hotshot_types::light_client::StateSignKey;
use hotshot_types::signature_key::BLSPrivKey;
use snafu::Snafu;
use std::{
    collections::{HashMap, HashSet},
    iter::once,
    path::PathBuf,
    str::FromStr,
    time::Duration,
};
use url::Url;

// This options struct is a bit unconventional. The sequencer has multiple optional modules which
// can be added, in any combination, to the service. These include, for example, the API server.
// Each of these modules has its own options, which are all required if the module is added but can
// be omitted otherwise. Clap doesn't have a good way to handle "grouped" arguments like this (they
// have something called an argument group, but it's different). Sub-commands do exactly this, but
// you can't have multiple sub-commands in a single command.
//
// What we do, then, is take the optional modules as if they were sub-commands, but we use a Clap
// `raw` argument to collect all the module commands and their options into a single string. This
// string is then parsed manually (using a secondary Clap `Parser`, the `SequencerModule` type) when
// the user calls `modules()`.
//
// One slightly unfortunate consequence of this is that the auto-generated help documentation for
// `SequencerModule` is not included in the help for this top-level type. Users can still get at the
// help for individual modules by passing `help` as a subcommand, as in
// `sequencer [options] -- help` or `sequencer [options] -- help <module>`. This means that IT IS
// BEST NOT TO ADD REQUIRED ARGUMENTS TO THIS TYPE, since the required arguments will be required
// even if the user is only asking for help on a module. Try to give every argument on this type a
// default value, even if it is a bit arbitrary.
#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// Unique identifier for this instance of the sequencer network.
    #[clap(long, env = "ESPRESSO_SEQUENCER_CHAIN_ID", default_value = "0")]
    pub chain_id: u16,

    /// URL of the HotShot orchestrator.
    #[clap(
        short,
        long,
        env = "ESPRESSO_SEQUENCER_ORCHESTRATOR_URL",
        default_value = "http://localhost:8080"
    )]
    pub orchestrator_url: Url,

    /// The socket address of the HotShot CDN's main entry point (the marshal)
    /// in `IP:port` form
    #[clap(
        short,
        long,
        env = "ESPRESSO_SEQUENCER_CDN_ENDPOINT",
        default_value = "127.0.0.1:8081"
    )]
    pub cdn_endpoint: String,

    /// The address to bind to for Libp2p (in `host:port` form)
    #[clap(
        short,
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_BIND_ADDRESS",
        default_value = "0.0.0.0:1769"
    )]
    pub libp2p_bind_address: String,

    /// The address we advertise to other nodes as being a Libp2p endpoint.
    /// Should be supplied in `host:port` form.
    #[clap(
        short,
        long,
        env = "ESPRESSO_SEQUENCER_LIBP2P_ADVERTISE_ADDRESS",
        default_value = "localhost:1769"
    )]
    pub libp2p_advertise_address: String,

    /// URL of the Light Client State Relay Server
    #[clap(
        short,
        long,
        env = "ESPRESSO_STATE_RELAY_SERVER_URL",
        default_value = "http://localhost:8083"
    )]
    pub state_relay_server_url: Url,

    /// Path to file containing private keys.
    ///
    /// The file should follow the .env format, with two keys:
    /// * ESPRESSO_SEQUENCER_PRIVATE_STAKING_KEY
    /// * ESPRESSO_SEQUENCER_PRIVATE_STATE_KEY
    ///
    /// Appropriate key files can be generated with the `keygen` utility program.
    #[clap(long, name = "KEY_FILE", env = "ESPRESSO_SEQUENCER_KEY_FILE")]
    pub key_file: Option<PathBuf>,

    /// Private staking key.
    ///
    /// This can be used as an alternative to KEY_FILE.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_PRIVATE_STAKING_KEY",
        conflicts_with = "key_file"
    )]
    pub private_staking_key: Option<BLSPrivKey>,

    /// Private state signing key.
    ///
    /// This can be used as an alternative to KEY_FILE.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_PRIVATE_STATE_KEY",
        conflicts_with = "key_file"
    )]
    pub private_state_key: Option<StateSignKey>,

    /// Add optional modules to the service.
    ///
    /// Modules are added by specifying the name of the module followed by it's arguments, as in
    ///
    /// sequencer [options] -- api --port 3000
    ///
    /// to run the API module with port 3000.
    ///
    /// To see a list of available modules and their arguments, use
    ///
    /// sequencer -- help
    ///
    /// Multiple modules can be specified, provided they are separated by --
    #[clap(raw = true)]
    modules: Vec<String>,

    /// Mnemonic phrase for builder account.
    ///
    /// This is the address fees will be charged to.
    /// It must be funded with ETH in the Espresso fee ledger
    #[clap(long, env = "ESPRESSO_SEQUENCER_ETH_MNEMONIC")]
    pub eth_mnemonic: String,

    /// Index of a funded account derived from eth-mnemonic.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_ETH_ACCOUNT_INDEX",
        default_value = "8"
    )]
    pub eth_account_index: u32,

    /// Prefunded the builder accounts. Use for demo purposes only.
    ///
    /// Comma-separated list of Ethereum addresses.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_PREFUNDED_BUILDER_ACCOUNTS",
        value_delimiter = ','
    )]
    pub prefunded_builder_accounts: Vec<Address>,

    /// Url we will use for RPC communication with L1.
    #[clap(long, env = "ESPRESSO_SEQUENCER_L1_PROVIDER")]
    pub l1_provider_url: Url,

    /// Peer nodes use to fetch missing state
    #[clap(long, env = "ESPRESSO_SEQUENCER_STATE_PEERS", value_delimiter = ',')]
    pub state_peers: Vec<Url>,

    /// Stake table capacity for the prover circuit
    #[clap(short, long, env = "ESPRESSO_SEQUENCER_STAKE_TABLE_CAPACITY", default_value_t = STAKE_TABLE_CAPACITY)]
    pub stake_table_capacity: usize,
    /// Maximum size in bytes of a block
    #[clap(long, env = "ESPRESSO_SEQUENCER_MAX_BLOCK_SIZE", value_parser = parse_size)]
    pub max_block_size: u64,

    #[clap(long, env = "ESPRESSO_SEQUENCER_BASE_FEE")]
    /// Minimum fee in WEI per byte of payload
    pub base_fee: U256,
}

impl Options {
    pub fn modules(&self) -> Modules {
        ModuleArgs(self.modules.clone()).parse()
    }

    pub fn private_keys(&self) -> anyhow::Result<(BLSPrivKey, StateSignKey)> {
        if let Some(path) = &self.key_file {
            let vars = dotenvy::from_path_iter(path)?.collect::<Result<HashMap<_, _>, _>>()?;
            let staking = vars
                .get("ESPRESSO_SEQUENCER_PRIVATE_STAKING_KEY")
                .context("key file missing ESPRESSO_SEQUENCER_PRIVATE_STAKING_KEY")?
                .parse()?;
            let state = vars
                .get("ESPRESSO_SEQUENCER_PRIVATE_STATE_KEY")
                .context("key file missing ESPRESSO_SEQUENCER_PRIVATE_STATE_KEY")?
                .parse()?;
            Ok((staking, state))
        } else if let (Some(staking), Some(state)) = (
            self.private_staking_key.clone(),
            self.private_state_key.clone(),
        ) {
            Ok((staking, state))
        } else {
            bail!("neither key file nor full set of private keys was provided")
        }
    }
}

#[derive(Clone, Debug, Snafu)]
pub struct ParseDurationError {
    reason: String,
}

pub fn parse_duration(s: &str) -> Result<Duration, ParseDurationError> {
    ClDuration::from_str(s)
        .map(Duration::from)
        .map_err(|err| ParseDurationError {
            reason: err.to_string(),
        })
}

#[derive(Clone, Debug, From, Snafu)]
pub struct ParseSizeError {
    msg: String,
}

pub fn parse_size(s: &str) -> Result<u64, ParseSizeError> {
    Ok(s.parse::<ByteSize>()?.0)
}

#[derive(Clone, Debug)]
struct ModuleArgs(Vec<String>);

impl ModuleArgs {
    fn parse(&self) -> Modules {
        match self.try_parse() {
            Ok(modules) => modules,
            Err(err) => err.exit(),
        }
    }

    fn try_parse(&self) -> Result<Modules, clap::Error> {
        let mut modules = Modules::default();
        let mut curr = self.0.clone();
        let mut provided = Default::default();

        while !curr.is_empty() {
            // The first argument (the program name) is used only for help generation. We include a
            // `--` so that the generated usage will look like `sequencer -- <command>` which is the
            // way these commands must be invoked due to the use of `raw` arguments.
            let module = SequencerModule::try_parse_from(
                once("sequencer --").chain(curr.iter().map(|s| s.as_str())),
            )?;
            match module {
                SequencerModule::Storage(m) => {
                    curr = m.add(&mut modules.storage_fs, &mut provided)?
                }
                SequencerModule::StorageFs(m) => {
                    curr = m.add(&mut modules.storage_fs, &mut provided)?
                }
                SequencerModule::StorageSql(m) => {
                    curr = m.add(&mut modules.storage_sql, &mut provided)?
                }
                SequencerModule::Http(m) => curr = m.add(&mut modules.http, &mut provided)?,
                SequencerModule::Query(m) => curr = m.add(&mut modules.query, &mut provided)?,
                SequencerModule::Submit(m) => curr = m.add(&mut modules.submit, &mut provided)?,
                SequencerModule::Status(m) => curr = m.add(&mut modules.status, &mut provided)?,
                SequencerModule::State(m) => curr = m.add(&mut modules.state, &mut provided)?,
                SequencerModule::Catchup(m) => curr = m.add(&mut modules.catchup, &mut provided)?,
                SequencerModule::HotshotEvents(m) => {
                    curr = m.add(&mut modules.hotshot_events, &mut provided)?
                }
            }
        }

        Ok(modules)
    }
}

trait ModuleInfo: Args + FromArgMatches {
    const NAME: &'static str;
    fn requires() -> Vec<&'static str>;
}

macro_rules! module {
    ($name:expr, $opt:ty $(,requires: $($req:expr),*)?) => {
        impl ModuleInfo for $opt {
            const NAME: &'static str = $name;

            fn requires() -> Vec<&'static str> {
                vec![$($($req),*)?]
            }
        }
    };
}

module!("storage-fs", persistence::fs::Options);
module!("storage-sql", persistence::sql::Options);
module!("http", api::options::Http);
module!("query", api::options::Query, requires: "http");
module!("submit", api::options::Submit, requires: "http");
module!("status", api::options::Status, requires: "http");
module!("state", api::options::State, requires: "http", "storage-sql");
module!("catchup", api::options::Catchup, requires: "http");
module!("hotshot-events", api::options::HotshotEvents, requires: "http");

#[derive(Clone, Debug, Args)]
struct Module<Options: ModuleInfo> {
    #[clap(flatten)]
    options: Box<Options>,

    /// Add more optional modules.
    #[clap(raw = true)]
    modules: Vec<String>,
}

impl<Options: ModuleInfo> Module<Options> {
    /// Add this as an optional module. Return the next optional module args.
    fn add(
        self,
        options: &mut Option<Options>,
        provided: &mut HashSet<&'static str>,
    ) -> Result<Vec<String>, clap::Error> {
        if options.is_some() {
            return Err(clap::Error::raw(
                ErrorKind::TooManyValues,
                format!("optional module {} can only be started once", Options::NAME),
            ));
        }
        for req in Options::requires() {
            if !provided.contains(&req) {
                return Err(clap::Error::raw(
                    ErrorKind::MissingRequiredArgument,
                    format!("module {} is missing required module {req}", Options::NAME),
                ));
            }
        }
        *options = Some(*self.options);
        provided.insert(Options::NAME);
        Ok(self.modules)
    }
}

#[derive(Clone, Debug, Parser)]
enum SequencerModule {
    /// Run an HTTP server.
    ///
    /// The basic HTTP server comes with healthcheck and version endpoints. Add additional endpoints
    /// by enabling additional modules:
    /// * query: add query service endpoints
    /// * submit: add transaction submission endpoints
    Http(Module<api::options::Http>),
    /// Alias for storage-fs.
    Storage(Module<persistence::fs::Options>),
    /// Use the file system for persistent storage.
    StorageFs(Module<persistence::fs::Options>),
    /// Use a Postgres database for persistent storage.
    StorageSql(Module<persistence::sql::Options>),
    /// Run the query API module.
    ///
    /// This module requires the http module to be started.
    Query(Module<api::options::Query>),
    /// Run the transaction submission API module.
    ///
    /// This module requires the http module to be started.
    Submit(Module<api::options::Submit>),
    /// Run the status API module.
    ///
    /// This module requires the http module to be started.
    Status(Module<api::options::Status>),
    /// Run the state catchup API module.
    ///
    /// This module requires the http module to be started.
    Catchup(Module<api::options::Catchup>),
    /// Run the merklized state  API module.
    ///
    /// This module requires the http and storage-sql modules to be started.
    State(Module<api::options::State>),
    /// Run the hotshot events API module.
    ///
    /// This module requires the http module to be started.
    HotshotEvents(Module<api::options::HotshotEvents>),
}

#[derive(Clone, Debug, Default)]
pub struct Modules {
    pub storage_fs: Option<persistence::fs::Options>,
    pub storage_sql: Option<persistence::sql::Options>,
    pub http: Option<api::options::Http>,
    pub query: Option<api::options::Query>,
    pub submit: Option<api::options::Submit>,
    pub status: Option<api::options::Status>,
    pub state: Option<api::options::State>,
    pub catchup: Option<api::options::Catchup>,
    pub hotshot_events: Option<api::options::HotshotEvents>,
}
