// use ark_bls12_381::Parameters as Param381;
use commit::{Commitment, Committable, RawCommitmentBuilder};
use espresso_systems_common::hotshot::tag;
use hotshot_signature_key::bn254::BLSPubKey;
use hotshot_types::traits::{
    election::{Checked, ElectionConfig, ElectionError, Membership, VoteToken},
    node_implementation::NodeType,
    signature_key::{EncodedSignature, SignatureKey},
};
#[allow(deprecated)]
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, num::NonZeroU64};
use tracing::debug;

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

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(bound(deserialize = ""))]
/// Vote token for a static committee
pub struct StaticVoteToken<K: SignatureKey> {
    /// signature
    signature: EncodedSignature,
    /// public key
    pub_key: K,
}

impl<PUBKEY: SignatureKey> VoteToken for StaticVoteToken<PUBKEY> {
    fn vote_count(&self) -> NonZeroU64 {
        NonZeroU64::new(1).unwrap()
    }
}

impl<PUBKEY: SignatureKey> Committable for StaticVoteToken<PUBKEY> {
    fn commit(&self) -> Commitment<Self> {
        RawCommitmentBuilder::new("StaticVoteToken")
            .var_size_field("signature", &self.signature.0)
            .var_size_field("pub_key", &self.pub_key.to_bytes().0)
            .finalize()
    }

    fn tag() -> String {
        tag::STATIC_VOTE_TOKEN.to_string()
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
    TYPES: NodeType<
        SignatureKey = PUBKEY,
        VoteTokenType = StaticVoteToken<PUBKEY>,
        ElectionConfigType = StaticElectionConfig,
    >,
{
    /// Clone the public key and corresponding stake table for current elected committee
    fn get_committee_qc_stake_table(&self) -> Vec<PUBKEY::StakeTableEntry> {
        self.committee_nodes_with_stake.clone()
    }

    /// Index the vector of public keys with the current view number
    fn get_leader(&self, view_number: TYPES::Time) -> PUBKEY {
        let index = (*view_number % self.nodes_with_stake.len() as u64) as usize;
        let res = self.nodes_with_stake[index].clone();
        TYPES::SignatureKey::get_public_key(&res)
    }

    /// Simply make the partial signature
    fn make_vote_token(
        &self,
        view_number: TYPES::Time,
        private_key: &<PUBKEY as SignatureKey>::PrivateKey,
    ) -> std::result::Result<Option<StaticVoteToken<PUBKEY>>, ElectionError> {
        let pub_key = PUBKEY::from_private(private_key);
        let entry = pub_key.get_stake_table_entry(1u64);
        if !self.committee_nodes_with_stake.contains(&entry) {
            return Ok(None);
        }
        let mut message: Vec<u8> = vec![];
        message.extend(view_number.to_le_bytes());
        // Change the length from 8 to 32 to make it consistent with other commitments, use defined constant? instead of 32.
        message.extend_from_slice(&[0u8; 32 - 8]);
        let signature = PUBKEY::sign(private_key, &message);
        Ok(Some(StaticVoteToken { signature, pub_key }))
    }

    fn validate_vote_token(
        &self,
        pub_key: PUBKEY,
        token: Checked<TYPES::VoteTokenType>,
    ) -> Result<Checked<TYPES::VoteTokenType>, ElectionError> {
        match token {
            Checked::Valid(t) | Checked::Unchecked(t) => {
                let entry = pub_key.get_stake_table_entry(1u64);
                if self.committee_nodes_with_stake.contains(&entry) {
                    Ok(Checked::Valid(t))
                } else {
                    Ok(Checked::Inval(t))
                }
            }
            Checked::Inval(t) => Ok(Checked::Inval(t)),
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
