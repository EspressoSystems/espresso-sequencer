//! Utilities for generating and storing the most recent light client state signatures.

use crate::context::SequencerContext;
use crate::{network, Leaf, SeqTypes};
use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;
use futures::stream::{Stream, StreamExt};
use hotshot::types::{Event, SignatureKey};
use hotshot_stake_table::vec_based::StakeTable;
use hotshot_types::light_client::{
    CircuitField, LightClientState, StateSignatureRequestBody, StateVerKey,
};
use hotshot_types::signature_key::BLSPubKey;
use hotshot_types::traits::node_implementation::ConsensusTime;
use hotshot_types::traits::signature_key::StakeTableEntryType;
use hotshot_types::traits::stake_table::{SnapshotVersion, StakeTableScheme as _};
use jf_primitives::crhf::{VariableLengthRescueCRHF, CRHF};
use jf_primitives::errors::PrimitivesError;
use std::collections::{HashMap, VecDeque};

/// A relay server that's collecting and serving the light client state signatures
pub mod relay_server;

/// Capacity for the in memory signature storage.
const SIGNATURE_STORAGE_CAPACITY: usize = 100;

pub(super) async fn state_signature_loop<N>(
    context: SequencerContext<N>,
    mut events: impl Stream<Item = Event<SeqTypes>> + Unpin,
) where
    N: network::Type,
{
    tracing::info!("State signature watcher is watching event stream for decided leaves.");
    let stake_table_comm = context.get_stake_table_comm();
    let key = context.get_state_ver_key();
    let client = context.get_state_relay_server_client();
    while let Some(event) = events.next().await {
        // Trigger the light client signature hook when a new leaf is decided
        if let Event {
            event: hotshot_types::event::EventType::Decide { leaf_chain, .. },
            ..
        } = event
        {
            if let Some((leaf, _)) = leaf_chain.first() {
                match form_light_client_state(leaf, stake_table_comm) {
                    Ok(state) => {
                        let signature = context.sign_new_state(&state).await;
                        tracing::debug!(
                            "New leaves decided. Latest block height: {}",
                            leaf.get_height(),
                        );

                        if let Some(client) = client {
                            let request_body = StateSignatureRequestBody {
                                key: key.clone(),
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
                                tracing::warn!(
                                    "Error posting signature to the relay server: {:?}",
                                    error
                                );
                            }
                        }
                    }
                    Err(err) => tracing::error!("Error generating light client state: {:?}", err),
                }
            }
        }
    }
    tracing::info!("And now his watch has ended.");
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
    known_nodes_with_stakes: &[<BLSPubKey as SignatureKey>::StakeTableEntry],
    state_ver_keys: &[StateVerKey],
    capacity: usize,
) -> (CircuitField, CircuitField, CircuitField) {
    let mut st = StakeTable::<BLSPubKey, StateVerKey, CircuitField>::new(capacity);
    known_nodes_with_stakes
        .iter()
        .zip(state_ver_keys)
        .for_each(|(entry, schnorr_key)| {
            // This `unwrap()` wont fail unless number of entries exceeds `capacity`
            st.register(*entry.get_key(), entry.get_stake(), schnorr_key.clone())
                .unwrap();
        });
    st.advance();
    st.advance();
    // This `unwrap()` won't fail
    st.commitment(SnapshotVersion::LastEpochStart).unwrap()
}
