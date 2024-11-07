use std::sync::Arc;

use super::{client_id::ClientId, espresso_inscription::InscriptionAndChainDetails};
use serde::{Deserialize, Serialize};

/// [ServerMessage] represents the messages that the server can send to the
/// client for a response.
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    /// This allows the client to know what client_id they have been assigned
    YouAre(ClientId),

    /// LatestInscription is a message that is meant to show the most recent
    /// inscription that has arrived.
    LatestInscription(Arc<InscriptionAndChainDetails>),

    /// InscriptionSnapshot is a message that is when a user is first
    /// connecting to the WebSocket.  It immediately relays to the user
    /// the current list of inscriptions that should be displayed.
    InscriptionSnapshot(Arc<Vec<InscriptionAndChainDetails>>),
}

#[cfg(test)]
impl PartialEq for ServerMessage {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::YouAre(lhs), Self::YouAre(rhg)) => lhs == rhg,
            (Self::InscriptionSnapshot(_), Self::InscriptionSnapshot(_)) => true,
            (Self::LatestInscription(_), Self::LatestInscription(_)) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {}
