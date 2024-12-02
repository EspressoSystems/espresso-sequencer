use std::marker::PhantomData;

#[cfg(feature = "hotshot-impls")]
use hotshot::traits::election::static_committee::StaticCommittee;
use hotshot_types::{
    data::{EpochNumber, ViewNumber},
    signature_key::BLSPubKey,
    traits::{
        node_implementation::{NodeType, Versions},
        signature_key::SignatureKey,
    },
};
use serde::{Deserialize, Serialize};

mod header;
mod impls;
pub mod traits;
mod utils;
pub use header::Header;

#[cfg(feature = "hotshot-impls")]
pub use impls::get_l1_deposits;
pub use impls::{
    retain_accounts, BuilderValidationError, FeeError, ProposalValidationError,
    StateValidationError,
};
pub use utils::*;
use vbs::version::{StaticVersion, StaticVersionType};

#[cfg(any(test, feature = "testing"))]
pub use impls::mock;

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
        $m!($($arg,)* v0_1, v0_2, v0_3);
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
    AccountQueryData,
    BlockMerkleCommitment,
    BlockMerkleTree,
    BuilderSignature,
    ChainId,
    Delta,
    FeeAccount,
    FeeAccountProof,
    FeeAmount,
    FeeInfo,
    FeeMerkleCommitment,
    FeeMerkleProof,
    FeeMerkleTree,
    Index,
    Iter,
    L1BlockInfo,
    L1Client,
    L1ClientOptions,
    L1Snapshot,
    NamespaceId,
    NsIndex,
    NsIter,
    NsPayload,
    NsPayloadBuilder,
    NsPayloadByteLen,
    NsPayloadOwned,
    NsPayloadRange,
    NsProof,
    NsTable,
    NsTableBuilder,
    NsTableValidationError,
    NumNss,
    NumTxs,
    NumTxsRange,
    NumTxsUnchecked,
    Payload,
    PayloadByteLen,
    Transaction,
    TxIndex,
    TxIter,
    TxPayload,
    TxPayloadRange,
    TxProof,
    TxTableEntries,
    TxTableEntriesRange,
    Upgrade,
    UpgradeType,
    UpgradeMode,
    TimeBasedUpgrade,
    ViewBasedUpgrade,
    BlockSize,
);
pub(crate) use v0_3::{L1Event, L1ReconnectTask, L1State, L1UpdateTask, RpcClient};

#[cfg(feature = "hotshot-impls")]
#[derive(
    Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct SeqTypes;

#[cfg(feature = "hotshot-impls")]
impl NodeType for SeqTypes {
    type View = ViewNumber;
    type Epoch = EpochNumber;
    type BlockHeader = Header;
    type BlockPayload = Payload;
    type SignatureKey = PubKey;
    type Transaction = Transaction;
    type InstanceState = NodeState;
    type ValidatedState = ValidatedState;
    type Membership = StaticCommittee<Self>;
    type BuilderSignatureKey = FeeAccount;
    type AuctionResult = SolverAuctionResults;
}
#[derive(Clone, Default, Debug, Copy)]
pub struct SequencerVersions<Base: StaticVersionType, Upgrade: StaticVersionType> {
    _pd: PhantomData<(Base, Upgrade)>,
}

impl<Base: StaticVersionType, Upgrade: StaticVersionType> SequencerVersions<Base, Upgrade> {
    pub fn new() -> Self {
        Self {
            _pd: Default::default(),
        }
    }
}

#[cfg(feature = "hotshot-impls")]
impl<Base: StaticVersionType + 'static, Upgrade: StaticVersionType + 'static> Versions
    for SequencerVersions<Base, Upgrade>
{
    type Base = Base;
    type Upgrade = Upgrade;
    const UPGRADE_HASH: [u8; 32] = [
        1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        0, 0,
    ];

    type Marketplace = MarketplaceVersion;
    type Epochs = EpochVersion;
}

pub type MockSequencerVersions = SequencerVersions<StaticVersion<0, 1>, StaticVersion<0, 2>>;

pub type V0_0 = StaticVersion<0, 0>;
pub type V0_1 = StaticVersion<0, 1>;
pub type FeeVersion = StaticVersion<0, 2>;
pub type MarketplaceVersion = StaticVersion<0, 3>;
pub type EpochVersion = StaticVersion<0, 4>;

#[cfg(feature = "hotshot-impls")]
pub type Leaf = hotshot_types::data::Leaf<SeqTypes>;
#[cfg(feature = "hotshot-impls")]
pub type Event = hotshot_types::event::Event<SeqTypes>;
pub type PubKey = BLSPubKey;
pub type PrivKey = <PubKey as SignatureKey>::PrivateKey;
pub type NetworkConfig = hotshot_types::network::NetworkConfig<PubKey>;

pub use self::impls::{NodeState, SolverAuctionResultsProvider, ValidatedState};
pub use crate::v0_1::{
    BLOCK_MERKLE_TREE_HEIGHT, FEE_MERKLE_TREE_HEIGHT, NS_ID_BYTE_LEN, NS_OFFSET_BYTE_LEN,
    NUM_NSS_BYTE_LEN, NUM_TXS_BYTE_LEN, TX_OFFSET_BYTE_LEN,
};
use crate::v0_3::SolverAuctionResults;
