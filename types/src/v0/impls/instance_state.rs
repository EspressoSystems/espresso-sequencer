use std::{collections::BTreeMap, sync::Arc};

use hotshot_types::{traits::{node_implementation::NodeType, states::InstanceState}, HotShotConfig};
use vbs::version::{StaticVersionType, Version};

use crate::{
    v0::traits::StateCatchup, ChainConfig, L1Client, NodeState, PubKey, SeqTypes, Timestamp, Upgrade, UpgradeMode, ValidatedState
};

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
            genesis_header: Default::default(),
            genesis_state: ValidatedState {
                chain_config: chain_config.into(),
                ..Default::default()
            },
            l1_genesis: None,
            upgrades: Default::default(),
            current_version: <SeqTypes as NodeType>::Base::version(),
        }
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn mock() -> Self {
        Self::new(
            0,
            ChainConfig::default(),
            L1Client::new("http://localhost:3331".parse().unwrap(), 10000),
            mock::MockStateCatchup::default(),
        )
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

    pub fn with_upgrades(mut self, upgrades: BTreeMap<Version, Upgrade>) -> Self {
        self.upgrades = upgrades;
        self
    }
}

// This allows us to turn on `Default` on InstanceState trait
// which is used in `HotShot` by `TestBuilderImplementation`.
#[cfg(any(test, feature = "testing"))]
impl Default for NodeState {
    fn default() -> Self {
        Self::new(
            1u64,
            ChainConfig::default(),
            L1Client::new("http://localhost:3331".parse().unwrap(), 10000),
            mock::MockStateCatchup::default(),
        )
    }
}

impl InstanceState for NodeState {}

impl Upgrade {
    pub fn set_hotshot_config_parameters(&self, config: &mut HotShotConfig<PubKey>) {
        match &self.mode {
            UpgradeMode::View(v) => {
                config.start_proposing_view = v.start_proposing_view;
                config.stop_proposing_view = v.stop_proposing_view;
                config.start_voting_view = v.start_voting_view.unwrap_or(0);
                config.stop_voting_view = v.stop_voting_view.unwrap_or(u64::MAX);
                config.start_proposing_time = 0;
                config.stop_proposing_time = u64::MAX;
                config.start_voting_time = 0;
                config.stop_voting_time = u64::MAX;
            }
            UpgradeMode::Time(t) => {
                config.start_proposing_time = t.start_proposing_time.unix_timestamp();
                config.stop_proposing_time = t.stop_proposing_time.unix_timestamp();
                config.start_voting_time = t.start_voting_time.unwrap_or_default().unix_timestamp();
                // this should not panic because Timestamp::max() constructs the maximum possible Unix timestamp
                // using i64::MAX
                config.stop_voting_time = t
                    .stop_voting_time
                    .unwrap_or(Timestamp::max().expect("overflow"))
                    .unix_timestamp();
                config.start_proposing_view = 0;
                config.stop_proposing_view = u64::MAX;
                config.start_voting_view = 0;
                config.stop_voting_view = u64::MAX;
            }
        }
    }
}

#[cfg(any(test, feature = "testing"))]
pub mod mock {
    use std::collections::HashMap;

    use async_trait::async_trait;
    use committable::Commitment;
    use hotshot_types::data::ViewNumber;
    use jf_merkle_tree::{ForgetableMerkleTreeScheme, MerkleTreeScheme};

    use super::*;
    use crate::{
        v0_1::{AccountQueryData, FeeAccountProof},
        BackoffParams, BlockMerkleTree, FeeAccount, FeeMerkleCommitment,
    };

    #[derive(Debug, Clone, Default)]
    pub struct MockStateCatchup {
        backoff: BackoffParams,
        state: HashMap<ViewNumber, Arc<ValidatedState>>,
    }

    impl FromIterator<(ViewNumber, Arc<ValidatedState>)> for MockStateCatchup {
        fn from_iter<I: IntoIterator<Item = (ViewNumber, Arc<ValidatedState>)>>(iter: I) -> Self {
            Self {
                backoff: Default::default(),
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

        async fn try_fetch_chain_config(
            &self,
            _commitment: Commitment<ChainConfig>,
        ) -> anyhow::Result<ChainConfig> {
            Ok(ChainConfig::default())
        }

        fn backoff(&self) -> &BackoffParams {
            &self.backoff
        }
    }
}
