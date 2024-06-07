use anyhow::Result;
use hotshot_types::message::Message;
use libp2p::BootstrapNode;
use persistence::NetworkConfig;

use super::*;

pub mod cdn;
pub mod libp2p;

/// The genesis network configuration. Overrides what is received from the orchestrator
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GenesisNetworkConfig {
    /// An optional list of bootstrap nodes (multiaddress and peer ID) to connect to
    pub bootstrap_nodes: Option<Vec<BootstrapNode>>,
}

impl GenesisNetworkConfig {
    /// Update the HotShot config with the genesis network config
    pub fn populate_config(&self, config: &mut NetworkConfig) -> Result<()> {
        // Attempt to populate the bootstrap nodes if present
        if let Some(bootstrap_nodes) = &self.bootstrap_nodes {
            // Make sure Libp2p is configured if we have supplied bootstrap nodes
            let Some(ref mut libp2p_config) = config.libp2p_config else {
                return Err(anyhow::anyhow!(
                    "Bootstrap nodes supplied but no libp2p configuration found"
                ));
            };

            // Replace the bootstrap nodes with the ones from the file
            libp2p_config.bootstrap_nodes = bootstrap_nodes
                .iter()
                .cloned()
                .map(|node| {
                    let BootstrapNode { address, peer_id } = node;
                    (peer_id, address)
                })
                .collect();
        }

        Ok(())
    }
}

pub trait Type: 'static {
    type DAChannel: ConnectedNetwork<Message<SeqTypes>, PubKey>;
    type QuorumChannel: ConnectedNetwork<Message<SeqTypes>, PubKey>;
}

#[derive(Clone, Copy, Default)]
pub struct Production;

#[cfg(feature = "libp2p")]
impl Type for Production {
    type DAChannel = CombinedNetworks<SeqTypes>;
    type QuorumChannel = CombinedNetworks<SeqTypes>;
}

#[cfg(not(feature = "libp2p"))]
impl Type for Production {
    type DAChannel = PushCdnNetwork<SeqTypes>;
    type QuorumChannel = PushCdnNetwork<SeqTypes>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Memory;

impl Type for Memory {
    type DAChannel = MemoryNetwork<Message<SeqTypes>, PubKey>;
    type QuorumChannel = MemoryNetwork<Message<SeqTypes>, PubKey>;
}
