use hotshot_types::message::Message;

use super::*;

pub mod cdn;
pub mod libp2p;

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
