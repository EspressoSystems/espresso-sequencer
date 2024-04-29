use std::{marker::PhantomData, num::NonZeroU64};

use ethereum_types::U256;
// use ark_bls12_381::Parameters as Param381;
use hotshot_types::traits::signature_key::StakeTableEntryType;
use hotshot_types::{
    signature_key::BLSPubKey,
    traits::{election::Membership, node_implementation::NodeType, signature_key::SignatureKey},
    PeerConfig,
};
#[cfg(feature = "randomized-leader-election")]
use rand::{rngs::StdRng, Rng};
use tracing::debug;

/// Dummy implementation of [`Membership`]

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GeneralStaticCommittee<T, PUBKEY: SignatureKey> {
    /// All the nodes participating and their stake
    all_nodes_with_stake: Vec<PUBKEY::StakeTableEntry>,
    /// The nodes on the static committee and their stake
    committee_nodes_with_stake: Vec<PUBKEY::StakeTableEntry>,
    /// builder nodes
    committee_nodes_without_stake: Vec<PUBKEY>,
    /// the number of fixed leader for gpuvid
    fixed_leader_for_gpuvid: usize,
    /// Node type phantom
    _type_phantom: PhantomData<T>,
}

/// static committee using a vrf kp
pub type StaticCommittee<T> = GeneralStaticCommittee<T, BLSPubKey>;

impl<T, PUBKEY: SignatureKey> GeneralStaticCommittee<T, PUBKEY> {
    /// Creates a new dummy elector
    #[must_use]
    pub fn new(
        _nodes: &[PUBKEY],
        nodes_with_stake: Vec<PUBKEY::StakeTableEntry>,
        nodes_without_stake: Vec<PUBKEY>,
        fixed_leader_for_gpuvid: usize,
    ) -> Self {
        Self {
            all_nodes_with_stake: nodes_with_stake.clone(),
            committee_nodes_with_stake: nodes_with_stake,
            committee_nodes_without_stake: nodes_without_stake,
            fixed_leader_for_gpuvid,
            _type_phantom: PhantomData,
        }
    }
}

impl<TYPES, PUBKEY: SignatureKey + 'static> Membership<TYPES>
    for GeneralStaticCommittee<TYPES, PUBKEY>
where
    TYPES: NodeType<SignatureKey = PUBKEY>,
{
    /// Clone the public key and corresponding stake table for current elected committee
    fn get_committee_qc_stake_table(&self) -> Vec<PUBKEY::StakeTableEntry> {
        self.committee_nodes_with_stake.clone()
    }

    #[cfg(not(any(
        feature = "randomized-leader-election",
        feature = "fixed-leader-election"
    )))]
    /// Index the vector of public keys with the current view number
    fn get_leader(&self, view_number: TYPES::Time) -> PUBKEY {
        let index = usize::try_from(*view_number % self.all_nodes_with_stake.len() as u64).unwrap();
        let res = self.all_nodes_with_stake[index].clone();
        TYPES::SignatureKey::get_public_key(&res)
    }

    #[cfg(feature = "fixed-leader-election")]
    /// Only get leader in fixed set
    /// Index the fixed vector (first fixed_leader_for_gpuvid element) of public keys with the current view number
    fn get_leader(&self, view_number: TYPES::Time) -> PUBKEY {
        if self.fixed_leader_for_gpuvid <= 0
            || self.fixed_leader_for_gpuvid > self.all_nodes_with_stake.len()
        {
            panic!("fixed_leader_for_gpuvid is not set correctly.");
        }
        let index = usize::try_from(*view_number % self.fixed_leader_for_gpuvid as u64).unwrap();
        let res = self.all_nodes_with_stake[index].clone();
        TYPES::SignatureKey::get_public_key(&res)
    }

    #[cfg(feature = "randomized-leader-election")]
    /// Index the vector of public keys with a random number generated using the current view number as a seed
    fn get_leader(&self, view_number: TYPES::Time) -> PUBKEY {
        let mut rng: StdRng = rand::SeedableRng::seed_from_u64(*view_number);
        let randomized_view_number: usize = rng.gen();
        let index = randomized_view_number % self.nodes_with_stake.len();
        let res = self.all_nodes_with_stake[index].clone();
        TYPES::SignatureKey::get_public_key(&res)
    }

    fn has_stake(&self, pub_key: &PUBKEY) -> bool {
        let entry = pub_key.get_stake_table_entry(1u64);
        self.committee_nodes_with_stake.contains(&entry)
    }

    fn get_stake(
        &self,
        pub_key: &<TYPES as NodeType>::SignatureKey,
    ) -> Option<<TYPES::SignatureKey as SignatureKey>::StakeTableEntry> {
        let entry = pub_key.get_stake_table_entry(1u64);
        if self.committee_nodes_with_stake.contains(&entry) {
            Some(entry)
        } else {
            None
        }
    }

    fn create_election(
        mut all_nodes: Vec<PeerConfig<PUBKEY>>,
        committee_members: Vec<PeerConfig<PUBKEY>>,
        fixed_leader_for_gpuvid: usize,
    ) -> Self {
        let mut committee_nodes_with_stake = Vec::new();
        let mut committee_nodes_without_stake = Vec::new();

        // Iterate over committee members
        for entry in committee_members
            .iter()
            .map(|entry| entry.stake_table_entry.clone())
        {
            if entry.get_stake() > U256::from(0) {
                // Positive stake
                committee_nodes_with_stake.push(entry);
            } else {
                // Zero stake
                committee_nodes_without_stake.push(PUBKEY::get_public_key(&entry));
            }
        }

        // Retain all nodes with stake
        all_nodes.retain(|entry| entry.stake_table_entry.get_stake() > U256::from(0));

        debug!(
            "Election Membership Size: {}",
            committee_nodes_with_stake.len()
        );

        Self {
            all_nodes_with_stake: all_nodes
                .into_iter()
                .map(|entry| entry.stake_table_entry)
                .collect(),
            committee_nodes_with_stake,
            committee_nodes_without_stake,
            fixed_leader_for_gpuvid,
            _type_phantom: PhantomData,
        }
    }

    fn total_nodes(&self) -> usize {
        self.committee_nodes_with_stake.len()
    }

    fn success_threshold(&self) -> NonZeroU64 {
        NonZeroU64::new(((self.committee_nodes_with_stake.len() as u64 * 2) / 3) + 1).unwrap()
    }

    fn failure_threshold(&self) -> NonZeroU64 {
        NonZeroU64::new(((self.committee_nodes_with_stake.len() as u64) / 3) + 1).unwrap()
    }

    fn upgrade_threshold(&self) -> NonZeroU64 {
        NonZeroU64::new(((self.committee_nodes_with_stake.len() as u64 * 9) / 10) + 1).unwrap()
    }

    fn get_staked_committee(
        &self,
        _view_number: <TYPES as NodeType>::Time,
    ) -> std::collections::BTreeSet<<TYPES as NodeType>::SignatureKey> {
        self.committee_nodes_with_stake
            .iter()
            .map(|node| <TYPES as NodeType>::SignatureKey::get_public_key(node))
            .collect()
    }

    fn get_non_staked_committee(
        &self,
        _view_number: <TYPES as NodeType>::Time,
    ) -> std::collections::BTreeSet<<TYPES as NodeType>::SignatureKey> {
        self.committee_nodes_without_stake.iter().cloned().collect()
    }

    fn get_whole_committee(
        &self,
        view_number: <TYPES as NodeType>::Time,
    ) -> std::collections::BTreeSet<<TYPES as NodeType>::SignatureKey> {
        let mut committee = self.get_staked_committee(view_number);
        committee.extend(self.get_non_staked_committee(view_number));
        committee
    }
}

impl<TYPES, PUBKEY: SignatureKey + 'static> GeneralStaticCommittee<TYPES, PUBKEY>
where
    TYPES: NodeType<SignatureKey = PUBKEY>,
{
    #[allow(clippy::must_use_candidate)]
    /// get the non-staked builder nodes
    pub fn non_staked_nodes_count(&self) -> usize {
        self.committee_nodes_without_stake.len()
    }
    #[allow(clippy::must_use_candidate)]
    /// get all the non-staked nodes
    pub fn get_non_staked_nodes(&self) -> Vec<PUBKEY> {
        self.committee_nodes_without_stake.clone()
    }
}
