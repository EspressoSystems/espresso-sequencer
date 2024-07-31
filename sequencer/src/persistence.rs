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
use committable::Commitment;
use espresso_types::v0_3::ChainConfig;

pub mod fs;
pub mod no_storage;
pub mod sql;

#[async_trait]
pub trait ChainConfigPersistence: Sized + Send + Sync + 'static {
    async fn insert_chain_config(&mut self, chain_config: ChainConfig) -> anyhow::Result<()>;
    async fn load_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig>;
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

    use anyhow::bail;
    use async_std::sync::{Arc, RwLock};
    use committable::Committable;
    use espresso_types::{
        traits::EventConsumer, Event, Leaf, NodeState, PubKey, SeqTypes, ValidatedState,
    };
    use hotshot::types::{BLSPubKey, SignatureKey};
    use hotshot_types::{
        data::{DaProposal, QuorumProposal, VidDisperseShare, ViewNumber},
        event::{EventType, HotShotAction, LeafInfo},
        message::Proposal,
        simple_certificate::QuorumCertificate,
        traits::{node_implementation::ConsensusTime, EncodeBytes},
        vid::vid_scheme,
    };
    use jf_vid::VidScheme;
    use sequencer_utils::test_utils::setup_test;
    use testing::TestablePersistence;

    use super::*;

    #[derive(Clone, Debug, Default)]
    struct EventCollector {
        events: Arc<RwLock<Vec<Event>>>,
    }

    #[async_trait]
    impl EventConsumer for EventCollector {
        async fn handle_event(&self, event: &Event) -> anyhow::Result<()> {
            self.events.write().await.push(event.clone());
            Ok(())
        }
    }

    #[async_std::test]
    pub async fn test_voted_view<P: TestablePersistence>() {
        setup_test();

        let tmp = P::tmp_storage().await;
        let mut storage = P::connect(&tmp).await;

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

    fn leaf_info(leaf: Leaf) -> LeafInfo<SeqTypes> {
        LeafInfo {
            leaf,
            vid_share: None,
            state: Default::default(),
            delta: None,
        }
    }

    #[async_std::test]
    pub async fn test_append_and_decide<P: TestablePersistence>() {
        setup_test();

        let tmp = P::tmp_storage().await;
        let mut storage = P::connect(&tmp).await;

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
            view_number: ViewNumber::new(0),
            payload_commitment: Default::default(),
            share: disperse.shares[0].clone(),
            common: disperse.common,
            recipient_key: pubkey,
        };
        let mut quorum_proposal = Proposal {
            data: QuorumProposal::<SeqTypes> {
                block_header: leaf.block_header().clone(),
                view_number: ViewNumber::genesis(),
                justify_qc: QuorumCertificate::genesis(
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

        let vid_share0 = vid.clone().to_proposal(&privkey).unwrap().clone();

        storage.append_vid(&vid_share0).await.unwrap();

        assert_eq!(
            storage.load_vid_share(ViewNumber::new(0)).await.unwrap(),
            Some(vid_share0.clone())
        );

        vid.view_number = ViewNumber::new(1);

        let vid_share1 = vid.clone().to_proposal(&privkey).unwrap().clone();
        storage.append_vid(&vid_share1).await.unwrap();

        assert_eq!(
            storage.load_vid_share(vid.view_number).await.unwrap(),
            Some(vid_share1.clone())
        );

        vid.view_number = ViewNumber::new(2);

        let vid_share2 = vid.clone().to_proposal(&privkey).unwrap().clone();
        storage.append_vid(&vid_share2).await.unwrap();

        assert_eq!(
            storage.load_vid_share(vid.view_number).await.unwrap(),
            Some(vid_share2.clone())
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
            view_number: ViewNumber::new(0),
        };

        let da_proposal = Proposal {
            data: da_proposal_inner,
            signature: block_payload_signature,
            _pd: Default::default(),
        };

        storage.append_da(&da_proposal).await.unwrap();

        assert_eq!(
            storage.load_da_proposal(ViewNumber::new(0)).await.unwrap(),
            Some(da_proposal.clone())
        );

        let mut da_proposal1 = da_proposal.clone();
        da_proposal1.data.view_number = ViewNumber::new(1);
        storage.append_da(&da_proposal1.clone()).await.unwrap();

        assert_eq!(
            storage
                .load_da_proposal(da_proposal1.data.view_number)
                .await
                .unwrap(),
            Some(da_proposal1.clone())
        );

        let mut da_proposal2 = da_proposal1.clone();
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
            BTreeMap::from_iter([(ViewNumber::genesis(), quorum_proposal1.clone())])
        );

        quorum_proposal.data.view_number = ViewNumber::new(1);
        let quorum_proposal2 = quorum_proposal.clone();
        storage
            .append_quorum_proposal(&quorum_proposal2)
            .await
            .unwrap();

        assert_eq!(
            storage.load_quorum_proposals().await.unwrap(),
            BTreeMap::from_iter([
                (ViewNumber::genesis(), quorum_proposal1.clone()),
                (ViewNumber::new(1), quorum_proposal2.clone())
            ])
        );

        quorum_proposal.data.view_number = ViewNumber::new(2);
        quorum_proposal.data.justify_qc.view_number = ViewNumber::new(1);
        let quorum_proposal3 = quorum_proposal.clone();
        storage
            .append_quorum_proposal(&quorum_proposal3)
            .await
            .unwrap();

        assert_eq!(
            storage.load_quorum_proposals().await.unwrap(),
            BTreeMap::from_iter([
                (ViewNumber::genesis(), quorum_proposal1.clone()),
                (ViewNumber::new(1), quorum_proposal2.clone()),
                (ViewNumber::new(2), quorum_proposal3.clone())
            ])
        );

        quorum_proposal.data.view_number = ViewNumber::new(3);
        quorum_proposal.data.justify_qc.view_number = ViewNumber::new(2);

        // This one should stick around after GC runs.
        let quorum_proposal4 = quorum_proposal.clone();
        storage
            .append_quorum_proposal(&quorum_proposal4)
            .await
            .unwrap();

        assert_eq!(
            storage.load_quorum_proposals().await.unwrap(),
            BTreeMap::from_iter([
                (ViewNumber::genesis(), quorum_proposal1.clone()),
                (ViewNumber::new(1), quorum_proposal2.clone()),
                (ViewNumber::new(2), quorum_proposal3.clone()),
                (ViewNumber::new(3), quorum_proposal4.clone())
            ])
        );

        // Test decide and garbage collection. Pass in a leaf chain with no VID shares or payloads,
        // so we have to fetch the missing data from storage.
        let leaves = [
            Leaf::from_quorum_proposal(&quorum_proposal1.data),
            Leaf::from_quorum_proposal(&quorum_proposal2.data),
            Leaf::from_quorum_proposal(&quorum_proposal3.data),
            Leaf::from_quorum_proposal(&quorum_proposal4.data),
        ];
        let mut final_qc = leaves[3].justify_qc();
        final_qc.view_number += 1;
        final_qc.data.leaf_commit = leaf.commit();
        let qcs = [
            leaves[1].justify_qc(),
            leaves[2].justify_qc(),
            leaves[3].justify_qc(),
            final_qc,
        ];

        let consumer = EventCollector::default();
        let leaf_chain = leaves
            .iter()
            .take(3)
            .map(|leaf| leaf_info(leaf.clone()))
            .zip(&qcs)
            .collect::<Vec<_>>();
        tracing::info!(?leaf_chain, "decide view 2");
        storage
            .append_decided_leaves(
                ViewNumber::new(2),
                leaf_chain.iter().map(|(leaf, qc)| (leaf, (*qc).clone())),
                &consumer,
            )
            .await
            .unwrap();

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
            Some(vid_share3.clone())
        );

        let proposals = storage.load_quorum_proposals().await.unwrap();
        assert_eq!(
            proposals,
            BTreeMap::from_iter([(ViewNumber::new(3), quorum_proposal4)])
        );

        // A decide event should have been processed.
        let events = consumer.events.read().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].view_number, ViewNumber::new(2));
        let EventType::Decide { qc, leaf_chain, .. } = &events[0].event else {
            panic!("expected decide event, got {:?}", events[0]);
        };
        assert_eq!(**qc, qcs[2]);
        assert_eq!(leaf_chain.len(), 3, "{leaf_chain:#?}");
        for (leaf, info) in leaves.iter().zip(leaf_chain.iter().rev()) {
            assert_eq!(info.leaf, *leaf);
            let decided_vid_share = info.vid_share.as_ref().unwrap();
            assert_eq!(decided_vid_share.view_number, leaf.view_number());
        }

        // The decided leaf should not have been garbage collected.
        assert_eq!(
            storage.load_anchor_leaf().await.unwrap(),
            Some((leaves[2].clone(), qcs[2].clone()))
        );

        // Process a second decide event.
        let consumer = EventCollector::default();
        tracing::info!(leaf = ?leaves[3], qc = ?qcs[3], "decide view 3");
        storage
            .append_decided_leaves(
                ViewNumber::new(3),
                vec![(&leaf_info(leaves[3].clone()), qcs[3].clone())],
                &consumer,
            )
            .await
            .unwrap();

        // A decide event should have been processed.
        let events = consumer.events.read().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].view_number, ViewNumber::new(3));
        let EventType::Decide { qc, leaf_chain, .. } = &events[0].event else {
            panic!("expected decide event, got {:?}", events[0]);
        };
        assert_eq!(**qc, qcs[3]);
        assert_eq!(leaf_chain.len(), 1);
        let info = &leaf_chain[0];
        assert_eq!(info.leaf, leaves[3]);

        // The remaining data should have been GCed.
        assert_eq!(
            storage.load_da_proposal(ViewNumber::new(3)).await.unwrap(),
            None
        );

        assert_eq!(
            storage.load_vid_share(ViewNumber::new(3)).await.unwrap(),
            None
        );
        assert_eq!(
            storage.load_quorum_proposals().await.unwrap(),
            BTreeMap::new()
        );
    }

    #[async_std::test]
    pub async fn test_decide_with_failing_event_consumer<P: TestablePersistence>() {
        #[derive(Clone, Copy, Debug)]
        struct FailConsumer;

        #[async_trait]
        impl EventConsumer for FailConsumer {
            async fn handle_event(&self, _: &Event) -> anyhow::Result<()> {
                bail!("mock error injection");
            }
        }

        setup_test();

        let tmp = P::tmp_storage().await;
        let mut storage = P::connect(&tmp).await;

        // Create a short blockchain.
        let mut chain = vec![];

        let leaf = Leaf::genesis(&ValidatedState::default(), &NodeState::mock()).await;
        let leaf_payload = leaf.block_payload().unwrap();
        let leaf_payload_bytes_arc = leaf_payload.encode();
        let disperse = vid_scheme(2)
            .disperse(leaf_payload_bytes_arc.clone())
            .unwrap();
        let (pubkey, privkey) = BLSPubKey::generated_from_seed_indexed([0; 32], 1);
        let mut vid = VidDisperseShare::<SeqTypes> {
            view_number: ViewNumber::new(0),
            payload_commitment: Default::default(),
            share: disperse.shares[0].clone(),
            common: disperse.common,
            recipient_key: pubkey,
        }
        .to_proposal(&privkey)
        .unwrap()
        .clone();
        let mut quorum_proposal = QuorumProposal::<SeqTypes> {
            block_header: leaf.block_header().clone(),
            view_number: ViewNumber::genesis(),
            justify_qc: QuorumCertificate::genesis(&ValidatedState::default(), &NodeState::mock())
                .await,
            upgrade_certificate: None,
            proposal_certificate: None,
        };
        let mut qc =
            QuorumCertificate::genesis(&ValidatedState::default(), &NodeState::mock()).await;

        let block_payload_signature = BLSPubKey::sign(&privkey, &leaf_payload_bytes_arc)
            .expect("Failed to sign block payload");
        let mut da_proposal = Proposal {
            data: DaProposal::<SeqTypes> {
                encoded_transactions: leaf_payload_bytes_arc,
                metadata: leaf_payload.ns_table().clone(),
                view_number: ViewNumber::new(0),
            },
            signature: block_payload_signature,
            _pd: Default::default(),
        };

        for i in 0..4 {
            quorum_proposal.view_number = ViewNumber::new(i);
            let leaf = Leaf::from_quorum_proposal(&quorum_proposal);
            qc.view_number = leaf.view_number();
            qc.data.leaf_commit = leaf.commit();
            vid.data.view_number = leaf.view_number();
            da_proposal.data.view_number = leaf.view_number();
            chain.push((leaf.clone(), qc.clone(), vid.clone(), da_proposal.clone()));
        }

        // Add proposals.
        for (_, _, vid, da) in &chain {
            storage.append_da(da).await.unwrap();
            storage.append_vid(vid).await.unwrap();
        }

        // Decide 2 leaves, but fail in event processing.
        let leaf_chain = chain
            .iter()
            .take(2)
            .map(|(leaf, qc, _, _)| (leaf_info(leaf.clone()), qc.clone()))
            .collect::<Vec<_>>();
        storage
            .append_decided_leaves(
                ViewNumber::new(1),
                leaf_chain.iter().map(|(leaf, qc)| (leaf, qc.clone())),
                &FailConsumer,
            )
            .await
            .unwrap();
        // No garbage collection should have run.
        for i in 0..4 {
            assert!(storage
                .load_vid_share(ViewNumber::new(i))
                .await
                .unwrap()
                .is_some());
            assert!(storage
                .load_da_proposal(ViewNumber::new(i))
                .await
                .unwrap()
                .is_some());
        }
        assert_eq!(
            storage
                .load_anchor_leaf()
                .await
                .unwrap()
                .unwrap()
                .0
                .view_number(),
            ViewNumber::new(1)
        );

        // Now decide remaining leaves successfully. We should now garbage collect and process a
        // decide event for all the leaves.
        let consumer = EventCollector::default();
        let leaf_chain = chain
            .iter()
            .skip(2)
            .map(|(leaf, qc, _, _)| (leaf_info(leaf.clone()), qc.clone()))
            .collect::<Vec<_>>();
        storage
            .append_decided_leaves(
                ViewNumber::new(3),
                leaf_chain.iter().map(|(leaf, qc)| (leaf, qc.clone())),
                &consumer,
            )
            .await
            .unwrap();
        // Garbage collection should have run.
        for i in 0..4 {
            assert!(storage
                .load_vid_share(ViewNumber::new(i))
                .await
                .unwrap()
                .is_none());
            assert!(storage
                .load_da_proposal(ViewNumber::new(i))
                .await
                .unwrap()
                .is_none());
        }
        assert_eq!(
            storage
                .load_anchor_leaf()
                .await
                .unwrap()
                .unwrap()
                .0
                .view_number(),
            ViewNumber::new(3)
        );

        // Check decide event.
        let events = consumer.events.read().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].view_number, ViewNumber::new(3));
        let EventType::Decide { qc, leaf_chain, .. } = &events[0].event else {
            panic!("expected decide event, got {:?}", events[0]);
        };
        assert_eq!(**qc, chain[3].1);
        assert_eq!(leaf_chain.len(), 4, "{leaf_chain:#?}");
        for ((leaf, _, _, _), info) in chain.iter().zip(leaf_chain.iter().rev()) {
            assert_eq!(info.leaf, *leaf);
            let decided_vid_share = info.vid_share.as_ref().unwrap();
            assert_eq!(decided_vid_share.view_number, leaf.view_number());
            assert!(info.leaf.block_payload().is_some());
        }
    }
}
