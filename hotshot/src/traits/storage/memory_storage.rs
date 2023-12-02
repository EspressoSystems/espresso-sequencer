//! [`HashMap`](std::collections::HashMap) and [`Vec`] based implementation of the storage trait
//!
//! This module provides a non-persisting, dummy adapter for the [`Storage`] trait
use async_lock::RwLock;
use async_trait::async_trait;
use hotshot_types::traits::{
    node_implementation::NodeType,
    storage::{
        Result, Storage, StorageError, StorageState, StoredView, TestableStorage, ViewEntry,
    },
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

/// Internal state for a [`MemoryStorage`]
struct MemoryStorageInternal<TYPES: NodeType> {
    /// The views that have been stored
    stored: BTreeMap<TYPES::Time, StoredView<TYPES>>,
    /// The views that have failed
    failed: BTreeSet<TYPES::Time>,
}

/// In memory, ephemeral, storage for a [`HotShot`](crate::HotShot) instance
#[derive(Clone)]
pub struct MemoryStorage<TYPES: NodeType> {
    /// The inner state of this [`MemoryStorage`]
    inner: Arc<RwLock<MemoryStorageInternal<TYPES>>>,
}

impl<TYPES: NodeType> MemoryStorage<TYPES> {
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
impl<TYPES: NodeType> TestableStorage<TYPES> for MemoryStorage<TYPES> {
    fn construct_tmp_storage() -> Result<Self> {
        Ok(Self::empty())
    }

    async fn get_full_state(&self) -> StorageState<TYPES> {
        let inner = self.inner.read().await;
        StorageState {
            stored: inner.stored.clone(),
            failed: inner.failed.clone(),
        }
    }
}

#[async_trait]
impl<TYPES: NodeType> Storage<TYPES> for MemoryStorage<TYPES> {
    async fn append(&self, views: Vec<ViewEntry<TYPES>>) -> Result {
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

    async fn get_anchored_view(&self) -> Result<StoredView<TYPES>> {
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
    use crate::traits::election::static_committee::{GeneralStaticCommittee, StaticElectionConfig};

    use super::*;
    use commit::Committable;
    use hotshot_signature_key::bn254::BLSPubKey;
    use hotshot_types::traits::signature_key::SignatureKey;
    use hotshot_types::{
        block_impl::{VIDBlockHeader, VIDBlockPayload, VIDTransaction},
        data::{fake_commitment, Leaf, ViewNumber},
        simple_certificate::QuorumCertificate,
        traits::{node_implementation::NodeType, state::dummy::DummyState, state::ConsensusTime},
    };
    use std::{fmt::Debug, hash::Hash, marker::PhantomData};
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
        type BlockHeader = VIDBlockHeader;
        type BlockPayload = VIDBlockPayload;
        type SignatureKey = BLSPubKey;
        type Transaction = VIDTransaction;
        type ElectionConfigType = StaticElectionConfig;
        type StateType = DummyState;
        type Membership = GeneralStaticCommittee<DummyTypes, BLSPubKey>;
    }

    fn random_stored_view(view_number: <DummyTypes as NodeType>::Time) -> StoredView<DummyTypes> {
        let payload = VIDBlockPayload::genesis();
        let header = VIDBlockHeader {
            block_number: 0,
            payload_commitment: payload.payload_commitment,
        };
        let dummy_leaf_commit = fake_commitment::<Leaf<DummyTypes>>();
        let data = hotshot_types::simple_vote::QuorumData {
            leaf_commit: dummy_leaf_commit,
        };
        let commit = data.commit();
        StoredView::from_qc_block_and_state(
            QuorumCertificate {
                is_genesis: view_number == <DummyTypes as NodeType>::Time::genesis(),
                data,
                vote_commitment: commit,
                signatures: None,
                view_number,
                _pd: PhantomData,
            },
            header,
            Some(payload),
            dummy_leaf_commit,
            Vec::new(),
            <<DummyTypes as NodeType>::SignatureKey as SignatureKey>::genesis_proposer_pk(),
        )
    }

    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    #[instrument]
    async fn memory_storage() {
        let storage = MemoryStorage::construct_tmp_storage().unwrap();
        let genesis = random_stored_view(<DummyTypes as NodeType>::Time::genesis());
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
