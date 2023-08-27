//! Contains the [`ValidatingLeader`] struct used for the leader step in the hotstuff consensus algorithm.

use crate::{CommitmentMap, Consensus};
use async_compatibility_layer::{
    art::{async_sleep, async_timeout},
    async_primitives::subscribable_rwlock::{ReadView, SubscribableRwLock},
};
use async_lock::RwLock;
use commit::Committable;
use hotshot_types::message::Message;
use hotshot_types::{
    certificate::QuorumCertificate,
    data::{ValidatingLeaf, ValidatingProposal},
    message::GeneralConsensusMessage,
    traits::{
        consensus_type::validating_consensus::ValidatingConsensus,
        election::SignedCertificate,
        node_implementation::{NodeImplementation, NodeType, QuorumProposalType, QuorumVoteType},
        signature_key::SignatureKey,
        Block, State,
    },
};
use hotshot_types::{
    message::Proposal,
    traits::election::{ConsensusExchange, QuorumExchangeType},
};
use std::marker::PhantomData;
use std::{sync::Arc, time::Instant};
use tracing::{error, info, instrument, warn};
