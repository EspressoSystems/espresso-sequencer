use clap::{Parser, Subcommand};
use hotshot_query_service::data_source::VersionedDataSource;
use sequencer::{
    api::data_source::{DataSourceOptions, SequencerDataSource},
    persistence,
};
use sequencer_utils::logging;

/// Options for resetting persistent storage.
///
/// This will remove all the persistent storage of a sequencer node or marketplace solver, effectively resetting it to
/// its genesis state. Do not run this program while the sequencer or solver is running.
#[derive(Clone, Debug, Parser)]
struct Options {
    #[clap(flatten)]
    logging: logging::Config,

    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    /// Contains subcommands for resetting sequencer storage.
    #[command(subcommand)]
    Sequencer(SequencerStorage),
    /// resetting marketplace solver storage.
    Solver(marketplace_solver::DatabaseOptions),
}

#[derive(Clone, Debug, Subcommand)]
enum SequencerStorage {
    /// Reset file system storage.
    Fs(persistence::fs::Options),
    /// Reset SQL storage.
    Sql(Box<persistence::sql::Options>),
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let opt = Options::parse();
    opt.logging.init();

    match opt.command {
        Command::Sequencer(query_resetter) => match query_resetter {
            SequencerStorage::Fs(opt) => {
                tracing::warn!("resetting sequencer file system storage {opt:?}");
                reset_storage(opt).await
            }
            SequencerStorage::Sql(opt) => {
                tracing::warn!("resetting sequencer SQL storage {opt:?}");
                reset_storage(*opt).await
            }
        },

        Command::Solver(opt) => {
            tracing::warn!("resetting solver SQL storage {opt:?}");
            let opts = opt.reset();
            opts.connect().await?;

            Ok(())
        }
    }
}

async fn reset_storage<O: DataSourceOptions>(opt: O) -> anyhow::Result<()> {
    // Reset query service storage.
    let mut ds = O::DataSource::create(opt.clone(), Default::default(), true).await?;
    ds.commit().await?;

    // Reset consensus storage.
    opt.reset().await?;

    Ok(())
}
