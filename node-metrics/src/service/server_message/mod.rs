use super::client_id::ClientId;
use hotshot_query_service::explorer::{BlockDetail, ExplorerHistograms};
use sequencer::SeqTypes;
use serde::{Deserialize, Serialize};

/// [ServerMessage] represents the messages that the server can send to the
/// client for a response.
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    /// This allows the client to know what client_id they have been assigned
    YouAre(ClientId),

    /// LatestBlock is a message that is meant to show the most recent block
    /// that has arrived.
    LatestBlock(BlockDetail<SeqTypes>),

    /// LatestNodeIdentity is a message that is meant to show the most recent
    /// node identity that has arrived.
    LatestNodeIdentity,

    /// BlocksSnapshot is a message that is sent in response to a request for
    /// the snapshot of block information that is available.
    BlocksSnapshot(Vec<BlockDetail<SeqTypes>>),

    /// NodeIdentitySnapshot is a message that is sent in response to a request
    /// for the snapshot of the current node identity information.
    NodeIdentitySnapshot,

    /// HistogramSnapshot is a message that is sent in response to to a request
    /// for the snapshot of the current histogram information.
    HistogramSnapshot(ExplorerHistograms),
}
