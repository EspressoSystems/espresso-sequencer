//! Utilities for generating and storing the most recent light client state signatures.

use crate::context::SequencerContext;
use crate::{network, Leaf, SeqTypes};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use futures::stream::{Stream, StreamExt};
use hotshot::types::{Event, SignatureKey};
use hotshot_stake_table::vec_based::StakeTable;
use hotshot_types::light_client::StateVerKey;
use hotshot_types::signature_key::BLSPubKey;
use hotshot_types::traits::signature_key::StakeTableEntryType;
use hotshot_types::traits::stake_table::{SnapshotVersion, StakeTableScheme as _};
use hotshot_types::traits::state::ConsensusTime;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use surf_disco::Client;
use tide_disco::error::ServerError;

/// Types related to the underlying signature schemes.
pub type StateSignatureScheme =
    jf_primitives::signatures::schnorr::SchnorrSignatureScheme<ark_ed_on_bn254::EdwardsConfig>;
pub use hotshot_stake_table::vec_based::config::FieldType;
pub use hotshot_types::light_client::StateKeyPair;
pub use hotshot_types::light_client::StateSignature;
pub type LightClientState = hotshot_types::light_client::LightClientState<FieldType>;

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
    let state_relay_server_url = context.get_state_relay_server_url();
    while let Some(event) = events.next().await {
        // Trigger the light client signature hook when a new leaf is decided
        if let Event {
            event: hotshot_types::event::EventType::Decide { leaf_chain, .. },
            ..
        } = event
        {
            if let Some(leaf) = leaf_chain.first() {
                let state = form_light_client_state(leaf, stake_table_comm);
                let signature = context.sign_new_state(&state).await;
                tracing::info!(
                    "New leaves decided. Latest block height: {}, posting to relay server {:?}",
                    leaf.get_height(),
                    state_relay_server_url,
                );

                if let Some(state_relay_server_url) = state_relay_server_url {
                    let client: Client<ServerError> = Client::new(state_relay_server_url.clone());
                    client.connect(None).await;
                    let request_body = StateSignatureRequestBody {
                        key: key.clone(),
                        state,
                        signature,
                    };
                    if let Err(error) = client
                        .post::<()>("state/post")
                        .body_binary(&request_body)
                        .unwrap()
                        .send()
                        .await
                    {
                        tracing::warn!("Error posting signature to the relay server: {:?}", error);
                    }
                }
            }
        }
    }
    tracing::info!("And now his watch has ended.");
}

fn form_light_client_state(
    leaf: &Leaf,
    stake_table_comm: &StakeTableCommitmentType,
) -> LightClientState {
    // TODO(Chengyu): fill these `default()` with actual value
    LightClientState {
        view_number: leaf.get_view_number().get_u64() as usize,
        block_height: leaf.get_height() as usize,
        block_comm_root: FieldType::default(),
        fee_ledger_comm: FieldType::default(),
        stake_table_comm: *stake_table_comm,
    }
}

/// Request body to send to the state relay server
#[derive(Clone, Debug, CanonicalSerialize, CanonicalDeserialize, Serialize, Deserialize)]
pub struct StateSignatureRequestBody {
    pub key: StateVerKey,
    pub state: LightClientState,
    pub signature: StateSignature,
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
pub type StakeTableCommitmentType = (FieldType, FieldType, FieldType);

/// Helper function for stake table commitment
pub(crate) fn static_stake_table_commitment(
    known_nodes_with_stakes: &[<BLSPubKey as SignatureKey>::StakeTableEntry],
    state_ver_keys: &[StateVerKey],
    capacity: usize,
) -> (FieldType, FieldType, FieldType) {
    let mut st = StakeTable::<BLSPubKey, StateVerKey, FieldType>::new(capacity);
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
