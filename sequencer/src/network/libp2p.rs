use anyhow::Result;
use libp2p::{multiaddr::Protocol, Multiaddr, PeerId};
use serde::Deserializer;

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

/// Serialize a `BootstrapNode` to a normal multiaddress in the `TOML` file
impl serde::Serialize for BootstrapNode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut address = self.address.clone();

        // The standard is to have the peer ID as the last part of the address
        address.push(Protocol::P2p(self.peer_id));

        address.serialize(s)
    }
}
