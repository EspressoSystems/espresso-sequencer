use anyhow::{bail, ensure, Context};
use async_trait::async_trait;
use committable::{Commitment, Committable};
use espresso_types::{
    get_l1_deposits,
    v0_99::{ChainConfig, IterableFeeInfo},
    BlockMerkleTree, FeeAccount, FeeMerkleTree, Leaf2, NodeState, ValidatedState,
};
use hotshot::traits::ValidatedState as _;
use hotshot_query_service::{
    availability::LeafId,
    data_source::{
        sql::{Config, SqlDataSource, Transaction},
        storage::{
            sql::{query_as, Db, TransactionMode, Write},
            AvailabilityStorage, MerklizedStateStorage, NodeStorage, SqlStorage,
        },
        VersionedDataSource,
    },
    merklized_state::Snapshot,
    Resolvable,
};
use hotshot_types::{
    data::{QuorumProposal2, ViewNumber},
    message::Proposal,
    traits::node_implementation::ConsensusTime,
};
use jf_merkle_tree::{
    prelude::MerkleNode, ForgetableMerkleTreeScheme, ForgetableUniversalMerkleTreeScheme,
    LookupResult, MerkleTreeScheme,
};
use sqlx::{Encode, Type};
use std::collections::{HashSet, VecDeque};

use super::{
    data_source::{Provider, SequencerDataSource},
    BlocksFrontier,
};
use crate::{
    catchup::{CatchupStorage, NullStateCatchup},
    persistence::{sql::Options, ChainConfigPersistence},
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

impl CatchupStorage for SqlStorage {
    async fn get_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<(FeeMerkleTree, Leaf2)> {
        let mut tx = self.read().await.context(format!(
            "opening transaction to fetch account {accounts:?}; height {height}"
        ))?;

        let block_height = NodeStorage::<SeqTypes>::block_height(&mut tx)
            .await
            .context("getting block height")? as u64;
        ensure!(
            block_height > 0,
            "cannot get accounts for height {height}: no blocks available"
        );

        // Check if we have the desired state snapshot. If so, we can load the desired accounts
        // directly.
        if height < block_height {
            load_accounts(&mut tx, height, accounts).await
        } else {
            // If we do not have the exact snapshot we need, we can try going back to the last
            // snapshot we _do_ have and replaying subsequent blocks to compute the desired state.
            let (state, leaf) =
                reconstruct_state(instance, &mut tx, block_height - 1, view, accounts).await?;
            Ok((state.fee_merkle_tree, leaf))
        }
    }

    async fn get_frontier(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
    ) -> anyhow::Result<BlocksFrontier> {
        let mut tx = self.read().await.context(format!(
            "opening transaction to fetch frontier at height {height}"
        ))?;

        let block_height = NodeStorage::<SeqTypes>::block_height(&mut tx)
            .await
            .context("getting block height")? as u64;
        ensure!(
            block_height > 0,
            "cannot get frontier for height {height}: no blocks available"
        );

        // Check if we have the desired state snapshot. If so, we can load the desired frontier
        // directly.
        if height < block_height {
            load_frontier(&mut tx, height).await
        } else {
            // If we do not have the exact snapshot we need, we can try going back to the last
            // snapshot we _do_ have and replaying subsequent blocks to compute the desired state.
            let (state, _) =
                reconstruct_state(instance, &mut tx, block_height - 1, view, &[]).await?;
            match state.block_merkle_tree.lookup(height - 1) {
                LookupResult::Ok(_, proof) => Ok(proof),
                _ => {
                    bail!("state snapshot {view:?},{height} was found but does not contain frontier at height {}; this should not be possible", height - 1);
                }
            }
        }
    }

    async fn get_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        let mut tx = self.read().await.context(format!(
            "opening transaction to fetch chain config {commitment}"
        ))?;
        load_chain_config(&mut tx, commitment).await
    }
}

impl CatchupStorage for DataSource {
    async fn get_accounts(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
        accounts: &[FeeAccount],
    ) -> anyhow::Result<(FeeMerkleTree, Leaf2)> {
        self.as_ref()
            .get_accounts(instance, height, view, accounts)
            .await
    }

    async fn get_frontier(
        &self,
        instance: &NodeState,
        height: u64,
        view: ViewNumber,
    ) -> anyhow::Result<BlocksFrontier> {
        self.as_ref().get_frontier(instance, height, view).await
    }

    async fn get_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig> {
        self.as_ref().get_chain_config(commitment).await
    }
}

#[async_trait]
impl ChainConfigPersistence for Transaction<Write> {
    async fn insert_chain_config(&mut self, chain_config: ChainConfig) -> anyhow::Result<()> {
        let commitment = chain_config.commitment();
        let data = bincode::serialize(&chain_config)?;
        self.upsert(
            "chain_config",
            ["commitment", "data"],
            ["commitment"],
            [(commitment.to_string(), data)],
        )
        .await
        .map_err(Into::into)
    }
}

async fn load_frontier<Mode: TransactionMode>(
    tx: &mut Transaction<Mode>,
    height: u64,
) -> anyhow::Result<BlocksFrontier> {
    tx.get_path(
        Snapshot::<SeqTypes, BlockMerkleTree, { BlockMerkleTree::ARITY }>::Index(height),
        height - 1,
    )
    .await
    .context(format!("fetching frontier at height {height}"))
}

async fn load_accounts<Mode: TransactionMode>(
    tx: &mut Transaction<Mode>,
    height: u64,
    accounts: &[FeeAccount],
) -> anyhow::Result<(FeeMerkleTree, Leaf2)> {
    let leaf = tx
        .get_leaf(LeafId::<SeqTypes>::from(height as usize))
        .await
        .context(format!("leaf {height} not available"))?;
    let header = leaf.header();

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

    Ok((snapshot, leaf.leaf().clone()))
}

async fn load_chain_config<Mode: TransactionMode>(
    tx: &mut Transaction<Mode>,
    commitment: Commitment<ChainConfig>,
) -> anyhow::Result<ChainConfig> {
    let (data,) = query_as::<(Vec<u8>,)>("SELECT data from chain_config where commitment = $1")
        .bind(commitment.to_string())
        .fetch_one(tx.as_mut())
        .await
        .unwrap();

    bincode::deserialize(&data[..]).context("failed to deserialize")
}

#[tracing::instrument(skip(instance, tx))]
async fn reconstruct_state<Mode: TransactionMode>(
    instance: &NodeState,
    tx: &mut Transaction<Mode>,
    from_height: u64,
    to_view: ViewNumber,
    accounts: &[FeeAccount],
) -> anyhow::Result<(ValidatedState, Leaf2)> {
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
    let to_leaf = get_leaf_from_proposal(tx, "view = $1", &(to_view.u64() as i64))
        .await
        .context(format!(
            "unable to reconstruct state because leaf {to_view:?} is not available"
        ))?;
    let mut parent = to_leaf.parent_commitment();
    tracing::debug!(?to_leaf, ?parent, view = ?to_view, "have required leaf");
    leaves.push_front(to_leaf.clone());
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
    let mut accounts = accounts.iter().copied().collect::<HashSet<_>>();
    // Add in all the accounts we will need to replay any of the headers, to ensure that we don't
    // need to do catchup recursively.
    let (catchup, dependencies) = header_dependencies(tx, instance, parent, &leaves).await?;
    accounts.extend(dependencies);
    let accounts = accounts.into_iter().collect::<Vec<_>>();
    state.fee_merkle_tree = load_accounts(tx, from_height, &accounts)
        .await
        .context("unable to reconstruct state because accounts are not available at origin")?
        .0;
    ensure!(
        state.fee_merkle_tree.commitment() == parent.block_header().fee_merkle_tree_root(),
        "loaded fee state does not match parent header"
    );

    // We need the blocks frontier as well, to apply the STF.
    let frontier = load_frontier(tx, from_height)
        .await
        .context("unable to reconstruct state because frontier is not available at origin")?;
    match frontier
        .proof
        .first()
        .context("empty proof for frontier at origin")?
    {
        MerkleNode::Leaf { pos, elem, .. } => state
            .block_merkle_tree
            .remember(*pos, *elem, frontier)
            .context("failed to remember frontier")?,
        _ => bail!("invalid frontier proof"),
    }

    // Apply subsequent headers to compute the later state.
    for proposal in &leaves {
        state = compute_state_update(&state, instance, &catchup, parent, proposal)
            .await
            .context(format!(
                "unable to reconstruct state because state update {} failed",
                proposal.height(),
            ))?
            .0;
        parent = proposal;
    }

    tracing::info!(from_height, ?to_view, "successfully reconstructed state");
    Ok((state, to_leaf))
}

/// Get the dependencies needed to apply the STF to the given list of headers.
///
/// Returns
/// * A state catchup implementation seeded with all the chain configs required to apply the headers
///   in `leaves`
/// * The set of accounts that must be preloaded to apply these headers
async fn header_dependencies<Mode: TransactionMode>(
    tx: &mut Transaction<Mode>,
    instance: &NodeState,
    mut parent: &Leaf2,
    leaves: impl IntoIterator<Item = &Leaf2>,
) -> anyhow::Result<(NullStateCatchup, HashSet<FeeAccount>)> {
    let mut catchup = NullStateCatchup::default();
    let mut accounts = HashSet::default();

    for proposal in leaves {
        let header = proposal.block_header();
        let height = header.height();
        let view = proposal.view_number();
        tracing::debug!(height, ?view, "fetching dependencies for proposal");

        let header_cf = header.chain_config();
        let chain_config = if header_cf.commit() == instance.chain_config.commit() {
            instance.chain_config
        } else {
            match header_cf.resolve() {
                Some(cf) => cf,
                None => {
                    tracing::info!(
                        height,
                        ?view,
                        commit = %header_cf.commit(),
                        "chain config not available, attempting to load from storage",
                    );
                    let cf = load_chain_config(tx, header_cf.commit())
                        .await
                        .context(format!(
                            "loading chain config {} for header {},{:?}",
                            header_cf.commit(),
                            header.height(),
                            proposal.view_number()
                        ))?;

                    // If we had to fetch a chain config now, store it in the catchup implementation
                    // so the STF will be able to look it up later.
                    catchup.add_chain_config(cf);
                    cf
                }
            }
        };

        accounts.insert(chain_config.fee_recipient);
        accounts.extend(
            get_l1_deposits(instance, header, parent, chain_config.fee_contract)
                .await
                .into_iter()
                .map(|fee| fee.account()),
        );
        accounts.extend(header.fee_info().accounts());
        parent = proposal;
    }
    Ok((catchup, accounts))
}

async fn get_leaf_from_proposal<Mode, P>(
    tx: &mut Transaction<Mode>,
    where_clause: &str,
    param: P,
) -> anyhow::Result<Leaf2>
where
    P: Type<Db> + for<'q> Encode<'q, Db>,
{
    let (data,) = query_as::<(Vec<u8>,)>(&format!(
        "SELECT data FROM quorum_proposals WHERE {where_clause} LIMIT 1",
    ))
    .bind(param)
    .fetch_one(tx.as_mut())
    .await?;
    let proposal: Proposal<SeqTypes, QuorumProposal2<SeqTypes>> = bincode::deserialize(&data)?;
    Ok(Leaf2::from_quorum_proposal(&proposal.data))
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
