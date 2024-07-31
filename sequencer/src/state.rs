use core::fmt::Debug;
use std::{sync::Arc, time::Duration};

use anyhow::{bail, ensure, Context};
use async_std::{stream::StreamExt, sync::RwLock};
use espresso_types::{
    v0_3::ChainConfig, BlockMerkleTree, Delta, FeeAccount, FeeMerkleTree, ValidatedState,
};
use futures::future::Future;
use hotshot::traits::ValidatedState as HotShotState;
use hotshot_query_service::{
    availability::{AvailabilityDataSource, LeafQueryData},
    data_source::VersionedDataSource,
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
    storage: &mut impl SequencerStateDataSource,
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
            storage,
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
            storage,
            proof,
            path,
            block_number,
        )
        .await
        .context("failed to store block merkle nodes")?;
    }

    storage
        .set_last_state_height(block_number as usize)
        .await
        .context("setting state height")?;
    storage.commit().await.context("committing state update")?;
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
async fn update_state_storage(
    parent_state: &ValidatedState,
    storage: &Arc<RwLock<impl SequencerStateDataSource>>,
    instance: &NodeState,
    parent_leaf: &LeafQueryData<SeqTypes>,
    proposed_leaf: &LeafQueryData<SeqTypes>,
) -> anyhow::Result<ValidatedState> {
    let parent_chain_config = parent_state.chain_config;

    let (state, delta) = compute_state_update(parent_state, instance, parent_leaf, proposed_leaf)
        .await
        .context("computing state update")?;

    let mut storage = storage.write().await;
    if let Err(err) = store_state_update(&mut *storage, proposed_leaf.height(), &state, delta).await
    {
        storage.revert().await;
        return Err(err);
    }

    if parent_chain_config != state.chain_config {
        let cf = state
            .chain_config
            .resolve()
            .context("failed to resolve to chain config")?;

        storage.insert_chain_config(cf).await?
    }

    Ok(state)
}

async fn store_genesis_state(
    storage: &mut impl SequencerStateDataSource,
    chain_config: ChainConfig,
    state: &ValidatedState,
) -> anyhow::Result<()> {
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
            storage, proof, path, 0,
        )
        .await
        .context("failed to store fee merkle nodes")?;
    }

    storage.insert_chain_config(chain_config).await?;

    storage.commit().await?;
    Ok(())
}

pub(crate) async fn update_state_storage_loop(
    storage: Arc<RwLock<impl SequencerStateDataSource>>,
    instance: impl Future<Output = NodeState>,
) -> anyhow::Result<()> {
    let mut instance = instance.await;
    instance.peers = Arc::new(SqlStateCatchup::new(storage.clone(), Default::default()));

    // get last saved merklized state
    let (last_height, parent_leaf, mut leaves) = {
        let state = storage.upgradable_read().await;

        let last_height = state.get_last_state_height().await?;
        let current_height = state.block_height().await?;
        tracing::info!(last_height, current_height, "updating state storage");

        let parent_leaf = state.get_leaf(last_height).await;
        let leaves = state.subscribe_leaves(last_height + 1).await;
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
        let mut storage = storage.write().await;
        if let Err(err) = store_genesis_state(
            &mut *storage,
            instance.chain_config,
            &instance.genesis_state,
        )
        .await
        {
            tracing::error!("failed to store genesis state: {err:#}");
            storage.revert().await;
            return Err(err);
        }
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
    + UpdateStateData<SeqTypes, FeeMerkleTree, { FeeMerkleTree::ARITY }>
    + UpdateStateData<SeqTypes, BlockMerkleTree, { BlockMerkleTree::ARITY }>
    + MerklizedStateHeightPersistence
    + ChainConfigPersistence
{
}

impl<T> SequencerStateDataSource for T where
    T: 'static
        + Debug
        + AvailabilityDataSource<SeqTypes>
        + StatusDataSource
        + VersionedDataSource
        + CatchupDataSource
        + UpdateStateData<SeqTypes, FeeMerkleTree, { FeeMerkleTree::ARITY }>
        + UpdateStateData<SeqTypes, BlockMerkleTree, { BlockMerkleTree::ARITY }>
        + MerklizedStateHeightPersistence
        + ChainConfigPersistence
{
}

#[cfg(test)]
mod test {
    use espresso_types::{
        v0_3::IterableFeeInfo, validate_proposal, BlockSize, FeeAccount, FeeAccountProof,
        FeeAmount, FeeError, FeeInfo, FeeMerkleProof, Leaf, ProposalValidationError,
    };
    use ethers::{abi::Address, types::U256};
    use hotshot_types::{
        traits::signature_key::BuilderSignatureKey,
        vid::{vid_scheme, VidSchemeType},
    };
    use jf_merkle_tree::{ForgetableMerkleTreeScheme, MerkleTreeError};
    use jf_vid::VidScheme;
    use sequencer_utils::{ser::FromStringOrInteger, test_utils::setup_test};

    use super::*;

    #[test]
    fn test_fee_proofs() {
        setup_test();

        let mut tree = ValidatedState::default().fee_merkle_tree;
        let account1 = Address::random();
        let account2 = Address::default();
        tracing::info!(%account1, %account2);

        let balance1 = U256::from(100);
        tree.update(FeeAccount(account1), FeeAmount(balance1))
            .unwrap();

        // Membership proof.
        let (proof1, balance) = FeeAccountProof::prove(&tree, account1).unwrap();
        tracing::info!(?proof1, %balance);
        assert_eq!(balance, balance1);
        assert!(matches!(proof1.proof, FeeMerkleProof::Presence(_)));
        assert_eq!(proof1.verify(&tree.commitment()).unwrap(), balance1);

        // Non-membership proof.
        let (proof2, balance) = FeeAccountProof::prove(&tree, account2).unwrap();
        tracing::info!(?proof2, %balance);
        assert_eq!(balance, 0.into());
        assert!(matches!(proof2.proof, FeeMerkleProof::Absence(_)));
        assert_eq!(proof2.verify(&tree.commitment()).unwrap(), 0.into());

        // Test forget/remember. We cannot generate proofs in a completely sparse tree:
        let mut tree = FeeMerkleTree::from_commitment(tree.commitment());
        assert!(FeeAccountProof::prove(&tree, account1).is_none());
        assert!(FeeAccountProof::prove(&tree, account2).is_none());
        // After remembering the proofs, we can generate proofs again:
        proof1.remember(&mut tree).unwrap();
        proof2.remember(&mut tree).unwrap();
        FeeAccountProof::prove(&tree, account1).unwrap();
        FeeAccountProof::prove(&tree, account2).unwrap();
    }

    #[async_std::test]
    async fn test_validation_max_block_size() {
        setup_test();

        const MAX_BLOCK_SIZE: usize = 10;
        let payload = [0; 2 * MAX_BLOCK_SIZE];
        let vid_common = vid_scheme(1).disperse(payload).unwrap().common;

        let state = ValidatedState::default();
        let instance = NodeState::mock().with_chain_config(ChainConfig {
            max_block_size: (MAX_BLOCK_SIZE as u64).into(),
            base_fee: 0.into(),
            ..Default::default()
        });
        let parent = Leaf::genesis(&instance.genesis_state, &instance).await;
        let header = parent.block_header();

        // Validation fails because the proposed block exceeds the maximum block size.
        let err = validate_proposal(&state, instance.chain_config, &parent, header, &vid_common)
            .unwrap_err();

        tracing::info!(%err, "task failed successfully");
        assert_eq!(
            ProposalValidationError::MaxBlockSizeExceeded {
                max_block_size: instance.chain_config.max_block_size,
                block_size: BlockSize::from_integer(
                    VidSchemeType::get_payload_byte_len(&vid_common).into()
                )
                .unwrap()
            },
            err
        );
    }

    #[async_std::test]
    async fn test_validation_base_fee() {
        setup_test();

        let max_block_size = 10;
        let payload = [0; 1];
        let vid_common = vid_scheme(1).disperse(payload).unwrap().common;

        let state = ValidatedState::default();
        let instance = NodeState::mock().with_chain_config(ChainConfig {
            base_fee: 1000.into(), // High base fee
            max_block_size: max_block_size.into(),
            ..Default::default()
        });
        let parent = Leaf::genesis(&instance.genesis_state, &instance).await;
        let header = parent.block_header();

        // Validation fails because the genesis fee (0) is too low.
        let err = validate_proposal(&state, instance.chain_config, &parent, header, &vid_common)
            .unwrap_err();

        tracing::info!(%err, "task failed successfully");
        assert_eq!(
            ProposalValidationError::InsufficientFee {
                max_block_size: instance.chain_config.max_block_size,
                base_fee: instance.chain_config.base_fee,
                proposed_fee: header.fee_info().amount().unwrap()
            },
            err
        );
    }

    #[test]
    fn test_charge_fee() {
        setup_test();

        let src = FeeAccount::generated_from_seed_indexed([0; 32], 0).0;
        let dst = FeeAccount::generated_from_seed_indexed([0; 32], 1).0;
        let amt = FeeAmount::from(1);

        let fee_info = FeeInfo::new(src, amt);

        let new_state = || {
            let mut state = ValidatedState::default();
            state.prefund_account(src, amt);
            state
        };

        tracing::info!("test successful fee");
        let mut state = new_state();
        state.charge_fee(fee_info, dst).unwrap();
        assert_eq!(state.balance(src), Some(0.into()));
        assert_eq!(state.balance(dst), Some(amt));

        tracing::info!("test insufficient balance");
        let err = state.charge_fee(fee_info, dst).unwrap_err();
        assert_eq!(state.balance(src), Some(0.into()));
        assert_eq!(state.balance(dst), Some(amt));
        assert_eq!(
            FeeError::InsufficientFunds {
                balance: None,
                amount: amt
            },
            err
        );

        tracing::info!("test src not in memory");
        let mut state = new_state();
        state.fee_merkle_tree.forget(src).expect_ok().unwrap();
        assert_eq!(
            FeeError::MerkleTreeError(MerkleTreeError::ForgottenLeaf),
            state.charge_fee(fee_info, dst).unwrap_err()
        );

        tracing::info!("test dst not in memory");
        let mut state = new_state();
        state.prefund_account(dst, amt);
        state.fee_merkle_tree.forget(dst).expect_ok().unwrap();
        assert_eq!(
            FeeError::MerkleTreeError(MerkleTreeError::ForgottenLeaf),
            state.charge_fee(fee_info, dst).unwrap_err()
        );
    }

    #[test]
    fn test_fee_amount_serde_json_as_decimal() {
        let amt = FeeAmount::from(123);
        let serialized = serde_json::to_string(&amt).unwrap();

        // The value is serialized as a decimal string.
        assert_eq!(serialized, "\"123\"");

        // Deserialization produces the original value
        let deserialized: FeeAmount = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, amt);
    }

    #[test]
    fn test_fee_amount_from_units() {
        for (unit, multiplier) in [
            ("wei", 1),
            ("gwei", 1_000_000_000),
            ("eth", 1_000_000_000_000_000_000),
        ] {
            let amt: FeeAmount = serde_json::from_str(&format!("\"1 {unit}\"")).unwrap();
            assert_eq!(amt, multiplier.into());
        }
    }

    #[test]
    fn test_fee_amount_serde_json_from_hex() {
        // For backwards compatibility, fee amounts can also be deserialized from a 0x-prefixed hex
        // string.
        let amt: FeeAmount = serde_json::from_str("\"0x123\"").unwrap();
        assert_eq!(amt, FeeAmount::from(0x123));
    }

    #[test]
    fn test_fee_amount_serde_json_from_number() {
        // For convenience, fee amounts can also be deserialized from a JSON number.
        let amt: FeeAmount = serde_json::from_str("123").unwrap();
        assert_eq!(amt, FeeAmount::from(123));
    }

    #[test]
    fn test_fee_amount_serde_bincode_unchanged() {
        // For non-human-readable formats, FeeAmount just serializes as the underlying U256.
        let n = U256::from(123);
        let amt = FeeAmount(n);
        assert_eq!(
            bincode::serialize(&n).unwrap(),
            bincode::serialize(&amt).unwrap(),
        );
    }
}
