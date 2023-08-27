/// Wrapper around gossipsub
pub mod gossip;

/// Wrapper around `RequestResponse`
pub mod direct_message;

/// exponential backoff type
pub mod exponential_backoff;

/// Implementation of a codec for sending messages
/// for `RequestResponse`
pub mod direct_message_codec;

/// Wrapper around Kademlia
pub mod dht;
