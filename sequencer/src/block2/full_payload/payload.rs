use crate::{
    block2::{
        full_payload::{
            iter::{Index, Iter},
            ns_table::{NsTable, NsTableBuilder},
            tx_proof::TxProof,
        },
        namespace_payload::{NsPayload, NsPayloadBuilder, NsPayloadRange},
    },
    NamespaceId, Transaction,
};
use commit::{Commitment, Committable};
use hotshot_query_service::availability::QueryablePayload;
use hotshot_types::{
    traits::BlockPayload,
    utils::BuilderCommitment,
    vid::{VidCommon, VidSchemeType},
};
use jf_primitives::vid::VidScheme;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{collections::HashMap, fmt::Display};

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
        PayloadByteLen(self.payload.len())
    }
    pub fn read_ns_payload(&self, range: &NsPayloadRange) -> &NsPayload {
        NsPayload::from_bytes_slice(&self.payload[range.as_block_range()])
    }

    /// Like [`QueryablePayload::transaction_with_proof`] except without the
    /// proof.
    pub fn transaction(&self, index: &Index) -> Option<Transaction> {
        if !self.ns_table().in_bounds(index.ns()) {
            return None; // error: ns index out of bounds
        }
        let ns_id = self.ns_table.read_ns_id(index.ns());
        let ns_range = self.ns_table().ns_range(index.ns(), &self.byte_len());
        let ns_payload = self.read_ns_payload(&ns_range);
        ns_payload.export_tx(&ns_id, index.tx())
    }
}

impl BlockPayload for Payload {
    type Error = crate::Error;
    type Transaction = Transaction;

    // TODO change to `NsTable` after `BlockPayload` trait has been changed to
    // remove `Self::Metadata` args.
    type Metadata = Vec<u8>;

    // TODO change `BlockPayload::Encode` trait bounds to enable copyless encoding such as AsRef<[u8]>
    // https://github.com/EspressoSystems/HotShot/issues/2115
    type Encode<'a> = std::iter::Cloned<<&'a Vec<u8> as IntoIterator>::IntoIter>;

    // TODO change `BlockPayload` trait: return type should not include `Self::Metadata`
    fn from_transactions(
        transactions: impl IntoIterator<Item = Self::Transaction>,
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
        let metadata = ns_table.as_bytes_slice().to_vec();
        Ok((Self { payload, ns_table }, metadata))
    }

    // TODO change `BlockPayload` trait: arg type `&Self::Metadata` should not
    // be a reference.
    fn from_bytes<I>(block_payload_bytes: I, ns_table: &Self::Metadata) -> Self
    where
        I: Iterator<Item = u8>,
    {
        Self {
            payload: block_payload_bytes.into_iter().collect(),
            ns_table: NsTable::from_bytes_vec(A(()), ns_table.clone()),
        }
    }

    // TODO change `BlockPayload` trait: return type should not include `Self::Metadata`
    fn genesis() -> (Self, Self::Metadata) {
        Self::from_transactions([]).unwrap()
    }

    // TODO change `BlockPayload::Encode` trait bounds to enable copyless encoding such as AsRef<[u8]>
    // https://github.com/EspressoSystems/HotShot/issues/2115
    fn encode(&self) -> Result<Self::Encode<'_>, Self::Error> {
        Ok(self.payload.iter().cloned())
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

    // TODO change `BlockPayload` trait: remove arg `Self::Metadata`
    // TODO change return type so it's not a reference! :facepalm:
    fn get_transactions(&self, _metadata: &Self::Metadata) -> &Vec<Self::Transaction> {
        todo!()
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
    fn commit(&self) -> commit::Commitment<Self> {
        todo!()
    }
}

/// TODO explain: ZST to unlock visibility in other modules. can only be
/// constructed in this module.
pub struct A(());

pub struct PayloadByteLen(usize);

impl PayloadByteLen {
    pub fn from_vid_common(common: &VidCommon) -> Self {
        Self(VidSchemeType::get_payload_byte_len(common))
    }
    pub fn is_consistent(&self, common: &VidCommon) -> bool {
        self.0 == VidSchemeType::get_payload_byte_len(common)
    }

    // TODO restrict visibility?
    pub fn as_usize(&self) -> usize {
        self.0
    }
}
