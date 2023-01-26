use crate::{
    state::State,
    transaction::{GenesisTransaction, SequencerTransaction},
    vm::Vm,
    Error,
};
use commit::{Commitment, Committable};
use hotshot::traits::Block as HotShotBlock;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Block {
    pub(crate) parent_state: Commitment<State>,
    pub(crate) transactions: Vec<SequencerTransaction>,
}

impl HotShotBlock for Block {
    type Error = Error;

    type Transaction = SequencerTransaction;

    fn add_transaction_raw(
        &self,
        tx: &Self::Transaction,
    ) -> std::result::Result<Self, Self::Error> {
        let mut new = self.clone();
        new.transactions.push(tx.clone());
        Ok(new)
    }

    fn contained_transactions(&self) -> std::collections::HashSet<Commitment<Self::Transaction>> {
        self.transactions.iter().map(|tx| tx.commit()).collect()
    }
}

impl Committable for Block {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("Block Comm")
            .field("Block parent", self.parent_state)
            .array_field(
                "txns",
                &self
                    .transactions
                    .iter()
                    .map(|x| x.commit())
                    .collect::<Vec<_>>(),
            )
            .finalize()
    }
}

impl Block {
    pub fn new(parent_state: Commitment<State>) -> Self {
        Self {
            parent_state,
            transactions: Default::default(),
        }
    }

    pub fn genesis(txn: GenesisTransaction) -> Self {
        Self {
            parent_state: State::default().commit(),
            transactions: vec![SequencerTransaction::Genesis(txn)],
        }
    }

    /// Visit all transactions in this block.
    pub fn transactions(&self) -> impl ExactSizeIterator<Item = &SequencerTransaction> + '_ {
        self.transactions.iter()
    }

    /// Visit the valid transactions for `V` in this block.
    pub fn vm_transactions<'a, V: Vm>(
        &'a self,
        vm: &'a V,
    ) -> impl Iterator<Item = V::Transaction> + 'a {
        self.transactions().filter_map(|txn| txn.as_vm(vm))
    }
}
