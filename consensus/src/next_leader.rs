//! Contains the [`NextValidatingLeader`] struct used for the next leader step in the hotstuff consensus algorithm.

use crate::ConsensusMetrics;
use async_compatibility_layer::channel::UnboundedReceiver;
use async_lock::Mutex;
use either::Either;
use hotshot_types::data::ValidatingLeaf;
use hotshot_types::message::Message;
use hotshot_types::message::ProcessedGeneralConsensusMessage;
use hotshot_types::traits::election::ConsensusExchange;
use hotshot_types::traits::node_implementation::{NodeImplementation, NodeType};
use hotshot_types::traits::signature_key::SignatureKey;
use hotshot_types::vote::VoteAccumulator;
use hotshot_types::{
    certificate::QuorumCertificate,
    message::{ConsensusMessageType, InternalTrigger},
    traits::consensus_type::validating_consensus::ValidatingConsensus,
    vote::QuorumVote,
};
use std::marker::PhantomData;
use std::time::Instant;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tracing::{info, instrument, warn};
