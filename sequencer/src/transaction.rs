use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use committable::{Commitment, Committable};
use derive_more::Display;
use hotshot_query_service::explorer::ExplorerTransaction;
use hotshot_types::traits::block_contents::Transaction as HotShotTransaction;
use jf_merkle_tree::namespaced_merkle_tree::{Namespace, Namespaced};
use serde::{de::Error, Deserialize, Deserializer, Serialize};

/// TODO [`NamespaceId`] has historical debt to repay:
/// - <https://github.com/EspressoSystems/espresso-sequencer/issues/1574>
/// - It must fit into 4 bytes in order to maintain serialization compatibility
///   for [`crate::block::NsTable`], yet it currently occupies 8 bytes in order
///   to maintain [`serde`] serialization compatibility with [`Transaction`].
/// - Thus, it's a newtype for `u64` that impls `From<u32>` and has a manual
///   impl for [`serde::Deserialize`] that deserializes a `u64` but then returns
///   an error if the value cannot fit into a `u32`. This is ugly. In the future
///   we need to break serialization compatibility so that `NsTable` and
///   `Transaction` can agree on the byte length for `NamespaceId` and all this
///   cruft should be removed.
/// - We should move [`NamespaceId`] to `crate::block::full_payload::ns_table`
///   module because that's where it's byte length is dictated, so that's where
///   it makes the most sense to put serialization. See
///   <https://github.com/EspressoSystems/espresso-sequencer/pull/1499#issuecomment-2134065090>
/// - It impls [`Namespace`] from [`jf_merkle_tree`], but this seems unneeded
///   now that we're not using jellyfish's namespace merkle tree.
/// - We derive lots of things that perhaps we shouldn't: `Into`, `From`,
///   `Default`, `Ord`. Perhaps derivations for [`NamespaceId`] should match
///   that of [`Transaction`].
#[derive(
    Clone,
    Copy,
    Serialize,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    Default,
    CanonicalDeserialize,
    CanonicalSerialize,
    PartialOrd,
    Ord,
)]
#[display(fmt = "{_0}")]
pub struct NamespaceId(u64);

impl From<u32> for NamespaceId {
    fn from(value: u32) -> Self {
        Self(value as u64)
    }
}

impl From<NamespaceId> for u32 {
    fn from(value: NamespaceId) -> Self {
        value.0 as Self
    }
}

impl<'de> Deserialize<'de> for NamespaceId {
    fn deserialize<D>(deserializer: D) -> Result<NamespaceId, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Unexpected;

        let ns_id = <u64 as Deserialize>::deserialize(deserializer)?;
        if ns_id > u32::MAX as u64 {
            Err(D::Error::invalid_value(
                Unexpected::Unsigned(ns_id),
                &"exceeds u32::MAX",
            ))
        } else {
            Ok(NamespaceId(ns_id))
        }
    }
}

impl NamespaceId {
    #[cfg(any(test, feature = "testing"))]
    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        Self(rng.next_u32() as u64)
    }
}

impl Namespace for NamespaceId {
    fn max() -> Self {
        Self(u32::max_value() as u64)
    }

    fn min() -> Self {
        Self(u32::min_value() as u64)
    }
}

#[derive(
    Clone,
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Eq,
    Hash,
    CanonicalSerialize,
    CanonicalDeserialize,
)]
pub struct Transaction {
    namespace: NamespaceId,
    #[serde(with = "base64_bytes")]
    payload: Vec<u8>,
    #[cfg(feature = "benchmarking")]
    has_timestamp: bool,
}

impl Transaction {
    #[cfg(not(feature = "benchmarking"))]
    pub fn new(namespace: NamespaceId, payload: Vec<u8>) -> Self {
        Self { namespace, payload }
    }

    #[cfg(feature = "benchmarking")]
    pub fn new(namespace: NamespaceId, payload: Vec<u8>, has_timestamp: bool) -> Self {
        Self {
            namespace,
            payload,
            has_timestamp,
        }
    }

    pub fn namespace(&self) -> NamespaceId {
        self.namespace
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn into_payload(self) -> Vec<u8> {
        self.payload
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        use rand::Rng;
        let len = rng.gen_range(0..100);
        Self::new(
            NamespaceId::random(rng),
            (0..len).map(|_| rand::random::<u8>()).collect::<Vec<_>>(),
        )
    }
    #[cfg(any(test, feature = "testing"))]
    /// Useful for when we want to test size of transaction(s)
    pub fn of_size(len: usize) -> Self {
        Self::new(
            NamespaceId(0),
            (0..len).map(|_| rand::random::<u8>()).collect::<Vec<_>>(),
        )
    }
}

impl HotShotTransaction for Transaction {}

// TODO seems that `Namespaced` is unneeded.
impl Namespaced for Transaction {
    type Namespace = NamespaceId;
    fn get_namespace(&self) -> Self::Namespace {
        self.namespace
    }
}

impl Committable for Transaction {
    fn commit(&self) -> Commitment<Self> {
        committable::RawCommitmentBuilder::new("Transaction")
            .u64_field("namespace", self.namespace.0)
            .var_size_bytes(&self.payload)
            .finalize()
    }

    fn tag() -> String {
        "TX".into()
    }
}

impl ExplorerTransaction for Transaction {
    type NamespaceId = NamespaceId;
    fn namespace_id(&self) -> Self::NamespaceId {
        self.namespace
    }
}
