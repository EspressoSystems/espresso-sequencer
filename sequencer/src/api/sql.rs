use anyhow::{bail, Context};
use async_trait::async_trait;
use committable::Commitment;
use espresso_types::{v0_3::ChainConfig, BlockMerkleTree, FeeAccount, FeeMerkleTree};
use hotshot_query_service::{
    availability::BlockId,
    data_source::{
        sql::{Config, SqlDataSource, Transaction},
        storage::{AvailabilityStorage, SqlStorage},
        VersionedDataSource,
    },
    merklized_state::{MerklizedStateDataSource, Snapshot},
    Resolvable,
};
use hotshot_types::data::ViewNumber;
use jf_merkle_tree::{
    prelude::MerkleNode, ForgetableMerkleTreeScheme, ForgetableUniversalMerkleTreeScheme,
    MerkleTreeScheme,
};

use super::{
    data_source::{CatchupDataSource, Provider, SequencerDataSource},
    BlocksFrontier,
};
use crate::{
    persistence::{
        sql::{sql_param, Options},
        ChainConfigPersistence,
    },
    SeqTypes,
};

pub type DataSource = SqlDataSource<SeqTypes, Provider>;

#[async_trait]
impl SequencerDataSource for DataSource {
    type Options = Options;

    async fn create(opt: Self::Options, provider: Provider, reset: bool) -> anyhow::Result<Self> {
        let fetch_limit = opt.fetch_rate_limit;
        let active_fetch_delay = opt.active_fetch_delay;
        let chunk_fetch_delay = opt.chunk_fetch_delay;
        let mut cfg = Config::try_from(opt)?;

        if reset {
            cfg = cfg.reset_schema();
        }

        let mut builder = cfg.builder(provider).await?;

        if let Some(limit) = fetch_limit {
            builder = builder.with_rate_limit(limit);
        }
        if let Some(delay) = active_fetch_delay {
            builder = builder.with_active_fetch_delay(delay);
        }
        if let Some(delay) = chunk_fetch_delay {
            builder = builder.with_chunk_fetch_delay(delay);
        }

        builder.build().await
    }
}

impl CatchupDataSource for SqlStorage {
    async fn get_accounts(
        &self,
        height: u64,
        _view: ViewNumber,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        let tx = self.read().await.context(format!(
            "opening transaction to fetch account {accounts:?}; height {height}"
        ))?;
        let header = tx
            .get_header(BlockId::<SeqTypes>::from(height as usize))
            .await
            .context(format!("loading header {height}"))?;
        let mut snapshot = FeeMerkleTree::from_commitment(header.fee_merkle_tree_root());
        for account in accounts {
            let proof = tx
                .get_path(
                    Snapshot::<SeqTypes, FeeMerkleTree, { FeeMerkleTree::ARITY }>::Index(height),
                    *account,
                )
                .await
                .context(format!("fetching account {account}; height {height}"))?;
            match proof.proof.first().context(format!(
                "empty proof for account {account}; height {height}"
            ))? {
                MerkleNode::Leaf { pos, elem, .. } => {
                    snapshot.remember(*pos, *elem, proof)?;
                }
                MerkleNode::Empty => {
                    snapshot.non_membership_remember(*account, proof)?;
                }
                _ => {
                    bail!("Invalid proof");
                }
            }
        }

        Ok(snapshot)
    }

    async fn get_frontier(&self, height: u64, _view: ViewNumber) -> anyhow::Result<BlocksFrontier> {
        self.read()
            .await
            .context(format!(
                "opening transaction to fetch frontier at height {height}"
            ))?
            .get_path(
                Snapshot::<SeqTypes, BlockMerkleTree, { BlockMerkleTree::ARITY }>::Index(height),
                height - 1,
            )
            .await
            .context(format!("fetching frontier at height {height}"))
    }

    async fn get_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        let query = self
            .read()
            .await
            .context(format!(
                "opening transaction to fetch chain config {commitment}"
            ))?
            .query_one(
                "SELECT * from chain_config where commitment = $1",
                [&commitment.to_string()],
            )
            .await?;

        let data: Vec<u8> = query.try_get("data")?;

        bincode::deserialize(&data[..]).context("failed to deserialize")
    }
}

impl CatchupDataSource for DataSource {
    async fn get_accounts(
        &self,
        height: u64,
        view: ViewNumber,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        self.as_ref().get_accounts(height, view, accounts).await
    }

    async fn get_frontier(&self, height: u64, view: ViewNumber) -> anyhow::Result<BlocksFrontier> {
        self.as_ref().get_frontier(height, view).await
    }
}

#[async_trait]
impl<'a> ChainConfigPersistence for Transaction<'a> {
    async fn insert_chain_config(&mut self, chain_config: ChainConfig) -> anyhow::Result<()> {
        let commitment = chain_config.commitment();
        let data = bincode::serialize(&chain_config)?;
        self.upsert(
            "chain_config",
            ["commitment", "data"],
            ["commitment"],
            [[sql_param(&(commitment.to_string())), sql_param(&data)]],
        )
        .await
        .map_err(Into::into)
    }
}

#[cfg(test)]
mod impl_testable_data_source {

    use hotshot_query_service::data_source::storage::sql::testing::TmpDb;

    use super::*;
    use crate::api::{self, data_source::testing::TestableSequencerDataSource};

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
    use super::{super::api_tests, DataSource};
    // For some reason this is the only way to import the macro defined in another module of this
    // crate.
    use crate::*;

    instantiate_api_tests!(DataSource);
}
