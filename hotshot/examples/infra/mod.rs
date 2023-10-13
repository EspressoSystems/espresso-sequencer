use clap::Parser;
use hotshot_orchestrator::{
    self,
    config::{NetworkConfig, NetworkConfigFile},
};
use hotshot_types::traits::{node_implementation::NodeType, signature_key::SignatureKey};
use libp2p::{
    identity::{
        ed25519::{Keypair as EdKeypair, SecretKey},
        Keypair,
    },
    multiaddr::{self},
    Multiaddr,
};
use std::{fmt::Debug, fs, net::IpAddr, str::FromStr};

// ORCHESTRATOR

#[derive(Parser, Debug, Clone)]
#[command(
    name = "Multi-machine consensus",
    about = "Simulates consensus among multiple machines"
)]
/// Arguments passed to the orchestrator
pub struct OrchestratorArgs {
    /// The address the orchestrator runs on
    pub host: IpAddr,
    /// The port the orchestrator runs on
    pub port: u16,
    /// The configuration file to be used for this run
    pub config_file: String,
}

/// Reads a network configuration from a given filepath
pub fn load_config_from_file<TYPES: NodeType>(
    config_file: String,
) -> NetworkConfig<
    TYPES::SignatureKey,
    <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
    <TYPES::SignatureKey as SignatureKey>::PrivateKey,
    TYPES::ElectionConfigType,
> {
    let config_file_as_string: String = fs::read_to_string(config_file.as_str())
        .unwrap_or_else(|_| panic!("Could not read config file located at {config_file}"));
    let config_toml: NetworkConfigFile =
        toml::from_str::<NetworkConfigFile>(&config_file_as_string)
            .expect("Unable to convert config file to TOML");

    let mut config: NetworkConfig<
        TYPES::SignatureKey,
        <TYPES::SignatureKey as SignatureKey>::StakeTableEntry,
        <TYPES::SignatureKey as SignatureKey>::PrivateKey,
        TYPES::ElectionConfigType,
    > = config_toml.into();

    // Generate network's public keys
    let known_nodes_sk = Vec::new();
    let known_nodes: Vec<_> = (0..config.config.total_nodes.get())
        .map(|node_id| {
            let (key_pair, sk) = TYPES::SignatureKey::generated_from_seed_indexed(
                config.seed,
                node_id.try_into().unwrap(),
            );
            known_nodes_sk.push(sk);
            key_pair
        })
        .collect();

    config.config.known_nodes_with_stake = (0..config.config.total_nodes.get())
        .map(|node_id| known_nodes[node_id].get_stake_table_entry(1u64))
        .collect();
    config.config.known_nodes_sk = known_nodes_sk;
    config
}

pub fn libp2p_generate_indexed_identity(seed: [u8; 32], index: u64) -> Keypair {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(&index.to_le_bytes());
    let new_seed = *hasher.finalize().as_bytes();
    let sk_bytes = SecretKey::try_from_bytes(new_seed).unwrap();
    <EdKeypair as From<SecretKey>>::from(sk_bytes).into()
}

/// libp2p helper function
/// convert node string into multi addr
/// node string of the form: "$IP:$PORT"
pub fn parse_dns(s: &str) -> Result<Multiaddr, multiaddr::Error> {
    let mut i = s.split(':');
    let ip = i.next().ok_or(multiaddr::Error::InvalidMultiaddr)?;
    let port = i.next().ok_or(multiaddr::Error::InvalidMultiaddr)?;
    Multiaddr::from_str(&format!("/dns/{ip}/udp/{port}/quic-v1"))
}

/// libp2p helper function
pub fn parse_ip(s: &str) -> Result<Multiaddr, multiaddr::Error> {
    let mut i = s.split(':');
    let ip = i.next().ok_or(multiaddr::Error::InvalidMultiaddr)?;
    let port = i.next().ok_or(multiaddr::Error::InvalidMultiaddr)?;
    Multiaddr::from_str(&format!("/ip4/{ip}/udp/{port}/quic-v1"))
}
