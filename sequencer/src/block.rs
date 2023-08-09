use crate::{Error, NMTRoot, NamespaceProofType, Transaction, TransactionNMT, VmId, MAX_NMT_DEPTH};
use async_std::task::{block_on, sleep};
use commit::{Commitment, Committable, RawCommitmentBuilder};
use ethers::prelude::{Http, Middleware, Provider, U256};
use hotshot::traits::Block as HotShotBlock;
use hotshot_query_service::QueryableBlock;
use hotshot_types::traits::state::TestableBlock;
use jf_primitives::merkle_tree::{
    examples::{Sha3Digest, Sha3Node},
    namespaced_merkle_tree::{BindNamespace, NamespacedMerkleTreeScheme},
    AppendableMerkleTreeScheme, LookupResult, MerkleCommitment, MerkleTreeScheme,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::env;
use std::fmt::{Debug, Display};
use std::time::Duration;
use time::OffsetDateTime;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Header {
    pub timestamp: u64,
    pub l1_block: L1BlockInfo,
    pub transactions_root: NMTRoot,
}

impl Header {
    pub fn commit(&self) -> Commitment<Block> {
        RawCommitmentBuilder::new("Block Comm")
            .u64_field("timestamp", self.timestamp)
            .field("l1_block", self.l1_block.commit())
            .field("transactions_root", self.transactions_root.commit())
            .finalize()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Block {
    timestamp: u64,
    l1_block: L1BlockInfo,

    #[serde(with = "nmt_serializer")]
    pub(crate) transaction_nmt: TransactionNMT,
}

impl From<&Block> for Header {
    fn from(b: &Block) -> Self {
        Self {
            timestamp: b.timestamp,
            l1_block: b.l1_block,
            transactions_root: NMTRoot {
                root: b.transaction_nmt.commitment().digest(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct L1BlockInfo {
    pub number: u64,
    pub timestamp: U256,
}

impl Committable for L1BlockInfo {
    fn commit(&self) -> Commitment<Self> {
        let mut timestamp = [0u8; 32];
        self.timestamp.to_little_endian(&mut timestamp);

        RawCommitmentBuilder::new("L1BlockInfo")
            .u64_field("number", self.number)
            .fixed_size_bytes(&timestamp)
            .finalize()
    }
}

mod nmt_serializer {
    use super::*;

    // Serialize the NMT as a compact Vec<Transaction>
    pub fn serialize<S>(nmt: &TransactionNMT, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let leaves = nmt.leaves().cloned().collect::<Vec<Transaction>>();
        leaves.serialize(s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<TransactionNMT, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de;

        let leaves = <Vec<Transaction>>::deserialize(deserializer)?;
        let nmt = TransactionNMT::from_elems(MAX_NMT_DEPTH, leaves)
            .map_err(|_| de::Error::custom("Failed to build NMT from serialized leaves"))?;
        Ok(nmt)
    }
}

impl QueryableBlock for Block {
    type TransactionIndex = u64;
    type InclusionProof = <TransactionNMT as MerkleTreeScheme>::MembershipProof;
    type Iter<'a> = Box<dyn Iterator<Item = u64>>;

    fn len(&self) -> usize {
        self.transaction_nmt.num_leaves() as usize
    }

    fn transaction_with_proof(
        &self,
        index: &Self::TransactionIndex,
    ) -> Option<(&Self::Transaction, Self::InclusionProof)> {
        match self.transaction_nmt.lookup(index) {
            LookupResult::Ok(txn, proof) => Some((txn, proof)),
            _ => None,
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        Box::new(0..self.len() as u64)
    }
}

impl HotShotBlock for Block {
    type Error = Error;

    type Transaction = Transaction;

    fn add_transaction_raw(
        &self,
        tx: &Self::Transaction,
    ) -> std::result::Result<Self, Self::Error> {
        let mut new = self.clone();
        new.transaction_nmt
            .push(tx.clone())
            .map_err(|e| Error::MerkleTreeError {
                error: e.to_string(),
            })?;
        Ok(new)
    }

    fn contained_transactions(&self) -> std::collections::HashSet<Commitment<Self::Transaction>> {
        self.transaction_nmt
            .leaves()
            .map(|tx| tx.commit())
            .collect()
    }

    fn new() -> Self {
        // The HotShot APIs should be redesigned so that
        // * they are async and fallible
        // * new blocks being created have access to the application state, which in our case could
        //   contain an already connected ETH client.
        // For now, as a workaround, we will create a new ETH client based on environment variables
        // and use `block_on` to query it.
        let l1_block = if let Some(l1_provider) = &*L1_PROVIDER {
            block_on(async move {
                // This cannot fail, retry until we succeed.
                loop {
                    let retry_delay = Duration::from_millis(100);
                    let chain_tip_number = match l1_provider.get_block_number().await {
                        Ok(number) => number,
                        Err(err) => {
                            tracing::error!(
                                "failed to get block number from L1 provider, retrying ({err})"
                            );
                            sleep(retry_delay).await;
                            continue;
                        }
                    };

                    // Use a block with a few confirmations to minimize the probability of reorgs.
                    let number = chain_tip_number.saturating_sub(3.into());

                    let block = match l1_provider.get_block(number).await {
                        Ok(Some(block)) => block,
                        Ok(None) => {
                            // This can only happen if there was a reorg that changed the block
                            // height. Just retry.
                            tracing::error!("L1 block {number} does not exist, retrying");
                            sleep(retry_delay).await;
                            continue;
                        }
                        Err(err) => {
                            tracing::error!(
                                "failed to get block {number} from L1 provider, retrying ({err})"
                            );
                            sleep(retry_delay).await;

                            // Retry the whole loop, including re-fetching the block number. One
                            // reason we might have failed here is if an L1 reorg changed the chain
                            // height, and the block at our current `number` may not become
                            // available again for a while.
                            continue;
                        }
                    };

                    let Some(number) = block.number else {
                    // This can also happen if there's a reorg.
                    tracing::error!("L1 block {number} is not committed, retrying");
                    sleep(retry_delay).await;
                    continue;
                };

                    break L1BlockInfo {
                        number: number.as_u64(),
                        timestamp: block.timestamp,
                    };
                }
            })
        } else {
            // For unit testing, we may disable the L1 provider and use mock L1 blocks instead.
            L1BlockInfo::default()
        };

        Self {
            timestamp: OffsetDateTime::now_utc().unix_timestamp() as u64,
            l1_block,
            transaction_nmt: TransactionNMT::from_elems(MAX_NMT_DEPTH, &[]).unwrap(),
        }
    }
}

lazy_static! {
    static ref L1_PROVIDER: Option<Provider<Http>> = {
        let Ok(url) = env::var("ESPRESSO_SEQUENCER_L1_PROVIDER") else {
            tracing::warn!("ESPRESSO_SEQUENCER_L1_PROVIDER is not set. Using mock L1 block numbers. This is suitable for testing but not production.");
            return None;
        };
        Some(
            url.try_into()
                .expect("invalid ESPRESSO_SEQUENCER_L1_PROVIDER URL"),
        )
    };
}

#[cfg(any(test, feature = "testing"))]
impl TestableBlock for Block {
    fn genesis() -> Self {
        Block::genesis()
    }

    fn txn_count(&self) -> u64 {
        self.transaction_nmt.num_leaves()
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Committable for Block {
    fn commit(&self) -> Commitment<Self> {
        Header::from(self).commit()
    }
}

impl Committable for NMTRoot {
    fn commit(&self) -> Commitment<Self> {
        let comm_bytes =
            <Sha3Digest as BindNamespace<Transaction, VmId, Sha3Node, _>>::generate_namespaced_commitment(
                self.root,
            );
        RawCommitmentBuilder::new("NMT Root Comm")
            .var_size_field("NMT Root", comm_bytes.as_ref())
            .finalize()
    }
}

impl Block {
    pub fn genesis() -> Self {
        Self {
            timestamp: 0,
            l1_block: L1BlockInfo {
                number: 0,
                timestamp: 0.into(),
            },
            transaction_nmt: TransactionNMT::from_elems(MAX_NMT_DEPTH, &[]).unwrap(),
        }
    }

    /// Visit all transactions in this block.
    pub fn transactions(&self) -> impl ExactSizeIterator<Item = &Transaction> + '_ {
        self.transaction_nmt.leaves()
    }

    /// Return namespace proof for a `V`, which can be used to extract the transactions for `V` in this block
    /// and the root of the NMT
    pub fn get_namespace_proof(&self, vm_id: VmId) -> NamespaceProofType {
        self.transaction_nmt.get_namespace_proof(vm_id)
    }

    /// Currently, HotShot consensus does not enforce any relationship between
    /// the NMT root and the block commitment. This returns the NMT root of the block,
    /// mocking the consistency check between the block and NMT commitments.
    pub fn get_nmt_root(&self) -> NMTRoot {
        NMTRoot {
            root: self.transaction_nmt.commitment().digest(),
        }
    }

    /// The block's timestamp.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Information about the L1 block with which this sequencer block is synchronized.
    pub fn l1_block(&self) -> &L1BlockInfo {
        &self.l1_block
    }
}
