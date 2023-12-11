use crate::{api, hotshot_commitment::CommitmentTaskOptions};
use clap::{error::ErrorKind, Args, FromArgMatches, Parser};
use std::collections::HashSet;
use std::iter::once;
use std::path::PathBuf;
use url::Url;

// This options struct is a bit unconventional. The sequencer has multiple optional modules which
// can be added, in any combination, to the service. These include, for example, the HotShot
// commitment task and the API server. Each of these modules has its own options, which are all
// required if the module is added but can be omitted otherwise. Clap doesn't have a good way to
// handle "grouped" arguments like this (they have something called an argument group, but it's
// different). Sub-commands do exactly this, but you can't have multiple sub-commands in a single
// command.
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

    /// URL of the HotShot DA web server.
    #[clap(
        short,
        long,
        env = "ESPRESSO_SEQUENCER_DA_SERVER_URL",
        default_value = "http://localhost:8081"
    )]
    pub da_server_url: Url,

    /// URL of the HotShot consensus web server.
    #[clap(
        short,
        long,
        env = "ESPRESSO_SEQUENCER_CONSENSUS_SERVER_URL",
        default_value = "http://localhost:8082"
    )]
    pub consensus_server_url: Url,

    /// Path to save and load consensus configuration.
    ///
    /// Allows for rejoining the network on a complete state loss.
    #[clap(short, long, env = "ESPRESSO_SEQUENCER_CONFIG_PATH")]
    pub config_path: Option<PathBuf>,

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
}

impl Options {
    pub fn modules(&self) -> Modules {
        ModuleArgs(self.modules.clone()).parse()
    }
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
                SequencerModule::Http(m) => curr = m.add(&mut modules.http, &mut provided)?,
                SequencerModule::Query(m) => curr = m.add(&mut modules.query_fs, &mut provided)?,
                SequencerModule::QueryFs(m) => {
                    curr = m.add(&mut modules.query_fs, &mut provided)?
                }
                SequencerModule::Submit(m) => curr = m.add(&mut modules.submit, &mut provided)?,
                SequencerModule::Status(m) => curr = m.add(&mut modules.status, &mut provided)?,
                SequencerModule::CommitmentTask(m) => {
                    curr = m.add(&mut modules.commitment_task, &mut provided)?
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

module!("http", api::options::Http);
module!("query-fs", api::options::Fs, requires: "http");
module!("submit", api::options::Submit, requires: "http");
module!("status", api::options::Status, requires: "http");
module!("commitment-task", CommitmentTaskOptions);

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
    /// Alias for query-fs.
    Query(Module<api::options::Fs>),
    /// Run the query service API module, backed by the file system.
    ///
    /// This module requires the http module to be started.
    QueryFs(Module<api::options::Fs>),
    /// Run the transaction submission API module.
    ///
    /// This module requires the http module to be started.
    Submit(Module<api::options::Submit>),
    /// Run the status API module.
    ///
    /// This module requires the http module to be started.
    Status(Module<api::options::Status>),
    CommitmentTask(Module<CommitmentTaskOptions>),
}

#[derive(Clone, Debug, Default)]
pub struct Modules {
    pub http: Option<api::options::Http>,
    pub query_fs: Option<api::options::Fs>,
    pub submit: Option<api::options::Submit>,
    pub status: Option<api::options::Status>,
    pub commitment_task: Option<CommitmentTaskOptions>,
}
