use super::*;

pub mod cdn;
pub mod libp2p;

pub trait Type: 'static {
    type DAChannel: ConnectedNetwork<PubKey>;
    type QuorumChannel: ConnectedNetwork<PubKey>;
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
    type DAChannel = MemoryNetwork<PubKey>;
    type QuorumChannel = MemoryNetwork<PubKey>;
}
