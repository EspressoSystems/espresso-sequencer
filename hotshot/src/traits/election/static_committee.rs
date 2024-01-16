// use ark_bls12_381::Parameters as Param381;
use hotshot_types::signature_key::BLSPubKey;
use hotshot_types::traits::{
    election::{ElectionConfig, Membership},
    node_implementation::NodeType,
    signature_key::SignatureKey,
};
#[allow(deprecated)]
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, num::NonZeroU64};
use tracing::debug;

#[cfg(feature = "randomized-leader-election")]
use rand::{rngs::StdRng, Rng};

/// Dummy implementation of [`Membership`]

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GeneralStaticCommittee<T, PUBKEY: SignatureKey> {
    /// All the nodes participating and their stake
    nodes_with_stake: Vec<PUBKEY::StakeTableEntry>,
    /// The nodes on the static committee and their stake
    committee_nodes_with_stake: Vec<PUBKEY::StakeTableEntry>,
    /// Node type phantom
    _type_phantom: PhantomData<T>,
}

/// static committee using a vrf kp
pub type StaticCommittee<T> = GeneralStaticCommittee<T, BLSPubKey>;

impl<T, PUBKEY: SignatureKey> GeneralStaticCommittee<T, PUBKEY> {
    /// Creates a new dummy elector
    #[must_use]
    pub fn new(_nodes: &[PUBKEY], nodes_with_stake: Vec<PUBKEY::StakeTableEntry>) -> Self {
        Self {
            nodes_with_stake: nodes_with_stake.clone(),
            committee_nodes_with_stake: nodes_with_stake,
            _type_phantom: PhantomData,
        }
    }
}

/// configuration for static committee. stub for now
#[derive(Default, Clone, Serialize, Deserialize, core::fmt::Debug)]
pub struct StaticElectionConfig {
    /// Number of nodes on the committee
    num_nodes: u64,
}

impl ElectionConfig for StaticElectionConfig {}

impl<TYPES, PUBKEY: SignatureKey + 'static> Membership<TYPES>
    for GeneralStaticCommittee<TYPES, PUBKEY>
where
    TYPES: NodeType<SignatureKey = PUBKEY, ElectionConfigType = StaticElectionConfig>,
{
    /// Clone the public key and corresponding stake table for current elected committee
    fn get_committee_qc_stake_table(&self) -> Vec<PUBKEY::StakeTableEntry> {
        self.committee_nodes_with_stake.clone()
    }

    #[cfg(not(feature = "randomized-leader-election"))]
    /// Index the vector of public keys with the current view number
    fn get_leader(&self, view_number: TYPES::Time) -> PUBKEY {
        let index = (*view_number % self.nodes_with_stake.len() as u64) as usize;
        let res = self.nodes_with_stake[index].clone();
        TYPES::SignatureKey::get_public_key(&res)
    }

    #[cfg(feature = "randomized-leader-election")]
    /// Index the vector of public keys with a random number generated using the current view number as a seed
    fn get_leader(&self, view_number: TYPES::Time) -> PUBKEY {
        let mut rng: StdRng = rand::SeedableRng::seed_from_u64(*view_number as u64);
        let randomized_view_number: u64 = rng.gen();
        let index = (randomized_view_number % self.nodes_with_stake.len() as u64) as usize;
        let res = self.nodes_with_stake[index].clone();
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

    fn default_election_config(num_nodes: u64) -> TYPES::ElectionConfigType {
        StaticElectionConfig { num_nodes }
    }

    fn create_election(
        keys_qc: Vec<PUBKEY::StakeTableEntry>,
        config: TYPES::ElectionConfigType,
    ) -> Self {
        let mut committee_nodes_with_stake = keys_qc.clone();
        debug!("Election Membership Size: {}", config.num_nodes);
        committee_nodes_with_stake.truncate(config.num_nodes.try_into().unwrap());
        Self {
            nodes_with_stake: keys_qc,
            committee_nodes_with_stake,
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

    fn get_committee(
        &self,
        _view_number: <TYPES as NodeType>::Time,
    ) -> std::collections::BTreeSet<<TYPES as NodeType>::SignatureKey> {
        // Transfer from committee_nodes_with_stake to pure committee_nodes
        (0..self.committee_nodes_with_stake.len())
            .map(|node_id| {
                <TYPES as NodeType>::SignatureKey::get_public_key(
                    &self.committee_nodes_with_stake[node_id],
                )
            })
            .collect()
    }
}
