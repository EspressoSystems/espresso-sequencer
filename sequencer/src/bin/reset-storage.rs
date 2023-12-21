use clap::Parser;
use hotshot_query_service::data_source::VersionedDataSource;
use sequencer::api::{
    data_source::{DataSourceOptions, SequencerDataSource},
    options,
};

/// Reset the persistent storage of a sequencer.
///
/// This will remove all the persistent storage of a sequencer node, effectively resetting it to
/// its genesis state. Do not run this program while the sequencer is running.
#[derive(Clone, Debug, Parser)]
enum Options {
    /// Reset file system storage.
    Fs(options::Fs),
    /// Reset SQL storage.
    Sql(options::Sql),
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let opt = Options::parse();
    match opt {
        Options::Fs(opt) => reset_storage(opt).await,
        Options::Sql(opt) => reset_storage(opt).await,
    }
}

async fn reset_storage<O: DataSourceOptions>(mut opt: O) -> anyhow::Result<()> {
    opt.reset_storage();
    let mut ds = O::DataSource::create(opt).await?;
    ds.commit().await?;
    Ok(())
}
