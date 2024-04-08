use crate::{NamespaceId, Transaction};
use commit::{Commitment, Committable};
use hotshot_types::{traits::BlockPayload, utils::BuilderCommitment};
use ns_payload::NamespacePayloadBuilder;
use payload_bytes::{ns_id_as_bytes, ns_offset_as_bytes, num_nss_as_bytes};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{collections::HashMap, fmt::Display};

mod ns_iter;
mod ns_payload;
mod ns_proof;
mod payload_bytes;
mod tx_iter;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Payload {
    // Concatenated payload bytes for each namespace
    #[serde(with = "base64_bytes")]
    payload: Vec<u8>,

    // namespace table bytes
    ns_table: Vec<u8>,
    // TODO Revisit caching of frequently used items
    //
    // TODO type should be `OnceLock<SmallRangeProofType>` instead of `OnceLock<Option<SmallRangeProofType>>`.
    // We can correct this after `once_cell_try` is stabilized <https://github.com/rust-lang/rust/issues/109737>.
    // #[derivative(Hash = "ignore")]
    // #[derivative(PartialEq = "ignore")]
    // #[serde(skip)]
    // pub tx_table_len_proof: OnceLock<Option<SmallRangeProofType>>,
}

impl BlockPayload for Payload {
    type Error = crate::Error;
    type Transaction = Transaction;
    type Metadata = Vec<u8>; // namespace table bytes

    // TODO change `BlockPayload::Encode` trait bounds to enable copyless encoding such as AsRef<[u8]>
    // https://github.com/EspressoSystems/HotShot/issues/2115
    type Encode<'a> = std::iter::Cloned<<&'a Vec<u8> as IntoIterator>::IntoIter>;

    // TODO change `BlockPayload` trait: return type should not include `Self::Metadata`
    fn from_transactions(
        transactions: impl IntoIterator<Item = Self::Transaction>,
    ) -> Result<(Self, Self::Metadata), Self::Error> {
        // add each tx to its namespace
        let mut namespaces = HashMap::<NamespaceId, NamespacePayloadBuilder>::new();
        for tx in transactions.into_iter() {
            let namespace = namespaces.entry(tx.namespace()).or_default();
            namespace.append_tx(tx);
        }

        // build block payload and namespace table
        let mut payload = Vec::new();
        let mut ns_table = Vec::from(num_nss_as_bytes(namespaces.len()));
        for (ns_id, namespace) in namespaces {
            payload.extend(namespace.into_bytes());
            ns_table.extend(ns_id_as_bytes(ns_id));
            ns_table.extend(ns_offset_as_bytes(payload.len()));
        }
        Ok((
            Self {
                payload,
                ns_table: ns_table.clone(),
            },
            ns_table,
        ))
    }

    fn from_bytes<I>(encoded_transactions: I, ns_table: &Self::Metadata) -> Self
    where
        I: Iterator<Item = u8>,
    {
        Self {
            payload: encoded_transactions.into_iter().collect(),
            ns_table: ns_table.clone(), // TODO don't clone ns_table
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
        let mut digest = sha2::Sha256::new();
        digest.update((self.payload.len() as u64).to_le_bytes());
        digest.update((self.ns_table.len() as u64).to_le_bytes());
        digest.update(&self.payload);
        digest.update(&self.ns_table);
        BuilderCommitment::from_raw_digest(digest.finalize())
    }

    // TODO change `BlockPayload` trait: remove arg `Self::Metadata`
    // TODO change return type so it's not a reference! :facepalm:
    fn get_transactions(&self, _metadata: &Self::Metadata) -> &Vec<Self::Transaction> {
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
