use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::transaction::ApplicationTransaction;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, From)]
pub struct VmId(pub(crate) u64);

pub trait Vm {
    type Transaction: VmTransaction;
    fn id() -> VmId;
}

pub trait VmTransaction: Sized + Send + Sync {
    fn encode(&self) -> Vec<u8>;
    fn decode(bytes: &[u8]) -> Option<Self>;
}

#[derive(Clone, Debug)]
pub(crate) struct TestVm;

impl Vm for TestVm {
    type Transaction = ApplicationTransaction;
    fn id() -> VmId {
        VmId(0)
    }
}
