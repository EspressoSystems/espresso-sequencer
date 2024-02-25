//! Library for p2p communication

/// Network logic
pub mod network;

/// symbols needed to implement a networking instance over libp2p-netorking
pub mod reexport {
    pub use libp2p::Multiaddr;
    pub use libp2p_identity::PeerId;
}
