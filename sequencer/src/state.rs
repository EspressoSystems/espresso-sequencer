use core::fmt::Debug;
use std::{sync::Arc, time::Duration};

use anyhow::{bail, ensure, Context};
use async_std::stream::StreamExt;
use espresso_types::{
    v0_3::ChainConfig, BlockMerkleTree, Delta, FeeAccount, FeeMerkleTree, ValidatedState,
};
use futures::future::Future;
use hotshot::traits::ValidatedState as HotShotState;
use hotshot_query_service::{
    availability::{AvailabilityDataSource, LeafQueryData},
    data_source::{Transaction, VersionedDataSource},
    merklized_state::{MerklizedStateHeightPersistence, UpdateStateData},
    status::StatusDataSource,
    types::HeightIndexed,
};
use jf_merkle_tree::{LookupResult, MerkleTreeScheme, ToTraversalPath, UniversalMerkleTreeScheme};

use crate::{
    api::data_source::CatchupDataSource, catchup::SqlStateCatchup,
    persistence::ChainConfigPersistence, NodeState, SeqTypes,
};

async fn compute_state_update(
    state: &ValidatedState,
    instance: &NodeState,
    parent_leaf: &LeafQueryData<SeqTypes>,
    proposed_leaf: &LeafQueryData<SeqTypes>,
) -> anyhow::Result<(ValidatedState, Delta)> {
    let proposed_leaf = proposed_leaf.leaf();
    let parent_leaf = parent_leaf.leaf();
    let header = proposed_leaf.block_header();

    // Check internal consistency.
    let parent_header = parent_leaf.block_header();
    ensure!(
        state.chain_config.commit() == parent_header.chain_config().commit(),
        "internal error! in-memory chain config {:?} does not match parent header {:?}",
        state.chain_config,
        parent_header.chain_config(),
    );
    ensure!(
        state.block_merkle_tree.commitment() == parent_header.block_merkle_tree_root(),
        "internal error! in-memory block tree {:?} does not match parent header {:?}",
        state.block_merkle_tree.commitment(),
        parent_header.block_merkle_tree_root()
    );
    ensure!(
        state.fee_merkle_tree.commitment() == parent_header.fee_merkle_tree_root(),
        "internal error! in-memory fee tree {:?} does not match parent header {:?}",
        state.fee_merkle_tree.commitment(),
        parent_header.fee_merkle_tree_root()
    );

    state
        .apply_header(instance, parent_leaf, header, header.version())
        .await
}

async fn store_state_update(
    tx: &mut impl SequencerStateUpdate,
    block_number: u64,
    state: &ValidatedState,
    delta: Delta,
) -> anyhow::Result<()> {
    let ValidatedState {
        fee_merkle_tree,
        block_merkle_tree,
        ..
    } = state;
    let Delta { fees_delta } = delta;

    // Insert fee merkle tree nodes
    for delta in fees_delta {
        let proof = match fee_merkle_tree.universal_lookup(delta) {
            LookupResult::Ok(_, proof) => proof,
            LookupResult::NotFound(proof) => proof,
            LookupResult::NotInMemory => bail!("missing merkle path for fee account {delta}"),
        };
        let path: Vec<usize> =
            <FeeAccount as ToTraversalPath<{ FeeMerkleTree::ARITY }>>::to_traversal_path(
                &delta,
                fee_merkle_tree.height(),
            );

        UpdateStateData::<SeqTypes, _, { FeeMerkleTree::ARITY }>::insert_merkle_nodes(
            tx,
            proof,
            path,
            block_number,
        )
        .await
        .context("failed to store fee merkle nodes")?;
    }

    // Insert block merkle tree nodes
    let (_, proof) = block_merkle_tree
        .lookup(block_number - 1)
        .expect_ok()
        .context("getting blocks frontier")?;
    let path = <u64 as ToTraversalPath<{ BlockMerkleTree::ARITY }>>::to_traversal_path(
        &(block_number - 1),
        block_merkle_tree.height(),
    );

    {
        UpdateStateData::<SeqTypes, _, { BlockMerkleTree::ARITY }>::insert_merkle_nodes(
            tx,
            proof,
            path,
            block_number,
        )
        .await
        .context("failed to store block merkle nodes")?;
    }

    UpdateStateData::<SeqTypes, _, { BlockMerkleTree::ARITY }>::set_last_state_height(
        tx,
        block_number as usize,
    )
    .await
    .context("setting state height")?;
    Ok(())
}

#[tracing::instrument(
    skip_all,
    fields(
        node_id = instance.node_id,
        view = ?parent_leaf.leaf().view_number(),
        height = parent_leaf.height(),
    ),
)]
async fn update_state_storage<T>(
    parent_state: &ValidatedState,
    storage: &Arc<T>,
    instance: &NodeState,
    parent_leaf: &LeafQueryData<SeqTypes>,
    proposed_leaf: &LeafQueryData<SeqTypes>,
) -> anyhow::Result<ValidatedState>
where
    T: SequencerStateDataSource,
    for<'a> T::Transaction<'a>: SequencerStateUpdate,
{
    let parent_chain_config = parent_state.chain_config;

    let (state, delta) = compute_state_update(parent_state, instance, parent_leaf, proposed_leaf)
        .await
        .context("computing state update")?;

    let mut tx = storage
        .write()
        .await
        .context("opening transaction for state update")?;
    store_state_update(&mut tx, proposed_leaf.height(), &state, delta).await?;

    if parent_chain_config != state.chain_config {
        let cf = state
            .chain_config
            .resolve()
            .context("failed to resolve to chain config")?;

        tx.insert_chain_config(cf).await?;
    }

    tx.commit().await?;
    Ok(state)
}

async fn store_genesis_state<T>(
    mut tx: T,
    chain_config: ChainConfig,
    state: &ValidatedState,
) -> anyhow::Result<()>
where
    T: SequencerStateUpdate,
{
    ensure!(
        state.block_merkle_tree.num_leaves() == 0,
        "genesis state with non-empty block tree is unsupported"
    );

    // Insert fee merkle tree nodes
    for (account, _) in state.fee_merkle_tree.iter() {
        let proof = match state.fee_merkle_tree.universal_lookup(account) {
            LookupResult::Ok(_, proof) => proof,
            LookupResult::NotFound(proof) => proof,
            LookupResult::NotInMemory => bail!("missing merkle path for fee account {account}"),
        };
        let path: Vec<usize> =
            <FeeAccount as ToTraversalPath<{ FeeMerkleTree::ARITY }>>::to_traversal_path(
                account,
                state.fee_merkle_tree.height(),
            );

        UpdateStateData::<SeqTypes, _, { FeeMerkleTree::ARITY }>::insert_merkle_nodes(
            &mut tx, proof, path, 0,
        )
        .await
        .context("failed to store fee merkle nodes")?;
    }

    tx.insert_chain_config(chain_config).await?;

    tx.commit().await?;
    Ok(())
}

pub(crate) async fn update_state_storage_loop<T>(
    storage: Arc<T>,
    instance: impl Future<Output = NodeState>,
) -> anyhow::Result<()>
where
    T: SequencerStateDataSource,
    for<'a> T::Transaction<'a>: SequencerStateUpdate,
{
    let mut instance = instance.await;
    instance.peers = Arc::new(SqlStateCatchup::new(storage.clone(), Default::default()));

    // get last saved merklized state
    let (last_height, parent_leaf, mut leaves) = {
        let last_height = storage.get_last_state_height().await?;
        let current_height = storage.block_height().await?;
        tracing::info!(last_height, current_height, "updating state storage");

        let parent_leaf = storage.get_leaf(last_height).await;
        let leaves = storage.subscribe_leaves(last_height + 1).await;
        (last_height, parent_leaf, leaves)
    };
    // resolve the parent leaf future _after_ dropping our lock on the state, in case it is not
    // ready yet and another task needs a mutable lock on the state to produce the parent leaf.
    let mut parent_leaf = parent_leaf.await;
    let mut parent_state = ValidatedState::from_header(parent_leaf.header());

    if last_height == 0 {
        // If the last height is 0, we need to insert the genesis state, since this state is
        // never the result of a state update and thus is not inserted in the loop below.
        tracing::info!("storing genesis merklized state");
        let tx = storage
            .write()
            .await
            .context("starting transaction for genesis state")?;
        store_genesis_state(tx, instance.chain_config, &instance.genesis_state)
            .await
            .context("storing genesis state")?;
    }

    while let Some(leaf) = leaves.next().await {
        loop {
            match update_state_storage(&parent_state, &storage, &instance, &parent_leaf, &leaf)
                .await
            {
                Ok(state) => {
                    parent_leaf = leaf;
                    parent_state = state;
                    break;
                }
                Err(err) => {
                    tracing::error!(height = leaf.height(), "failed to updated state: {err:#}");
                    // If we fail, delay for a second and retry.
                    async_std::task::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    Ok(())
}

pub(crate) trait SequencerStateDataSource:
    'static
    + Debug
    + AvailabilityDataSource<SeqTypes>
    + StatusDataSource
    + VersionedDataSource
    + CatchupDataSource
    + MerklizedStateHeightPersistence
{
}

impl<T> SequencerStateDataSource for T where
    T: 'static
        + Debug
        + AvailabilityDataSource<SeqTypes>
        + StatusDataSource
        + VersionedDataSource
        + CatchupDataSource
        + MerklizedStateHeightPersistence
{
}

pub(crate) trait SequencerStateUpdate:
    Transaction
    + UpdateStateData<SeqTypes, FeeMerkleTree, { FeeMerkleTree::ARITY }>
    + UpdateStateData<SeqTypes, BlockMerkleTree, { BlockMerkleTree::ARITY }>
    + ChainConfigPersistence
{
}

impl<T> SequencerStateUpdate for T where
    T: Transaction
        + UpdateStateData<SeqTypes, FeeMerkleTree, { FeeMerkleTree::ARITY }>
        + UpdateStateData<SeqTypes, BlockMerkleTree, { BlockMerkleTree::ARITY }>
        + ChainConfigPersistence
{
}
