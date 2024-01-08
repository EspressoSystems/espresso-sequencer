use crate::state_signature::{
    LightClientState, StateKeyPair, StateSignature, StateSignatureScheme,
};
use hotshot::types::SystemContextHandle;
use jf_primitives::signatures::SignatureScheme;
use std::sync::{Arc, RwLock};

use crate::{
    network,
    state_signature::{self, StateSignatureMemStorage},
    Node, SeqTypes,
};

/// The consensus handle
pub type Consensus<N> = SystemContextHandle<SeqTypes, Node<N>>;

/// The sequencer context contains a consensus handle and other sequencer specific information.
pub struct SequencerContext<N: network::Type> {
    /// The consensus handle
    handle: Consensus<N>,

    /// Index of this sequencer node
    #[allow(dead_code)]
    node_index: u64,

    /// Key pair for signing a new light client state
    state_key_pair: Arc<StateKeyPair>,

    /// The most recent light client state signatures
    state_signatures: Arc<RwLock<StateSignatureMemStorage>>,
}

impl<N: network::Type> Clone for SequencerContext<N> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            node_index: self.node_index,
            state_key_pair: self.state_key_pair.clone(),
            state_signatures: self.state_signatures.clone(),
        }
    }
}

impl<N: network::Type> SequencerContext<N> {
    /// Constructor
    pub fn new(handle: Consensus<N>, node_index: u64, state_key_pair: StateKeyPair) -> Self {
        Self {
            handle,
            node_index,
            state_key_pair: Arc::new(state_key_pair),
            state_signatures: Default::default(),
        }
    }

    /// Return a reference to the underlying consensus handle.
    pub fn consensus(&self) -> &Consensus<N> {
        &self.handle
    }

    /// Return a mutable reference to the underlying consensus handle.
    pub fn consensus_mut(&mut self) -> &mut Consensus<N> {
        &mut self.handle
    }

    /// Return a signature of a light client state at given height.
    pub fn get_state_signature(&self, height: u64) -> Option<StateSignature> {
        let pool_guard = self.state_signatures.read().unwrap();
        pool_guard.get_signature(height)
    }

    /// Sign the light client state at given height and store it.
    pub fn sign_new_state(&self, state: &LightClientState) {
        let state_msg: [state_signature::FieldType; 7] = state.into();
        let state_signature = StateSignatureScheme::sign(
            &(),
            self.state_key_pair.sign_key_ref(),
            state_msg,
            &mut rand::thread_rng(),
        )
        .unwrap();
        let mut pool_guard = self.state_signatures.write().unwrap();
        pool_guard.push(state.block_height as u64, state_signature);
        tracing::info!(
            "New signature added for block height {}",
            state.block_height
        );
    }
}
