use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use committable::{Commitment, Committable};
use derive_more::{Display, From, Into};
use hotshot_types::traits::block_contents::Transaction as HotShotTransaction;
use jf_primitives::merkle_tree::namespaced_merkle_tree::{Namespace, Namespaced};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    Into,
    From,
    Default,
    CanonicalDeserialize,
    CanonicalSerialize,
    PartialOrd,
    Ord,
)]
#[display(fmt = "{_0}")]
pub struct NamespaceId(u64);

impl Namespace for NamespaceId {
    fn max() -> Self {
        Self(u64::max_value())
    }

    fn min() -> Self {
        Self(u64::min_value())
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
}

impl Transaction {
    pub fn new(namespace: NamespaceId, payload: Vec<u8>) -> Self {
        Self { namespace, payload }
    }

    pub fn namespace(&self) -> NamespaceId {
        self.namespace
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        use rand::Rng;
        let len = rng.gen_range(0..100);
        Self::new(
            NamespaceId(rng.gen_range(0..10)),
            (0..len).map(|_| rand::random::<u8>()).collect::<Vec<_>>(),
        )
    }
}

impl HotShotTransaction for Transaction {}

impl Namespaced for Transaction {
    type Namespace = NamespaceId;
    fn get_namespace(&self) -> Self::Namespace {
        self.namespace
    }
}

impl Committable for Transaction {
    fn commit(&self) -> Commitment<Self> {
        committable::RawCommitmentBuilder::new("Transaction")
            .u64_field("namespace", self.namespace.into())
            .var_size_bytes(&self.payload)
            .finalize()
    }

    fn tag() -> String {
        "TX".into()
    }
}
