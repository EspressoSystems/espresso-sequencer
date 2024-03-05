use std::marker::PhantomData;

use hotshot_example_types::{
    block_types::{TestBlockHeader, TestBlockPayload, TestTransaction},
    node_types::{MemoryImpl, TestTypes},
    state_types::TestInstanceState,
};

use crate::task_helpers::{
    build_cert, build_da_certificate, build_vid_proposal, da_payload_commitment, key_pair_for_id,
};
use commit::Committable;

use hotshot::types::{BLSPubKey, SignatureKey, SystemContextHandle};

use hotshot_types::{
    data::{Leaf, QuorumProposal, VidDisperse, ViewNumber},
    message::Proposal,
    simple_certificate::{DACertificate, QuorumCertificate, UpgradeCertificate},
    simple_vote::{UpgradeProposalData, UpgradeVote},
    traits::{
        consensus_api::ConsensusApi,
        node_implementation::{ConsensusTime, NodeType},
    },
};

use hotshot_types::simple_vote::QuorumData;
use hotshot_types::simple_vote::QuorumVote;

#[derive(Clone)]
pub struct TestView {
    pub quorum_proposal: Proposal<TestTypes, QuorumProposal<TestTypes>>,
    pub leaf: Leaf<TestTypes>,
    pub view_number: ViewNumber,
    pub quorum_membership: <TestTypes as NodeType>::Membership,
    pub vid_proposal: (
        Proposal<TestTypes, VidDisperse<TestTypes>>,
        <TestTypes as NodeType>::SignatureKey,
    ),
    pub leader_public_key: <TestTypes as NodeType>::SignatureKey,
    pub da_certificate: DACertificate<TestTypes>,
    pub transactions: Vec<TestTransaction>,
    upgrade_data: Option<UpgradeProposalData<TestTypes>>,
}

impl TestView {
    pub fn genesis(quorum_membership: &<TestTypes as NodeType>::Membership) -> Self {
        let genesis_view = ViewNumber::new(1);

        let transactions = Vec::new();

        let (private_key, public_key) = key_pair_for_id(*genesis_view);

        let leader_public_key = public_key;

        let payload_commitment = da_payload_commitment(quorum_membership, transactions.clone());

        let vid_proposal = build_vid_proposal(
            quorum_membership,
            genesis_view,
            transactions.clone(),
            &private_key,
        );

        let da_certificate = build_da_certificate(
            quorum_membership,
            genesis_view,
            transactions.clone(),
            &public_key,
            &private_key,
        );

        let block_header = TestBlockHeader {
            block_number: 1,
            timestamp: 1,
            payload_commitment,
        };

        let proposal = QuorumProposal::<TestTypes> {
            block_header: block_header.clone(),
            view_number: genesis_view,
            justify_qc: QuorumCertificate::genesis(),
            timeout_certificate: None,
            upgrade_certificate: None,
            proposer_id: public_key,
        };

        let leaf = Leaf {
            view_number: genesis_view,
            justify_qc: QuorumCertificate::genesis(),
            parent_commitment: Leaf::genesis(&TestInstanceState {}).commit(),
            block_header: block_header.clone(),
            // Note: this field is not relevant in calculating the leaf commitment.
            block_payload: Some(TestBlockPayload {
                transactions: transactions.clone(),
            }),
            // Note: this field is not relevant in calculating the leaf commitment.
            proposer_id: public_key,
        };

        let signature = <BLSPubKey as SignatureKey>::sign(&private_key, leaf.commit().as_ref())
            .expect("Failed to sign leaf commitment!");

        let quorum_proposal = Proposal {
            data: proposal,
            signature,
            _pd: PhantomData,
        };

        TestView {
            quorum_proposal,
            leaf,
            view_number: genesis_view,
            quorum_membership: quorum_membership.clone(),
            vid_proposal: (vid_proposal, public_key),
            da_certificate,
            transactions,
            leader_public_key,
            upgrade_data: None,
        }
    }

    pub fn next_view(&self) -> Self {
        let old = self;
        let old_view = old.view_number;
        let next_view = old_view + 1;

        let quorum_membership = &self.quorum_membership;
        let transactions = &self.transactions;

        let quorum_data = QuorumData {
            leaf_commit: old.leaf.commit(),
        };

        let (old_private_key, old_public_key) = key_pair_for_id(*old_view);

        let (private_key, public_key) = key_pair_for_id(*next_view);

        let leader_public_key = public_key;

        let payload_commitment = da_payload_commitment(quorum_membership, transactions.clone());

        let vid_proposal = build_vid_proposal(
            quorum_membership,
            next_view,
            transactions.clone(),
            &private_key,
        );

        let da_certificate = build_da_certificate(
            quorum_membership,
            next_view,
            transactions.clone(),
            &public_key,
            &private_key,
        );

        let quorum_certificate = build_cert::<
            TestTypes,
            QuorumData<TestTypes>,
            QuorumVote<TestTypes>,
            QuorumCertificate<TestTypes>,
        >(
            quorum_data,
            quorum_membership,
            old_view,
            &old_public_key,
            &old_private_key,
        );

        let upgrade_certificate = if let Some(ref data) = self.upgrade_data {
            let cert = build_cert::<
                TestTypes,
                UpgradeProposalData<TestTypes>,
                UpgradeVote<TestTypes>,
                UpgradeCertificate<TestTypes>,
            >(
                data.clone(),
                quorum_membership,
                next_view,
                &public_key,
                &private_key,
            );

            Some(cert)
        } else {
            None
        };

        let block_header = TestBlockHeader {
            block_number: *next_view,
            timestamp: *next_view,
            payload_commitment,
        };

        let leaf = Leaf {
            view_number: next_view,
            justify_qc: quorum_certificate.clone(),
            parent_commitment: old.leaf.commit(),
            block_header: block_header.clone(),
            // Note: this field is not relevant in calculating the leaf commitment.
            block_payload: Some(TestBlockPayload {
                transactions: transactions.clone(),
            }),
            // Note: this field is not relevant in calculating the leaf commitment.
            proposer_id: public_key,
        };

        let signature = <BLSPubKey as SignatureKey>::sign(&private_key, leaf.commit().as_ref())
            .expect("Failed to sign leaf commitment.");

        let proposal = QuorumProposal::<TestTypes> {
            block_header: block_header.clone(),
            view_number: next_view,
            justify_qc: quorum_certificate.clone(),
            timeout_certificate: None,
            upgrade_certificate,
            proposer_id: public_key,
        };

        let quorum_proposal = Proposal {
            data: proposal,
            signature,
            _pd: PhantomData,
        };

        TestView {
            quorum_proposal,
            leaf,
            view_number: next_view,
            quorum_membership: quorum_membership.clone(),
            vid_proposal: (vid_proposal, public_key),
            da_certificate,
            leader_public_key,
            // Transactions and upgrade data need to be manually injected each view,
            // so we reset for the next view.
            transactions: Vec::new(),
            upgrade_data: None,
        }
    }

    pub fn create_vote(
        &self,
        handle: &SystemContextHandle<TestTypes, MemoryImpl>,
    ) -> QuorumVote<TestTypes> {
        QuorumVote::<TestTypes>::create_signed_vote(
            QuorumData {
                leaf_commit: self.leaf.commit(),
            },
            self.view_number,
            handle.public_key(),
            handle.private_key(),
        )
        .expect("Failed to generate a signature on QuorumVote")
    }
}

pub struct TestViewGenerator {
    pub current_view: Option<TestView>,
    pub quorum_membership: <TestTypes as NodeType>::Membership,
}

impl TestViewGenerator {
    pub fn generate(quorum_membership: <TestTypes as NodeType>::Membership) -> Self {
        TestViewGenerator {
            current_view: None,
            quorum_membership,
        }
    }

    pub fn add_upgrade(&mut self, upgrade_proposal_data: UpgradeProposalData<TestTypes>) {
        if let Some(ref view) = self.current_view {
            self.current_view = Some(TestView {
                upgrade_data: Some(upgrade_proposal_data),
                ..view.clone()
            });
        } else {
            tracing::error!("Cannot attach upgrade proposal to the genesis view.");
        }
    }

    pub fn add_transactions(&mut self, transactions: Vec<TestTransaction>) {
        if let Some(ref view) = self.current_view {
            self.current_view = Some(TestView {
                transactions,
                ..view.clone()
            });
        } else {
            tracing::error!("Cannot attach transactions to the genesis view.");
        }
    }
}

impl Iterator for TestViewGenerator {
    type Item = TestView;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(view) = &self.current_view {
            self.current_view = Some(TestView::next_view(view));
        } else {
            self.current_view = Some(TestView::genesis(&self.quorum_membership));
        }

        self.current_view.clone()
    }
}
