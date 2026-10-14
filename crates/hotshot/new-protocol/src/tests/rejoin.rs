//! A node restarted with nothing stored signs nothing until it knows a view
//! bar and has adopted the first decide above it.

use hotshot::types::BLSPubKey;
use hotshot_example_types::node_types::TestTypes;
use hotshot_types::{
    data::{EpochNumber, ViewNumber},
    traits::{block_contents::BlockHeader, signature_key::SignatureKey},
};

use super::common::utils::{ConsensusHarness, TestData, TestView, build_cert1, build_cert2};
use crate::{
    cert_verifier::ValidCert,
    consensus::{ConsensusInput, ConsensusOutput},
    helpers::{proposal_commitment, test_timeout_epoch_lock},
    message::{CatchupEvidence, TimeoutVote},
    rejoin::evidence_key,
    tests::common::assertions::{
        is_leaf_decided, is_send_timeout_vote, is_vote1, is_vote1_for_view, is_vote2,
    },
};

const EPOCH_HEIGHT: u64 = 10;
/// Not the leader of view 10, the first view after the anchor at 9.
const NODE: u64 = 1;

fn peer(i: u64) -> BLSPubKey {
    BLSPubKey::generated_from_seed_indexed([0u8; 32], i).0
}

/// The first `count` peers other than this node.
fn peers(count: usize) -> impl Iterator<Item = BLSPubKey> {
    (0..10).filter(|i| *i != NODE).take(count).map(peer)
}

async fn wiped_node() -> ConsensusHarness {
    let mut harness =
        ConsensusHarness::new_with_upgrade_lock(NODE, EPOCH_HEIGHT, test_timeout_epoch_lock())
            .await;
    harness.consensus.start_rejoin();
    harness
}

fn fetched_proposal(view: &TestView) -> ConsensusInput<TestTypes> {
    ConsensusInput::FetchedProposal(view.proposal_message())
}

/// `count` peers report `view`'s `Certificate1` as the newest they hold, and
/// it passes verification.
async fn claim_cert1(harness: &mut ConsensusHarness, view: &TestView, count: usize) {
    let key = evidence_key(&CatchupEvidence::Qc(view.cert1.clone()));
    for peer in peers(count) {
        harness.consensus.rejoin_claim(peer, Some(key));
    }
    harness
        .apply(ConsensusInput::AdvanceView(ValidCert::new(
            view.cert1.clone(),
            view.epoch_number,
        )))
        .await;
}

/// A wiped node with bar 7: a quorum reported the `Certificate1` at view 5.
async fn waiting_at_bar_7(test_data: &TestData) -> ConsensusHarness {
    let mut harness = wiped_node().await;
    claim_cert1(&mut harness, &test_data.views[4], 7).await;
    assert_eq!(harness.consensus.rejoin_bar(), Some(ViewNumber::new(7)));
    harness
}

fn anchored(harness: &ConsensusHarness) -> Vec<(ViewNumber, ViewNumber)> {
    harness
        .outputs()
        .iter()
        .filter_map(|o| match o {
            ConsensusOutput::RejoinAnchored { leaf, bar, .. } => Some((leaf.view_number(), *bar)),
            _ => None,
        })
        .collect()
}

fn cert1_fetches(harness: &ConsensusHarness) -> Vec<(ViewNumber, EpochNumber)> {
    harness
        .outputs()
        .iter()
        .filter_map(|o| match o {
            ConsensusOutput::FetchCert1 { view, epoch } => Some((*view, *epoch)),
            _ => None,
        })
        .collect()
}

fn signs_nothing(harness: &ConsensusHarness) -> bool {
    !harness.outputs().iter().any(|o| {
        is_vote1(o)
            || is_vote2(o)
            || is_send_timeout_vote(o)
            || matches!(
                o,
                ConsensusOutput::SendProposal(_) | ConsensusOutput::SendReVote(_)
            )
    })
}

#[tokio::test]
async fn silent_while_querying() {
    let test_data = TestData::new(4).await;
    let mut harness = wiped_node().await;

    harness
        .apply(ConsensusInput::Timeout(ViewNumber::new(1)))
        .await;
    assert!(
        harness
            .outputs()
            .iter()
            .any(|o| matches!(o, ConsensusOutput::FetchNewestCerts))
    );

    harness
        .apply_pair(test_data.views[0].proposal_input_consensus(&peer(NODE)))
        .await;
    for view in &test_data.views[..3] {
        harness
            .apply(ConsensusInput::AdvanceView(ValidCert::new(
                view.cert1.clone(),
                view.epoch_number,
            )))
            .await;
        harness.apply(view.cert2_input()).await;
    }

    assert!(signs_nothing(&harness));
    assert!(!harness.outputs().iter().any(is_leaf_decided));
    assert_eq!(harness.consensus.rejoin_bar(), None);
}

#[tokio::test]
async fn bar_is_the_highest_verified_claim_plus_two() {
    let test_data = TestData::new(5).await;
    let mut harness = wiped_node().await;

    claim_cert1(&mut harness, &test_data.views[4], 6).await;
    assert_eq!(harness.consensus.rejoin_bar(), None);

    harness.consensus.rejoin_claim(
        peers(7).last().expect("seven peers"),
        Some(evidence_key(&CatchupEvidence::Qc(
            test_data.views[4].cert1.clone(),
        ))),
    );
    assert_eq!(harness.consensus.rejoin_bar(), Some(ViewNumber::new(7)));
}

#[tokio::test]
async fn nothing_held_gives_bar_2() {
    let mut harness = wiped_node().await;
    for peer in peers(7) {
        harness.consensus.rejoin_claim(peer, None);
    }
    assert_eq!(harness.consensus.rejoin_bar(), Some(ViewNumber::new(2)));
}

#[tokio::test]
async fn unsolicited_claims_are_dropped() {
    let mut harness =
        ConsensusHarness::new_with_upgrade_lock(NODE, EPOCH_HEIGHT, test_timeout_epoch_lock())
            .await;
    for peer in peers(7) {
        harness.consensus.rejoin_claim(peer, None);
    }
    assert_eq!(harness.consensus.rejoin_bar(), None);
    assert!(!harness.consensus.signing_barred());
}

#[tokio::test]
async fn waiting_ignores_decides_at_or_below_the_bar() {
    let test_data = TestData::new(7).await;
    let mut harness = waiting_at_bar_7(&test_data).await;
    let lock = harness.consensus.locked_cert().cloned();
    let at_bar = &test_data.views[6];

    harness.apply(at_bar.cert2_input()).await;
    harness.apply(fetched_proposal(at_bar)).await;
    harness.apply(at_bar.cert1_input()).await;
    harness
        .apply(ConsensusInput::Timeout(at_bar.view_number))
        .await;

    assert!(anchored(&harness).is_empty());
    assert_eq!(harness.consensus.locked_cert().cloned(), lock);
    assert!(signs_nothing(&harness));
}

#[tokio::test]
async fn adopts_the_first_decide_above_the_bar() {
    let test_data = TestData::new(10).await;
    let mut harness = waiting_at_bar_7(&test_data).await;
    let anchor = &test_data.views[8];

    harness.apply(anchor.cert2_input()).await;
    harness.apply(fetched_proposal(anchor)).await;
    harness.apply(anchor.cert1_input()).await;

    assert_eq!(
        anchored(&harness),
        vec![(anchor.view_number, ViewNumber::new(7))]
    );
    assert!(
        harness
            .outputs()
            .iter()
            .any(|o| matches!(o, ConsensusOutput::PersistHighQc(cert1) if *cert1 == anchor.cert1))
    );
    assert_eq!(harness.consensus.locked_cert(), Some(&anchor.cert1));
    assert_eq!(
        harness.consensus.timeout_vote_bar(),
        Some(anchor.view_number)
    );
    assert_eq!(harness.consensus.last_decided_view(), anchor.view_number);
    assert_eq!(harness.consensus.rejoin_bar(), None);

    let next = &test_data.views[9];
    harness
        .apply_pair(next.proposal_input_consensus(&peer(NODE)))
        .await;
    assert!(
        harness
            .outputs()
            .iter()
            .any(|o| is_vote1_for_view(o, *next.view_number))
    );

    harness
        .apply(ConsensusInput::Timeout(next.view_number))
        .await;
    let locks: Vec<_> = harness
        .outputs()
        .iter()
        .filter_map(|o| match o {
            ConsensusOutput::SendTimeoutVote(TimeoutVote::V3(ballot), _) => {
                Some(ballot.lock().cloned())
            },
            _ => None,
        })
        .collect();
    assert_eq!(locks, vec![Some(anchor.cert1.clone())]);

    harness.apply(test_data.views[7].cert1_input()).await;
    assert!(!harness.outputs().iter().any(is_vote2));
}

#[tokio::test]
async fn fetches_the_anchor_certificate1_it_lacks() {
    let test_data = TestData::new(9).await;
    let mut harness = waiting_at_bar_7(&test_data).await;
    let anchor = &test_data.views[8];
    let fetch = (anchor.view_number, anchor.epoch_number);

    harness.apply(anchor.cert2_input()).await;
    harness.apply(fetched_proposal(anchor)).await;
    let fetches = cert1_fetches(&harness);
    assert!(fetches.iter().all(|f| *f == fetch) && !fetches.is_empty());
    assert!(anchored(&harness).is_empty());

    harness
        .apply(ConsensusInput::Timeout(ViewNumber::new(8)))
        .await;
    assert_eq!(cert1_fetches(&harness).len(), fetches.len() + 1);

    harness.apply(anchor.cert1_input()).await;
    assert_eq!(
        anchored(&harness),
        vec![(anchor.view_number, ViewNumber::new(7))]
    );
}

/// Neither a commit of an epoch's last block nor a re-vote of it is adopted:
/// the next epoch may have started before either.
#[tokio::test]
async fn skips_a_last_block_and_its_revote() {
    let test_data = TestData::new_with_epoch_height(12, EPOCH_HEIGHT).await;
    let mut harness = waiting_at_bar_7(&test_data).await;
    let (boundary, first) = (&test_data.views[9], &test_data.views[10]);

    harness.apply(boundary.cert2_input()).await;
    harness.apply(fetched_proposal(boundary)).await;
    harness.apply(boundary.cert1_input()).await;

    let membership = harness
        .membership_coordinator
        .membership_for_epoch(Some(boundary.epoch_number))
        .unwrap();
    let (pk, sk) = BLSPubKey::generated_from_seed_indexed([0u8; 32], 0);
    let leaf = proposal_commitment(&boundary.proposal.data);
    let block = BlockHeader::<TestTypes>::block_number(&boundary.proposal.data.block_header);
    let (revote_view, epoch) = (ViewNumber::new(12), boundary.epoch_number);
    let cert2 = build_cert2(leaf, epoch, block, &membership, revote_view, &pk, &sk);
    let cert1 = build_cert1(leaf, epoch, block, &membership, revote_view, &pk, &sk);
    harness
        .apply(ConsensusInput::Certificate2(ValidCert::new(cert2, epoch)))
        .await;
    harness
        .apply(ConsensusInput::Certificate1(ValidCert::new(cert1, epoch)))
        .await;
    harness
        .apply(ConsensusInput::Timeout(ViewNumber::new(12)))
        .await;

    assert!(anchored(&harness).is_empty());
    assert!(cert1_fetches(&harness).is_empty());
    assert_eq!(harness.consensus.rejoin_bar(), Some(ViewNumber::new(7)));

    harness.apply(first.cert2_input()).await;
    harness.apply(fetched_proposal(first)).await;
    harness.apply(first.cert1_input()).await;

    assert_eq!(
        anchored(&harness),
        vec![(first.view_number, ViewNumber::new(7))]
    );
    assert_eq!(harness.consensus.decide_floor(), first.view_number);
    assert_eq!(harness.consensus.current_epoch(), Some(first.epoch_number));
    assert!(signs_nothing(&harness));
}

/// A verified answer from a later epoch moves the query to that committee.
#[tokio::test]
async fn later_epoch_restarts_the_query() {
    let test_data = TestData::new_with_epoch_height(11, EPOCH_HEIGHT).await;
    let mut harness = wiped_node().await;
    let later = &test_data.views[10];
    harness
        .membership_coordinator
        .membership()
        .register_epoch(later.epoch_number, [0u8; 32]);

    claim_cert1(&mut harness, &test_data.views[4], 6).await;
    let later_key = evidence_key(&CatchupEvidence::Qc(later.cert1.clone()));
    let seventh = peers(7).last().expect("seven peers");
    harness.consensus.rejoin_claim(seventh, Some(later_key));
    harness
        .apply(ConsensusInput::AdvanceView(ValidCert::new(
            later.cert1.clone(),
            later.epoch_number,
        )))
        .await;
    assert_eq!(harness.consensus.rejoin_bar(), None);

    claim_cert1(&mut harness, later, 7).await;
    assert_eq!(harness.consensus.rejoin_bar(), Some(later.view_number + 2));
}

#[tokio::test]
#[should_panic(expected = "does not rejoin as wiped")]
async fn a_node_restarted_from_storage_does_not_rejoin() {
    let test_data = TestData::new(2).await;
    let anchor = &test_data.views[1];
    let mut harness =
        ConsensusHarness::restarted_from(0, anchor.proposal.data.clone(), anchor.cert1.clone(), [])
            .await;
    harness.consensus.start_rejoin();
}
