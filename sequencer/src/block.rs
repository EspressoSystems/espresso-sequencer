use crate::{
    state::State,
    transaction::{GenesisTransaction, SequencerTransaction},
    Error,
};
use commit::{Commitment, Committable};
use hotshot::traits::Block as HotShotBlock;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub(crate) struct Block {
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
    #[allow(unused)]
    pub fn new(parent_state: Commitment<State>) -> Self {
        Self {
            parent_state,
            transactions: Default::default(),
        }
    }

    #[allow(unused)]
    pub fn genesis(txn: GenesisTransaction) -> Self {
        Self {
            parent_state: State::default().commit(),
            transactions: vec![SequencerTransaction::Genesis(txn)],
        }
    }
}
