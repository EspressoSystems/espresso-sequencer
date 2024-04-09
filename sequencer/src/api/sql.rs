use super::data_source::{Provider, SequencerDataSource};
use crate::{persistence::sql::Options, SeqTypes};
use anyhow::Context;
use async_trait::async_trait;
use hotshot_query_service::{
    data_source::sql::{Config, SqlDataSource},
    merklized_state::{MerklizedState, UpdateStateData},
};
use jf_primitives::merkle_tree::prelude::MerklePath;

pub type DataSource = SqlDataSource<SeqTypes, Provider>;

#[async_trait]
impl SequencerDataSource for DataSource {
    type Options = Options;

    async fn create(opt: Self::Options, provider: Provider, reset: bool) -> anyhow::Result<Self> {
        let mut cfg = Config::try_from(opt)?;
        if reset {
            cfg = cfg.reset_schema();
        }

        Ok(cfg.connect(provider).await?)
    }

    async fn store_state<S: MerklizedState<SeqTypes, A>, const A: usize>(
        &mut self,
        path: MerklePath<S::Entry, S::Key, S::T>,
        traversal_path: Vec<usize>,
        block_number: u64,
    ) -> anyhow::Result<()> {
        <DataSource as UpdateStateData<SeqTypes, S, A>>::insert_merkle_nodes(
            self,
            path,
            traversal_path,
            block_number,
        )
        .await
        .context("failed to insert merkle nodes! ")
    }
}

#[cfg(test)]
mod impl_testable_data_source {
    use super::*;
    use crate::{
        api::{self, data_source::testing::TestableSequencerDataSource},
        persistence::PersistenceOptions,
    };
    use hotshot_query_service::data_source::storage::sql::{testing::TmpDb, SqlStorage};

    fn tmp_options(db: &TmpDb) -> Options {
        Options {
            port: Some(db.port()),
            host: Some(db.host()),
            user: Some("postgres".into()),
            password: Some("password".into()),
            ..Default::default()
        }
    }

    #[async_trait]
    impl TestableSequencerDataSource for DataSource {
        type Storage = TmpDb;
        type Persistence = SqlStorage;

        async fn create_storage() -> Self::Storage {
            TmpDb::init().await
        }

        async fn connect(storage: &Self::Storage) -> Self::Persistence {
            tmp_options(storage).create().await.unwrap()
        }

        fn options(storage: &Self::Storage, opt: api::Options) -> api::Options {
            opt.query_sql(Default::default(), tmp_options(storage))
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
