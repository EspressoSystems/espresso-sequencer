use serde::{Deserialize, Serialize};

/// example message that may be sent to the swarm. Used in the UI
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Message {
    /// the peerid of the sender
    pub sender: String,
    /// the content of the message
    pub content: String,
    /// the topic associated with the msg
    pub topic: String,
}
