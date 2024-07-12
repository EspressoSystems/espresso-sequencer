use crate::{FeeAccount, FeeAmount, NamespaceId};
use ethers::types::Signature;
use hotshot_types::data::ViewNumber;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
/// Wrapper enum for Full Network Transactions. Each transaction type
/// will be a variant of this enum.
pub enum FullNetworkTx {
    Bid(BidTx),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
/// A transaction to bid for the sequencing rights of a namespace. It
/// is the `signed` form of `BidTxBody`. Expected usage is *build*
/// it by calling `signed` on `BidTxBody`.
pub struct BidTx {
    pub(crate) body: BidTxBody,
    pub(crate) signature: Signature,
}

/// A transaction body holding data required for bid submission.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Hash)]
pub struct BidTxBody {
    /// Account responsible for the signature
    pub(crate) account: FeeAccount,
    /// Fee to be sequenced in the network.  Different than the bid_amount fee
    // FULL_NETWORK_GAS * MINIMUM_GAS_PRICE
    pub(crate) gas_price: FeeAmount,
    /// The bid amount designated in Wei.  This is different than
    /// the sequencing fee (gas price) for this transaction
    pub(crate) bid_amount: FeeAmount,
    // TODO I think this will end up being a `FeeAccount`
    /// The public key of this sequencer
    pub(crate) public_key: FeeAccount,
    /// The URL the HotShot leader will use to request a bundle
    /// from this sequencer if they win the auction
    pub(crate) url: Url,
    /// The slot this bid is for
    pub(crate) view: ViewNumber,
    /// The set of namespace ids the sequencer is bidding for
    pub(crate) namespaces: Vec<NamespaceId>,
}