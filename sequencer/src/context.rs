use hotshot::{traits::NodeImplementation, types::SystemContextHandle};
use hotshot_types::{
    light_client::{LightClientState, StateKeyPair, StateSignature},
    traits::node_implementation::NodeType,
};
use jf_primitives::signatures::SignatureScheme;
use std::sync::{Arc, RwLock};

use crate::state_signature::{self, StateSignatureMemStorage};

/// The sequencer context contains a consensus handle and other sequencer specific information.
#[derive(Clone)]
pub struct SequencerContext<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// The consensus handle
    handle: SystemContextHandle<TYPES, I>,

    /// Index of this sequencer node
    #[allow(dead_code)]
    node_index: u64,

    /// Key pair for signing a new light client state
    state_key_pair: Arc<StateKeyPair>,

    /// The most recent light client state signatures
    state_signatures: Arc<RwLock<StateSignatureMemStorage>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> SequencerContext<TYPES, I> {
    /// Constructor
    pub fn new(
        handle: SystemContextHandle<TYPES, I>,
        node_index: u64,
        state_key_pair: StateKeyPair,
    ) -> Self {
        Self {
            handle,
            node_index,
            state_key_pair: Arc::new(state_key_pair),
            state_signatures: Default::default(),
        }
    }

    /// Return a reference to the underlying consensus handle.
    pub fn consensus(&self) -> &SystemContextHandle<TYPES, I> {
        &self.handle
    }

    /// Return a mutable reference to the underlying consensus handle.
    pub fn consensus_mut(&mut self) -> &mut SystemContextHandle<TYPES, I> {
        &mut self.handle
    }

    /// Return a signature of a light client state at given height.
    pub fn get_state_signature(&self, height: u64) -> Option<StateSignature> {
        let pool_guard = self.state_signatures.read().unwrap();
        pool_guard.get_signature(height)
    }

    /// Sign the light client state at given height and store it.
    pub fn sign_new_state(&self, state: &LightClientState<state_signature::BaseField>) {
        let state_msg: [state_signature::BaseField; 7] = state.into();
        let state_signature = state_signature::StateSignatureScheme::sign(
            &(),
            self.state_key_pair.sign_key_ref(),
            state_msg,
            &mut rand::thread_rng(),
        )
        .unwrap();
        let mut pool_guard = self.state_signatures.write().unwrap();
        pool_guard.push(state.block_height as u64, state_signature);
    }
}
