use sequencer::{api::data_source::SequencerDataSource, persistence};

use clap::Subcommand;
use espresso_types::v0::traits::PersistenceOptions;
use sequencer::api::sql;
/// Options for resetting persistent storage.
///
/// This will remove all the persistent storage of a sequencer node or marketplace solver, effectively resetting it to
/// its genesis state. Do not run this program while the sequencer or solver is running.
#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Contains subcommands for resetting sequencer storage.
    #[command(subcommand)]
    Sequencer(SequencerStorage),
    /// resetting marketplace solver storage.
    Solver(marketplace_solver::DatabaseOptions),
}

#[derive(Clone, Debug, Subcommand)]
pub enum SequencerStorage {
    /// Reset SQL storage.
    Sql(Box<persistence::sql::Options>),
}

pub async fn run(opt: Commands) -> anyhow::Result<()> {
    match opt {
        Commands::Sequencer(query_resetter) => match query_resetter {
            SequencerStorage::Sql(opt) => {
                tracing::warn!("resetting SQL storage {opt:?}");

                sql::DataSource::create(*opt.clone(), Default::default(), true).await?;
                opt.reset().await
            }
        },

        Commands::Solver(opt) => {
            tracing::warn!("resetting solver SQL storage {opt:?}");
            let opts = opt.reset();
            opts.connect().await?;

            Ok(())
        }
    }
}
