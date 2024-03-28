use hotshot::traits::{
    election::static_committee::GeneralStaticCommittee, implementations::PushCdnNetwork,
};

use crate::{
    block_types::{TestBlockHeader, TestBlockPayload, TestTransaction},
    state_types::{TestInstanceState, TestValidatedState},
    storage_types::TestStorage,
};

use hotshot::traits::{
    election::static_committee::{StaticCommittee, StaticElectionConfig},
    implementations::{CombinedNetworks, Libp2pNetwork, MemoryNetwork, WebServerNetwork},
    NodeImplementation,
};

use hotshot_types::constants::WebServerVersion;

use hotshot_types::{
    data::ViewNumber, message::Message, signature_key::BLSPubKey,
    traits::node_implementation::NodeType,
};
use serde::{Deserialize, Serialize};

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
    type Time = ViewNumber;
    type BlockHeader = TestBlockHeader;
    type BlockPayload = TestBlockPayload;
    type SignatureKey = BLSPubKey;
    type Transaction = TestTransaction;
    type ElectionConfigType = StaticElectionConfig;
    type ValidatedState = TestValidatedState;
    type InstanceState = TestInstanceState;
    type Membership = GeneralStaticCommittee<TestTypes, Self::SignatureKey>;
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

// Push CDN communication channels
type StaticPushCdnQuorumComm = PushCdnNetwork<TestTypes>;
type StaticPushCdnDAComm = PushCdnNetwork<TestTypes>;

/// memory network
pub type StaticMemoryDAComm =
    MemoryNetwork<Message<TestTypes>, <TestTypes as NodeType>::SignatureKey>;

/// libp2p network
type StaticLibp2pDAComm = Libp2pNetwork<Message<TestTypes>, <TestTypes as NodeType>::SignatureKey>;

/// web server network communication channel
type StaticWebDAComm = WebServerNetwork<TestTypes, WebServerVersion>;

/// combined network
type StaticCombinedDAComm = CombinedNetworks<TestTypes>;

/// memory comm channel
pub type StaticMemoryQuorumComm =
    MemoryNetwork<Message<TestTypes>, <TestTypes as NodeType>::SignatureKey>;

/// libp2p comm channel
type StaticLibp2pQuorumComm =
    Libp2pNetwork<Message<TestTypes>, <TestTypes as NodeType>::SignatureKey>;

/// web server comm channel
type StaticWebQuorumComm = WebServerNetwork<TestTypes, WebServerVersion>;

/// combined network (libp2p + web server)
type StaticCombinedQuorumComm = CombinedNetworks<TestTypes>;

impl NodeImplementation<TestTypes> for PushCdnImpl {
    type QuorumNetwork = StaticPushCdnQuorumComm;
    type CommitteeNetwork = StaticPushCdnDAComm;
    type Storage = TestStorage<TestTypes>;
}

impl NodeImplementation<TestTypes> for Libp2pImpl {
    type QuorumNetwork = StaticLibp2pQuorumComm;
    type CommitteeNetwork = StaticLibp2pDAComm;
    type Storage = TestStorage<TestTypes>;
}

impl NodeImplementation<TestTypes> for MemoryImpl {
    type QuorumNetwork = StaticMemoryQuorumComm;
    type CommitteeNetwork = StaticMemoryDAComm;
    type Storage = TestStorage<TestTypes>;
}

impl NodeImplementation<TestTypes> for WebImpl {
    type QuorumNetwork = StaticWebQuorumComm;
    type CommitteeNetwork = StaticWebDAComm;
    type Storage = TestStorage<TestTypes>;
}

impl NodeImplementation<TestTypes> for CombinedImpl {
    type QuorumNetwork = StaticCombinedQuorumComm;
    type CommitteeNetwork = StaticCombinedDAComm;
    type Storage = TestStorage<TestTypes>;
}
