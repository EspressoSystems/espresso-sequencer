use hotshot_types::light_client::StateSignature;
use std::collections::{HashMap, VecDeque};

pub(crate) const SIGNATURE_POOL_CAPACITY: usize = 100;

#[derive(Debug, Default)]
pub struct StateSignaturePool {
    pool: HashMap<u64, StateSignature>,
    deque: VecDeque<u64>,
}

impl StateSignaturePool {
    pub fn push(&mut self, height: u64, signature: StateSignature) {
        self.pool.insert(height, signature);
        self.deque.push_back(height);
        if self.pool.len() > SIGNATURE_POOL_CAPACITY {
            self.pool.remove(&self.deque.pop_front().unwrap());
        }
    }

    pub fn get_signature(&self, height: u64) -> Option<StateSignature> {
        self.pool.get(&height).cloned()
    }
}
