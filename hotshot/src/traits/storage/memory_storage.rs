//! [`HashMap`](std::collections::HashMap) and [`Vec`] based implementation of the storage trait
//!
//! This module provides a non-persisting, dummy adapter for the [`Storage`] trait
use async_lock::RwLock;
use async_trait::async_trait;
use hotshot_types::{
    data::LeafType,
    traits::{
        node_implementation::NodeType,
        storage::{
            Result, Storage, StorageError, StorageState, StoredView, TestableStorage, ViewEntry,
        },
    },
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

/// Internal state for a [`MemoryStorage`]
struct MemoryStorageInternal<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> {
    /// The views that have been stored
    stored: BTreeMap<TYPES::Time, StoredView<TYPES, LEAF>>,
    /// The views that have failed
    failed: BTreeSet<TYPES::Time>,
}

/// In memory, ephemeral, storage for a [`HotShot`](crate::HotShot) instance
#[derive(Clone)]
pub struct MemoryStorage<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> {
    /// The inner state of this [`MemoryStorage`]
    inner: Arc<RwLock<MemoryStorageInternal<TYPES, LEAF>>>,
}

impl<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> MemoryStorage<TYPES, LEAF> {
    /// Create a new instance of the memory storage with the given block and state
    #[must_use]
    pub fn empty() -> Self {
        let inner = MemoryStorageInternal {
            stored: BTreeMap::new(),
            failed: BTreeSet::new(),
        };
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> TestableStorage<TYPES, LEAF>
    for MemoryStorage<TYPES, LEAF>
{
    fn construct_tmp_storage() -> Result<Self> {
        Ok(Self::empty())
    }

    async fn get_full_state(&self) -> StorageState<TYPES, LEAF> {
        let inner = self.inner.read().await;
        StorageState {
            stored: inner.stored.clone(),
            failed: inner.failed.clone(),
        }
    }
}

#[async_trait]
impl<TYPES: NodeType, LEAF: LeafType<NodeType = TYPES>> Storage<TYPES, LEAF>
    for MemoryStorage<TYPES, LEAF>
{
    async fn append(&self, views: Vec<ViewEntry<TYPES, LEAF>>) -> Result {
        let mut inner = self.inner.write().await;
        for view in views {
            match view {
                ViewEntry::Failed(num) => {
                    inner.failed.insert(num);
                }
                ViewEntry::Success(view) => {
                    inner.stored.insert(view.view_number, view);
                }
            }
        }
        Ok(())
    }

    async fn cleanup_storage_up_to_view(&self, view: TYPES::Time) -> Result<usize> {
        let mut inner = self.inner.write().await;

        // .split_off will return everything after the given key, including the key.
        let stored_after = inner.stored.split_off(&view);
        // .split_off will return the map we want to keep stored, so we need to swap them
        let old_stored = std::mem::replace(&mut inner.stored, stored_after);

        // same for the BTreeSet
        let failed_after = inner.failed.split_off(&view);
        let old_failed = std::mem::replace(&mut inner.failed, failed_after);

        Ok(old_stored.len() + old_failed.len())
    }

    async fn get_anchored_view(&self) -> Result<StoredView<TYPES, LEAF>> {
        let inner = self.inner.read().await;
        let last = inner
            .stored
            .values()
            .next_back()
            .ok_or(StorageError::NoGenesisView)?;
        Ok(last.clone())
    }

    async fn commit(&self) -> Result {
        Ok(()) // do nothing
    }
}

#[cfg(test)]
mod test {
    use crate::traits::election::static_committee::{StaticElectionConfig, StaticVoteToken};

    use super::*;
    use hotshot_signature_key::bn254::BN254Pub;
    use hotshot_types::{
        certificate::{AssembledSignature, QuorumCertificate},
        data::{fake_commitment, genesis_proposer_id, ValidatingLeaf, ViewNumber},
        traits::{
            block_contents::dummy::{DummyBlock, DummyState},
            node_implementation::NodeType,
            state::ConsensusTime,
            Block,
        },
    };
    use std::{fmt::Debug, hash::Hash};
    use tracing::instrument;

    #[derive(
        Copy,
        Clone,
        Debug,
        Default,
        Hash,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        serde::Serialize,
        serde::Deserialize,
    )]
    struct DummyTypes;

    impl NodeType for DummyTypes {
        type Time = ViewNumber;
        type BlockType = DummyBlock;
        type SignatureKey = BN254Pub;
        type VoteTokenType = StaticVoteToken<Self::SignatureKey>;
        type Transaction = <DummyBlock as Block>::Transaction;
        type ElectionConfigType = StaticElectionConfig;
        type StateType = DummyState;
    }

    #[instrument(skip(rng))]
    fn random_stored_view(
        rng: &mut dyn rand::RngCore,
        view_number: <DummyTypes as NodeType>::Time,
    ) -> StoredView<DummyTypes, ValidatingLeaf<DummyTypes>> {
        // TODO is it okay to be using genesis here?
        let _dummy_block_commit = fake_commitment::<DummyBlock>();
        let dummy_leaf_commit = fake_commitment::<ValidatingLeaf<DummyTypes>>();
        StoredView::from_qc_block_and_state(
            QuorumCertificate {
                // block_commitment: dummy_block_commit,
                is_genesis: view_number == <DummyTypes as NodeType>::Time::genesis(),
                leaf_commitment: dummy_leaf_commit,
                signatures: AssembledSignature::Genesis(),
                view_number,
            },
            DummyBlock::random(rng),
            DummyState::random(rng),
            rng.next_u64(),
            dummy_leaf_commit,
            Vec::new(),
            genesis_proposer_id(),
        )
    }

    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    #[instrument]
    async fn memory_storage() {
        let mut rng = rand::thread_rng();
        let storage = MemoryStorage::construct_tmp_storage().unwrap();
        let genesis = random_stored_view(&mut rng, <DummyTypes as NodeType>::Time::genesis());
        storage
            .append_single_view(genesis.clone())
            .await
            .expect("Could not append block");
        assert_eq!(storage.get_anchored_view().await.unwrap(), genesis);
        storage
            .cleanup_storage_up_to_view(genesis.view_number)
            .await
            .unwrap();
        assert_eq!(storage.get_anchored_view().await.unwrap(), genesis);
        storage
            .cleanup_storage_up_to_view(genesis.view_number + 1)
            .await
            .unwrap();
        assert!(storage.get_anchored_view().await.is_err());
    }
}
