use crate::{api, hotshot_commitment::CommitmentTaskOptions};
use clap::{error::ErrorKind, Args, FromArgMatches, Parser};
use std::iter::once;
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

    /// URL of the HotShot web server.
    #[clap(
        short,
        long,
        env = "ESPRESSO_SEQUENCER_WEB_SERVER_URL",
        default_value = "http://localhost:50000"
    )]
    pub web_server_url: Url,

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

        while !curr.is_empty() {
            // The first argument (the program name) is used only for help generation. We include a
            // `--` so that the generated usage will look like `sequencer -- <command>` which is the
            // way these commands must be invoked due to the use of `raw` arguments.
            let module = SequencerModule::try_parse_from(
                once("sequencer --").chain(curr.iter().map(|s| s.as_str())),
            )?;
            match module {
                SequencerModule::Api(m) => curr = m.add("api", &mut modules.api)?,
                SequencerModule::CommitmentTask(m) => {
                    curr = m.add("commitment-task", &mut modules.commitment_task)?
                }
            }
        }

        Ok(modules)
    }
}

#[derive(Clone, Debug, Args)]
struct Module<Options: Args + FromArgMatches> {
    #[clap(flatten)]
    options: Box<Options>,

    /// Add more optional modules.
    #[clap(raw = true)]
    modules: Vec<String>,
}

impl<Options: Args + FromArgMatches> Module<Options> {
    /// Add this as an optional module. Return the next optional module args.
    fn add(self, name: &str, options: &mut Option<Options>) -> Result<Vec<String>, clap::Error> {
        if options.is_some() {
            return Err(clap::Error::raw(
                ErrorKind::TooManyValues,
                format!("optional module {name} can only be started once"),
            ));
        }
        *options = Some(*self.options);
        Ok(self.modules)
    }
}

#[derive(Clone, Debug, Parser)]
enum SequencerModule {
    Api(Module<api::Options>),
    CommitmentTask(Module<CommitmentTaskOptions>),
}

#[derive(Clone, Debug, Default)]
pub struct Modules {
    pub api: Option<api::Options>,
    pub commitment_task: Option<CommitmentTaskOptions>,
}
