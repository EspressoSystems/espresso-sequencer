use super::LocationDetails;
use hotshot_types::signature_key::BLSPubKey;
use sequencer::state::FeeAccount;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

/// [NodeIdentity] represents the identity of the node that is participating
/// in the network.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeIdentity {
    public_key: BLSPubKey,
    name: String,
    wallet_address: FeeAccount,
    ip_addresses: Vec<IpAddr>,
    company: String,
    location: Option<LocationDetails>,
    operating_system: String,
    node_type: String,
    network_type: String,
}

impl NodeIdentity {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        public_key: BLSPubKey,
        name: String,
        wallet_address: FeeAccount,
        ip_addresses: Vec<IpAddr>,
        company: String,
        location: Option<LocationDetails>,
        operating_system: String,
        node_type: String,
        network_type: String,
    ) -> Self {
        Self {
            public_key,
            name,
            wallet_address,
            ip_addresses,
            company,
            location,
            operating_system,
            node_type,
            network_type,
        }
    }

    pub fn public_key(&self) -> &BLSPubKey {
        &self.public_key
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn wallet_address(&self) -> &FeeAccount {
        &self.wallet_address
    }

    pub fn ip_addresses(&self) -> &[IpAddr] {
        &self.ip_addresses
    }

    pub fn company(&self) -> &str {
        &self.company
    }

    pub fn location(&self) -> Option<&LocationDetails> {
        self.location.as_ref()
    }

    pub fn operating_system(&self) -> &str {
        &self.operating_system
    }

    pub fn node_type(&self) -> &str {
        &self.node_type
    }

    pub fn network_type(&self) -> &str {
        &self.network_type
    }

    pub fn from_public_key(public_key: BLSPubKey) -> Self {
        Self {
            public_key,
            name: String::new(),
            wallet_address: Default::default(),
            ip_addresses: vec![],
            company: String::new(),
            location: None,
            operating_system: String::new(),
            node_type: String::new(),
            network_type: String::new(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::LocationDetails;
    use super::NodeIdentity;
    use hotshot_types::signature_key::BLSPubKey;
    use hotshot_types::traits::signature_key::SignatureKey;
    use std::net::IpAddr;
    use std::net::Ipv4Addr;

    pub fn create_test_node(index: u64) -> NodeIdentity {
        let (pub_key, _) = BLSPubKey::generated_from_seed_indexed([0; 32], index);

        NodeIdentity::new(
            pub_key,
            "a".to_string(),
            Default::default(),
            vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))],
            "company".to_string(),
            Some(LocationDetails::new((0.0, 0.0), "US".to_string())),
            "Windows 11".to_string(),
            "espresso".to_string(),
            "residential".to_string(),
        )
    }

    #[test]
    fn test_node_identity_eq() {
        let node_identity_1 = create_test_node(1);
        let node_identity_2 = create_test_node(1);
        let node_identity_3 = create_test_node(2);

        assert_eq!(node_identity_1, node_identity_2);
        assert_ne!(node_identity_1, node_identity_3);
        assert_ne!(node_identity_2, node_identity_3);
    }

    #[test]
    fn test_node_identity_eq_clone() {
        let node_identity_1 = create_test_node(1);
        let node_identity_2 = node_identity_1.clone();

        assert_eq!(node_identity_1, node_identity_2);
    }

    #[test]
    #[cfg(feature = "testing")]
    fn test_node_identity_serialize() {
        use serde_json;

        let node_identity = create_test_node(1);
        let serialized = serde_json::to_string(&node_identity).unwrap();
        let deserialized: NodeIdentity = serde_json::from_str(&serialized).unwrap();

        assert_eq!(node_identity, deserialized);
    }

    #[test]
    fn test_node_identity_public_key() {
        let node_identity = create_test_node(1);
        let public_key = node_identity.public_key();

        assert_eq!(
            public_key,
            &BLSPubKey::generated_from_seed_indexed([0; 32], 1).0
        );
    }

    #[test]
    fn test_node_identity_name() {
        let node_identity = create_test_node(1);
        let name = node_identity.name();

        assert_eq!(name, "a");
    }

    #[test]
    fn test_node_identity_wallet_address() {
        let node_identity = create_test_node(1);
        let wallet_address = node_identity.wallet_address();

        assert_eq!(wallet_address, &Default::default());
    }

    #[test]
    fn test_node_identity_ip_addresses() {
        let node_identity = create_test_node(1);
        let ip_addresses = node_identity.ip_addresses();

        assert_eq!(ip_addresses, &[IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))]);
    }

    #[test]
    fn test_node_identity_company() {
        let node_identity = create_test_node(1);
        let company = node_identity.company();

        assert_eq!(company, "company");
    }

    #[test]
    fn test_node_identity_location() {
        let node_identity = create_test_node(1);
        let location = node_identity.location();

        assert_eq!(
            location,
            Some(&LocationDetails::new((0.0, 0.0), "US".to_string()))
        );
    }

    #[test]
    fn test_node_identity_operating_system() {
        let node_identity = create_test_node(1);
        let operating_system = node_identity.operating_system();

        assert_eq!(operating_system, "Windows 11");
    }

    #[test]
    fn test_node_identity_node_type() {
        let node_identity = create_test_node(1);
        let node_type = node_identity.node_type();

        assert_eq!(node_type, "espresso");
    }

    #[test]
    fn test_node_identity_network_type() {
        let node_identity = create_test_node(1);
        let network_type = node_identity.network_type();

        assert_eq!(network_type, "residential");
    }
}
