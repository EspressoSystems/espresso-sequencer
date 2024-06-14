/// Sortition trait
pub mod election;
mod networking;
mod node_implementation;

pub use hotshot_types::traits::{BlockPayload, ValidatedState};
pub use libp2p_networking::network::NetworkNodeConfigBuilder;
pub use networking::{NetworkError, NetworkReliability};
pub use node_implementation::{NodeImplementation, TestableNodeImplementation};

/// Module for publicly usable implementations of the traits
pub mod implementations {
    pub use super::networking::{
        combined_network::{CombinedNetworks, UnderlyingCombinedNetworks},
        libp2p_network::{
            derive_libp2p_keypair, derive_libp2p_peer_id, Libp2pMetricsValue, Libp2pNetwork,
            PeerInfoVec,
        },
        memory_network::{MasterMap, MemoryNetwork},
        push_cdn_network::{
            CdnMetricsValue, KeyPair, ProductionDef, PushCdnNetwork, TestingDef, Topic,
            WrappedSignatureKey,
        },
    };
}
