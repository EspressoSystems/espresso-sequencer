use bitvec::vec::BitVec;
use circular_buffer::CircularBuffer;
use futures::{Stream, StreamExt};
use hotshot_query_service::{
    availability::QueryableHeader,
    explorer::{BlockDetail, ExplorerHeader, Timestamp},
    Leaf, Resolvable,
};
use hotshot_stake_table::vec_based::StakeTable;
use hotshot_types::{
    light_client::{CircuitField, StateVerKey},
    signature_key::BLSPubKey,
    traits::{
        block_contents::BlockHeader,
        stake_table::{SnapshotVersion, StakeTableScheme},
        BlockPayload,
    },
};
use sequencer::{Header, Payload, SeqTypes};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{Receiver, Sender};
use std::{
    collections::HashSet,
    iter::zip,
    net::IpAddr,
    sync::{Arc, RwLock},
};
use time::OffsetDateTime;

/// MAX_HISTORY represents the last N records that are stored within the
/// DataState structure for the various different sample types.
const MAX_HISTORY: usize = 50;

/// [LocationDetails] represents the details of the location of the node.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct LocationDetails {
    coords: (f64, f64),
    country: String,
}

impl LocationDetails {
    pub fn new(coords: (f64, f64), country: String) -> Self {
        Self { coords, country }
    }
}

/// [NodeIdentity] represents the identity of the node that is participating
/// in the network.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeIdentity {
    public_key: BLSPubKey,
    name: String,
    wallet_address: String,
    ip_addresses: Vec<IpAddr>,
    company: String,
    location: Option<LocationDetails>,
    operating_system: String,
    node_type: String,
    network_type: String,
}

impl NodeIdentity {
    pub fn new(
        public_key: BLSPubKey,
        name: String,
        wallet_address: String,
        ip_addresses: Vec<IpAddr>,
        company: String,
        location: Option<LocationDetails>,
        operating_system: String,
        node_type: String,
        network_type: String,
    ) -> Self {
        Self {
            public_key,
            name,
            wallet_address,
            ip_addresses,
            company,
            location,
            operating_system,
            node_type,
            network_type,
        }
    }

    pub fn from_public_key(public_key: BLSPubKey) -> Self {
        Self {
            public_key,
            name: String::new(),
            wallet_address: String::new(),
            ip_addresses: vec![],
            company: String::new(),
            location: None,
            operating_system: String::new(),
            node_type: String::new(),
            network_type: String::new(),
        }
    }
}

/// [DataState] represents the state of the data that is being stored within
/// the service.
pub struct DataState {
    pub latest_blocks: CircularBuffer<MAX_HISTORY, BlockDetail<SeqTypes>>,
    pub latest_voters: CircularBuffer<MAX_HISTORY, BitVec>,
    pub stake_table: StakeTable<BLSPubKey, StateVerKey, CircuitField>,
    // Do we need any other data at the moment?
    pub node_identity: Vec<(BLSPubKey, NodeIdentity)>,
}

/// [create_block_detail_from_leaf] is a helper function that will build a
/// [BlockDetail] from the reference to [Leaf].
fn create_block_detail_from_leaf(leaf: &Leaf<SeqTypes>) -> BlockDetail<SeqTypes> {
    let block_header = leaf.block_header();
    let block_payload = leaf.block_payload().unwrap_or(Payload::empty().0);

    BlockDetail::<SeqTypes> {
        hash: block_header.commitment(),
        height: block_header.height,
        time: Timestamp(
            OffsetDateTime::from_unix_timestamp(block_header.timestamp as i64)
                .unwrap_or(OffsetDateTime::UNIX_EPOCH),
        ),
        proposer_id: block_header.proposer_id(),
        num_transactions: block_payload.num_transactions(block_header.metadata()) as u64,
        block_reward: vec![block_header.fee_info_balance().into()],
        fee_recipient: block_header.fee_info_account(),
        size: block_payload
            .transactions(block_header.metadata())
            .fold(0, |acc, tx| acc + tx.payload().len() as u64),
    }
}

fn process_incoming_leaf(
    leaf: Leaf<SeqTypes>,
    data_state: Arc<RwLock<DataState>>,
    block_sender: Sender<BlockDetail<SeqTypes>>,
) -> Result<(), ()>
where
    Header: BlockHeader<SeqTypes> + QueryableHeader<SeqTypes> + ExplorerHeader<SeqTypes>,
    Payload: BlockPayload<SeqTypes>,
{
    let block_detail = create_block_detail_from_leaf(&leaf);
    let block_detail_copy = create_block_detail_from_leaf(&leaf);

    let certificate = leaf.justify_qc();
    let signatures = &certificate.signatures;

    // Let's take a look at the quorum certificate signatures.
    // It looks like all of these blocks are being decided by the
    // same Quorum Certificate.

    // Where's the stake table?
    let signatures = signatures.as_ref();

    // Let's determine the the participants of the voter participants
    // in the Quorum Certificate.

    // We shouldn't ever have a BitVec that is empty, with the possible
    // exception of the genesis block.
    let stake_table_voters_bit_vec = signatures.map_or(Default::default(), |sig| sig.1.clone());

    // This BitVec should be in the same order as the Stake Table.
    // The StakeTable will be able to change its order between epochs,
    // which means that its order can change between blocks.
    // However, the BitVec is a really nice size in order for storing
    // information.  We should be able to remap the BitVec order from
    // the StakeTable order to our installed order representation.  This
    // should allow us to still store as a BitVec while containing our
    // out order of the voters.
    // We will need to recompute these BitVecs if the node information that
    // is stored shrinks instead of growing.

    let mut data_state_write_lock_guard = match data_state.write() {
        Ok(guard) => guard,
        Err(_) => {
            // This lock is poisoned, and we won't ever be able to
            // acquire it. So we should just exit here.
            return Err(());
        }
    };

    let stake_table = &data_state_write_lock_guard.stake_table;
    let stable_table_entries_vec = stake_table
        .try_iter(SnapshotVersion::LastEpochStart)
        .map_or(vec![], |into_iter| into_iter.collect::<Vec<_>>());

    // We have a BitVec of voters who signed the QC.
    // We can use this to determine the weight of the QC
    let stake_table_entry_voter_participation_and_entries_pairs =
        zip(stake_table_voters_bit_vec, stable_table_entries_vec);
    let stake_table_keys_that_voted = stake_table_entry_voter_participation_and_entries_pairs
        .filter(|(bit_ref, _)| *bit_ref)
        .map(|(_, entry)| {
            // Alright this is our entry that we care about.
            // In this case, we just want to determine who voted for this
            // Leaf.

            let (key, _, _): (BLSPubKey, _, _) = entry;
            key
        });

    let voters_set: HashSet<BLSPubKey> = stake_table_keys_that_voted.collect();

    let voters_bitvec = data_state_write_lock_guard.node_identity.iter().fold(
        BitVec::with_capacity(data_state_write_lock_guard.node_identity.len()),
        |mut acc, key| {
            if voters_set.contains(&key.0) {
                acc.push(true);
            } else {
                acc.push(false);
            }
            acc
        },
    );

    data_state_write_lock_guard
        .latest_blocks
        .push_back(block_detail);
    data_state_write_lock_guard
        .latest_voters
        .push_back(voters_bitvec);

    drop(data_state_write_lock_guard);

    if let Err(_) = block_sender.send(block_detail_copy) {
        // We have an error that prevents us from continuing
        return Err(());
    }

    Ok(())
}

/// [process_leaf_stream] allows for the consumption of a [Stream] when
/// attempting to process new incoming [Leaf]s.
pub async fn process_leaf_stream<S>(
    mut stream: S,
    data_state: Arc<RwLock<DataState>>,
    block_sender: Sender<BlockDetail<SeqTypes>>,
) where
    S: Stream<Item = Leaf<SeqTypes>> + Unpin,
    Header: BlockHeader<SeqTypes> + QueryableHeader<SeqTypes> + ExplorerHeader<SeqTypes>,
    Payload: BlockPayload<SeqTypes>,
{
    while let Some(leaf) = stream.next().await {
        if let Err(_) = process_incoming_leaf(leaf, data_state.clone(), block_sender.clone()) {
            // We have an error that prevents us from continuing
            break;
        }
    }
}

/// [process_leaf_thread] allows for the consumption of a [Receiver] when
/// attempting to process new incoming [Leaf]s.
pub fn process_leaf_thread(
    receiver: Receiver<Leaf<SeqTypes>>,
    data_state: Arc<RwLock<DataState>>,
    block_sender: Sender<BlockDetail<SeqTypes>>,
) where
    Header: BlockHeader<SeqTypes> + QueryableHeader<SeqTypes> + ExplorerHeader<SeqTypes>,
    Payload: BlockPayload<SeqTypes>,
{
    while let Ok(leaf) = receiver.recv() {
        if let Err(_) = process_incoming_leaf(leaf, data_state.clone(), block_sender.clone()) {
            // We have an error that prevents us from continuing
            break;
        }
    }
}
