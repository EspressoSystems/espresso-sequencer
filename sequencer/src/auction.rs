use crate::{state::FeeAmount, NamespaceId};
use hotshot_types::data::ViewNumber;
use std::num::NonZeroU64;

// - needs to be configured in genesis block
// - needs to be updatable
/// Configuration for the auction system
struct AuctionConfig {
    bid_phase_num_views: NonZeroU64,
    auction_phase_num_views: NonZeroU64,
    sequencing_phase_num_views: NonZeroU64,
}

/// Uniquely identifies an auction for sequencing rights of namespaces in the network
struct AuctionId(u64);

/// Uniquely identifies one auction phase for a specific auction
struct AuctionPhaseId(AuctionId, AuctionPhaseKind);

/// Describes one auction phase for a specific auction
struct AuctionPhase {
    id: AuctionPhaseId,
    kind: AuctionPhaseKind,
    start: ViewNumber,
    end: ViewNumber,
}

/// Describes the 3 kinds of phases an active auction can be in
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
