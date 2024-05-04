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
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use custom_debug::Debug;
use hotshot_types::traits::metrics::{Counter, Gauge, Histogram, Label, Metrics, NoMetrics};
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

/// The wrapper with a string name for the networking metrics
#[derive(Clone, Debug)]
pub struct NetworkingMetrics {
    /// a prefix which tracks the name of the metric
    prefix: String,
    /// a map of values
    values: Arc<Mutex<InnerNetworkingMetrics>>,
}

/// the set of counters and gauges for the networking metrics
#[derive(Clone, Debug, Default)]
pub struct InnerNetworkingMetrics {
    /// All the counters of the networking metrics
    counters: HashMap<String, usize>,
    /// All the gauges of the networking metrics
    gauges: HashMap<String, usize>,
    /// All the histograms of the networking metrics
    histograms: HashMap<String, Vec<f64>>,
    /// All the labels of the networking metrics
    labels: HashMap<String, String>,
}

impl NetworkingMetrics {
    /// For the creation and naming of gauge, counter, histogram and label.
    pub fn sub(&self, name: String) -> Self {
        let prefix = if self.prefix.is_empty() {
            name
        } else {
            format!("{}-{name}", self.prefix)
        };
        Self {
            prefix,
            values: Arc::clone(&self.values),
        }
    }
}

impl Metrics for NetworkingMetrics {
    fn create_counter(&self, label: String, _unit_label: Option<String>) -> Box<dyn Counter> {
        Box::new(self.sub(label))
    }

    fn create_gauge(&self, label: String, _unit_label: Option<String>) -> Box<dyn Gauge> {
        Box::new(self.sub(label))
    }

    fn create_histogram(&self, label: String, _unit_label: Option<String>) -> Box<dyn Histogram> {
        Box::new(self.sub(label))
    }

    fn create_label(&self, label: String) -> Box<dyn Label> {
        Box::new(self.sub(label))
    }

    fn subgroup(&self, subgroup_name: String) -> Box<dyn Metrics> {
        Box::new(self.sub(subgroup_name))
    }
}

impl Counter for NetworkingMetrics {
    fn add(&self, amount: usize) {
        *self
            .values
            .lock()
            .unwrap()
            .counters
            .entry(self.prefix.clone())
            .or_default() += amount;
    }
}

impl Gauge for NetworkingMetrics {
    fn set(&self, amount: usize) {
        *self
            .values
            .lock()
            .unwrap()
            .gauges
            .entry(self.prefix.clone())
            .or_default() = amount;
    }
    fn update(&self, delta: i64) {
        let mut values = self.values.lock().unwrap();
        let value = values.gauges.entry(self.prefix.clone()).or_default();
        let signed_value = i64::try_from(*value).unwrap_or(i64::MAX);
        *value = usize::try_from(signed_value + delta).unwrap_or(0);
    }
}

impl Histogram for NetworkingMetrics {
    fn add_point(&self, point: f64) {
        self.values
            .lock()
            .unwrap()
            .histograms
            .entry(self.prefix.clone())
            .or_default()
            .push(point);
    }
}

impl Label for NetworkingMetrics {
    fn set(&self, value: String) {
        *self
            .values
            .lock()
            .unwrap()
            .labels
            .entry(self.prefix.clone())
            .or_default() = value;
    }
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
