use std::fs::read_to_string;

use anyhow::{Context, Result};
use libp2p::{multiaddr::Protocol, Multiaddr, PeerId};
use serde::{Deserialize, Deserializer};

use crate::persistence::NetworkConfig;

#[derive(Deserialize, Clone, Debug)]
pub struct BootstrapInfo {
    pub bootstrap_nodes: Vec<BootstrapNode>,
}

/// A bootstrap node. Contains the multiaddress and peer ID
#[derive(Clone, Debug)]
pub struct BootstrapNode {
    pub address: Multiaddr,
    pub peer_id: PeerId,
}

/// Deserialize a `BootstrapNode` from a normal multiaddress in the `TOML` file
impl<'de> serde::Deserialize<'de> for BootstrapNode {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let mut address = Multiaddr::deserialize(d)?;

        // The standard is to have the peer ID as the last part of the address
        let Some(Protocol::P2p(peer_id)) = address.pop() else {
            return Err(serde::de::Error::custom(
                "Failed to parse peer ID from address",
            ));
        };

        Ok(BootstrapNode { address, peer_id })
    }
}

impl BootstrapInfo {
    /// Load the bootstrap info from a file
    pub fn load_from_file(path: String) -> Result<BootstrapInfo> {
        // Read the bootstrap nodes from the file
        let bootstrap_info =
            read_to_string(&path).with_context(|| "Failed to load bootstrap nodes from file")?;

        // Parse as TOML
        let bootstrap_info: BootstrapInfo = toml::from_str(&bootstrap_info)
            .with_context(|| format!("Failed to parse bootstrap nodes from file: {}", path))?;

        Ok(bootstrap_info)
    }

    /// Update the HotShot config with the bootstrap information
    pub fn populate_config(&self, config: &mut NetworkConfig) -> Result<()> {
        // Make sure Libp2p is configured
        let Some(ref mut libp2p_config) = config.libp2p_config else {
            return Err(anyhow::anyhow!("No libp2p configuration found"));
        };

        // Replace the bootstrap nodes with the ones from the file
        libp2p_config.bootstrap_nodes = self
            .bootstrap_nodes
            .iter()
            .cloned()
            .map(|node| {
                let BootstrapNode { address, peer_id } = node;
                (peer_id, address)
            })
            .collect();

        Ok(())
    }
}
