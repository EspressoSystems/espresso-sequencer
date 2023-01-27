use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::transaction::{ApplicationTransaction, Transaction};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, From)]
pub struct VmId(pub(crate) u64);

pub trait Vm {
    type Transaction: VmTransaction;
    fn id(&self) -> VmId;

    fn wrap(&self, txn: &Self::Transaction) -> Transaction {
        Transaction::new(self.id(), txn.encode())
    }
}

pub trait VmTransaction: Sized + Send + Sync {
    fn encode(&self) -> Vec<u8>;
    fn decode(bytes: &[u8]) -> Option<Self>;
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct TestVm;

impl Vm for TestVm {
    type Transaction = ApplicationTransaction;
    fn id(&self) -> VmId {
        VmId(0)
    }
}
