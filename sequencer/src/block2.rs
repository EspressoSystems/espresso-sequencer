use crate::{BlockBuildingSnafu, Transaction};
use commit::{Commitment, Committable};
use hotshot_query_service::availability::QueryablePayload;
use hotshot_types::traits::BlockPayload;
use serde::{Deserialize, Serialize};
use snafu::OptionExt;

pub mod entry;
pub mod payload;
pub mod queryable;
mod tables;
pub mod tx_iterator;

use crate::block2::entry::TxTableEntryWord;
use crate::block2::payload::NameSpaceTable;
use payload::Payload;

impl BlockPayload for Payload<TxTableEntryWord> {
    type Error = crate::Error;
    type Transaction = Transaction;
    type Metadata = Vec<u8>;

    type Encode<'a> = std::iter::Cloned<<&'a Vec<u8> as IntoIterator>::IntoIter>;

    /// Returns (Self, metadata).
    ///
    /// `metadata` is a bytes representation of the namespace table.
    /// Why bytes? To make it easy to move metdata into payload in the future.
    ///
    /// Namespace table defined as follows for j>0:
    /// word[0]:    [number of entries in namespace table]
    /// word[2j-1]: [id for the jth namespace]
    /// word[2j]:   [end byte index of the jth namespace in the payload]
    ///
    /// Thus, for j>2 the jth namespace payload bytes range is word[2(j-1)]..word[2j].
    /// Edge case: for j=1 the jth namespace start index is implicitly 0.
    ///
    /// Word type is `TxTableEntry`.
    /// TODO(746) don't use `TxTableEntry`; make a different type for type safety.
    ///
    /// TODO final entry should be implicit:
    /// https://github.com/EspressoSystems/espresso-sequencer/issues/757
    ///
    /// TODO(746) refactor and make pretty "table" code for tx, namespace tables?
    fn from_transactions(
        txs: impl IntoIterator<Item = Self::Transaction>,
    ) -> Result<(Self, Self::Metadata), Self::Error> {
        let structured_payload = Payload::from_txs(txs)?;

        Some((
            structured_payload.clone(),
            structured_payload.get_ns_table_bytes(),
        ))
        .context(BlockBuildingSnafu)
    }

    // TODO(746) from_bytes doesn't need `metadata`!
    fn from_bytes<I>(encoded_transactions: I, _metadata: &Self::Metadata) -> Self
    where
        I: Iterator<Item = u8>,
    {
        Self {
            raw_payload: encoded_transactions.into_iter().collect(),
            tx_table_len_proof: Default::default(),
            ns_table: NameSpaceTable::from_bytes(_metadata),
            namespaces: Default::default(), // TODO (philippe) update
        }
    }

    fn genesis() -> (Self, Self::Metadata) {
        Self::from_transactions([]).unwrap()
    }

    fn encode(&self) -> Result<Self::Encode<'_>, Self::Error> {
        Ok(self.raw_payload.iter().cloned())
    }

    fn transaction_commitments(&self, meta: &Self::Metadata) -> Vec<Commitment<Self::Transaction>> {
        self.enumerate(meta).map(|(_, tx)| tx.commit()).collect()
    }
}
