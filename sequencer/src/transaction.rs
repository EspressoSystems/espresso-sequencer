use commit::{Commitment, Committable};
use hotshot_types::traits::block_contents::Transaction as HotShotTransaction;
use serde::{Deserialize, Serialize};

use crate::vm::{Vm, VmId, VmTransaction};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Transaction {
    vm: VmId,
    payload: Vec<u8>,
}

impl Transaction {
    pub fn new(vm: VmId, payload: Vec<u8>) -> Self {
        Self { vm, payload }
    }

    pub fn vm(&self) -> VmId {
        self.vm
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn as_vm<V: Vm>(&self, vm: &V) -> Option<V::Transaction> {
        if self.vm() == vm.id() {
            V::Transaction::decode(self.payload())
        } else {
            None
        }
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        use rand::Rng;
        let len = rng.gen_range(0..100);
        Self::new(
            VmId(rng.gen_range(0..10)),
            (0..len).map(|_| rand::random::<u8>()).collect(),
        )
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ApplicationTransaction(Vec<u8>);

impl ApplicationTransaction {
    #[allow(unused)]
    pub(crate) fn new(payload: Vec<u8>) -> Self {
        Self(payload)
    }
}

impl VmTransaction for ApplicationTransaction {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    fn decode(bytes: &[u8]) -> Option<Self> {
        bincode::deserialize(bytes).ok()
    }
}

impl HotShotTransaction for Transaction {}

impl Committable for Transaction {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("Transaction")
            .u64_field("vm", self.vm.0)
            .var_size_bytes(&self.payload) // TODO how can we specify a field name like "payload"
            .finalize()
    }
}
