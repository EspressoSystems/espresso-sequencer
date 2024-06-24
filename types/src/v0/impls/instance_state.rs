use std::{collections::HashMap, str::FromStr, sync::Arc, time::Duration};

use committable::{Commitment, Committable, RawCommitmentBuilder};
use ethers::providers::{Http, Provider};
use hotshot_types::{
    consensus::CommitmentMap,
    data::{DaProposal, VidDisperseShare, ViewNumber},
    event::HotShotAction,
    light_client::{StateKeyPair, StateSignKey},
    message::Proposal,
    signature_key::{BLSPrivKey, BLSPubKey},
    simple_certificate::QuorumCertificate,
    traits::{
        metrics::Metrics,
        network::ConnectedNetwork,
        node_implementation::{NodeImplementation, NodeType},
        states::InstanceState,
        storage::Storage,
    },
    utils::{BuilderCommitment, View},
    ValidatorConfig,
};

use url::Url;

use crate::{ChainConfig, L1Client, NodeState, StateCatchup, ValidatedState};

impl NodeState {
    pub fn new(
        node_id: u64,
        chain_config: ChainConfig,
        l1_client: L1Client,
        catchup: impl StateCatchup + 'static,
    ) -> Self {
        Self {
            node_id,
            chain_config,
            l1_client,
            peers: Arc::new(catchup),
            genesis_state: Default::default(),
            l1_genesis: None,
        }
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn mock() -> Self {
        use mock::MockStateCatchup;

        Self::new(
            0,
            ChainConfig::default(),
            L1Client::new("http://localhost:3331".parse().unwrap(), 10000),
            MockStateCatchup::default(),
        )
    }

    pub fn chain_config(&self) -> &ChainConfig {
        &self.chain_config
    }

    pub fn with_l1(mut self, l1_client: L1Client) -> Self {
        self.l1_client = l1_client;
        self
    }

    pub fn with_genesis(mut self, state: ValidatedState) -> Self {
        self.genesis_state = state;
        self
    }

    pub fn with_chain_config(mut self, cfg: ChainConfig) -> Self {
        self.chain_config = cfg;
        self
    }

    pub(crate) fn l1_client(&self) -> &L1Client {
        &self.l1_client
    }
}

// This allows us to turn on `Default` on InstanceState trait
// which is used in `HotShot` by `TestBuilderImplementation`.
#[cfg(any(test, feature = "testing"))]
impl Default for NodeState {
    fn default() -> Self {
        let provider = Arc::new(Provider::new(Http::new(
            Url::from_str("http://localhost:3331").unwrap(),
        )));

        Self::new(
            1u64,
            ChainConfig::default(),
            L1Client {
                retry_delay: Duration::from_secs(1),
                provider,
                events_max_block_range: 10000,
            },
            mock::MockStateCatchup::default(),
        )
    }
}

impl InstanceState for NodeState {}

// move

#[cfg(any(test, feature = "testing"))]
pub mod mock {
    use crate::{AccountQueryData, BlockMerkleTree, FeeAccount, FeeAccountProof};

    use super::*;
    use crate::FeeMerkleCommitment;
    use async_trait::async_trait;
    use jf_merkle_tree::{ForgetableMerkleTreeScheme, MerkleTreeScheme};
    use std::collections::HashMap;

    #[derive(Debug, Clone, Default)]
    pub struct MockStateCatchup {
        state: HashMap<ViewNumber, Arc<ValidatedState>>,
    }

    impl FromIterator<(ViewNumber, Arc<ValidatedState>)> for MockStateCatchup {
        fn from_iter<I: IntoIterator<Item = (ViewNumber, Arc<ValidatedState>)>>(iter: I) -> Self {
            Self {
                state: iter.into_iter().collect(),
            }
        }
    }

    #[async_trait]
    impl StateCatchup for MockStateCatchup {
        async fn try_fetch_account(
            &self,
            _height: u64,
            view: ViewNumber,
            fee_merkle_tree_root: FeeMerkleCommitment,
            account: FeeAccount,
        ) -> anyhow::Result<AccountQueryData> {
            let src = &self.state[&view].fee_merkle_tree;
            assert_eq!(src.commitment(), fee_merkle_tree_root);

            tracing::info!("catchup: fetching account {account:?} for view {view:?}");
            Ok(FeeAccountProof::prove(src, account.into())
                .unwrap_or_else(|| panic!("Account {account:?} not in memory"))
                .into())
        }

        async fn try_remember_blocks_merkle_tree(
            &self,
            _height: u64,
            view: ViewNumber,
            mt: &mut BlockMerkleTree,
        ) -> anyhow::Result<()> {
            tracing::info!("catchup: fetching frontier for view {view:?}");
            let src = &self.state[&view].block_merkle_tree;

            assert_eq!(src.commitment(), mt.commitment());
            assert!(
                src.num_leaves() > 0,
                "catchup should not be triggered when blocks tree is empty"
            );

            let index = src.num_leaves() - 1;
            let (elem, proof) = src.lookup(index).expect_ok().unwrap();
            mt.remember(index, elem, proof.clone())
                .expect("Proof verifies");

            Ok(())
        }
    }
}
