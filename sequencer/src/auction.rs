use crate::{state::FeeAmount, NamespaceId, Transaction};
use hotshot_types::data::ViewNumber;
use std::num::NonZeroU64;

/// State Machine for Auction
#[derive(Clone, Copy)]
struct Auction {
    /// current phase.
    phase: AuctionPhase,
    /// A structure to find which Phase should be the current one.
    // This could probably be an enum if we we structure.
    possible_phases: [AuctionPhase; 3],
}

impl Auction {
    fn update(mut self, view: ViewNumber) {
        self.phase = self
            .possible_phases
            .into_iter()
            .find(|phase| (view > phase.start && view < phase.end))
            .unwrap();
    }
    fn recv_bid(&self, view: ViewNumber, bid: BidTx) {
        self.update(view);
        match self.phase.kind {
            AuctionPhaseKind::Bid => self.send_txn(bid.as_txn()),
            _ => unimplemented!(),
        }
    }
    fn send_txn(&self, txn: Transaction) {
        unimplemented!();
    }
}

// - needs to be configured in genesis block
// - needs to be updatable
/// Configuration for the auction system
struct AuctionConfig {
    bid_phase_num_views: NonZeroU64,
    auction_phase_num_views: NonZeroU64,
    sequencing_phase_num_views: NonZeroU64,
}

/// Uniquely identifies an auction for sequencing rights of namespaces in the network
#[derive(Clone, Copy)]
struct AuctionId(u64);

/// Uniquely identifies one auction phase for a specific auction
#[derive(Clone, Copy)]
struct AuctionPhaseId(AuctionId, AuctionPhaseKind);

/// Describes one auction phase for a specific auction
#[derive(Clone, Copy)]
struct AuctionPhase {
    id: AuctionPhaseId,
    kind: AuctionPhaseKind,
    start: ViewNumber,
    end: ViewNumber,
}

/// Describes the 3 kinds of phases an active auction can be in
#[derive(Clone, Copy)]
enum AuctionPhaseKind {
    Bid,
    Assign,
    Sequence,
}

struct Signature;

/// A transaction to bid for the sequencing rights of a namespace
struct BidTx {
    auction: AuctionId,
    amount: FeeAmount,
    namespace: NamespaceId,
    nonce: Nonce,
    signature: Signature,
}

impl BidTx {
    // maybe better implemented as a From on Transaction
    fn as_txn(&self) -> Transaction {
        unimplemented!();
    }
}

/// A solution to one auction of sequencing rights for namespaces
struct AuctionResultTx {
    auction: AuctionId,
    nonce: Nonce,
    winners: Vec<BidTx>,
    signature: Signature,
}

// Not sure if needed
struct BidRefundTx {
    nonce: Nonce,
    txns_to_refund: Vec<BidTx>,
    signature: Signature,
}

/// Nonce for special (auction) transactions
struct Nonce(u64);
