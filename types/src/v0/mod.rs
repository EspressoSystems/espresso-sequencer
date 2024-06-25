use derive_more::From;
use hotshot::traits::election::static_committee::GeneralStaticCommittee;
use hotshot_types::{
    data::ViewNumber,
    signature_key::BLSPubKey,
    traits::{node_implementation::NodeType, signature_key::SignatureKey},
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use v0_1::Payload;

mod impls;

// This is the single source of truth for minor versions supported by this major version.
//
// It is written as a higher-level macro which takes a macro invocation as an argument and appends
// the comma-separated list of minor version identifiers to the arguments of the given invocation.
// This is to get around Rust's lazy macro expansion: this macro forces expansion of the given
// invocation. We would rather write something like `some_macro!(args, minor_versions!())`, but the
// `minor_versions!()` argument would not be expanded for pattern-matching in `some_macro!`, so
// instead we write `with_minor_versions!(some_macro!(args))`.
macro_rules! with_minor_versions {
    ($m:ident!($($arg:tt),*)) => {
        $m!($($arg,)* v0_1, v0_2);
    };
}

// Define sub-modules for each supported minor version.
macro_rules! define_modules {
    ($($m:ident),+) => {
        $(pub mod $m;)+
    };
}
with_minor_versions!(define_modules!());

macro_rules! assert_eq_all_versions_of_type {
    ($t:ident, $($m:ident),+) => {
        static_assertions::assert_type_eq_all!($($m::$t),+);
    };
}

macro_rules! reexport_latest_version_of_type {
    ($t:ident, $m:ident) => { pub use $m::$t; };
    ($t:ident, $m1:ident, $($m:ident),+) => {
        reexport_latest_version_of_type!($t, $($m),+);
    }
}

/// Re-export types which have not changed across any minor version.
macro_rules! reexport_unchanged_types {
    ($($t:ident),+ $(,)?) => {
        $(
            with_minor_versions!(assert_eq_all_versions_of_type!($t));
            with_minor_versions!(reexport_latest_version_of_type!($t));
        )+
    }
}
reexport_unchanged_types!(
    BlockMerkleCommitment,
    BlockMerkleTree,
    BuilderSignature,
    ChainConfig,
    ChainId,
    FeeAccount,
    FeeAmount,
    FeeInfo,
    FeeMerkleCommitment,
    FeeMerkleTree,
    L1BlockInfo,
    ResolvableChainConfig,
    NsTable,
    ValidatedState,
    NodeState,
    L1Client,
    L1Snapshot,
    Transaction,
    NamespaceId,
);

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq, From)]
pub enum Header {
    V1(v0_1::Header),
    V2(v0_2::Header),
}

#[derive(
    Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct SeqTypes;

impl NodeType for SeqTypes {
    type Time = ViewNumber;
    type BlockHeader = Header;
    type BlockPayload = Payload;
    type SignatureKey = PubKey;
    type Transaction = Transaction;
    type InstanceState = NodeState;
    type ValidatedState = ValidatedState;
    type Membership = GeneralStaticCommittee<Self, PubKey>;
    type BuilderSignatureKey = FeeAccount;
}

pub type Leaf = hotshot_types::data::Leaf<SeqTypes>;
pub type PubKey = BLSPubKey;
pub type PrivKey = <PubKey as SignatureKey>::PrivateKey;

#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
pub enum Error {
    // TODO: Can we nest these errors in a `ValidationError` to group them?

    // Parent state commitment of block doesn't match current state commitment
    IncorrectParent,

    // New view number isn't strictly after current view
    IncorrectView,

    // Genesis block either has zero or more than one transaction
    GenesisWrongSize,

    // Genesis transaction not present in genesis block
    MissingGenesis,

    // Genesis transaction in non-genesis block
    UnexpectedGenesis,

    // Merkle tree error
    MerkleTreeError { error: String },

    BlockBuilding,
}
