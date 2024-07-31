/// Trait implementations for the CDN
use std::marker::PhantomData;

use bincode::Options;
use cdn_broker::reexports::{
    connection::protocols::{Quic, Tcp},
    crypto::signature::{Serializable, SignatureScheme},
    def::{ConnectionDef, RunDef, Topic as TopicTrait},
    discovery::{Embedded, Redis},
};
use hotshot::types::SignatureKey;
use hotshot_types::{
    traits::{network::Topic as HotShotTopic, node_implementation::NodeType},
    utils::bincode_opts,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use static_assertions::const_assert_eq;

/// The enum for the topics we can subscribe to in the Push CDN
#[repr(u8)]
#[derive(IntoPrimitive, TryFromPrimitive, Clone, PartialEq, Eq)]
pub enum Topic {
    /// The global topic
    Global = 0,
    /// The DA topic
    Da = 1,
}

// Make sure the topics are the same as defined in `HotShot`.
const_assert_eq!(Topic::Global as u8, HotShotTopic::Global as u8);
const_assert_eq!(Topic::Da as u8, HotShotTopic::Da as u8);

/// Implement the `TopicTrait` for our `Topic` enum. This lets us define
/// compatible topics at the broker-level. Others will be rejected.
impl TopicTrait for Topic {}

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
        let signature: T::PureAssembledSignatureType = match bincode_opts().deserialize(signature) {
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
pub struct ProductionDef<TYPES: NodeType>(PhantomData<TYPES>);
impl<TYPES: NodeType> RunDef for ProductionDef<TYPES> {
    type User = UserDef<TYPES>;
    type Broker = BrokerDef<TYPES>;
    type DiscoveryClientType = Redis;
    type Topic = Topic;
}

/// The user definition for the Push CDN.
/// Uses the Quic protocol and untrusted middleware.
pub struct UserDef<TYPES: NodeType>(PhantomData<TYPES>);
impl<TYPES: NodeType> ConnectionDef for UserDef<TYPES> {
    type Scheme = WrappedSignatureKey<TYPES::SignatureKey>;
    type Protocol = Quic;
}

/// The broker definition for the Push CDN.
/// Uses the TCP protocol and trusted middleware.
pub struct BrokerDef<TYPES: NodeType>(PhantomData<TYPES>);
impl<TYPES: NodeType> ConnectionDef for BrokerDef<TYPES> {
    type Scheme = WrappedSignatureKey<TYPES::SignatureKey>;
    type Protocol = Tcp;
}

/// The client definition for the Push CDN. Uses the Quic
/// protocol and no middleware. Differs from the user
/// definition in that is on the client-side.
#[derive(Clone)]
pub struct ClientDef<TYPES: NodeType>(PhantomData<TYPES>);
impl<TYPES: NodeType> ConnectionDef for ClientDef<TYPES> {
    type Scheme = WrappedSignatureKey<TYPES::SignatureKey>;
    type Protocol = Quic;
}

/// The testing run definition for the Push CDN.
/// Uses the real protocols, but with an embedded discovery client.
pub struct TestingDef<TYPES: NodeType>(PhantomData<TYPES>);
impl<TYPES: NodeType> RunDef for TestingDef<TYPES> {
    type User = UserDef<TYPES>;
    type Broker = BrokerDef<TYPES>;
    type DiscoveryClientType = Embedded;
    type Topic = Topic;
}
