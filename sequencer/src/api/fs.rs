use super::data_source::{Provider, SequencerDataSource};
use crate::{persistence::fs::Options, SeqTypes};
use async_trait::async_trait;
use hotshot_query_service::{
    data_source::{storage::FileSystemStorage, FileSystemDataSource},
    merklized_state::MerklizedState,
    merklized_state::{MerklizedStateDataSource, Snapshot, UpdateStateData},
    QueryError, QueryResult,
};
use jf_primitives::merkle_tree::prelude::MerklePath;
use std::path::Path;

pub type DataSource = FileSystemDataSource<SeqTypes, Provider>;

#[async_trait]
impl SequencerDataSource for DataSource {
    type Options = Options;

    async fn create(opt: Self::Options, provider: Provider, reset: bool) -> anyhow::Result<Self> {
        let path = Path::new(&opt.path);
        let data_source = {
            if reset {
                FileSystemDataSource::create(path, provider).await?
            } else {
                FileSystemDataSource::open(path, provider).await?
            }
        };

        Ok(data_source)
    }
}

// TODO : Move these blanket impls into query service
#[async_trait]
impl<State: MerklizedState<SeqTypes>> UpdateStateData<SeqTypes, State>
    for FileSystemStorage<SeqTypes>
{
    async fn insert_merkle_nodes(
        &mut self,
        _path: MerklePath<State::Entry, State::Key, State::T>,
        _traversal_path: Vec<usize>,
        _block_number: u64,
    ) -> QueryResult<()> {
        Ok(())
    }
}

// TODO : Move these blanket impls into query service
#[async_trait]
impl<State> MerklizedStateDataSource<SeqTypes, State> for FileSystemStorage<SeqTypes>
where
    State: MerklizedState<SeqTypes> + 'static,
{
    /// Retrieves a Merkle path from the database
    async fn get_path(
        &self,
        _snapshot: Snapshot<SeqTypes, State>,
        _key: State::Key,
    ) -> QueryResult<MerklePath<State::Entry, State::Key, State::T>> {
        Err(QueryError::Missing)
    }
}

#[cfg(test)]
mod impl_testable_data_source {
    use super::*;
    use crate::{
        api::{self, data_source::testing::TestableSequencerDataSource},
        persistence::{fs, PersistenceOptions},
    };
    use tempfile::TempDir;

    #[async_trait]
    impl TestableSequencerDataSource for DataSource {
        type Storage = TempDir;
        type Persistence = fs::Persistence;

        async fn create_storage() -> Self::Storage {
            TempDir::new().unwrap()
        }

        async fn connect(storage: &Self::Storage) -> Self::Persistence {
            Options {
                path: storage.path().into(),
            }
            .create()
            .await
            .unwrap()
        }

        fn options(storage: &Self::Storage, opt: api::Options) -> api::Options {
            opt.query_fs(
                Default::default(),
                Options {
                    path: storage.path().into(),
                },
            )
        }
    }
}

#[cfg(test)]
mod generic_tests {
    use super::super::api_tests;
    use super::DataSource;

    // For some reason this is the only way to import the macro defined in another module of this
    // crate.
    use crate::*;

    instantiate_api_tests!(DataSource);
}
