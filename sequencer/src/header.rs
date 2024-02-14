use crate::{
    l1_client::{L1Client, L1ClientOptions, L1Snapshot},
    state::{fetch_fee_receipts, BlockMerkleCommitment, FeeInfo, FeeMerkleCommitment, FeeReceipt},
    L1BlockInfo, NMTRoot, Payload, ValidatedState,
};
use ark_serialize::CanonicalSerialize;
use async_std::task::{block_on, sleep};
use commit::{Commitment, Committable, RawCommitmentBuilder};
use ethers::{
    core::k256::ecdsa::{self, signature::Signer, SigningKey},
    signers::{Signer as _, Wallet},
    types,
};
use hotshot_types::{
    data::VidCommitment,
    traits::{
        block_contents::{vid_commitment, BlockHeader, BlockPayload},
        ValidatedState as HotShotState,
    },
};
use jf_primitives::merkle_tree::prelude::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{env, fmt::Debug, ops::Add, time::Duration};
use time::OffsetDateTime;

/// A header is like a [`Block`] with the body replaced by a digest.
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Header {
    pub height: u64,
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
    /// Root Commitment of Block Merkle Tree
    pub block_merkle_tree_root: BlockMerkleCommitment,
    /// Root Commitment of `FeeMerkleTree`
    pub fee_merkle_tree_root: FeeMerkleCommitment,
    /// Account (etheruem address) of builder
    pub builder_signature: Option<types::Signature>,
    pub fee_info: FeeInfo,
}

impl Committable for Header {
    fn commit(&self) -> Commitment<Self> {
        let mut bmt_bytes = vec![];
        self.block_merkle_tree_root
            .serialize_with_mode(&mut bmt_bytes, ark_serialize::Compress::Yes)
            .unwrap();
        let mut fmt_bytes = vec![];
        self.fee_merkle_tree_root
            .serialize_with_mode(&mut fmt_bytes, ark_serialize::Compress::Yes)
            .unwrap();
        RawCommitmentBuilder::new(&Self::tag())
            .u64_field("height", self.height)
            .u64_field("timestamp", self.timestamp)
            .u64_field("l1_head", self.l1_head)
            .optional("l1_finalized", &self.l1_finalized)
            .constant_str("payload_commitment")
            .fixed_size_bytes(self.payload_commitment.as_ref().as_ref())
            .field("transactions_root", self.transactions_root.commit())
            .var_size_field("block_merkle_tree_root", &bmt_bytes)
            .var_size_field("fee_merkle_tree_root", &fmt_bytes)
            .field("fee_info", self.fee_info.commit())
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

impl Header {
    #[allow(clippy::too_many_arguments)]
    // TODO pub or merely pub(super)?
    pub fn from_info(
        payload_commitment: VidCommitment,
        transactions_root: NMTRoot,
        parent: &Self,
        mut l1: L1Snapshot,
        mut timestamp: u64,
        fee_merkle_tree_root: FeeMerkleCommitment,
        block_merkle_tree_root: BlockMerkleCommitment,
        builder_address: Wallet<SigningKey>,
    ) -> Self {
        // Increment height.
        let height = parent.height + 1;

        // Ensure the timestamp does not decrease. We can trust `parent.timestamp` because `parent`
        // has already been voted on by consensus. If our timestamp is behind, either f + 1 nodes
        // are lying about the current time, or our clock is just lagging.
        if timestamp < parent.timestamp {
            tracing::warn!(
                "Espresso timestamp {timestamp} behind parent {}, local clock may be out of sync",
                parent.timestamp
            );
            timestamp = parent.timestamp;
        }

        // Ensure the L1 block references don't decrease. Again, we can trust `parent.l1_*` are
        // accurate.
        if l1.head < parent.l1_head {
            tracing::warn!(
                "L1 head {} behind parent {}, L1 client may be lagging",
                l1.head,
                parent.l1_head
            );
            l1.head = parent.l1_head;
        }
        if l1.finalized < parent.l1_finalized {
            tracing::warn!(
                "L1 finalized {:?} behind parent {:?}, L1 client may be lagging",
                l1.finalized,
                parent.l1_finalized
            );
            l1.finalized = parent.l1_finalized;
        }

        // Enforce that the sequencer block timestamp is not behind the L1 block timestamp. This can
        // only happen if our clock is badly out of sync with L1.
        if let Some(l1_block) = &l1.finalized {
            let l1_timestamp = l1_block.timestamp.as_u64();
            if timestamp < l1_timestamp {
                tracing::warn!("Espresso timestamp {timestamp} behind L1 timestamp {l1_timestamp}, local clock may be out of sync");
                timestamp = l1_timestamp;
            }
        }

        let header = Self {
            height,
            timestamp,
            l1_head: l1.head,
            l1_finalized: l1.finalized,
            payload_commitment,
            transactions_root,
            fee_merkle_tree_root,
            block_merkle_tree_root,
            fee_info: parent.fee_info,
            builder_signature: None,
        };

        // Sign Header with builder wallet from state and save the
        // signature on the Header
        let signing_key: &SigningKey = builder_address.signer();
        let header_signature: ecdsa::Signature = signing_key.sign(header.commit().as_ref());

        Self {
            builder_signature: Some(
                types::Signature::try_from(&header_signature.to_vec()[..]).unwrap(),
            ),
            ..header
        }
    }
}

impl BlockHeader for Header {
    type Payload = Payload;
    type State = ValidatedState;

    fn new(
        parent_state: &Self::State,
        instance_state: &<Self::State as HotShotState>::Instance,
        parent_header: &Self,
        payload_commitment: VidCommitment,
        metadata: <Self::Payload as BlockPayload>::Metadata,
    ) -> Self {
        let ValidatedState {
            mut block_merkle_tree,
            mut fee_merkle_tree,
            ..
        } = parent_state.clone();
        block_merkle_tree.push(parent_header.commit()).unwrap();

        let block_merkle_tree_root = block_merkle_tree.commitment();

        // fetch receipts from the l1
        let receipts = fetch_fee_receipts(parent_header);
        for FeeReceipt { recipient, amount } in receipts {
            // Get the balance in order to add amount, ignoring the proof.
            match fee_merkle_tree.universal_lookup(recipient) {
                LookupResult::Ok(balance, _) => fee_merkle_tree
                    .update(recipient, balance.add(amount))
                    .unwrap(),
                // Handle `NotFound` and `NotInMemory` by initializing
                // state.
                _ => fee_merkle_tree.update(recipient, amount).unwrap(),
            };
        }

        let fee_merkle_tree_root = fee_merkle_tree.commitment();

        // The HotShot APIs should be redesigned so that
        // * they are async
        // * new blocks being created have access to the application state, which in our case could
        //   contain an already connected L1 client.
        // For now, as a workaround, we will create a new L1 client based on environment variables
        // and use `block_on` to query it.

        let l1 = if let Some(l1_client) = &*L1_CLIENT {
            block_on(l1_client.snapshot())
        } else {
            // For unit testing, we may disable the L1 client and use mock L1 blocks instead.
            L1Snapshot {
                finalized: None,
                head: 0,
            }
        };

        Self::from_info(
            payload_commitment,
            metadata,
            parent_header,
            l1,
            OffsetDateTime::now_utc().unix_timestamp() as u64,
            fee_merkle_tree_root,
            block_merkle_tree_root,
            instance_state.builder_address.clone(),
        )
    }

    fn genesis(
        instance_state: &<Self::State as HotShotState>::Instance,
    ) -> (
        Self,
        Self::Payload,
        <Self::Payload as BlockPayload>::Metadata,
    ) {
        let (payload, transactions_root) = Payload::genesis();
        let payload_commitment = vid_commitment(&payload.encode().unwrap().collect(), 1);
        let ValidatedState {
            fee_merkle_tree,
            block_merkle_tree,
        } = ValidatedState::genesis(instance_state);
        let block_merkle_tree_root = block_merkle_tree.commitment();
        let fee_merkle_tree_root = fee_merkle_tree.commitment();

        let header = Self {
            // The genesis header needs to be completely deterministic, so we can't sample real
            // timestamps or L1 values.
            height: 0,
            timestamp: 0,
            l1_head: 0,
            l1_finalized: None,
            payload_commitment,
            transactions_root,
            block_merkle_tree_root,
            fee_merkle_tree_root,
            fee_info: FeeInfo::new(instance_state.builder_address.address().into()),
            builder_signature: None,
        };
        (header, payload, transactions_root)
    }

    fn block_number(&self) -> u64 {
        self.height
    }

    fn payload_commitment(&self) -> VidCommitment {
        self.payload_commitment
    }

    fn metadata(&self) -> &NMTRoot {
        &self.transactions_root
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
mod test_headers {
    use crate::{
        state::{BlockMerkleTree, FeeMerkleTree},
        NodeState,
    };

    use super::*;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};

    #[derive(Debug, Default)]
    #[must_use]
    struct TestCase {
        // Parent header info.
        parent_timestamp: u64,
        parent_l1_head: u64,
        parent_l1_finalized: Option<L1BlockInfo>,

        // Environment at the time the new header is created.
        l1_head: u64,
        l1_finalized: Option<L1BlockInfo>,
        timestamp: u64,

        // Expected new header info.
        expected_timestamp: u64,
        expected_l1_head: u64,
        expected_l1_finalized: Option<L1BlockInfo>,
    }

    impl TestCase {
        fn run(self) {
            setup_logging();
            setup_backtrace();

            // Check test case validity.
            assert!(self.expected_timestamp >= self.parent_timestamp);
            assert!(self.expected_l1_head >= self.parent_l1_head);
            assert!(self.expected_l1_finalized >= self.parent_l1_finalized);

            let state = NodeState {};
            let (genesis, _, metadata) = Header::genesis(&state);

            let mut parent = genesis.clone();
            parent.timestamp = self.parent_timestamp;
            parent.l1_head = self.parent_l1_head;
            parent.l1_finalized = self.parent_l1_finalized;
            let block_merkle_tree =
                BlockMerkleTree::from_elems(Some(32), Vec::<Commitment<Header>>::new()).unwrap();
            let block_merkle_tree_root = block_merkle_tree.commitment();

            let FeeReceipt { recipient, amount } = FeeReceipt::default();
            let fee_merkle_tree =
                FeeMerkleTree::from_kv_set(20, Vec::from([(recipient, amount)])).unwrap();
            let fee_merkle_tree_root = fee_merkle_tree.commitment();

            let header = Header::from_info(
                genesis.payload_commitment,
                metadata,
                &parent,
                L1Snapshot {
                    head: self.l1_head,
                    finalized: self.l1_finalized,
                },
                self.timestamp,
                block_merkle_tree_root,
                fee_merkle_tree_root,
            );
            assert_eq!(header.height, parent.height + 1);
            assert_eq!(header.timestamp, self.expected_timestamp);
            assert_eq!(header.l1_head, self.expected_l1_head);
            assert_eq!(header.l1_finalized, self.expected_l1_finalized);

            assert_eq!(
                block_merkle_tree,
                BlockMerkleTree::from_elems(Some(32), Vec::<Commitment<Header>>::new()).unwrap()
            );
        }
    }

    fn l1_block(number: u64) -> L1BlockInfo {
        L1BlockInfo {
            number,
            ..Default::default()
        }
    }

    #[test]
    fn test_new_header() {
        // Simplest case: building on genesis, L1 info and timestamp unchanged.
        TestCase::default().run()
    }

    #[test]
    fn test_new_header_advance_timestamp() {
        TestCase {
            timestamp: 1,
            expected_timestamp: 1,
            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_advance_l1_block() {
        TestCase {
            parent_l1_head: 0,
            parent_l1_finalized: Some(l1_block(0)),

            l1_head: 1,
            l1_finalized: Some(l1_block(1)),

            expected_l1_head: 1,
            expected_l1_finalized: Some(l1_block(1)),

            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_advance_l1_finalized_from_none() {
        TestCase {
            l1_finalized: Some(l1_block(1)),
            expected_l1_finalized: Some(l1_block(1)),
            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_timestamp_behind_finalized_l1_block() {
        let l1_finalized = Some(L1BlockInfo {
            number: 1,
            timestamp: 1.into(),
            ..Default::default()
        });
        TestCase {
            l1_head: 1,
            l1_finalized,
            timestamp: 0,

            expected_l1_head: 1,
            expected_l1_finalized: l1_finalized,
            expected_timestamp: 1,

            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_timestamp_behind() {
        TestCase {
            parent_timestamp: 1,
            timestamp: 0,
            expected_timestamp: 1,

            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_l1_head_behind() {
        TestCase {
            parent_l1_head: 1,
            l1_head: 0,
            expected_l1_head: 1,

            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_l1_finalized_behind_some() {
        TestCase {
            parent_l1_finalized: Some(l1_block(1)),
            l1_finalized: Some(l1_block(0)),
            expected_l1_finalized: Some(l1_block(1)),

            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_new_header_l1_finalized_behind_none() {
        TestCase {
            parent_l1_finalized: Some(l1_block(0)),
            l1_finalized: None,
            expected_l1_finalized: Some(l1_block(0)),

            ..Default::default()
        }
        .run()
    }

    #[test]
    fn test_validate_proposal_error_cases() {
        let (mut header, ..) = Header::genesis(&NodeState {});
        let mut validated_state = ValidatedState::default();
        let mut block_merkle_tree = validated_state.block_merkle_tree.clone();

        // Populate the tree with an initial `push`.
        block_merkle_tree.push(header.commit()).unwrap();
        let block_merkle_tree_root = block_merkle_tree.commitment();
        validated_state.block_merkle_tree = block_merkle_tree.clone();
        header.block_merkle_tree_root = block_merkle_tree_root;
        let parent = header.clone();
        let mut proposal = parent.clone();

        // Advance `proposal.height` to trigger validation error.
        let result = validated_state
            .validate_proposal(&parent.clone(), &proposal)
            .unwrap_err();
        assert_eq!(
            format!("{}", result.root_cause()),
            "Invalid Height Error: 0, 0"
        );

        // proposed `Header` root should include parent +
        // parent.commit
        proposal.height += 1;
        let result = validated_state
            .validate_proposal(&parent.clone(), &proposal)
            .unwrap_err();
        // Fails b/c `proposal` has not advanced from `parent`
        assert!(format!("{}", result.root_cause()).contains("Invalid Block Root Error"));
    }

    #[test]
    fn test_validate_proposal_success() {
        let state = NodeState {};
        let (mut header, _, metadata) = Header::genesis(&state);
        let mut parent_state = ValidatedState::default();
        let mut block_merkle_tree = parent_state.block_merkle_tree.clone();

        // Populate the tree with an initial `push`.
        block_merkle_tree.push(header.commit()).unwrap();
        let block_merkle_tree_root = block_merkle_tree.commitment();
        parent_state.block_merkle_tree = block_merkle_tree.clone();
        header.block_merkle_tree_root = block_merkle_tree_root;

        let parent = header.clone();

        // get a proposal from a parent
        let proposal = Header::new(
            &parent_state,
            &state,
            &parent,
            parent.payload_commitment,
            metadata,
        );

        let proposal_state = parent_state.clone();
        let mut block_merkle_tree = proposal_state.block_merkle_tree.clone();
        block_merkle_tree.push(proposal.commit()).unwrap();
        let result = proposal_state
            .validate_proposal(&parent.clone(), &proposal.clone())
            .unwrap();
        assert_eq!(
            result.block_merkle_tree.commitment(),
            proposal.block_merkle_tree_root
        );
    }
}
