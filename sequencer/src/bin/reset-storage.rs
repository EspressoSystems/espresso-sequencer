use clap::{Parser, Subcommand};
use espresso_types::v0::traits::PersistenceOptions;
use sequencer::{
    api::{data_source::SequencerDataSource, sql},
    persistence,
};
use sequencer_utils::logging;
/// Reset the persistent storage of a sequencer.
///
/// This will remove all the persistent storage of a sequencer node, effectively resetting it to
/// its genesis state. Do not run this program while the sequencer is running.
#[derive(Clone, Debug, Parser)]
struct Options {
    #[clap(flatten)]
    logging: logging::Config,

    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    /// Reset SQL storage.
    Sql(Box<persistence::sql::Options>),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Options::parse();
    opt.logging.init();

    match opt.command {
        Command::Sql(opt) => {
            tracing::warn!("resetting SQL storage {opt:?}");

            sql::DataSource::create(*opt.clone(), Default::default(), true).await?;
            opt.reset().await
        }
    }
}
