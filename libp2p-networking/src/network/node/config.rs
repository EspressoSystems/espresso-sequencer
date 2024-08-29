// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{collections::HashSet, num::NonZeroUsize, time::Duration};

use hotshot_types::traits::signature_key::SignatureKey;
use libp2p::{identity::Keypair, Multiaddr};
use libp2p_identity::PeerId;

use super::MAX_GOSSIP_MSG_SIZE;

/// The default Kademlia replication factor
pub const DEFAULT_REPLICATION_FACTOR: Option<NonZeroUsize> = NonZeroUsize::new(10);

/// describe the configuration of the network
#[derive(Clone, Default, derive_builder::Builder, custom_debug::Debug)]
pub struct NetworkNodeConfig<K: SignatureKey + 'static> {
    /// The keypair for the node
    #[builder(setter(into, strip_option), default)]
    #[debug(skip)]
    pub keypair: Option<Keypair>,
    /// The address to bind to
    #[builder(default)]
    pub bind_address: Option<Multiaddr>,
    /// Replication factor for entries in the DHT
    #[builder(setter(into, strip_option), default = "DEFAULT_REPLICATION_FACTOR")]
    pub replication_factor: Option<NonZeroUsize>,

    #[builder(default)]
    /// Configuration for `GossipSub`
    pub gossip_config: GossipConfig,

    /// list of addresses to connect to at initialization
    pub to_connect_addrs: HashSet<(PeerId, Multiaddr)>,
    /// republication interval in DHT, must be much less than `ttl`
    #[builder(default)]
    pub republication_interval: Option<Duration>,
    /// expiratiry for records in DHT
    #[builder(default)]
    pub ttl: Option<Duration>,

    /// The stake table. Used for authenticating other nodes. If not supplied
    /// we will not check other nodes against the stake table
    #[builder(default)]
    pub stake_table: Option<HashSet<K>>,

    /// The signed authentication message sent to the remote peer
    /// If not supplied we will not send an authentication message during the handshake
    #[builder(default)]
    pub auth_message: Option<Vec<u8>>,
}

/// Configuration for Libp2p's Gossipsub
#[derive(Clone, Debug)]
pub struct GossipConfig {
    /// The heartbeat interval
    pub heartbeat_interval: Duration,

    /// The number of past heartbeats to gossip about
    pub history_gossip: usize,
    /// The number of past heartbeats to remember the full messages for
    pub history_length: usize,

    /// The target number of peers in the mesh
    pub mesh_n: usize,
    /// The maximum number of peers in the mesh
    pub mesh_n_high: usize,
    /// The minimum number of peers in the mesh
    pub mesh_n_low: usize,
    /// The minimum number of mesh peers that must be outbound
    pub mesh_outbound_min: usize,

    /// The maximum gossip message size
    pub max_transmit_size: usize,
}

impl Default for GossipConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: Duration::from_secs(1), // Default of Libp2p

            // The following are slightly modified defaults of Libp2p
            history_gossip: 6, // The number of past heartbeats to gossip about
            history_length: 8, // The number of past heartbeats to remember the full messages for

            // The mesh parameters are borrowed from Ethereum:
            // https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/p2p-interface.md#the-gossip-domain-gossipsub
            mesh_n: 8,            // The target number of peers in the mesh
            mesh_n_high: 12,      // The maximum number of peers in the mesh
            mesh_n_low: 6,        // The minimum number of peers in the mesh
            mesh_outbound_min: 2, // The minimum number of mesh peers that must be outbound

            max_transmit_size: MAX_GOSSIP_MSG_SIZE, // The maximum gossip message size
        }
    }
}
