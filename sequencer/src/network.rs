use hotshot_types::message::Message;

use super::*;

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

/// Trait implementations for the CDN
pub mod cdn {
    use std::marker::PhantomData;

    use bincode::Options;
    use cdn_broker::reexports::{
        connection::protocols::{Quic, Tcp},
        crypto::signature::{Serializable, SignatureScheme},
        def::RunDef,
        discovery::{Embedded, Redis},
    };
    use hotshot::types::SignatureKey;
    use hotshot_types::{traits::node_implementation::NodeType, utils::bincode_opts};

    /// A wrapped `SignatureKey`. We need to implement the Push CDN's `SignatureScheme`
    /// trait in order to sign and verify messages to/from the CDN.
    #[derive(Clone, Eq, PartialEq)]
    pub struct WrappedSignatureKey<T: SignatureKey + 'static>(pub T);
    impl<T: SignatureKey> SignatureScheme for WrappedSignatureKey<T> {
        type PrivateKey = T::PrivateKey;
        type PublicKey = Self;

        /// Sign a message of arbitrary data and return the serialized signature
        fn sign(private_key: &Self::PrivateKey, message: &[u8]) -> anyhow::Result<Vec<u8>> {
            let signature = T::sign(private_key, message)?;
            // TODO: replace with rigorously defined serialization scheme...
            // why did we not make `PureAssembledSignatureType` be `CanonicalSerialize + CanonicalDeserialize`?
            Ok(bincode_opts().serialize(&signature)?)
        }

        /// Verify a message of arbitrary data and return the result
        fn verify(public_key: &Self::PublicKey, message: &[u8], signature: &[u8]) -> bool {
            // TODO: replace with rigorously defined signing scheme
            let signature: T::PureAssembledSignatureType =
                match bincode_opts().deserialize(signature) {
                    Ok(key) => key,
                    Err(_) => return false,
                };

            public_key.0.validate(&signature, message)
        }
    }

    /// We need to implement the `Serializable` so the Push CDN can serialize the signatures
    /// and public keys and send them over the wire.
    impl<T: SignatureKey> Serializable for WrappedSignatureKey<T> {
        fn serialize(&self) -> anyhow::Result<Vec<u8>> {
            Ok(self.0.to_bytes())
        }

        fn deserialize(serialized: &[u8]) -> anyhow::Result<Self> {
            Ok(WrappedSignatureKey(T::from_bytes(serialized)?))
        }
    }

    /// The production run definition for the Push CDN.
    /// Uses the real protocols and a Redis discovery client.
    pub struct ProductionDef<TYPES: NodeType> {
        /// Phantom data to hold the type
        pd: PhantomData<TYPES>,
    }

    /// The production run definition for the Push CDN.
    /// Uses the real protocols and a Redis discovery client.
    impl<TYPES: NodeType> RunDef for ProductionDef<TYPES> {
        type BrokerScheme = WrappedSignatureKey<TYPES::SignatureKey>;
        type BrokerProtocol = Tcp;

        type UserScheme = WrappedSignatureKey<TYPES::SignatureKey>;
        type UserProtocol = Quic;

        type DiscoveryClientType = Redis;
    }

    /// The testing run definition for the Push CDN.
    /// Uses the real protocols, but with an embedded discovery client.
    pub struct TestingDef<TYPES: NodeType> {
        /// Phantom data to hold the type
        pd: PhantomData<TYPES>,
    }

    impl<TYPES: NodeType> RunDef for TestingDef<TYPES> {
        type BrokerScheme = WrappedSignatureKey<TYPES::SignatureKey>;
        type BrokerProtocol = Tcp;

        type UserScheme = WrappedSignatureKey<TYPES::SignatureKey>;
        type UserProtocol = Quic;

        type DiscoveryClientType = Embedded;
    }
}
