//! Network access compatibility
//!
//! This module contains a trait abstracting over network access, as well as implementations of that
//! trait. Currently this includes
//! - [`MemoryNetwork`](memory_network::MemoryNetwork), an in memory testing-only implementation
//! - [`Libp2pNetwork`](libp2p_network::Libp2pNetwork), a production-ready networking implementation built on top of libp2p-rs.

pub mod combined_network;
pub mod libp2p_network;
pub mod memory_network;
/// The Push CDN network
pub mod push_cdn_network;

use custom_debug::Debug;
use hotshot_types::traits::metrics::{Counter, Gauge, Metrics, NoMetrics};
pub use hotshot_types::traits::network::{
    FailedToSerializeSnafu, NetworkError, NetworkReliability,
};

/// Contains several `NetworkingMetrics` that we're interested in from the networking interfaces
#[derive(Clone, Debug)]
pub struct NetworkingMetricsValue {
    #[allow(dead_code)]
    /// A [`Gauge`] which tracks how many peers are connected
    pub connected_peers: Box<dyn Gauge>,
    /// A [`Counter`] which tracks how many messages have been received
    pub incoming_message_count: Box<dyn Counter>,
    /// A [`Counter`] which tracks how many messages have been send directly
    pub outgoing_direct_message_count: Box<dyn Counter>,
    /// A [`Counter`] which tracks how many messages have been send by broadcast
    pub outgoing_broadcast_message_count: Box<dyn Counter>,
    /// A [`Counter`] which tracks how many messages failed to send
    pub message_failed_to_send: Box<dyn Counter>,
    // A [`Gauge`] which tracks how many connected entries there are in the gossipsub mesh
    // pub gossipsub_mesh_connected: Box<dyn Gauge>,
    // A [`Gauge`] which tracks how many kademlia entries there are
    // pub kademlia_entries: Box<dyn Gauge>,
    // A [`Gauge`] which tracks how many kademlia buckets there are
    // pub kademlia_buckets: Box<dyn Gauge>,
}

impl NetworkingMetricsValue {
    /// Create a new instance of this [`NetworkingMetricsValue`] struct, setting all the counters and gauges
    #[must_use]
    pub fn new(metrics: &dyn Metrics) -> Self {
        Self {
            connected_peers: metrics.create_gauge(String::from("connected_peers"), None),
            incoming_message_count: metrics
                .create_counter(String::from("incoming_message_count"), None),
            outgoing_direct_message_count: metrics
                .create_counter(String::from("outgoing_direct_message_count"), None),
            outgoing_broadcast_message_count: metrics
                .create_counter(String::from("outgoing_broadcast_message_count"), None),
            message_failed_to_send: metrics
                .create_counter(String::from("message_failed_to_send"), None),
        }
    }
}

impl Default for NetworkingMetricsValue {
    fn default() -> Self {
        Self::new(&*NoMetrics::boxed())
    }
}
