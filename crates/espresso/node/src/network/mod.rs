use espresso_types::PubKey;

use super::*;

pub mod cdn;

pub type Production = CombinedNetworks<SeqTypes>;

pub type Memory = MemoryNetwork<PubKey>;
