/// Sortition trait
pub mod election;
mod networking;
mod node_implementation;
mod storage;

pub use hotshot_types::traits::{BlockPayload, State};
pub use networking::{NetworkError, NetworkReliability};
pub use node_implementation::{NodeImplementation, TestableNodeImplementation};
pub use storage::{Result as StorageResult, Storage};

/// Module for publicly usable implementations of the traits
pub mod implementations {
    pub use super::{
        networking::{
            libp2p_network::{Libp2pCommChannel, Libp2pNetwork, PeerInfoVec},
            memory_network::{DummyReliability, MasterMap, MemoryCommChannel, MemoryNetwork},
            web_server_libp2p_fallback::{CombinedNetworks, WebServerWithFallbackCommChannel},
            web_server_network::{WebCommChannel, WebServerNetwork},
        },
        storage::memory_storage::MemoryStorage, // atomic_storage::AtomicStorage,
    };
}

/// Dummy testing implementations
pub mod dummy {
    pub use hotshot_types::traits::state::dummy::DummyState;
}
