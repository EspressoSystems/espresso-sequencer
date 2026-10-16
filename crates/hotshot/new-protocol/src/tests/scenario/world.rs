//! The real blocks and certificates a scenario's abstract ones stand for.
//!
//! A scenario names a block by an identity and a certificate by the block it is
//! over, its epoch, height and view (`super::format`). This builds each from
//! those, the first time it is named, and keeps it, so a block named twice is
//! the same proposal both times. Certificates are signed by every test key, as
//! the other tests' certificates are: what consensus checks of them is the
//! committee's threshold, not who signed.
//!
//! What a real proposal carries and the specification does not model is filled
//! in as the Rust node expects it: the parent's `Certificate2` for the first
//! block of an epoch, the state certificate after an epoch root, and the DRB
//! result in an epoch's transition blocks, which is the harness's
//! `TEST_DRB_RESULT` throughout.

use std::collections::{BTreeMap, HashMap};

use committable::{Commitment, Committable};
use futures::future::BoxFuture;
use hotshot::types::{BLSPrivKey, BLSPubKey};
use hotshot_example_types::{
    block_types::{TestBlockHeader, TestBlockPayload, TestMetadata, TestTransaction},
    node_types::{TEST_VERSIONS, TestTypes},
    state_types::{TestInstanceState, TestValidatedState},
};
use hotshot_types::{
    data::{
        EpochNumber, Leaf2, VidCommitment, VidDisperse, VidDisperse2, VidDisperseShare2, ViewNumber,
    },
    epoch_membership::EpochMembershipCoordinator,
    message::Proposal as SignedProposal,
    simple_certificate::{TimeoutCertificate3, TimeoutEvidence},
    stake_table::HSStakeTable,
    traits::{
        BlockPayload, block_contents::BlockHeader, election::Membership,
        signature_key::SignatureKey,
    },
    utils::{epoch_from_block_number, is_epoch_root, is_epoch_transition, is_last_block},
};

use super::format::{self, Cert, TimeoutCert};
use crate::{
    helpers::test_upgrade_lock,
    message::{Certificate1, Certificate2, Proposal},
    tests::common::{
        coordinator_builder::build_genesis_cert1,
        utils::{
            MockBlock, TEST_DRB_RESULT, build_cert1, build_cert2, build_state_cert_for_test,
            build_timeout_cert3_with_lock, extract_vid_shares, key_map_with_num_nodes,
            mock_membership_with_num_nodes,
        },
    },
};

type StateCert = hotshot_types::simple_certificate::LightClientStateUpdateCertificateV2<TestTypes>;

/// The anchor's identity in every scenario.
pub const ANCHOR: u64 = 1;

/// A block built for a scenario.
#[derive(Clone)]
pub struct Built {
    pub proposal: SignedProposal<TestTypes, Proposal<TestTypes>>,
    pub leaf: Leaf2<TestTypes>,
    pub payload: TestBlockPayload,
    pub vid: VidDisperse2<TestTypes>,
    pub shares: Vec<VidDisperseShare2<TestTypes>>,
    pub leader: BLSPubKey,
}

impl Built {
    /// The VID share for `recipient`.
    pub fn share_for(&self, recipient: &BLSPubKey) -> VidDisperseShare2<TestTypes> {
        self.shares
            .iter()
            .find(|s| s.recipient_key == *recipient)
            .expect("a share for every member")
            .clone()
    }
}

pub struct World {
    nodes: u64,
    epoch_height: u64,
    membership: EpochMembershipCoordinator<TestTypes>,
    keys: BTreeMap<BLSPubKey, BLSPrivKey>,
    genesis: Leaf2<TestTypes>,
    genesis_cert: Certificate1<TestTypes>,
    defs: HashMap<u64, format::Block>,
    built: HashMap<u64, Built>,
    /// The blocks the node itself proposed (`World::adopt_own`).
    own: std::collections::HashSet<u64>,
    /// The `Certificate2`s the node was given, by the block they are over.
    cert2s: HashMap<u64, Vec<Cert>>,
}

impl World {
    pub async fn new(nodes: u64, epoch_height: u64, blocks: &[format::Block]) -> Self {
        let (pk0, _) = BLSPubKey::generated_from_seed_indexed([0u8; 32], 0);
        let (membership, ..) = mock_membership_with_num_nodes(nodes as usize, epoch_height, pk0);
        let genesis = Leaf2::<TestTypes>::genesis(
            &TestValidatedState::default(),
            &TestInstanceState::default(),
            TEST_VERSIONS.vid2.base,
        )
        .await;
        let genesis_cert = build_genesis_cert1(&genesis);
        Self {
            nodes,
            epoch_height,
            membership,
            keys: key_map_with_num_nodes(nodes),
            genesis,
            genesis_cert,
            defs: blocks.iter().map(|b| (b.id, *b)).collect(),
            built: HashMap::new(),
            own: std::collections::HashSet::new(),
            cert2s: HashMap::new(),
        }
    }

    /// Node `node`'s keys; nodes are numbered from 1.
    pub fn key(node: u64) -> (BLSPubKey, BLSPrivKey) {
        BLSPubKey::generated_from_seed_indexed([0u8; 32], node - 1)
    }

    fn signer(&self) -> (BLSPubKey, BLSPrivKey) {
        Self::key(1)
    }

    /// The leader of `view` in `epoch`, with its private key.
    pub fn leader(&self, epoch: u64, view: u64) -> (BLSPubKey, BLSPrivKey) {
        let leader = self
            .membership
            .membership_for_epoch(Some(EpochNumber::new(epoch)))
            .expect("the epoch's committee is known")
            .leader(ViewNumber::new(view))
            .expect("every view has a leader");
        let key = self
            .keys
            .get(&leader)
            .expect("the leader is a test key")
            .clone();
        (leader, key)
    }

    /// The node a key belongs to, numbered from 1.
    pub fn node_of(&self, key: &BLSPubKey) -> Option<u64> {
        (1..=self.nodes).find(|&n| Self::key(n).0 == *key)
    }

    /// The block with the identity `id`, built on first use.
    pub fn block(&mut self, id: u64) -> BoxFuture<'_, Built> {
        Box::pin(async move {
            if let Some(b) = self.built.get(&id) {
                return b.clone();
            }
            let def = *self
                .defs
                .get(&id)
                .unwrap_or_else(|| panic!("block {id} is in the scenario's block list"));
            let b = self.build(def).await;
            self.built.insert(id, b.clone());
            b
        })
    }

    /// The block a payload the scenario names was proposed with.
    pub fn block_with_payload(&self, view: u64, payload: u64) -> Option<u64> {
        self.defs
            .values()
            .find(|b| b.view == view && b.payload == payload)
            .map(|b| b.id)
    }

    /// Use the proposal the node itself sent for the block `id`, if it is on the
    /// parent the scenario names; the node's own block stands for it from then on.
    pub async fn adopt_own(
        &mut self,
        id: u64,
        proposal: SignedProposal<TestTypes, Proposal<TestTypes>>,
    ) -> bool {
        let Some(def) = self.defs.get(&id).copied() else {
            return false;
        };
        if self.built.contains_key(&id) {
            return false;
        }
        if proposal.data.justify_qc.data.leaf_commit != self.leaf_commit(def.parent.block).await
            || proposal.data.view_number != ViewNumber::new(def.view)
        {
            return false;
        }
        let mock = MockBlock::new();
        let epoch = proposal.data.epoch;
        let vid = self
            .disperse(&mock.block, &mock.metadata, def.view, epoch)
            .await;
        let leader = self.leader(def.epoch, def.view).0;
        let leaf = Leaf2::from(proposal.data.clone());
        if is_epoch_root(def.height, self.epoch_height) {
            self.register_epoch_root(&proposal.data.block_header).await;
        }
        self.own.insert(id);
        self.built.insert(
            id,
            Built {
                proposal,
                leaf,
                payload: mock.block,
                shares: extract_vid_shares(&vid),
                vid,
                leader,
            },
        );
        true
    }

    /// Whether the block `id` is one the node itself proposed.
    pub fn is_own(&self, id: u64) -> bool {
        self.own.contains(&id)
    }

    /// Note a `Certificate2` the node was given, for the first block of the next
    /// epoch to name.
    pub fn saw_cert2(&mut self, c: Cert) {
        self.cert2s.entry(c.block).or_default().push(c);
    }

    async fn leaf_commit(&mut self, id: u64) -> Commitment<Leaf2<TestTypes>> {
        if id == ANCHOR {
            return self.genesis.commit();
        }
        self.block(id).await.leaf.commit()
    }

    pub async fn cert1(&mut self, c: Cert) -> Certificate1<TestTypes> {
        if c.block == ANCHOR && c.view == 0 {
            return self.genesis_cert.clone();
        }
        let commit = self.leaf_commit(c.block).await;
        let (pk, sk) = self.signer();
        let membership = self.committee(c.epoch);
        build_cert1(
            commit,
            EpochNumber::new(c.epoch),
            c.height,
            &membership,
            ViewNumber::new(c.view),
            &pk,
            &sk,
        )
    }

    pub async fn cert2(&mut self, c: Cert) -> Certificate2<TestTypes> {
        let commit = self.leaf_commit(c.block).await;
        let (pk, sk) = self.signer();
        let membership = self.committee(c.epoch);
        build_cert2(
            commit,
            EpochNumber::new(c.epoch),
            c.height,
            &membership,
            ViewNumber::new(c.view),
            &pk,
            &sk,
        )
    }

    pub async fn timeout_cert(&mut self, tc: TimeoutCert) -> TimeoutCertificate3<TestTypes> {
        let lock = if tc.lock.block == ANCHOR && tc.lock.view == 0 {
            None
        } else {
            Some(self.cert1(tc.lock).await)
        };
        let membership = self.committee(tc.epoch);
        build_timeout_cert3_with_lock(
            ViewNumber::new(tc.view),
            EpochNumber::new(tc.epoch),
            lock,
            &membership,
        )
    }

    fn committee(&self, epoch: u64) -> hotshot_types::epoch_membership::EpochMembership<TestTypes> {
        self.membership
            .membership_for_epoch(Some(EpochNumber::new(epoch)))
            .unwrap_or_else(|e| panic!("the committee of epoch {epoch} is known: {e}"))
    }

    async fn disperse(
        &self,
        payload: &TestBlockPayload,
        metadata: &TestMetadata,
        view: u64,
        epoch: EpochNumber,
    ) -> VidDisperse2<TestTypes> {
        let disperse = VidDisperse::calculate_vid_disperse(
            payload,
            &self.membership,
            ViewNumber::new(view),
            Some(epoch),
            Some(epoch),
            metadata,
            &test_upgrade_lock(),
        )
        .await
        .expect("dispersal succeeds");
        let VidDisperse::V2(vid) = disperse.disperse else {
            panic!("a V2 dispersal");
        };
        vid
    }

    async fn build(&mut self, def: format::Block) -> Built {
        let epoch = EpochNumber::new(def.epoch);
        let justify_qc = self.cert1(def.parent).await;
        let transactions = vec![TestTransaction::new(def.payload.to_le_bytes().to_vec())];
        let (payload, metadata) = <TestBlockPayload as BlockPayload<TestTypes>>::from_transactions(
            &transactions,
            &TestValidatedState::default(),
            &TestInstanceState::default(),
        )
        .await
        .expect("a payload from one transaction");
        let builder_commitment =
            <TestBlockPayload as BlockPayload<TestTypes>>::builder_commitment(&payload, &metadata);
        let vid = self.disperse(&payload, &metadata, def.view, epoch).await;
        let block_header = TestBlockHeader {
            block_number: def.height,
            payload_commitment: VidCommitment::V2(vid.payload_commitment),
            builder_commitment,
            metadata,
            timestamp: def.view,
            timestamp_millis: def.view * 1_000,
            random: def.id,
            version: TEST_VERSIONS.test.base,
        };
        let next_epoch_justify_qc = if is_last_block(def.parent.height, self.epoch_height) {
            Some(self.parent_cert2(def.parent).await)
        } else {
            None
        };
        let state_cert = self.state_cert(def.parent).await;
        let next_drb_result = (epoch > EpochNumber::genesis()
            && is_epoch_transition(def.height, self.epoch_height))
        .then_some(TEST_DRB_RESULT);
        let view_change_evidence = match def.evidence {
            Some(tc) => Some(TimeoutEvidence::V3(self.timeout_cert(tc).await)),
            None => None,
        };
        let proposal = Proposal {
            block_header,
            view_number: ViewNumber::new(def.view),
            epoch,
            justify_qc,
            next_epoch_justify_qc,
            upgrade_certificate: None,
            view_change_evidence,
            next_drb_result,
            state_cert,
        };
        let leaf = Leaf2::from(proposal.clone());
        let (leader, leader_key) = self.leader(def.epoch, def.view);
        let signature = <BLSPubKey as SignatureKey>::sign(&leader_key, leaf.commit().as_ref())
            .expect("the leader signs the leaf");
        if is_epoch_root(def.height, self.epoch_height) {
            self.register_epoch_root(&proposal.block_header).await;
        }
        Built {
            proposal: SignedProposal {
                data: proposal,
                signature,
                _pd: std::marker::PhantomData,
            },
            leaf,
            payload,
            shares: extract_vid_shares(&vid),
            vid,
            leader,
        }
    }

    /// The light-client state certificate that goes with a `Cert1` over an epoch
    /// root, at the certificate's view, as the node's state-vote tally pairs them;
    /// none for any other block.
    pub async fn state_cert(&mut self, c: Cert) -> Option<StateCert> {
        if !is_epoch_root(c.height, self.epoch_height) {
            return None;
        }
        let root = self.block(c.block).await;
        let next = self
            .committee(c.epoch)
            .next_epoch_stake_table()
            .expect("the next epoch's committee is known");
        let next_state = HSStakeTable::from_iter(next.stake_table())
            .commitment(self.nodes as usize)
            .expect("a stake table commitment");
        Some(build_state_cert_for_test(
            &root.proposal.data.block_header,
            ViewNumber::new(c.view),
            EpochNumber::new(c.epoch),
            &next_state,
            (2 * self.nodes) / 3 + 1,
        ))
    }

    /// The `Certificate2` over an epoch's last block that the first block of the
    /// next one names: the latest the node was given at the parent's view or
    /// later, else one at the parent's view.
    async fn parent_cert2(&mut self, parent: Cert) -> Certificate2<TestTypes> {
        let seen = self
            .cert2s
            .get(&parent.block)
            .and_then(|cs| {
                cs.iter()
                    .filter(|c| c.view >= parent.view)
                    .max_by_key(|c| c.view)
            })
            .copied();
        self.cert2(seen.unwrap_or(parent)).await
    }

    /// Register the epochs an epoch root decides, as the harness does once the
    /// root is decided, so the committee that signs the later epochs' certificates
    /// is known here too.
    async fn register_epoch_root(&mut self, header: &TestBlockHeader) {
        let height = <TestBlockHeader as BlockHeader<TestTypes>>::block_number(header);
        self.membership
            .add_epoch_root(header.clone())
            .await
            .expect("an epoch root registers");
        let target = EpochNumber::new(epoch_from_block_number(height, self.epoch_height) + 2);
        self.membership
            .membership()
            .add_drb_result(target, TEST_DRB_RESULT);
    }
}
