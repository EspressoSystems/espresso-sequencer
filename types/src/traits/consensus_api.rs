//! Contains the [`ConsensusApi`] trait.

use std::{num::NonZeroUsize, time::Duration};

use async_trait::async_trait;

use crate::{
    event::Event,
    traits::{
        node_implementation::{NodeImplementation, NodeType},
        signature_key::SignatureKey,
    },
};

/// The API that tasks use to talk to the system
/// TODO we plan to drop this <https://github.com/EspressoSystems/HotShot/issues/2294>
#[async_trait]
pub trait ConsensusApi<TYPES: NodeType, I: NodeImplementation<TYPES>>: Send + Sync {
    /// Total number of nodes in the network. Also known as `n`.
    fn total_nodes(&self) -> NonZeroUsize;

    /// The minimum amount of time a leader has to wait before sending a propose
    fn propose_min_round_time(&self) -> Duration;

    /// The maximum amount of time a leader can wait before sending a propose.
    /// If this time is reached, the leader has to send a propose without transactions.
    fn propose_max_round_time(&self) -> Duration;

    /// Retuns the maximum transactions allowed in a block
    fn max_transactions(&self) -> NonZeroUsize;

    /// Returns the minimum transactions that must be in a block
    fn min_transactions(&self) -> usize;

    /// Get a reference to the public key.
    fn public_key(&self) -> &TYPES::SignatureKey;

    /// Get a reference to the private key.
    fn private_key(&self) -> &<TYPES::SignatureKey as SignatureKey>::PrivateKey;

    /// Notify the system of an event within `hotshot-consensus`.
    async fn send_event(&self, event: Event<TYPES>);
}
