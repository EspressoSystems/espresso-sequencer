//! Rejoining the network after a restart with blank storage.
//!
//! A node that lost its storage has no record of what it signed, so it signs
//! nothing until it knows a view `bar` at or above every view it signed in.
//! It asks peers for the newest certificate each holds. If it signed in view
//! `L`, a certificate for `L - 1` existed, so `f + 1` honest nodes were in
//! `L - 1` and hold a certificate for `L - 2` or later. The maximum view among
//! verified claims of a quorum is at least `L - 2`, so `bar` is that plus 2.

use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
};

use alloy::primitives::U256;
use hotshot_types::{
    data::{EpochNumber, ViewNumber},
    simple_vote::LockView,
    traits::node_implementation::NodeType,
    vote::HasViewNumber,
};

use crate::{helpers::ViewEpoch, message::CatchupEvidence};

/// How a node starts: from what it stored, or with nothing stored.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RejoinMode {
    #[default]
    Fresh,
    Wiped,
}

/// Where a node is on its way back into the network.
#[derive(Clone, Debug, Default)]
pub enum Rejoin<K> {
    /// Not rejoining.
    #[default]
    Fresh,
    /// Collecting the newest certificates of peers to fix the bar.
    Querying(BarQuery<K>),
    /// Views up to `bar` are barred; waiting for a decide above it.
    Waiting { bar: ViewNumber },
    /// Anchored; signing as usual.
    Done,
}

/// The newest certificates reported by peers, counted once per peer.
#[derive(Clone, Debug)]
pub struct BarQuery<K> {
    epoch: EpochNumber,
    /// What each peer last claimed to hold. `None` is a peer holding nothing.
    claims: HashMap<K, Option<ViewEpoch>>,
    /// Keys of certificates a verifier accepted.
    verified: BTreeSet<ViewEpoch>,
}

/// The position of the certificate in `evidence`.
pub fn evidence_key<T: NodeType>(evidence: &CatchupEvidence<T>) -> ViewEpoch {
    match evidence {
        CatchupEvidence::Qc(qc) => ViewEpoch(qc.view_number(), LockView::of(qc).epoch),
        CatchupEvidence::Tc(tc) => ViewEpoch(
            tc.view_number(),
            tc.data.epoch.unwrap_or_else(EpochNumber::genesis),
        ),
        CatchupEvidence::Tc3(tc) => ViewEpoch(tc.view_number(), tc.epoch),
    }
}

impl<K: Eq + Hash + Clone> BarQuery<K> {
    /// A query of the committee of `epoch`.
    pub fn new(epoch: EpochNumber) -> Self {
        Self {
            epoch,
            claims: HashMap::new(),
            verified: BTreeSet::new(),
        }
    }

    /// The epoch whose committee is queried.
    pub fn epoch(&self) -> EpochNumber {
        self.epoch
    }

    /// Query the committee of `epoch` instead. Claims are asked for again;
    /// a verifier reports each certificate once, so verified keys are kept.
    pub fn restart(&mut self, epoch: EpochNumber) {
        self.epoch = epoch;
        self.claims.clear();
    }

    /// Record what `peer` holds; its latest claim replaces an earlier one.
    pub fn claim(&mut self, peer: K, key: Option<ViewEpoch>) {
        self.claims.insert(peer, key);
    }

    /// Record that the certificate at `key` passed verification.
    pub fn verified(&mut self, key: ViewEpoch) {
        self.verified.insert(key);
    }

    /// The bar, once peers holding `threshold` stake have claimed a verified
    /// certificate or none: the highest claimed view plus 2.
    ///
    /// An unverified claim counts for nothing, so the bar never exceeds real
    /// progress. A peer outside the committee has no stake.
    pub fn bar(
        &self,
        stake_of: impl Fn(&K) -> Option<U256>,
        threshold: U256,
    ) -> Option<ViewNumber> {
        let mut stake = U256::ZERO;
        let mut highest = ViewNumber::genesis();
        for (peer, claim) in &self.claims {
            let view = match claim {
                None => ViewNumber::genesis(),
                Some(key) if self.verified.contains(key) => key.0,
                Some(_) => continue,
            };
            let Some(peer_stake) = stake_of(peer) else {
                continue;
            };
            stake += peer_stake;
            highest = highest.max(view);
        }
        (stake >= threshold).then(|| highest + 2)
    }

    /// The latest epoch after the queried one that a verified claim names.
    ///
    /// The queried committee may not be the one that holds the newest
    /// certificates; the query then restarts at that epoch.
    pub fn later_epoch(&self) -> Option<EpochNumber> {
        self.claims
            .values()
            .flatten()
            .filter(|key| self.verified.contains(key))
            .map(|key| key.1)
            .filter(|epoch| *epoch > self.epoch)
            .max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PEERS: u64 = 10;
    const THRESHOLD: u64 = 7;

    fn key(view: u64) -> ViewEpoch {
        ViewEpoch(ViewNumber::new(view), EpochNumber::new(1))
    }

    fn bar(query: &BarQuery<u64>) -> Option<ViewNumber> {
        query.bar(
            |peer| (*peer < PEERS).then_some(U256::ONE),
            U256::from(THRESHOLD),
        )
    }

    fn claimed(claims: &[Option<u64>]) -> BarQuery<u64> {
        let mut query = BarQuery::new(EpochNumber::new(1));
        for (peer, claim) in claims.iter().enumerate() {
            let key = claim.map(key);
            query.claim(peer as u64, key);
            if let Some(key) = key {
                query.verified(key);
            }
        }
        query
    }

    #[test]
    fn below_threshold_has_no_bar() {
        assert_eq!(bar(&claimed(&[Some(5); 6])), None);
    }

    #[test]
    fn threshold_gives_highest_view_plus_two() {
        let mut claims = vec![Some(3); 6];
        claims.push(Some(5));
        assert_eq!(bar(&claimed(&claims)), Some(ViewNumber::new(7)));
    }

    #[test]
    fn nothing_held_counts_at_genesis() {
        assert_eq!(bar(&claimed(&[None; 7])), Some(ViewNumber::new(2)));
    }

    #[test]
    fn unverified_claim_does_not_count_until_verified() {
        let mut query = claimed(&[Some(3); 6]);
        query.claim(6, Some(key(40)));
        assert_eq!(bar(&query), None);

        query.verified(key(40));
        assert_eq!(bar(&query), Some(ViewNumber::new(42)));
    }

    #[test]
    fn latest_claim_of_a_peer_replaces_the_first() {
        let mut query = claimed(&[Some(3); 7]);
        query.claim(0, Some(key(9)));
        assert_eq!(bar(&query), None);

        query.verified(key(9));
        assert_eq!(bar(&query), Some(ViewNumber::new(11)));
    }

    #[test]
    fn a_peer_outside_the_committee_has_no_stake() {
        let mut query = claimed(&[Some(3); 6]);
        query.claim(PEERS, Some(key(3)));
        assert_eq!(bar(&query), None);
    }

    #[test]
    fn later_epoch_needs_a_verified_claim() {
        let later = ViewEpoch(ViewNumber::new(30), EpochNumber::new(3));
        let mut query = claimed(&[Some(3); 2]);
        query.claim(2, Some(later));
        assert_eq!(query.later_epoch(), None);

        query.verified(later);
        assert_eq!(query.later_epoch(), Some(EpochNumber::new(3)));
    }

    #[test]
    fn restart_drops_claims_and_keeps_verification() {
        let mut query = claimed(&[Some(3); 7]);
        query.restart(EpochNumber::new(2));
        assert_eq!(bar(&query), None);

        for peer in 0..7 {
            query.claim(peer, Some(key(3)));
        }
        assert_eq!(bar(&query), Some(ViewNumber::new(5)));
    }
}
