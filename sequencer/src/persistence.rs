//! Sequencer node persistence.
//!
//! This module implements the persistence required for a sequencer node to rejoin the network and
//! resume participating in consensus, in the event that its process crashes or is killed and loses
//! all in-memory state.
//!
//! This is distinct from the query service persistent storage found in the `api` module, which is
//! an extension that node operators can opt into. This module defines the minimum level of
//! persistence which is _required_ to run a node.

use async_trait::async_trait;
use espresso_types::v0_3::ChainConfig;
use hotshot_query_service::data_source::fetching;

use crate::SeqTypes;

pub mod fs;
pub mod no_storage;
pub mod sql;

#[async_trait]
pub trait ChainConfigPersistence: Sized + Send + Sync {
    async fn insert_chain_config(&mut self, chain_config: ChainConfig) -> anyhow::Result<()>;
}

#[async_trait]
impl<'a, T> ChainConfigPersistence for fetching::Transaction<'a, SeqTypes, T>
where
    T: ChainConfigPersistence,
{
    async fn insert_chain_config(&mut self, chain_config: ChainConfig) -> anyhow::Result<()> {
        self.as_mut().insert_chain_config(chain_config).await
    }
}

#[cfg(any(test, feature = "testing"))]
mod testing {

    use espresso_types::v0::traits::SequencerPersistence;

    use super::*;
    #[allow(dead_code)]
    #[async_trait]
    pub trait TestablePersistence: SequencerPersistence {
        type Storage;

        async fn tmp_storage() -> Self::Storage;
        async fn connect(storage: &Self::Storage) -> Self;
    }
}

#[cfg(test)]
#[espresso_macros::generic_tests]
mod persistence_tests {
    use std::collections::BTreeMap;

    use committable::Committable;
    use espresso_types::{Leaf, NodeState, PubKey, SeqTypes, ValidatedState};
    use hotshot::types::{BLSPubKey, SignatureKey};
    use hotshot_example_types::node_types::TestVersions;
    use hotshot_types::{
        data::{DaProposal, QuorumProposal, VidDisperseShare, ViewNumber},
        event::HotShotAction,
        message::Proposal,
        simple_certificate::QuorumCertificate,
        simple_vote::QuorumData,
        traits::{node_implementation::ConsensusTime, EncodeBytes},
        vid::vid_scheme,
    };
    use jf_vid::VidScheme;
    use sequencer_utils::test_utils::setup_test;
    use std::marker::PhantomData;
    use testing::TestablePersistence;

    use super::*;

    #[async_std::test]
    pub async fn test_anchor_leaf<P: TestablePersistence>() {
        setup_test();

        let tmp = P::tmp_storage().await;
        let storage = P::connect(&tmp).await;

        // Initially, there is no saved leaf.
        assert_eq!(storage.load_anchor_leaf().await.unwrap(), None);

        // Store a leaf.
        let leaf1 = Leaf::genesis(&ValidatedState::default(), &NodeState::mock()).await;
        let qc1 = QuorumCertificate::genesis::<TestVersions>(
            &ValidatedState::default(),
            &NodeState::mock(),
        )
        .await;
        storage.save_anchor_leaf(&leaf1, &qc1).await.unwrap();
        assert_eq!(
            storage.load_anchor_leaf().await.unwrap().unwrap(),
            (leaf1.clone(), qc1.clone())
        );

        // Store a newer leaf, make sure storage gets updated.
        let mut leaf2 = leaf1.clone();
        *leaf2.block_header_mut().height_mut() += 1;
        let qc2 = QuorumCertificate::new(
            qc1.data.clone(),
            QuorumData {
                leaf_commit: <Leaf as Committable>::commit(&leaf2),
            }
            .commit(),
            qc1.view_number,
            qc1.signatures.clone(),
            PhantomData,
        );
        storage.save_anchor_leaf(&leaf2, &qc2).await.unwrap();
        assert_eq!(
            storage.load_anchor_leaf().await.unwrap().unwrap(),
            (leaf2.clone(), qc2.clone())
        );

        // Store an old leaf, make sure storage is unchanged.
        storage.save_anchor_leaf(&leaf1, &qc1).await.unwrap();
        assert_eq!(
            storage.load_anchor_leaf().await.unwrap().unwrap(),
            (leaf2, qc2)
        );
    }

    #[async_std::test]
    pub async fn test_voted_view<P: TestablePersistence>() {
        setup_test();

        let tmp = P::tmp_storage().await;
        let storage = P::connect(&tmp).await;

        // Initially, there is no saved view.
        assert_eq!(storage.load_latest_acted_view().await.unwrap(), None);

        // Store a view.
        let view1 = ViewNumber::genesis();
        storage
            .record_action(view1, HotShotAction::Vote)
            .await
            .unwrap();
        assert_eq!(
            storage.load_latest_acted_view().await.unwrap().unwrap(),
            view1
        );

        // Store a newer view, make sure storage gets updated.
        let view2 = view1 + 1;
        storage
            .record_action(view2, HotShotAction::Vote)
            .await
            .unwrap();
        assert_eq!(
            storage.load_latest_acted_view().await.unwrap().unwrap(),
            view2
        );

        // Store an old view, make sure storage is unchanged.
        storage
            .record_action(view1, HotShotAction::Vote)
            .await
            .unwrap();
        assert_eq!(
            storage.load_latest_acted_view().await.unwrap().unwrap(),
            view2
        );
    }

    #[async_std::test]
    pub async fn test_append_and_collect_garbage<P: TestablePersistence>() {
        setup_test();

        let tmp = P::tmp_storage().await;
        let storage = P::connect(&tmp).await;

        // Test append VID
        assert_eq!(
            storage.load_vid_share(ViewNumber::new(0)).await.unwrap(),
            None
        );

        let leaf = Leaf::genesis(&ValidatedState::default(), &NodeState::mock()).await;
        let leaf_payload = leaf.block_payload().unwrap();
        let leaf_payload_bytes_arc = leaf_payload.encode();
        let disperse = vid_scheme(2)
            .disperse(leaf_payload_bytes_arc.clone())
            .unwrap();
        let (pubkey, privkey) = BLSPubKey::generated_from_seed_indexed([0; 32], 1);
        let signature = PubKey::sign(&privkey, &[]).unwrap();
        let mut vid = VidDisperseShare::<SeqTypes> {
            view_number: ViewNumber::new(1),
            payload_commitment: Default::default(),
            share: disperse.shares[0].clone(),
            common: disperse.common,
            recipient_key: pubkey,
        };
        let mut quorum_proposal = Proposal {
            data: QuorumProposal::<SeqTypes> {
                block_header: leaf.block_header().clone(),
                view_number: ViewNumber::genesis(),
                justify_qc: QuorumCertificate::genesis::<TestVersions>(
                    &ValidatedState::default(),
                    &NodeState::mock(),
                )
                .await,
                upgrade_certificate: None,
                proposal_certificate: None,
            },
            signature,
            _pd: Default::default(),
        };

        let vid_share1 = vid.clone().to_proposal(&privkey).unwrap().clone();

        storage.append_vid(&vid_share1).await.unwrap();

        assert_eq!(
            storage.load_vid_share(ViewNumber::new(1)).await.unwrap(),
            Some(vid_share1)
        );

        vid.view_number = ViewNumber::new(2);

        let vid_share2 = vid.clone().to_proposal(&privkey).unwrap().clone();
        storage.append_vid(&vid_share2).await.unwrap();

        assert_eq!(
            storage.load_vid_share(vid.view_number).await.unwrap(),
            Some(vid_share2)
        );

        vid.view_number = ViewNumber::new(3);

        let vid_share3 = vid.clone().to_proposal(&privkey).unwrap().clone();
        storage.append_vid(&vid_share3).await.unwrap();

        assert_eq!(
            storage.load_vid_share(vid.view_number).await.unwrap(),
            Some(vid_share3.clone())
        );

        let block_payload_signature = BLSPubKey::sign(&privkey, &leaf_payload_bytes_arc)
            .expect("Failed to sign block payload");

        let da_proposal_inner = DaProposal::<SeqTypes> {
            encoded_transactions: leaf_payload_bytes_arc,
            metadata: leaf_payload.ns_table().clone(),
            view_number: ViewNumber::new(1),
        };

        let da_proposal = Proposal {
            data: da_proposal_inner,
            signature: block_payload_signature,
            _pd: Default::default(),
        };

        storage.append_da(&da_proposal).await.unwrap();

        assert_eq!(
            storage.load_da_proposal(ViewNumber::new(1)).await.unwrap(),
            Some(da_proposal.clone())
        );

        let mut da_proposal2 = da_proposal.clone();
        da_proposal2.data.view_number = ViewNumber::new(2);
        storage.append_da(&da_proposal2.clone()).await.unwrap();

        assert_eq!(
            storage
                .load_da_proposal(da_proposal2.data.view_number)
                .await
                .unwrap(),
            Some(da_proposal2.clone())
        );

        let mut da_proposal3 = da_proposal2.clone();
        da_proposal3.data.view_number = ViewNumber::new(3);
        storage.append_da(&da_proposal3.clone()).await.unwrap();

        assert_eq!(
            storage
                .load_da_proposal(da_proposal3.data.view_number)
                .await
                .unwrap(),
            Some(da_proposal3.clone())
        );

        let quorum_proposal1 = quorum_proposal.clone();
        storage
            .append_quorum_proposal(&quorum_proposal1)
            .await
            .unwrap();

        assert_eq!(
            storage.load_quorum_proposals().await.unwrap(),
            Some(BTreeMap::from_iter([(
                ViewNumber::genesis(),
                quorum_proposal1.clone()
            )]))
        );

        quorum_proposal.data.view_number = ViewNumber::new(1);
        let quorum_proposal2 = quorum_proposal.clone();
        storage
            .append_quorum_proposal(&quorum_proposal2)
            .await
            .unwrap();

        assert_eq!(
            storage.load_quorum_proposals().await.unwrap(),
            Some(BTreeMap::from_iter([
                (ViewNumber::genesis(), quorum_proposal1.clone()),
                (ViewNumber::new(1), quorum_proposal2.clone())
            ]))
        );

        quorum_proposal.data.view_number = ViewNumber::new(2);
        let quorum_proposal3 = quorum_proposal.clone();
        storage
            .append_quorum_proposal(&quorum_proposal3)
            .await
            .unwrap();

        assert_eq!(
            storage.load_quorum_proposals().await.unwrap(),
            Some(BTreeMap::from_iter([
                (ViewNumber::genesis(), quorum_proposal1.clone()),
                (ViewNumber::new(1), quorum_proposal2.clone()),
                (ViewNumber::new(2), quorum_proposal3.clone())
            ]))
        );

        quorum_proposal.data.view_number = ViewNumber::new(10);

        // This one should stick around after GC runs.
        let quorum_proposal4 = quorum_proposal.clone();
        storage
            .append_quorum_proposal(&quorum_proposal4)
            .await
            .unwrap();

        assert_eq!(
            storage.load_quorum_proposals().await.unwrap(),
            Some(BTreeMap::from_iter([
                (ViewNumber::genesis(), quorum_proposal1),
                (ViewNumber::new(1), quorum_proposal2),
                (ViewNumber::new(2), quorum_proposal3),
                (ViewNumber::new(10), quorum_proposal4.clone())
            ]))
        );

        // Test garbage collection
        // Deleting da proposals and vid shares with view number <=2
        storage.collect_garbage(ViewNumber::new(2)).await.unwrap();

        for i in 0..=2 {
            assert_eq!(
                storage.load_da_proposal(ViewNumber::new(i)).await.unwrap(),
                None
            );

            assert_eq!(
                storage.load_vid_share(ViewNumber::new(i)).await.unwrap(),
                None
            );
        }

        assert_eq!(
            storage.load_da_proposal(ViewNumber::new(3)).await.unwrap(),
            Some(da_proposal3)
        );

        assert_eq!(
            storage.load_vid_share(ViewNumber::new(3)).await.unwrap(),
            Some(vid_share3)
        );

        let proposals = storage.load_quorum_proposals().await.unwrap();
        assert_eq!(
            proposals,
            Some(BTreeMap::from_iter([(
                ViewNumber::new(10),
                quorum_proposal4
            )]))
        )
    }
}
