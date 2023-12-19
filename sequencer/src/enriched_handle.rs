use hotshot::{traits::NodeImplementation, types::SystemContextHandle};
use hotshot_types::{light_client::StateSignature, traits::node_implementation::NodeType};
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

use crate::api::signature_pool::StateSignaturePool;

pub struct EnrichedSystemContextHandle<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    handle: SystemContextHandle<TYPES, I>,

    state_signature_pool: Arc<RwLock<StateSignaturePool>>,
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> Deref
    for EnrichedSystemContextHandle<TYPES, I>
{
    type Target = SystemContextHandle<TYPES, I>;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> DerefMut
    for EnrichedSystemContextHandle<TYPES, I>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handle
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> From<SystemContextHandle<TYPES, I>>
    for EnrichedSystemContextHandle<TYPES, I>
{
    fn from(handle: SystemContextHandle<TYPES, I>) -> Self {
        Self {
            handle,
            state_signature_pool: Default::default(),
        }
    }
}

impl<TYPES: NodeType, I: NodeImplementation<TYPES>> EnrichedSystemContextHandle<TYPES, I> {
    pub fn get_state_signature(&self, height: u64) -> Option<StateSignature> {
        let pool_guard = self.state_signature_pool.read().unwrap();
        pool_guard.get_signature(height)
    }

    pub fn add_new_state_signature(&mut self, height: u64, signature: StateSignature) {
        let mut pool_guard = self.state_signature_pool.write().unwrap();
        pool_guard.push(height, signature)
    }
}
