mod block;
mod chain_variables;
mod state;
mod transaction;
mod vm;

use crate::{block::Block, state::State};
use hotshot::traits::{
    election::{
        static_committee::{StaticCommittee, StaticElectionConfig, StaticVoteToken},
        vrf::JfPubKey,
    },
    implementations::{MemoryNetwork, MemoryStorage},
    NodeImplementation,
};
use hotshot_types::{data::ViewNumber, traits::node_implementation::NodeTypes};
use jf_primitives::signatures::BLSSignatureScheme;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Debug;
use transaction::SequencerTransaction;

#[derive(Debug, Clone)]
struct Node;

impl NodeImplementation<SeqTypes> for Node {
    type Storage = MemoryStorage<SeqTypes>;

    type Networking = MemoryNetwork<SeqTypes>;

    type Election = StaticCommittee<SeqTypes>;
}

#[derive(
    Clone, Copy, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize,
)]
struct SeqTypes;

type Param381 = ark_bls12_381::Parameters;
type SignatureSchemeType = BLSSignatureScheme<Param381>;
type SignatureKeyType = JfPubKey<SignatureSchemeType>;

impl NodeTypes for SeqTypes {
    type Time = ViewNumber;

    type BlockType = Block;

    type SignatureKey = SignatureKeyType;

    type VoteTokenType = StaticVoteToken<SignatureKeyType>;

    type Transaction = SequencerTransaction;

    type ElectionConfigType = StaticElectionConfig;

    type StateType = State;
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
}

#[cfg(test)]
mod test {

    use crate::{
        transaction::{ApplicationTransaction, Transaction},
        vm::{TestVm, Vm},
    };

    use super::*;
    use hotshot::{
        traits::implementations::{MasterMap, MemoryNetwork},
        types::HotShotHandle,
        HotShot, HotShotInitializer,
    };
    use hotshot_types::{traits::metrics::NoMetrics, ExecutionType, HotShotConfig};
    use jf_primitives::signatures::SignatureScheme; // This trait provides the `key_gen` method.
    use rand::thread_rng;
    use std::time::Duration;

    #[async_std::test]
    async fn test_skeleton_instantiation() -> Result<(), ()> {
        // The minimal number of nodes is 4
        let num_nodes = 4usize;

        // Generate keys for the nodes.
        let nodes_key_pairs = (0..num_nodes)
            .map(|_| SignatureSchemeType::key_gen(&(), &mut thread_rng()).unwrap())
            .collect::<Vec<_>>();

        // Convert public keys to JfPubKey
        let nodes_pub_keys = nodes_key_pairs
            .iter()
            .map(|(_sign_key, ver_key)| JfPubKey::from_native(ver_key.clone()))
            .collect::<Vec<_>>();

        let mut handles = vec![];

        // Create HotShot instances.
        for (node_id, (sign_key, ver_key)) in nodes_key_pairs.iter().enumerate() {
            // Create public and private keys for the node.
            let public_key = JfPubKey::from_native(ver_key.clone());

            let config: HotShotConfig<_, _> = HotShotConfig {
                execution_type: ExecutionType::Continuous,
                total_nodes: num_nodes.try_into().unwrap(),
                min_transactions: 0,
                max_transactions: 1usize.try_into().unwrap(),
                known_nodes: nodes_pub_keys.clone(),
                next_view_timeout: Duration::from_secs(60).as_millis() as u64,
                timeout_ratio: (10, 11),
                round_start_delay: Duration::from_millis(1).as_millis() as u64,
                start_delay: Duration::from_millis(1).as_millis() as u64,
                num_bootstrap: 1usize,
                propose_min_round_time: Duration::from_secs(1),
                propose_max_round_time: Duration::from_secs(30),
                election_config: Some(StaticElectionConfig {}),
            };

            let network = MemoryNetwork::<SeqTypes>::new(
                public_key.clone(),
                NoMetrics::new(),
                MasterMap::new(),
                None,
            );
            let storage = MemoryStorage::<SeqTypes>::new();
            let election = StaticCommittee::<SeqTypes>::new(nodes_pub_keys.clone());
            let genesis_block = Block::genesis(Default::default());
            let initializer = HotShotInitializer::<SeqTypes>::from_genesis(genesis_block).unwrap();
            let metrics = NoMetrics::new();

            let handle: HotShotHandle<SeqTypes, Node> = HotShot::init(
                public_key,
                (sign_key.clone(), ver_key.clone()),
                node_id as u64,
                config,
                network,
                storage,
                election,
                initializer,
                metrics,
            )
            .await
            .unwrap();

            handles.push(handle);
        }

        for handle in handles.iter() {
            handle.start().await;
        }
        println!("Started");

        let event = handles[0].next_event().await;
        println!("Event: {:?}", event);

        let txn = ApplicationTransaction::new(vec![1, 2, 3]);

        handles[0]
            .submit_transaction(SequencerTransaction::Wrapped(Transaction::new(
                TestVm::id(),
                bincode::serialize(&txn).unwrap(),
            )))
            .await
            .expect("Failed to submit transaction");

        println!("Submitted: {:?}", txn);

        let event = handles[0].next_event().await;
        println!("Event: {:?}", event);

        Ok(())
    }
}
