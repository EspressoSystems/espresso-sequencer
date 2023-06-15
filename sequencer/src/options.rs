use crate::{api, hotshot_commitment::CommitmentTaskOptions};
use clap::Parser;
use std::{iter::once, str::FromStr};
use url::Url;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// Unique identifier for this instance of the sequencer network.
    #[clap(long, env = "ESPRESSO_SEQUENCER_CHAIN_ID", default_value = "0")]
    pub chain_id: u16,

    /// URL of the HotShot CDN.
    #[clap(short, long, env = "ESPRESSO_SEQUENCER_CDN_URL")]
    pub cdn_url: Url,

    // Run the commitment task module.
    //
    // This module will stream committed block commitments from a HotShot query service and send
    // those commitments, with justifying QCs, to the L1 sequencer contract for verification.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_COMMITMENT_TASK",
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = ""
    )]
    pub commitment_task: Option<CommitmentTaskOptions>,

    /// Serve the sequencer API.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_API",
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = ""
    )]
    pub api: Option<api::Options>,
}

macro_rules! sub_command_from_str {
    ($t:ty) => {
        impl FromStr for $t {
            type Err = clap::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let args = shell_words::split(s)
                    .map_err(|err| clap::Error::raw(clap::error::ErrorKind::InvalidValue, err))?;

                // The first argument (the program name) is not used except for help. The remaining
                // arguments will be parsed as a `$t` command
                Self::try_parse_from(once("sequencer").chain(args.iter().map(|s| s.as_str())))
            }
        }
    };
}

sub_command_from_str!(CommitmentTaskOptions);
sub_command_from_str!(api::Options);
