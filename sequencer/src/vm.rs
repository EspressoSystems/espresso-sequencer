use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::transaction::ApplicationTransaction;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub(crate) struct VmId(pub(crate) u64);

pub(crate) trait Vm {
    type Transaction: DeserializeOwned + Serialize + Sync + Send;
    fn id() -> VmId;
}

#[derive(Clone, Debug)]
pub(crate) struct TestVm;

impl Vm for TestVm {
    type Transaction = ApplicationTransaction;
    fn id() -> VmId {
        VmId(0)
    }
}
