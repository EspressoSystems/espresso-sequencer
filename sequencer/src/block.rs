use crate::{
    transaction::{GenesisTransaction, SequencerTransaction},
    vm::Vm,
    Error,
};
use commit::{Commitment, Committable};
use hotshot::traits::Block as HotShotBlock;
use hotshot_query_service::QueryableBlock;
use hotshot_types::traits::state::TestableBlock;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Block {
    pub(crate) transactions: Vec<SequencerTransaction>,
}

// TODO(#345) implement
impl QueryableBlock for Block {
    type TransactionIndex = u64;
    type InclusionProof = ();
    type Iter<'a> = Box<dyn Iterator<Item = u64>>;

    fn len(&self) -> usize {
        self.transactions.len()
    }

    fn transaction_with_proof(
        &self,
        _index: &Self::TransactionIndex,
    ) -> Option<(&Self::Transaction, Self::InclusionProof)> {
        unimplemented!()
    }

    fn transaction(&self, index: &Self::TransactionIndex) -> Option<&Self::Transaction> {
        self.transactions.get(*index as usize)
    }

    fn iter(&self) -> Self::Iter<'_> {
        Box::new(0..self.len() as u64)
    }
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

    fn new() -> Self {
        Self {
            transactions: vec![],
        }
    }
}

#[cfg(any(test, feature = "testing"))]
impl TestableBlock for Block {
    fn genesis() -> Self {
        Block::genesis(Default::default())
    }

    fn txn_count(&self) -> u64 {
        self.transactions.len() as u64
    }
}

// Required for TestableBlock
#[cfg(any(test, feature = "testing"))]
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Committable for Block {
    fn commit(&self) -> Commitment<Self> {
        commit::RawCommitmentBuilder::new("Block Comm")
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
    pub fn genesis(txn: GenesisTransaction) -> Self {
        Self {
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
