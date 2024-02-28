pub mod api;
pub mod block;
mod chain_variables;
pub mod context;
mod header;
pub mod hotshot_commitment;
pub mod options;
pub mod state_signature;
use block::entry::TxTableEntryWord;
use context::SequencerContext;
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{coins_bip39::English, MnemonicBuilder, Signer as _, Wallet},
    types::{Address, U256},
};
// Should move `STAKE_TABLE_CAPACITY` in the sequencer repo when we have variate stake table support
use hotshot_stake_table::config::STAKE_TABLE_CAPACITY;
use state::FeeAccount;
use state_signature::static_stake_table_commitment;
use url::Url;
mod l1_client;
pub mod persistence;
mod state;
pub mod transaction;
mod vm;

use ark_ec::models::CurveConfig;
use ark_ed_on_bn254::EdwardsConfig;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use derivative::Derivative;
use hotshot::traits::implementations::MemoryNetwork;
use hotshot::{
    traits::{
        election::static_committee::{GeneralStaticCommittee, StaticElectionConfig},
        implementations::{MemoryStorage, NetworkingMetricsValue, WebServerNetwork},
    },
    types::{SignatureKey, SystemContextHandle},
    HotShotInitializer, Memberships, Networks, SystemContext,
};
use hotshot_types::{traits::network::ConnectedNetwork, ValidatorConfig};
use jf_primitives::signatures::schnorr;
use tagged_base64::tagged;
use zeroize::Zeroize;

use hotshot_orchestrator::{
    client::{OrchestratorClient, ValidatorArgs},
    config::NetworkConfig,
};
use hotshot_types::{
    consensus::ConsensusMetricsValue,
    data::ViewNumber,
    light_client::StateKeyPair,
    signature_key::{BLSPrivKey, BLSPubKey},
    traits::{
        election::Membership,
        metrics::Metrics,
        node_implementation::{NodeImplementation, NodeType},
        states::InstanceState,
    },
    HotShotConfig,
};

use persistence::SequencerPersistence;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::net::Ipv4Addr;
use std::time::Duration;
use std::{fmt::Debug, sync::Arc};
use std::{marker::PhantomData, net::IpAddr};

pub use block::payload::Payload;
pub use chain_variables::ChainVariables;
pub use header::Header;
pub use l1_client::L1BlockInfo;
pub use options::Options;
pub use state::ValidatedState;
pub use transaction::Transaction;
pub use vm::{Vm, VmId, VmTransaction};

/// Initialize the static variables for the sequencer
///
/// Calling it early on startup makes it easier to catch errors.
pub fn init_static() {
    lazy_static::initialize(&header::L1_CLIENT);
}

pub mod network {
    use hotshot_types::message::Message;

    use super::*;

    pub trait Type: 'static {
        type DAChannel: ConnectedNetwork<Message<SeqTypes>, PubKey> + Debug;
        type QuorumChannel: ConnectedNetwork<Message<SeqTypes>, PubKey> + Debug;
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Web;

    impl Type for Web {
        type DAChannel = WebServerNetwork<SeqTypes>;
        type QuorumChannel = WebServerNetwork<SeqTypes>;
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Memory;

    impl Type for Memory {
        type DAChannel = MemoryNetwork<Message<SeqTypes>, PubKey>;
        type QuorumChannel = MemoryNetwork<Message<SeqTypes>, PubKey>;
    }
}

/// The Sequencer node is generic over the hotshot CommChannel.
#[derive(Derivative, Serialize, Deserialize)]
#[derivative(
    Copy(bound = ""),
    Debug(bound = ""),
    Default(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
pub struct Node<N: network::Type>(PhantomData<fn(&N)>);

// Using derivative to derive Clone triggers the clippy lint
// https://rust-lang.github.io/rust-clippy/master/index.html#/incorrect_clone_impl_on_copy_type
impl<N: network::Type> Clone for Node<N> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(
    Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct SeqTypes;

pub type Leaf = hotshot_types::data::Leaf<SeqTypes>;
pub type Storage = MemoryStorage<SeqTypes>;
pub type Event = hotshot::types::Event<SeqTypes>;

pub type PubKey = BLSPubKey;
pub type PrivKey = <PubKey as SignatureKey>::PrivateKey;

/// A private signing key for the Schnorr signature scheme.
///
/// This key is used to sign state commitments for use with the light client.
// Note: defining this type here, rather than using `SignKey` from jf-primitives, is a workaround to
// allow us to extract the inner field element and thus generate a `KeyPair`.
// See https://github.com/EspressoSystems/jellyfish/issues/497.
#[tagged("STATEKEY")]
#[derive(Clone, Copy, Debug, Default, CanonicalSerialize, CanonicalDeserialize, Zeroize)]
pub struct SchnorrPrivKey(<EdwardsConfig as CurveConfig>::ScalarField);

pub type SchnorrPubKey = schnorr::VerKey<EdwardsConfig>;

impl SchnorrPrivKey {
    pub fn generate_from_seed_indexed(seed: [u8; 32], index: u64) -> Self {
        Self(
            *StateKeyPair::generate_from_seed_indexed(seed, index)
                .0
                .sign_key_internal(),
        )
    }

    pub fn key_pair(self) -> StateKeyPair {
        StateKeyPair(schnorr::KeyPair::generate_with_sign_key(self.0))
    }

    pub fn pub_key(self) -> SchnorrPubKey {
        self.key_pair().0.ver_key()
    }
}

type ElectionConfig = StaticElectionConfig;

impl<N: network::Type> NodeImplementation<SeqTypes> for Node<N> {
    type Storage = Storage;
    type QuorumNetwork = N::QuorumChannel;
    type CommitteeNetwork = N::DAChannel;
}

#[derive(Clone, Debug)]
pub struct NodeState {
    genesis_state: ValidatedState,
    builder_address: Wallet<SigningKey>,
}

impl Default for NodeState {
    fn default() -> Self {
        let wallet = FeeAccount::test_wallet();

        Self {
            genesis_state: ValidatedState::default(),
            builder_address: wallet,
        }
    }
}

impl InstanceState for NodeState {}

impl NodeType for SeqTypes {
    type Time = ViewNumber;
    type BlockHeader = Header;
    type BlockPayload = Payload<TxTableEntryWord>;
    type SignatureKey = PubKey;
    type Transaction = Transaction;
    type ElectionConfigType = ElectionConfig;
    type InstanceState = NodeState;
    type ValidatedState = ValidatedState;
    type Membership = GeneralStaticCommittee<Self, PubKey>;
}

#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
pub enum Error {
    // TODO: Can we nest these errors in a `ValidationError` to group them?

    // Parent state commitment of block doesn't match current state commitment
    IncorrectParent,

    // New view number isn't strictly after current view
    IncorrectView,

    // Genesis block either has zero or more than one transaction
    GenesisWrongSize,

    // Genesis transaction not present in genesis block
    MissingGenesis,

    // Genesis transaction in non-genesis block
    UnexpectedGenesis,

    // Merkle tree error
    MerkleTreeError { error: String },

    BlockBuilding,
}

#[allow(clippy::too_many_arguments)]
async fn init_hotshot<N: network::Type>(
    node_id: usize,
    priv_key: PrivKey,
    networks: Networks<SeqTypes, Node<N>>,
    config: HotShotConfig<PubKey, ElectionConfig>,
    metrics: &dyn Metrics,
    instance_state: NodeState,
) -> SystemContextHandle<SeqTypes, Node<N>> {
    let pub_key = config.known_nodes_with_stake[node_id]
        .stake_table_entry
        .stake_key;
    let election_config = GeneralStaticCommittee::<SeqTypes, PubKey>::default_election_config(
        config.total_nodes.get() as u64,
    );
    let membership = GeneralStaticCommittee::create_election(
        config.known_nodes_with_stake.clone(),
        election_config,
    );
    let memberships = Memberships {
        quorum_membership: membership.clone(),
        da_membership: membership.clone(),
        vid_membership: membership.clone(),
        view_sync_membership: membership,
    };

    SystemContext::init(
        pub_key,
        priv_key,
        node_id as u64,
        config,
        Storage::empty(),
        memberships,
        networks,
        HotShotInitializer::from_genesis(instance_state).unwrap(),
        ConsensusMetricsValue::new(metrics),
    )
    .await
    .unwrap()
    .0
}

#[derive(Clone, Debug)]
pub struct NetworkParams {
    pub da_server_url: Url,
    pub consensus_server_url: Url,
    pub orchestrator_url: Url,
    pub state_relay_server_url: Url,
    pub webserver_poll_interval: Duration,
    pub private_staking_key: BLSPrivKey,
    pub private_state_key: SchnorrPrivKey,
}

#[derive(Clone, Debug)]
pub struct BuilderParams {
    pub mnemonic: String,
    pub eth_account_index: u32,
    pub prefunded_accounts: Vec<Address>,
}

pub async fn init_node(
    network_params: NetworkParams,
    metrics: &dyn Metrics,
    persistence: &mut impl SequencerPersistence,
    builder_params: BuilderParams,
) -> anyhow::Result<SequencerContext<network::Web>> {
    // Orchestrator client
    let validator_args = ValidatorArgs {
        url: network_params.orchestrator_url,
        public_ip: None,
        network_config_file: None,
    };
    // This "public" IP only applies to libp2p network configurations, so we can supply any value here
    let public_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let orchestrator_client = OrchestratorClient::new(validator_args, public_ip.to_string());
    let my_config = ValidatorConfig {
        public_key: BLSPubKey::from_private(&network_params.private_staking_key),
        private_key: network_params.private_staking_key,
        stake_value: 1,
        state_key_pair: network_params.private_state_key.key_pair(),
    };

    let (config, wait_for_orchestrator) = match persistence.load_config().await? {
        Some(config) => {
            tracing::info!("loaded network config from storage, rejoining existing network");
            (config, false)
        }
        None => {
            tracing::info!("loading network config from orchestrator");
            let config =
                NetworkConfig::get_complete_config(&orchestrator_client, my_config.clone(), None)
                    .await
                    .0;
            tracing::info!("loaded config, we are node {}", config.node_index);
            persistence.save_config(&config).await?;
            (config, true)
        }
    };
    let node_index = config.node_index;

    // Initialize networking.
    let networks = Networks {
        da_network: Arc::new(WebServerNetwork::create(
            network_params.da_server_url,
            network_params.webserver_poll_interval,
            my_config.public_key,
            true,
        )),
        quorum_network: Arc::new(WebServerNetwork::create(
            network_params.consensus_server_url,
            network_params.webserver_poll_interval,
            my_config.public_key,
            false,
        )),
        _pd: Default::default(),
    };

    // The web server network doesn't have any metrics. By creating and dropping a
    // `NetworkingMetricsValue`, we ensure the networking metrics are created, but just not
    // populated, so that monitoring software built to work with network-related metrics doesn't
    // crash horribly just because we're not using the P2P network yet.
    let _ = NetworkingMetricsValue::new(metrics);

    let wallet = MnemonicBuilder::<English>::default()
        .phrase::<&str>(&builder_params.mnemonic)
        .index(builder_params.eth_account_index)?
        .build()?;
    tracing::info!("Builder account address {:?}", wallet.address());

    let mut genesis_state = ValidatedState::default();
    for address in builder_params.prefunded_accounts {
        tracing::warn!("Prefunding account {:?} for demo", address);
        genesis_state.prefund_account(address.into(), U256::max_value().into());
    }

    let instance_state = NodeState {
        builder_address: wallet,
        genesis_state,
    };
    let stake_table_comm =
        static_stake_table_commitment(&config.config.known_nodes_with_stake, STAKE_TABLE_CAPACITY);

    let hotshot = init_hotshot(
        node_index as usize,
        my_config.private_key,
        networks,
        config.config,
        metrics,
        instance_state,
    )
    .await;
    let mut ctx = SequencerContext::new(
        hotshot,
        node_index,
        my_config.state_key_pair,
        stake_table_comm,
    )
    .with_state_relay_server(network_params.state_relay_server_url);
    if wait_for_orchestrator {
        ctx = ctx.wait_for_orchestrator(orchestrator_client);
    }
    Ok(ctx)
}

#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use commit::Committable;
    use futures::{Stream, StreamExt};
    use hotshot::traits::implementations::{MasterMap, MemoryNetwork};
    use hotshot::types::EventType::Decide;
    use hotshot_types::{
        traits::block_contents::{BlockHeader, BlockPayload},
        PeerConfig,
    };

    use hotshot_types::{
        light_client::StateKeyPair, traits::metrics::NoMetrics, ExecutionType, ValidatorConfig,
    };
    use std::time::Duration;

    pub async fn init_hotshot_handles() -> Vec<SystemContextHandle<SeqTypes, Node<network::Memory>>>
    {
        init_hotshot_handles_with_metrics(&NoMetrics).await
    }

    pub async fn init_hotshot_handles_with_metrics(
        metrics: &dyn Metrics,
    ) -> Vec<SystemContextHandle<SeqTypes, Node<network::Memory>>> {
        setup_logging();
        setup_backtrace();

        let num_nodes = 5;

        // Generate keys for the nodes.
        let priv_keys = (0..num_nodes)
            .map(|_| PrivKey::generate(&mut rand::thread_rng()))
            .collect::<Vec<_>>();
        let pub_keys = priv_keys
            .iter()
            .map(PubKey::from_private)
            .collect::<Vec<_>>();
        let state_key_pairs = (0..num_nodes)
            .map(|_| StateKeyPair::generate())
            .collect::<Vec<_>>();
        let known_nodes_with_stake = pub_keys
            .iter()
            .zip(&state_key_pairs)
            .map(|(pub_key, state_key_pair)| PeerConfig::<PubKey> {
                stake_table_entry: pub_key.get_stake_table_entry(1),
                state_ver_key: state_key_pair.ver_key(),
            })
            .collect::<Vec<_>>();

        let mut handles = vec![];

        let master_map = MasterMap::new();

        let config: HotShotConfig<_, _> = HotShotConfig {
            execution_type: ExecutionType::Continuous,
            total_nodes: num_nodes.try_into().unwrap(),
            min_transactions: 1,
            max_transactions: 10000.try_into().unwrap(),
            known_nodes_with_stake,
            next_view_timeout: Duration::from_secs(5).as_millis() as u64,
            timeout_ratio: (10, 11),
            round_start_delay: Duration::from_millis(1).as_millis() as u64,
            start_delay: Duration::from_millis(1).as_millis() as u64,
            num_bootstrap: 1usize,
            propose_min_round_time: Duration::from_secs(0),
            propose_max_round_time: Duration::from_secs(1),
            election_config: None,
            da_committee_size: num_nodes,
            my_own_validator_config: Default::default(),
        };

        // Create HotShot instances.
        for node_id in 0..num_nodes {
            let metrics = if node_id == 0 { metrics } else { &NoMetrics };

            let mut config = config.clone();
            config.my_own_validator_config = ValidatorConfig {
                public_key: pub_keys[node_id],
                private_key: priv_keys[node_id].clone(),
                stake_value: config.known_nodes_with_stake[node_id]
                    .stake_table_entry
                    .stake_amount
                    .as_u64(),
                state_key_pair: state_key_pairs[node_id].clone(),
            };

            let network = Arc::new(MemoryNetwork::new(
                pub_keys[node_id],
                NetworkingMetricsValue::new(metrics),
                master_map.clone(),
                None,
            ));
            let networks = Networks {
                da_network: network.clone(),
                quorum_network: network,
                _pd: Default::default(),
            };
            let instance_state = NodeState::default();
            let handle = init_hotshot(
                node_id,
                priv_keys[node_id].clone(),
                networks,
                config,
                metrics,
                instance_state,
            )
            .await;

            handles.push(handle);
        }
        handles
    }

    // Wait for decide event, make sure it matches submitted transaction. Return the block number
    // containing the transaction.
    pub async fn wait_for_decide_on_handle(
        events: &mut (impl Stream<Item = Event> + Unpin),
        submitted_txn: &Transaction,
    ) -> u64 {
        let commitment = submitted_txn.commit();

        // Keep getting events until we see a Decide event
        loop {
            let event = events.next().await.unwrap();
            tracing::info!("Received event from handle: {event:?}");

            if let Decide { leaf_chain, .. } = event.event {
                if let Some(height) = leaf_chain.iter().find_map(|(leaf, _)| {
                    if leaf
                        .block_payload
                        .as_ref()?
                        .transaction_commitments(leaf.get_block_header().metadata())
                        .contains(&commitment)
                    {
                        Some(leaf.get_block_header().block_number())
                    } else {
                        None
                    }
                }) {
                    return height;
                }
            } else {
                // Keep waiting
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::{transaction::ApplicationTransaction, vm::TestVm, *};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use futures::StreamExt;
    use hotshot::types::EventType::Decide;

    use hotshot_types::traits::block_contents::{
        vid_commitment, BlockHeader, BlockPayload, GENESIS_VID_NUM_STORAGE_NODES,
    };
    use testing::{init_hotshot_handles, wait_for_decide_on_handle};

    #[async_std::test]
    async fn test_skeleton_instantiation() {
        setup_logging();
        setup_backtrace();

        let handles = init_hotshot_handles().await;
        let mut events = handles[0].get_event_stream();
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        // Submit target transaction to handle
        let txn = ApplicationTransaction::new(vec![1, 2, 3]);
        let submitted_txn = Transaction::new(TestVm {}.id(), bincode::serialize(&txn).unwrap());
        handles[0]
            .submit_transaction(submitted_txn.clone())
            .await
            .expect("Failed to submit transaction");
        tracing::info!("Submitted transaction to handle: {txn:?}");

        wait_for_decide_on_handle(&mut events, &submitted_txn).await;
    }

    #[async_std::test]
    async fn test_header_invariants() {
        setup_logging();
        setup_backtrace();

        let success_height = 30;

        let handles = init_hotshot_handles().await;
        let mut events = handles[0].get_event_stream();
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        let mut parent = {
            // TODO refactor repeated code from other tests
            let (genesis_payload, genesis_ns_table) = Payload::genesis();
            let genesis_commitment = {
                // TODO we should not need to collect payload bytes just to compute vid_commitment
                let payload_bytes = genesis_payload
                    .encode()
                    .expect("unable to encode genesis payload")
                    .collect();
                vid_commitment(&payload_bytes, GENESIS_VID_NUM_STORAGE_NODES)
            };
            let genesis_state = NodeState::default();
            Header::genesis(&genesis_state, genesis_commitment, genesis_ns_table)
        };

        loop {
            let event = events.next().await.unwrap();
            let Decide { leaf_chain, .. } = event.event else {
                continue;
            };
            tracing::info!("Got decide {leaf_chain:?}");

            // Check that each successive header satisfies invariants relative to its parent: all
            // the fields which should be monotonic are.
            for (leaf, _) in leaf_chain.iter().rev() {
                let header = leaf.block_header.clone();
                if header.height == 0 {
                    parent = header;
                    continue;
                }
                dbg!(header.height, parent.height);
                assert_eq!(header.height, parent.height + 1);
                assert!(header.timestamp >= parent.timestamp);
                assert!(header.l1_head >= parent.l1_head);
                assert!(header.l1_finalized >= parent.l1_finalized);
                parent = header;
            }

            if parent.height >= success_height {
                break;
            }
        }
    }
}
