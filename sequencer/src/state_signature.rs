//! Utilities for generating and storing the most recent light client state signatures.

use crate::context::SequencerContext;
use crate::{network, Leaf, SeqTypes};
use futures::stream::{Stream, StreamExt};
use hotshot::types::Event;
use hotshot_types::light_client::StateSignature;
use hotshot_types::{light_client::LightClientState, traits::state::ConsensusTime};
use std::collections::{HashMap, VecDeque};

/// Types related to the underlying signature schemes.
pub type StateSignatureScheme =
    jf_primitives::signatures::schnorr::SchnorrSignatureScheme<ark_ed_on_bn254::EdwardsConfig>;
pub use hotshot_stake_table::vec_based::config::FieldType as BaseField;

/// Capacity for the in memory signature storage.
const SIGNATURE_STORAGE_CAPACITY: usize = 100;

pub(super) async fn state_signature_loop<N>(
    context: SequencerContext<N>,
    mut events: impl Stream<Item = Event<SeqTypes>> + Unpin,
) where
    N: network::Type,
{
    tracing::debug!("Watching event stream for decided leaves.");
    while let Some(event) = events.next().await {
        tracing::info!("got event {:?}", event);

        // Trigger the light client signature hook when a new leaf is decided
        if let Event {
            event: hotshot_types::event::EventType::Decide { leaf_chain, .. },
            ..
        } = event
        {
            if let Some(leaf) = leaf_chain.first() {
                tracing::info!("New leaves decided. Newest leaf: {:?}", leaf);
                let new_state = form_light_client_state(leaf);
                context.sign_new_state(&new_state);
            }
        }
    }
    tracing::warn!("And now his watch has ended.");
}

fn form_light_client_state(leaf: &Leaf) -> LightClientState<BaseField> {
    // TODO(Chengyu): fill these `default()` with actual value
    LightClientState::<BaseField> {
        view_number: leaf.get_view_number().get_u64() as usize,
        block_height: leaf.get_height() as usize,
        block_comm_root: BaseField::default(),
        fee_ledger_comm: BaseField::default(),
        stake_table_comm: (
            BaseField::default(),
            BaseField::default(),
            BaseField::default(),
        ),
    }
}

/// A rolling in-memory storage for the most recent light client state signatures.
#[derive(Debug, Default)]
pub struct StateSignatureMemStorage {
    pool: HashMap<u64, StateSignature>,
    deque: VecDeque<u64>,
}

impl StateSignatureMemStorage {
    pub fn push(&mut self, height: u64, signature: StateSignature) {
        self.pool.insert(height, signature);
        self.deque.push_back(height);
        if self.pool.len() > SIGNATURE_STORAGE_CAPACITY {
            self.pool.remove(&self.deque.pop_front().unwrap());
        }
    }

    pub fn get_signature(&self, height: u64) -> Option<StateSignature> {
        self.pool.get(&height).cloned()
    }
}
