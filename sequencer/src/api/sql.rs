use super::{
    data_source::{CatchupDataSource, Provider, SequencerDataSource},
    AccountQueryData, BlocksFrontier,
};
use crate::{
    persistence::{
        sql::{sql_param, Options},
        ChainConfigPersistence,
    },
    state::{BlockMerkleTree, FeeAccountProof, FeeMerkleTree},
    ChainConfig, SeqTypes,
};
use anyhow::{bail, Context};
use async_trait::async_trait;
use committable::Commitment;
use ethers::prelude::Address;
use hotshot_query_service::{
    data_source::{
        sql::{Config, Query, SqlDataSource},
        storage::SqlStorage,
        VersionedDataSource,
    },
    merklized_state::{MerklizedStateDataSource, Snapshot},
    Resolvable,
};
use hotshot_types::data::ViewNumber;
use jf_merkle_tree::{prelude::MerkleNode, MerkleTreeScheme};

pub type DataSource = SqlDataSource<SeqTypes, Provider>;

#[async_trait]
impl SequencerDataSource for DataSource {
    type Options = Options;

    async fn create(opt: Self::Options, provider: Provider, reset: bool) -> anyhow::Result<Self> {
        let fetch_limit = opt.fetch_rate_limit;
        let mut cfg = Config::try_from(opt)?;

        if reset {
            cfg = cfg.reset_schema();
        }

        let mut builder = cfg.builder(provider).await?;

        if let Some(limit) = fetch_limit {
            builder = builder.with_rate_limit(limit);
        }

        builder.build().await
    }
}

impl CatchupDataSource for SqlStorage {
    async fn get_account(
        &self,
        height: u64,
        _view: ViewNumber,
        account: Address,
    ) -> anyhow::Result<AccountQueryData> {
        let proof = self
            .get_path(
                Snapshot::<SeqTypes, FeeMerkleTree, { FeeMerkleTree::ARITY }>::Index(height),
                account.into(),
            )
            .await
            .context(format!("fetching account {account}; height {height}"))?;

        match proof.proof.first().context(format!(
            "empty proof for account {account}; height {height}"
        ))? {
            MerkleNode::Leaf { pos, elem, .. } => Ok(AccountQueryData {
                balance: (*elem).into(),
                proof: FeeAccountProof::presence(*pos, proof),
            }),

            MerkleNode::Empty => Ok(AccountQueryData {
                balance: 0_u64.into(),
                proof: FeeAccountProof::absence(account.into(), proof),
            }),
            _ => {
                bail!("Invalid proof");
            }
        }
    }

    async fn get_frontier(&self, height: u64, _view: ViewNumber) -> anyhow::Result<BlocksFrontier> {
        self.get_path(
            Snapshot::<SeqTypes, BlockMerkleTree, { BlockMerkleTree::ARITY }>::Index(height),
            height - 1,
        )
        .await
        .context(format!("fetching frontier at height {height}"))
    }

    async fn get_chain_config(
        &self,
        commitment: committable::Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        self.load_chain_config(commitment).await
    }
}

impl CatchupDataSource for DataSource {
    async fn get_account(
        &self,
        height: u64,
        view: ViewNumber,
        account: Address,
    ) -> anyhow::Result<AccountQueryData> {
        (*self.storage().await)
            .get_account(height, view, account)
            .await
    }

    async fn get_frontier(&self, height: u64, view: ViewNumber) -> anyhow::Result<BlocksFrontier> {
        self.storage().await.get_frontier(height, view).await
    }
}

#[async_trait]
impl ChainConfigPersistence for SqlStorage {
    async fn insert_chain_config(&mut self, chain_config: ChainConfig) -> anyhow::Result<()> {
        let commitment = chain_config.commitment();
        let data = bincode::serialize(&chain_config)?;

        let mut transaction = self.transaction().await?;

        transaction
            .upsert(
                "chain_config",
                ["commitment", "data"],
                ["commitment"],
                [[sql_param(&(commitment.to_string())), sql_param(&data)]],
            )
            .await?;

        self.commit().await?;

        Ok(())
    }

    async fn load_chain_config(
        &self,
        commitment: committable::Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        let query = self
            .query_one(
                "SELECT * from chain_config where commitment = $1",
                [&commitment.to_string()],
            )
            .await?;

        let data: Vec<u8> = query.try_get("data")?;

        bincode::deserialize(&data[..]).context("failed to deserialize")
    }
}

#[async_trait]
impl ChainConfigPersistence for DataSource {
    async fn insert_chain_config(&mut self, chain_config: ChainConfig) -> anyhow::Result<()> {
        (*self.storage_mut().await)
            .insert_chain_config(chain_config)
            .await
    }

    async fn load_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        self.storage().await.load_chain_config(commitment).await
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
