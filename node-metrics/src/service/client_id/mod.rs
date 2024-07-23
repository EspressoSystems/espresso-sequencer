use serde::{Deserialize, Serialize};

/// [ClientId] represents the unique identifier for a client that is connected
/// to the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClientId(u128);

impl ClientId {
    pub fn from_count(count: u128) -> Self {
        ClientId(count)
    }
}
