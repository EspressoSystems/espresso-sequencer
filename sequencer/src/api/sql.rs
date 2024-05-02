use super::data_source::{Provider, SequencerDataSource};
use crate::{persistence::sql::Options, SeqTypes};
use async_trait::async_trait;
use hotshot_query_service::data_source::sql::{Config, SqlDataSource};

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
}

#[cfg(test)]
mod impl_testable_data_source {
    use super::*;
    use crate::api::{self, data_source::testing::TestableSequencerDataSource};
    use hotshot_query_service::data_source::storage::sql::testing::TmpDb;

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

        async fn create_storage() -> Self::Storage {
            TmpDb::init().await
        }

        fn persistence_options(storage: &Self::Storage) -> Self::Options {
            tmp_options(storage)
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
