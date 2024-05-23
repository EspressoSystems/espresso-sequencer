use crate::{
    block2::{
        full_payload::ns_table::{NsIndex, NsTable, NsTableBuilder},
        namespace_payload::{Index, Iter, NsPayload, NsPayloadBuilder, NsPayloadRange, TxProof},
    },
    NamespaceId, NodeState, Transaction,
};
use committable::{Commitment, Committable};
use hotshot_query_service::availability::QueryablePayload;
use hotshot_types::{
    traits::{BlockPayload, EncodeBytes},
    utils::BuilderCommitment,
    vid::{VidCommon, VidSchemeType},
};
use jf_vid::VidScheme;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{collections::HashMap, fmt::Display, sync::Arc};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Payload {
    // Concatenated payload bytes for each namespace
    #[serde(with = "base64_bytes")]
    payload: Vec<u8>,

    ns_table: NsTable,
}

impl Payload {
    pub fn as_byte_slice(&self) -> &[u8] {
        &self.payload
    }
    pub fn ns_table(&self) -> &NsTable {
        &self.ns_table
    }
    pub fn byte_len(&self) -> PayloadByteLen {
        PayloadByteLen(self.payload.len().try_into().unwrap())
    }
    pub fn read_ns_payload(&self, range: &NsPayloadRange) -> &NsPayload {
        NsPayload::from_bytes_slice(&self.payload[range.as_block_range()])
    }

    /// Convenience wrapper for [`Self::read_ns_payload`].
    pub fn ns_payload(&self, index: &NsIndex) -> &NsPayload {
        let ns_payload_range = self.ns_table().ns_range(index, &self.byte_len());
        self.read_ns_payload(&ns_payload_range)
    }

    /// Like [`QueryablePayload::transaction_with_proof`] except without the
    /// proof.
    pub fn transaction(&self, index: &Index) -> Option<Transaction> {
        if !self.ns_table().in_bounds(index.ns()) {
            return None; // error: ns index out of bounds
        }
        let ns_id = self.ns_table.read_ns_id(index.ns());
        let ns_payload = self.ns_payload(index.ns());
        ns_payload.export_tx(&ns_id, index.tx())
    }
}

impl BlockPayload for Payload {
    type Error = crate::Error;
    type Transaction = Transaction;
    type Instance = NodeState;
    type Metadata = NsTable;

    // TODO change `BlockPayload` trait: return type should not include `Self::Metadata`
    fn from_transactions(
        transactions: impl IntoIterator<Item = Self::Transaction>,
        _instance_state: &Self::Instance, // TODO use this arg
    ) -> Result<(Self, Self::Metadata), Self::Error> {
        // add each tx to its namespace
        let mut ns_builders = HashMap::<NamespaceId, NsPayloadBuilder>::new();
        for tx in transactions.into_iter() {
            let ns_builder = ns_builders.entry(tx.namespace()).or_default();
            ns_builder.append_tx(tx);
        }

        // build block payload and namespace table
        let mut payload = Vec::new();
        let mut ns_table_builder = NsTableBuilder::new();
        for (ns_id, ns_builder) in ns_builders {
            payload.extend(ns_builder.into_bytes());
            ns_table_builder.append_entry(ns_id, payload.len());
        }
        let ns_table = ns_table_builder.into_ns_table();
        let metadata = ns_table.clone();
        Ok((Self { payload, ns_table }, metadata))
    }

    // TODO avoid cloning the entire payload here?
    fn from_bytes(block_payload_bytes: &[u8], ns_table: &Self::Metadata) -> Self {
        Self {
            payload: block_payload_bytes.to_vec(),
            ns_table: ns_table.clone(),
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

    // TODO change `BlockPayload` trait: remove arg `Self::Metadata`
    fn transaction_commitments(
        &self,
        _metadata: &Self::Metadata,
    ) -> Vec<Commitment<Self::Transaction>> {
        todo!()
    }

    // TODO change `BlockPayload` trait: remove arg `Self::Metadata`
    fn builder_commitment(&self, _metadata: &Self::Metadata) -> BuilderCommitment {
        let ns_table_bytes = self.ns_table.as_bytes_slice();
        let mut digest = sha2::Sha256::new();
        digest.update((self.payload.len() as u64).to_le_bytes());
        digest.update((ns_table_bytes.len() as u64).to_le_bytes());
        digest.update(&self.payload);
        digest.update(ns_table_bytes);
        BuilderCommitment::from_raw_digest(digest.finalize())
    }

    fn transactions<'a>(
        &'a self,
        metadata: &'a Self::Metadata,
    ) -> impl 'a + Iterator<Item = Self::Transaction> {
        self.enumerate(metadata).map(|(_, t)| t)
    }
}

impl QueryablePayload for Payload {
    // TODO change `QueryablePayload` trait: remove `Ord` bound from `TransactionIndex`
    type TransactionIndex = Index;
    type Iter<'a> = Iter<'a>;
    type InclusionProof = TxProof;

    // TODO change `QueryablePayload` trait: remove arg `Self::Metadata`
    fn len(&self, _meta: &Self::Metadata) -> usize {
        // Counting txs is nontrivial. The easiest solution is to consume an
        // iterator. If performance is a concern then we could cache this count
        // on construction of `Payload`.
        self.iter(_meta).count()
    }

    // TODO change `QueryablePayload` trait: remove arg `Self::Metadata`
    fn iter<'a>(&'a self, _meta: &'a Self::Metadata) -> Self::Iter<'a> {
        Iter::new(self)
    }

    // TODO change `QueryablePayload` trait: remove arg `Self::Metadata`
    fn transaction_with_proof(
        &self,
        _meta: &Self::Metadata,
        _index: &Self::TransactionIndex,
    ) -> Option<(Self::Transaction, Self::InclusionProof)> {
        todo!()
    }
}

impl Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Committable for Payload {
    fn commit(&self) -> committable::Commitment<Self> {
        todo!()
    }
}

impl EncodeBytes for Payload {
    fn encode(&self) -> Arc<[u8]> {
        Arc::from(self.payload.as_ref())
    }
}

/// TODO explain: ZST to unlock visibility in other modules. can only be
/// constructed in this module.
pub struct A(());

pub struct PayloadByteLen(u32);

impl PayloadByteLen {
    pub fn from_vid_common(common: &VidCommon) -> Self {
        Self(VidSchemeType::get_payload_byte_len(common))
    }
    pub fn is_consistent(&self, common: &VidCommon) -> bool {
        self.0 == VidSchemeType::get_payload_byte_len(common)
    }

    // TODO restrict visibility?
    pub fn as_usize(&self) -> usize {
        self.0.try_into().unwrap()
    }
}

#[cfg(any(test, feature = "testing"))]
impl hotshot_types::traits::block_contents::TestableBlock for Payload {
    fn genesis() -> Self {
        BlockPayload::from_transactions([], &Default::default())
            .unwrap()
            .0
    }

    fn txn_count(&self) -> u64 {
        self.len(&self.ns_table) as u64
    }
}
