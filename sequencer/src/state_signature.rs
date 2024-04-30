//! Utilities for generating and storing the most recent light client state signatures.

use crate::{Leaf, SeqTypes, StateKeyPair};
use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;
use async_std::sync::RwLock;
use hotshot::types::{Event, EventType};
use hotshot_stake_table::vec_based::StakeTable;
use hotshot_types::light_client::{
    CircuitField, LightClientState, StateSignatureRequestBody, StateVerKey,
};
use hotshot_types::{
    event::LeafInfo,
    light_client::{StateSignature, StateSignatureScheme},
    signature_key::BLSPubKey,
    traits::{
        node_implementation::ConsensusTime,
        signature_key::StakeTableEntryType,
        stake_table::{SnapshotVersion, StakeTableScheme as _},
    },
    PeerConfig,
};
use jf_primitives::{
    crhf::{VariableLengthRescueCRHF, CRHF},
    errors::PrimitivesError,
    signatures::SignatureScheme,
};
use std::collections::{HashMap, VecDeque};
use surf_disco::{Client, Url};
use tide_disco::error::ServerError;
use vbs::version::StaticVersionType;

/// A relay server that's collecting and serving the light client state signatures
pub mod relay_server;

/// Capacity for the in memory signature storage.
const SIGNATURE_STORAGE_CAPACITY: usize = 100;

#[derive(Debug)]
pub struct StateSigner<Ver: StaticVersionType> {
    /// Key pair for signing a new light client state
    key_pair: StateKeyPair,

    /// The most recent light client state signatures
    signatures: RwLock<StateSignatureMemStorage>,

    /// Commitment for current fixed stake table
    stake_table_comm: StakeTableCommitmentType,

    /// The state relay server url
    relay_server_client: Option<Client<ServerError, Ver>>,
}

impl<Ver: StaticVersionType> StateSigner<Ver> {
    pub fn new(key_pair: StateKeyPair, stake_table_comm: StakeTableCommitmentType) -> Self {
        Self {
            key_pair,
            stake_table_comm,
            signatures: Default::default(),
            relay_server_client: Default::default(),
        }
    }

    /// Connect to the given state relay server to send signed HotShot states to.
    pub fn with_relay_server(mut self, url: Url) -> Self {
        self.relay_server_client = Some(Client::new(url));
        self
    }

    pub(super) async fn handle_event(&self, event: &Event<SeqTypes>) {
        let EventType::Decide { leaf_chain, .. } = &event.event else {
            return;
        };
        let Some(LeafInfo { leaf, .. }) = leaf_chain.first() else {
            return;
        };
        match form_light_client_state(leaf, &self.stake_table_comm) {
            Ok(state) => {
                let signature = self.sign_new_state(&state).await;
                tracing::debug!(
                    "New leaves decided. Latest block height: {}",
                    leaf.get_height(),
                );

                if let Some(client) = &self.relay_server_client {
                    let request_body = StateSignatureRequestBody {
                        key: self.key_pair.ver_key(),
                        state,
                        signature,
                    };
                    if let Err(error) = client
                        .post::<()>("api/state")
                        .body_binary(&request_body)
                        .unwrap()
                        .send()
                        .await
                    {
                        tracing::warn!("Error posting signature to the relay server: {:?}", error);
                    }
                }
            }
            Err(err) => {
                tracing::error!("Error generating light client state: {:?}", err)
            }
        }
    }

    /// Return a signature of a light client state at given height.
    pub async fn get_state_signature(&self, height: u64) -> Option<StateSignatureRequestBody> {
        let pool_guard = self.signatures.read().await;
        pool_guard.get_signature(height)
    }

    /// Sign the light client state at given height and store it.
    async fn sign_new_state(&self, state: &LightClientState) -> StateSignature {
        let msg: [CircuitField; 7] = state.into();
        let signature = StateSignatureScheme::sign(
            &(),
            self.key_pair.sign_key_ref(),
            msg,
            &mut rand::thread_rng(),
        )
        .unwrap();
        let mut pool_guard = self.signatures.write().await;
        pool_guard.push(
            state.block_height as u64,
            StateSignatureRequestBody {
                key: self.key_pair.ver_key(),
                state: state.clone(),
                signature: signature.clone(),
            },
        );
        tracing::debug!(
            "New signature added for block height {}",
            state.block_height
        );
        signature
    }
}

fn hash_bytes_to_field(bytes: &[u8]) -> Result<CircuitField, PrimitivesError> {
    // make sure that `mod_order` won't happen.
    let bytes_len = ((<CircuitField as PrimeField>::MODULUS_BIT_SIZE + 7) / 8 - 1) as usize;
    let elem = bytes
        .chunks(bytes_len)
        .map(CircuitField::from_le_bytes_mod_order)
        .collect::<Vec<_>>();
    Ok(VariableLengthRescueCRHF::<_, 1>::evaluate(elem)?[0])
}

fn form_light_client_state(
    leaf: &Leaf,
    stake_table_comm: &StakeTableCommitmentType,
) -> Result<LightClientState, PrimitivesError> {
    let header = leaf.get_block_header();
    let mut block_comm_root_bytes = vec![];
    header
        .block_merkle_tree_root
        .serialize_compressed(&mut block_comm_root_bytes)?;

    let mut fee_ledger_comm_bytes = vec![];
    header
        .fee_merkle_tree_root
        .serialize_compressed(&mut fee_ledger_comm_bytes)?;
    Ok(LightClientState {
        view_number: leaf.get_view_number().get_u64() as usize,
        block_height: leaf.get_height() as usize,
        block_comm_root: hash_bytes_to_field(&block_comm_root_bytes)?,
        fee_ledger_comm: hash_bytes_to_field(&fee_ledger_comm_bytes)?,
        stake_table_comm: *stake_table_comm,
    })
}

/// A rolling in-memory storage for the most recent light client state signatures.
#[derive(Debug, Default)]
pub struct StateSignatureMemStorage {
    pool: HashMap<u64, StateSignatureRequestBody>,
    deque: VecDeque<u64>,
}

impl StateSignatureMemStorage {
    pub fn push(&mut self, height: u64, signature: StateSignatureRequestBody) {
        self.pool.insert(height, signature);
        self.deque.push_back(height);
        if self.pool.len() > SIGNATURE_STORAGE_CAPACITY {
            self.pool.remove(&self.deque.pop_front().unwrap());
        }
    }

    pub fn get_signature(&self, height: u64) -> Option<StateSignatureRequestBody> {
        self.pool.get(&height).cloned()
    }
}

/// Type for stake table commitment
pub type StakeTableCommitmentType = (CircuitField, CircuitField, CircuitField);

/// Helper function for stake table commitment
pub fn static_stake_table_commitment(
    known_nodes_with_stakes: &[PeerConfig<BLSPubKey>],
    capacity: usize,
) -> (CircuitField, CircuitField, CircuitField) {
    let mut st = StakeTable::<BLSPubKey, StateVerKey, CircuitField>::new(capacity);
    known_nodes_with_stakes.iter().for_each(|peer| {
        // This `unwrap()` won't fail unless number of entries exceeds `capacity`
        st.register(
            *peer.stake_table_entry.get_key(),
            peer.stake_table_entry.get_stake(),
            peer.state_ver_key.clone(),
        )
        .unwrap();
    });
    st.advance();
    st.advance();
    // This `unwrap()` won't fail
    st.commitment(SnapshotVersion::LastEpochStart).unwrap()
}
