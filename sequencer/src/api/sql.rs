use anyhow::{bail, ensure, Context};
use async_std::sync::Arc;
use async_trait::async_trait;
use committable::{Commitment, Committable};
use espresso_types::{
    v0_3::ChainConfig, BlockMerkleTree, FeeAccount, FeeMerkleTree, Leaf, NodeState, ValidatedState,
};
use hotshot::traits::ValidatedState as _;
use hotshot_query_service::{
    availability::BlockId,
    data_source::{
        sql::{postgres::types::ToSql, Config, SqlDataSource, Transaction},
        storage::{AvailabilityStorage, SqlStorage},
        VersionedDataSource,
    },
    merklized_state::{MerklizedStateDataSource, Snapshot},
    node::NodeDataSource,
    Resolvable,
};
use hotshot_types::{
    data::{QuorumProposal, ViewNumber},
    message::Proposal,
    traits::node_implementation::ConsensusTime,
};
use jf_merkle_tree::{
    prelude::MerkleNode, ForgetableMerkleTreeScheme, ForgetableUniversalMerkleTreeScheme,
    MerkleTreeScheme,
};
use std::collections::VecDeque;

use super::{
    data_source::{CatchupDataSource, Provider, SequencerDataSource},
    BlocksFrontier,
};
use crate::{
    catchup::SqlStateCatchup,
    persistence::{
        sql::{sql_param, Options},
        ChainConfigPersistence,
    },
    state::compute_state_update,
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
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        let tx = self.read().await.context(format!(
            "opening transaction to fetch account {accounts:?}; height {height}"
        ))?;

        let block_height = NodeDataSource::<SeqTypes>::block_height(&*tx)
            .await
            .context("getting block height")? as u64;
        ensure!(
            block_height > 0,
            "cannot get accounts for height {height}: no blocks available"
        );

        // Check if we have the desired state snapshot. If so, we can load the desired accounts
        // directly.
        if height < block_height {
            load_accounts(&tx, height, accounts).await
        } else {
            // If we do not have the exact snapshot we need, we can try going back to the last
            // snapshot we _do_ have and replaying subsequent blocks to compute the desired state.
            let state = reconstruct_state(instance, &tx, block_height - 1, view, accounts).await?;
            Ok(state.fee_merkle_tree)
        }
    }

    async fn get_frontier(&self, height: u64, view: ViewNumber) -> anyhow::Result<BlocksFrontier> {
        let tx = self.read().await.context(format!(
            "opening transaction to fetch frontier at height {height}"
        ))?;
        (&*tx).get_frontier(height, view).await
    }

    async fn get_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        let tx = self.read().await.context(format!(
            "opening transaction to fetch chain config {commitment}"
        ))?;
        (&*tx).get_chain_config(commitment).await
    }
}

impl<'a> CatchupDataSource for &Transaction<'a> {
    async fn get_accounts(
        &self,
        _instance: &NodeState,
        height: u64,
        _view: ViewNumber,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        load_accounts(self, height, accounts).await
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
        commitment: Commitment<ChainConfig>,
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

impl CatchupDataSource for DataSource {
    async fn get_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<FeeMerkleTree> {
        self.as_ref()
            .get_accounts(instance, height, view, accounts)
            .await
    }

    async fn get_frontier(&self, height: u64, view: ViewNumber) -> anyhow::Result<BlocksFrontier> {
        self.as_ref().get_frontier(height, view).await
    }

    async fn get_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        self.as_ref().get_chain_config(commitment).await
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

async fn load_accounts<'a>(
    tx: &Transaction<'a>,
    height: u64,
    accounts: &[FeeAccount],
) -> anyhow::Result<FeeMerkleTree> {
    let header = tx
        .get_header(BlockId::<SeqTypes>::from(height as usize))
        .await
        .context(format!("header {height} not available"))?;

    let mut snapshot = FeeMerkleTree::from_commitment(header.fee_merkle_tree_root());
    for account in accounts {
        let proof = tx
            .get_path(
                Snapshot::<SeqTypes, FeeMerkleTree, { FeeMerkleTree::ARITY }>::Index(
                    header.height(),
                ),
                *account,
            )
            .await
            .context(format!(
                "fetching account {account}; height {}",
                header.height()
            ))?;
        match proof.proof.first().context(format!(
            "empty proof for account {account}; height {}",
            header.height()
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

#[tracing::instrument(skip(instance, tx))]
async fn reconstruct_state<'a>(
    instance: &NodeState,
    tx: &Transaction<'a>,
    from_height: u64,
    to_view: ViewNumber,
    accounts: &[FeeAccount],
) -> anyhow::Result<ValidatedState> {
    tracing::info!("attempting to reconstruct fee state");
    let from_leaf = tx
        .get_leaf((from_height as usize).into())
        .await
        .context(format!("leaf {from_height} not available"))?;
    let from_leaf = from_leaf.leaf();
    ensure!(
        from_leaf.view_number() < to_view,
        "state reconstruction: starting state {:?} must be before ending state {to_view:?}",
        from_leaf.view_number(),
    );

    // Get the sequence of headers we will be applying to compute the latest state.
    let mut leaves = VecDeque::new();
    let leaf = get_leaf_from_proposal(tx, "view = $1", &(to_view.u64() as i64))
        .await
        .context(format!(
            "unable to reconstruct state because leaf {to_view:?} is not available"
        ))?;
    let mut parent = leaf.parent_commitment();
    tracing::debug!(?leaf, ?parent, view = ?to_view, "have required leaf");
    leaves.push_front(leaf);
    while parent != Committable::commit(from_leaf) {
        let leaf = get_leaf_from_proposal(tx, "leaf_hash = $1", &parent.to_string())
            .await
            .context(format!(
                "unable to reconstruct state because leaf {parent} is not available"
            ))?;
        parent = leaf.parent_commitment();
        tracing::debug!(?leaf, ?parent, "have required leaf");
        leaves.push_front(leaf);
    }

    // Get the initial state.
    let mut parent = from_leaf;
    let mut state = ValidatedState::from_header(parent.block_header());
    // Pre-load the state with the accounts we care about to ensure they will be present in the
    // final state.
    state.fee_merkle_tree = load_accounts(tx, from_height, accounts)
        .await
        .context("unable to reconstruct state because accounts are not available at origin")?;
    ensure!(
        state.fee_merkle_tree.commitment() == parent.block_header().fee_merkle_tree_root(),
        "loaded fee state does not match parent header"
    );

    // For catchup that is required during the following state replay, use the local database via
    // this transaction.
    let peers = SqlStateCatchup::new(Arc::new(tx), Default::default());

    // Apply subsequent headers to compute the later state.
    for proposal in &leaves {
        state = compute_state_update(&state, instance, &peers, parent, proposal)
            .await
            .context(format!(
                "unable to reconstruct state because state update {} failed",
                proposal.height(),
            ))?
            .0;
        parent = proposal;
    }

    tracing::info!(from_height, ?to_view, "successfully reconstructed state");
    Ok(state)
}

async fn get_leaf_from_proposal<'a>(
    tx: &Transaction<'a>,
    where_clause: &str,
    param: &(dyn ToSql + Sync),
) -> anyhow::Result<Leaf> {
    let row = tx
        .query_one(
            &format!("SELECT data FROM quorum_proposals WHERE {where_clause} LIMIT 1",),
            [param],
        )
        .await?;
    let data: Vec<u8> = row.try_get("data")?;
    let proposal: Proposal<SeqTypes, QuorumProposal<SeqTypes>> = bincode::deserialize(&data)?;
    Ok(Leaf::from_quorum_proposal(&proposal.data))
}

#[cfg(any(test, feature = "testing"))]
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
