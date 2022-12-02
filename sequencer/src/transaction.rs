use commit::{Commitment, Committable};
use hotshot_types::traits::block_contents::Transaction as HotShotTransaction;
use serde::{Deserialize, Serialize};

use crate::{chain_variables::ChainVariables, vm::VmId};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Transaction {
    vm: VmId,
    payload: Vec<u8>,
}

impl Transaction {
    #[allow(unused)]
    pub(crate) fn new(vm: VmId, payload: Vec<u8>) -> Self {
        Self { vm, payload }
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

impl HotShotTransaction for SequencerTransaction {}

impl Committable for Transaction {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("Transaction")
            .u64_field("vm", self.vm.0)
            .var_size_bytes(&self.payload) // TODO how can we specify a field name like "payload"
            .finalize()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GenesisTransaction {
    pub chain_variables: ChainVariables,
}

impl Committable for GenesisTransaction {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("GenesisTransaction")
            .field("chain_variables", self.chain_variables.commit())
            .finalize()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// A transaction tht can be either a CAP transaction or a collect reward transaction
pub enum SequencerTransaction {
    Genesis(GenesisTransaction),
    Wrapped(Transaction),
}

impl Committable for SequencerTransaction {
    fn commit(&self) -> Commitment<Self> {
        let bytes = bincode::serialize(self).unwrap(); // TODO not safe unwrap?
        commit::RawCommitmentBuilder::new("SequencerTransaction")
            .var_size_bytes(&bytes)
            .finalize()
    }
}
