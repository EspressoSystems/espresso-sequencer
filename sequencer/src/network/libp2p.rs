use anyhow::Result;
use libp2p::{multiaddr::Protocol, Multiaddr, PeerId};
use serde::{Deserialize, Serialize};

/// A bootstrap node. Contains the multiaddress and peer ID
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(try_from = "Multiaddr", into = "Multiaddr")]
pub struct BootstrapNode {
    pub address: Multiaddr,
    pub peer_id: PeerId,
}

impl From<BootstrapNode> for Multiaddr {
    fn from(node: BootstrapNode) -> Self {
        let mut address = node.address;

        // The standard is to have the peer ID as the last part of the address
        address.push(Protocol::P2p(node.peer_id));

        address
    }
}

impl TryFrom<Multiaddr> for BootstrapNode {
    type Error = anyhow::Error;

    fn try_from(address: Multiaddr) -> Result<Self> {
        // Clone the address so we can pop the peer ID off the end
        let mut address = address.clone();

        // The standard is to have the peer ID as the last part of the address
        let Some(Protocol::P2p(peer_id)) = address.pop() else {
            return Err(anyhow::anyhow!("Failed to parse peer ID from address"));
        };

        Ok(BootstrapNode { address, peer_id })
    }
}
