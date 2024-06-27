use hotshot::traits::{
    election::static_committee::{GeneralStaticCommittee, StaticCommittee},
    implementations::{CombinedNetworks, Libp2pNetwork, MemoryNetwork, PushCdnNetwork},
    NodeImplementation,
};
use hotshot_types::{
    data::ViewNumber,
    signature_key::{BLSPubKey, BuilderKey},
    traits::node_implementation::NodeType,
};
use serde::{Deserialize, Serialize};
use vbs::version::StaticVersion;

use crate::{
    auction_results_provider_types::TestAuctionResultsProvider,
    block_types::{TestBlockHeader, TestBlockPayload, TestTransaction},
    state_types::{TestInstanceState, TestValidatedState},
    storage_types::TestStorage,
};

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
/// filler struct to implement node type and allow us
/// to select our traits
pub struct TestTypes;
impl NodeType for TestTypes {
    type Base = StaticVersion<0, 1>;
    type Upgrade = StaticVersion<0, 2>;
    const UPGRADE_HASH: [u8; 32] = [
        1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        0, 0,
    ];
    type Time = ViewNumber;
    type BlockHeader = TestBlockHeader;
    type BlockPayload = TestBlockPayload;
    type SignatureKey = BLSPubKey;
    type Transaction = TestTransaction;
    type ValidatedState = TestValidatedState;
    type InstanceState = TestInstanceState;
    type Membership = GeneralStaticCommittee<TestTypes, Self::SignatureKey>;
    type BuilderSignatureKey = BuilderKey;
}

/// The Push CDN implementation
#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct PushCdnImpl;

/// Memory network implementation
#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct MemoryImpl;

/// Libp2p network implementation
#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct Libp2pImpl;

/// Web server network implementation
#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct WebImpl;

/// Combined Network implementation (libp2p + web sever)
#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct CombinedImpl;

/// static committee type alias
pub type StaticMembership = StaticCommittee<TestTypes>;

impl<TYPES: NodeType> NodeImplementation<TYPES> for PushCdnImpl {
    type Network = PushCdnNetwork<TYPES>;
    type Storage = TestStorage<TYPES>;
    type AuctionResultsProvider = TestAuctionResultsProvider;
}

impl<TYPES: NodeType> NodeImplementation<TYPES> for MemoryImpl {
    type Network = MemoryNetwork<TYPES::SignatureKey>;
    type Storage = TestStorage<TYPES>;
    type AuctionResultsProvider = TestAuctionResultsProvider;
}

impl<TYPES: NodeType> NodeImplementation<TYPES> for CombinedImpl {
    type Network = CombinedNetworks<TYPES>;
    type Storage = TestStorage<TYPES>;
    type AuctionResultsProvider = TestAuctionResultsProvider;
}

impl<TYPES: NodeType> NodeImplementation<TYPES> for Libp2pImpl {
    type Network = Libp2pNetwork<TYPES::SignatureKey>;
    type Storage = TestStorage<TYPES>;
    type AuctionResultsProvider = TestAuctionResultsProvider;
}
