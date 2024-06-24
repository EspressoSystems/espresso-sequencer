use crate::{BlockBuildingSnafu, Transaction};
use committable::{Commitment, Committable};
use hotshot_query_service::availability::QueryablePayload;
use hotshot_types::traits::{BlockPayload, EncodeBytes};
use hotshot_types::utils::BuilderCommitment;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use snafu::OptionExt;
use std::sync::Arc;

pub mod entry;
pub mod payload;
pub mod queryable;
pub mod tables;
pub mod tx_iterator;

use entry::TxTableEntryWord;
use payload::Payload;
use tables::NameSpaceTable;

pub type NsTable = NameSpaceTable<TxTableEntryWord>;
impl EncodeBytes for Payload<TxTableEntryWord> {
    fn encode(&self) -> Arc<[u8]> {
        Arc::from(self.raw_payload.clone())
    }
}
impl BlockPayload for Payload<TxTableEntryWord> {
    type Error = crate::Error;
    type Transaction = Transaction;
    type Instance = NodeState;
    type Metadata = NsTable;

    /// Returns (Self, metadata).
    ///
    /// `metadata` is a bytes representation of the namespace table.
    /// Why bytes? To make it easy to move metadata into payload in the future.
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
        instance_state: &Self::Instance,
    ) -> Result<(Self, Self::Metadata), Self::Error> {
        let payload = Payload::from_txs(txs, &instance_state.chain_config)?;
        let ns_table = payload.get_ns_table().clone(); // TODO don't clone ns_table
        Some((payload, ns_table)).context(BlockBuildingSnafu)
    }

    fn from_bytes(encoded_transactions: &[u8], metadata: &Self::Metadata) -> Self {
        Self {
            raw_payload: encoded_transactions.to_vec(),
            ns_table: metadata.clone(), // TODO don't clone ns_table
        }
    }

    // TODO remove
    fn genesis() -> (Self, Self::Metadata) {
        // this is only called from `Leaf::genesis`. Since we are
        // passing empty list, max_block_size is irrelevant so we can
        // use the mock NodeState. A future update to HotShot should
        // make a change there to remove the need for this workaround.

        Self::from_transactions([], &NodeState::mock()).unwrap()
    }

    fn transaction_commitments(&self, meta: &Self::Metadata) -> Vec<Commitment<Self::Transaction>> {
        self.enumerate(meta).map(|(_, tx)| tx.commit()).collect()
    }

    /// Generate commitment that builders use to sign block options.
    fn builder_commitment(&self, metadata: &Self::Metadata) -> BuilderCommitment {
        let mut digest = sha2::Sha256::new();
        digest.update((self.raw_payload.len() as u64).to_le_bytes());
        digest.update((self.ns_table.bytes.len() as u64).to_le_bytes());
        digest.update((metadata.bytes.len() as u64).to_le_bytes());
        digest.update(&self.raw_payload);
        digest.update(&self.ns_table.bytes);
        digest.update(&metadata.bytes);
        BuilderCommitment::from_raw_digest(digest.finalize())
    }

    fn transactions<'a>(
        &'a self,
        metadata: &'a Self::Metadata,
    ) -> impl 'a + Iterator<Item = Self::Transaction> {
        self.enumerate(metadata).map(|(_, t)| t)
    }
}
