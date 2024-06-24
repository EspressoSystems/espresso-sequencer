use crate::NamespaceId;
use core::fmt;
use hotshot_types::traits::EncodeBytes;
use serde::{Deserialize, Serialize};
use std::mem::size_of;

use std::marker::PhantomData;

use std::ops::Range;
use std::sync::Arc;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use derivative::Derivative;
use hotshot::traits::BlockPayload;
use hotshot_types::vid::{
    vid_scheme, LargeRangeProofType, VidCommitment, VidCommon, VidSchemeType,
};
use jf_vid::{
    payload_prover::{PayloadProver, Statement},
    VidScheme,
};
use num_traits::PrimInt;
use std::default::Default;
use trait_set::trait_set;

// Use newtype pattern so that tx table entries cannot be confused with other types.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Default)]
pub struct TxTableEntry(pub(crate) TxTableEntryWord);
// TODO Get rid of TxTableEntryWord. We might use const generics in order to parametrize the set of functions below with u32,u64  etc...
// See https://github.com/EspressoSystems/espresso-sequencer/issues/1076
pub type TxTableEntryWord = u32;

trait_set! {

    pub trait TableWordTraits = CanonicalSerialize
        + CanonicalDeserialize
        + TryFrom<usize>
        + TryInto<usize>
        + Default
         + PrimInt
        + std::marker::Sync;

    // Note: this trait is not used yet as for now the Payload structs are only parametrized with the TableWord parameter.
    pub trait OffsetTraits = CanonicalSerialize
        + CanonicalDeserialize
        + TryFrom<usize>
        + TryInto<usize>
        + Default
        + std::marker::Sync;

    // Note: this trait is not used yet as for now the Payload structs are only parametrized with the TableWord parameter.
    pub trait NsIdTraits =CanonicalSerialize + CanonicalDeserialize + Default + std::marker::Sync;
}
pub struct NamespaceInfo {
    // `tx_table` is a bytes representation of the following table:
    // word[0]: [number n of entries in tx table]
    // word[j>0]: [end byte index of the (j-1)th tx in the payload]
    //
    // Thus, the ith tx payload bytes range is word[i-1]..word[i].
    // Edge case: tx_table[-1] is implicitly 0.
    //
    // Word type is `TxTableEntry`.
    //
    // TODO final entry should be implicit:
    // https://github.com/EspressoSystems/espresso-sequencer/issues/757
    pub(crate) tx_table: Vec<u8>,
    pub(crate) tx_bodies: Vec<u8>, // concatenation of all tx payloads
    pub(crate) tx_bytes_end: TxTableEntry, // TODO make this field a usize instead
    pub(crate) tx_table_len: TxTableEntry, // TODO make this field a usize instead
}

#[allow(dead_code)] // TODO temporary
#[derive(Clone, Debug, Derivative, Deserialize, Eq, Serialize)]
#[derivative(Hash, PartialEq)]
// TODO remove the generic type param, use local constants instead
pub struct Payload<TableWord: TableWordTraits> {
    // Sequence of bytes representing the concatenated payloads for each namespace
    #[serde(with = "base64_bytes")]
    pub(crate) raw_payload: Vec<u8>,

    // Sequence of bytes representing the namespace table
    pub(crate) ns_table: NameSpaceTable<TableWord>,
    // TODO(X) Revisit caching of frequently used items
    //
    // TODO type should be `OnceLock<SmallRangeProofType>` instead of `OnceLock<Option<SmallRangeProofType>>`.
    // We can correct this after `once_cell_try` is stabilized <https://github.com/rust-lang/rust/issues/109737>.
    // #[derivative(Hash = "ignore")]
    // #[derivative(PartialEq = "ignore")]
    // #[serde(skip)]
    // pub tx_table_len_proof: OnceLock<Option<SmallRangeProofType>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(bound = "")] // for V
pub enum NamespaceProof {
    Existence {
        #[serde(with = "base64_bytes")]
        ns_payload_flat: Vec<u8>,
        ns_id: NamespaceId,
        ns_proof: LargeRangeProofType,
        vid_common: VidCommon,
    },
    NonExistence {
        ns_id: NamespaceId,
    },
}

pub trait Table<TableWord: TableWordTraits> {
    // Read TxTableEntry::byte_len() bytes from `table_bytes` starting at `offset`.
    // if `table_bytes` has too few bytes at this `offset` then pad with zero.
    // Parse these bytes into a `TxTableEntry` and return.
    // Returns raw bytes, no checking for large values
    fn get_table_len(&self, offset: usize) -> TxTableEntry;

    fn byte_len() -> usize {
        size_of::<TableWord>()
    }
}

#[derive(Clone, Debug, Derivative, Deserialize, Eq, Serialize, Default)]
#[derivative(Hash, PartialEq)]
pub struct NameSpaceTable<TableWord: TableWordTraits> {
    #[serde(with = "base64_bytes")]
    pub(crate) bytes: Vec<u8>,
    #[serde(skip)]
    pub(crate) phantom: PhantomData<TableWord>,
}

pub struct TxTable {}

/// TODO do we really need `PartialOrd`, `Ord` here?
/// Could the `Ord` bound be removed from `QueryablePayload::TransactionIndex`?`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TxIndex {
    pub ns_idx: usize,
    pub tx_idx: usize,
}

/// TODO Decompose this iterator into
/// - a tx iterator `T` over only 1 namespace
/// - a namespace-tx iterator that reuses `T` over all namespaces
pub struct TxIterator<'a, TableWord: TableWordTraits> {
    pub(crate) ns_idx: usize, // simpler than using `Peekable`
    pub(crate) ns_iter: Range<usize>,
    pub(crate) tx_iter: Range<usize>,
    pub(crate) block_payload: &'a Payload<TableWord>,
    pub(crate) ns_table: &'a NameSpaceTable<TableWord>,
}

pub type NsTable = NameSpaceTable<TxTableEntryWord>;
