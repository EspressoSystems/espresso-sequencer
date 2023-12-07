use crate::{
    Error, L1BlockInfo, NMTRoot, NamespaceProofType, Transaction, TransactionNMT, VmId,
    MAX_NMT_DEPTH,
};
use ark_serialize::CanonicalSerialize;
use commit::{Commitment, Committable, RawCommitmentBuilder};
use hotshot_query_service::availability::QueryablePayload;
use hotshot_types::traits::block_contents::{BlockHeader, BlockPayload};
use jf_primitives::merkle_tree::{
    namespaced_merkle_tree::NamespacedMerkleTreeScheme, AppendableMerkleTreeScheme, LookupResult,
    MerkleCommitment, MerkleTreeScheme,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display};
use time::OffsetDateTime;

/// A header is like a [`Block`] with the body replaced by a digest.
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Header {
    pub timestamp: u64,

    /// The Espresso block header includes a reference to the current head of the L1 chain.
    ///
    /// Rollups can use this to facilitate bridging between the L1 and L2 in a deterministic way.
    /// This field deterministically associates an L2 block with a recent L1 block the instant the
    /// L2 block is sequenced. Rollups can then define the L2 state after this block as the state
    /// obtained by executing all the transactions in this block _plus_ all the L1 deposits up to
    /// the given L1 block number. Since there is no need to wait for the L2 block to be reflected
    /// on the L1, this bridge design retains the low confirmation latency of HotShot.
    ///
    /// This block number indicates the unsafe head of the L1 chain, so it is subject to reorgs. For
    /// this reason, the Espresso header does not include any information that might change in a
    /// reorg, such as the L1 block timestamp or hash. It includes only the L1 block number, which
    /// will always refer to _some_ block after a reorg: if the L1 head at the time this block was
    /// sequenced gets reorged out, the L1 chain will eventually (and probably quickly) grow to the
    /// same height once again, and a different block will exist with the same height. In this way,
    /// Espresso does not have to handle L1 reorgs, and the Espresso blockchain will always be
    /// reflective of the current state of the L1 blockchain. Rollups that use this block number
    /// _do_ have to handle L1 reorgs, but each rollup and each rollup client can decide how many
    /// confirmations they want to wait for on top of this `l1_head` before they consider an L2
    /// block finalized. This offers a tradeoff between low-latency L1-L2 bridges and finality.
    ///
    /// Rollups that want a stronger guarantee of finality, or that want Espresso to attest to data
    /// from the L1 block that might change in reorgs, can instead use the latest L1 _finalized_
    /// block at the time this L2 block was sequenced: `l1_finalized`.
    pub l1_head: u64,

    /// The Espresso block header includes information a bout the latest finalized L1 block.
    ///
    /// Similar to `l1_head`, rollups can use this information to implement a bridge between the L1
    /// and L2 while retaining the finality of low-latency block confirmations from HotShot. Since
    /// this information describes the finalized L1 block, a bridge using this L1 block will have
    /// much higher latency than a bridge using `l1_head`. In exchange, rollups that use the
    /// finalized block do not have to worry about L1 reorgs, and can inject verifiable attestations
    /// to the L1 block metadata (such as its timestamp or hash) into their execution layers, since
    /// Espresso replicas will sign this information for the finalized L1 block.
    ///
    /// This block may be `None` in the rare case where Espresso has started shortly after the
    /// genesis of the L1, and the L1 has yet to finalize a block. In all other cases it will be
    /// `Some`.
    pub l1_finalized: Option<L1BlockInfo>,

    pub payload_commitment: VidCommitment,
    pub transactions_root: NMTRoot,
}

impl Committable for Header {
    fn commit(&self) -> Commitment<Self> {
        RawCommitmentBuilder::new(&Block::tag())
            .u64_field("timestamp", self.timestamp())
            .u64_field("l1_head", self.l1_head())
            .array_field(
                "l1_finalized",
                self.l1_finalized()
                    .iter()
                    .map(|block| block.commit())
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .field("transactions_root", self.transactions_root.commit())
            .finalize()
    }

    fn tag() -> String {
        "BLOCK".into()
    }
}

impl Committable for NMTRoot {
    fn commit(&self) -> Commitment<Self> {
        let mut comm_bytes = vec![];
        self.root
            .serialize_with_mode(&mut comm_bytes, ark_serialize::Compress::Yes)
            .unwrap();
        RawCommitmentBuilder::new(&Self::tag())
            .var_size_field("root", &comm_bytes)
            .finalize()
    }

    fn tag() -> String {
        "NMTROOT".into()
    }
}

impl BlockHeader for Header {
    type Payload = Payload;

    fn new(payload_commitment: VidCommitment, metadata: NMTRoot, parent_header: &Self) -> Self {
        // The HotShot APIs should be redesigned so that
        // * they are async
        // * new blocks being created have access to the application state, which in our case could
        //   contain an already connected L1 client.
        // For now, as a workaround, we will create a new L1 client based on environment variables
        // and use `block_on` to query it.
        let L1Snapshot { finalized, head } = if let Some(l1_client) = &*L1_CLIENT {
            block_on(l1_client.snapshot())
        } else {
            // For unit testing, we may disable the L1 client and use mock L1 blocks instead.
            L1Snapshot {
                finalized: None,
                head: 0,
            }
        };

        // Sample a timestamp, ensuring that it does not decrease.
        let mut timestamp = OffsetDateTime::now_utc().unix_timestamp() as u64;
        if timestamp < parent_header.timestamp {
            tracing::warn!(
                "Espresso timestamp {timestamp} behind parent {}",
                parent_header.timestamp
            );
            timestamp = parent_header.timestamp;
        }

        // Enforce that the sequencer block timestamp is not behind the L1 block timestamp. This can
        // only happen if our clock is badly out of sync with L1.
        if let Some(l1_block) = &l1_finalized {
            let l1_timestamp = l1_block.timestamp.as_u64();
            if timestamp < l1_timestamp {
                tracing::warn!("Espresso timestamp {timestamp} behind L1 timestamp {l1_timestamp}");
                timestamp = l1_timestamp;
            }
        }

        Self {
            timestamp,
            l1_head,
            l1_finalized,
            payload_commitment,
            transactions_root: metadata,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Payload {
    #[serde(with = "nmt_serializer")]
    pub(crate) transaction_nmt: TransactionNMT,
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

impl QueryablePayload for Payload {
    type TransactionIndex = u64;
    type InclusionProof = <TransactionNMT as MerkleTreeScheme>::MembershipProof;
    type Iter<'a> = Box<dyn Iterator<Item = u64>>;

    fn len(&self) -> usize {
        self.transaction_nmt.num_leaves() as usize
    }

    fn transaction_with_proof(
        &self,
        index: &Self::TransactionIndex,
    ) -> Option<(Self::Transaction, Self::InclusionProof)> {
        match self.transaction_nmt.lookup(index) {
            LookupResult::Ok(txn, proof) => Some((txn.clone(), proof)),
            _ => None,
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        Box::new(0..self.len() as u64)
    }
}

impl BlockPayload for Payload {
    type Error = Error;
    type Metadata = NMTRoot;

    fn genesis() -> (Self, Self::Metadata) {
        let transaction_nmt = TransactionNMT::from_elems(MAX_NMT_DEPTH, &[]).unwrap();
        let root = NMTRoot {
            root: transaction_nmt.commitment().digest(),
        };
        (Self { transaction_nmt }, root)
    }
}

#[cfg(any(test, feature = "testing"))]
impl hotshot_types::traits::state::TestableBlock for Block {
    fn genesis() -> Self {
        Block::genesis().0
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

impl Block {
    /// Return namespace proof for a `V`, which can be used to extract the transactions for `V` in this block
    /// and the root of the NMT
    pub fn get_namespace_proof(&self, vm_id: VmId) -> NamespaceProofType {
        self.transaction_nmt.get_namespace_proof(vm_id)
    }
}

lazy_static! {
    pub(crate) static ref L1_CLIENT: Option<L1Client> = {
        let Ok(url) = env::var("ESPRESSO_SEQUENCER_L1_WS_PROVIDER") else {
            #[cfg(any(test, feature = "testing"))]
            {
                tracing::warn!("ESPRESSO_SEQUENCER_L1_WS_PROVIDER is not set. Using mock L1 block numbers. This is suitable for testing but not production.");
                return None;
            }
            #[cfg(not(any(test, feature = "testing")))]
            {
                panic!("ESPRESSO_SEQUENCER_L1_WS_PROVIDER must be set.");
            }
        };

        let mut opt = L1ClientOptions::with_url(url.parse().unwrap());
        // For testing with a pre-merge geth node that does not support the finalized block tag we
        // allow setting an environment variable to use the latest block instead. This feature is
        // used in the OP devnet which uses the docker images built in this repo. Therefore it's not
        // hidden behind the testing flag.
        if let Ok(val) = env::var("ESPRESSO_SEQUENCER_L1_USE_LATEST_BLOCK_TAG") {
            match val.as_str() {
               "y" | "yes" | "t"|  "true" | "on" | "1"  => {
                    tracing::warn!(
                        "ESPRESSO_SEQUENCER_L1_USE_LATEST_BLOCK_TAG is set. Using latest block tag\
                         instead of finalized block tag. This is suitable for testing but not production."
                    );
                    opt = opt.with_latest_block_tag();
                },
                "n" | "no" | "f" | "false" | "off" | "0" => {}
                _ => panic!("invalid ESPRESSO_SEQUENCER_L1_USE_LATEST_BLOCK_TAG value: {}", val)
            }
        }

        block_on(async move {
            // Starting the client can fail due to transient errors in the L1 RPC. This could make
            // it very annoying to start up the sequencer node, so retry until we succeed.
            loop {
                match opt.clone().start().await {
                    Ok(client) => break Some(client),
                    Err(err) => {
                        tracing::error!("failed to start L1 client, retrying: {err}");
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        })
    };
}

#[cfg(test)]
mod reference {
    //! Reference data types.
    //!
    //! This module provides some reference instantiations of various data types which have an
    //! external, language-independent interface (e.g. commitment scheme). Ports of the sequencer to
    //! other languages, as well as downstream packages written in other languages, can use these
    //! references objects and their known commitments to check that their implementations of the
    //! commitment scheme are compatible with this reference implementation. To get the byte
    //! representation or U256 representation of a commitment for testing in other packages, run the
    //! tests and look for "commitment bytes" or "commitment U256" in the logs.
    //!
    //! For convenience, the reference objects are provided in serialized form, as they will appear
    //! in query service responses and the like, in the JSON files in the `data` directory of the
    //! repo for this crate. These JSON files are compiled into the crate binary and deserialized in
    //! this module to generate tests for the serialization format and commitment scheme.
    //!
    //! These tests may fail if you make a breaking change to a commitment scheme, serialization,
    //! etc. If this happens, be sure you _want_ to break the API, and, if so, simply replace the
    //! relevant constant in this module with the "actual" value that can be found in the logs of
    //! the failing test.

    use super::*;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use lazy_static::lazy_static;
    use sequencer_utils::commitment_to_u256;
    use serde::de::DeserializeOwned;
    use serde_json::Value;

    macro_rules! load_reference {
        ($name:expr) => {
            serde_json::from_str(include_str!(std::concat!("../../data/", $name, ".json"))).unwrap()
        };
    }

    lazy_static! {
        pub static ref NMT_ROOT: Value = load_reference!("nmt_root");
        pub static ref L1_BLOCK: Value = load_reference!("l1_block");
        pub static ref HEADER: Value = load_reference!("header");
        pub static ref BLOCK: Value = load_reference!("block");
    }

    fn reference_test<T: DeserializeOwned, C: Committable>(
        reference: Value,
        expected: &str,
        commit: impl FnOnce(&T) -> Commitment<C>,
    ) {
        setup_logging();
        setup_backtrace();

        let reference: T = serde_json::from_value(reference).unwrap();
        let actual = commit(&reference);

        // Print information about the commitment that might be useful in generating tests for other
        // languages.
        let bytes: &[u8] = actual.as_ref();
        let u256 = commitment_to_u256(actual);
        tracing::info!("actual commitment: {}", actual);
        tracing::info!("commitment bytes: {:?}", bytes);
        tracing::info!("commitment U256: {}", u256);

        assert_eq!(actual, expected.parse().unwrap());
    }

    #[test]
    fn test_reference_nmt_root() {
        reference_test::<NMTRoot, _>(
            NMT_ROOT.clone(),
            "NMTROOT~-1Dow1sCihLw5x-sNsxaKtcqSLsPHIBDlXUacug5vgpx",
            |root| root.commit(),
        );
    }

    #[test]
    fn test_reference_l1_block() {
        reference_test::<L1BlockInfo, _>(
            L1_BLOCK.clone(),
            "L1BLOCK~4HpzluLK2Isz3RdPNvNrDAyQcWOF2c9JeLZzVNLmfpQ9",
            |block| block.commit(),
        );
    }

    #[test]
    fn test_reference_header() {
        reference_test::<Header, _>(
            HEADER.clone(),
            "BLOCK~Gk26ovvxhxeEBcTPg0DP142QkkGeHqlm-7dllaitoZW0",
            |header| header.commit(),
        );
    }

    #[test]
    fn test_reference_block() {
        reference_test::<Block, _>(
            BLOCK.clone(),
            "BLOCK~Gk26ovvxhxeEBcTPg0DP142QkkGeHqlm-7dllaitoZW0",
            |block| block.commit(),
        );
    }

    #[test]
    fn test_header_block_commitment_equivalence() {
        let block: Block = serde_json::from_value(BLOCK.clone()).unwrap();
        assert_eq!(block.commit(), block.header().commit());
    }
}
