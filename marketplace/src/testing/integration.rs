//! This module implements interfaces necessary to run marketplace builder
//! in HotShot testing harness.

use std::{collections::HashMap, fmt::Display, marker::PhantomData, sync::Arc, time::Duration};

use async_lock::RwLock;
use async_trait::async_trait;
use hotshot::types::SignatureKey;
use hotshot_example_types::node_types::TestVersions;
use hotshot_testing::{
    block_builder::{BuilderTask, TestBuilderImplementation},
    test_builder::BuilderChange,
};
use hotshot_types::{
    data::{Leaf, ViewNumber},
    message::UpgradeLock,
    traits::{
        block_contents::{vid_commitment, BlockPayload, EncodeBytes},
        node_implementation::{ConsensusTime, NodeType},
        signature_key::BuilderSignatureKey,
        states::ValidatedState,
    },
};
use marketplace_builder_shared::block::ParentBlockReferences;
use tagged_base64::TaggedBase64;
use tokio::spawn;
use url::Url;
use vbs::version::StaticVersion;

use crate::{
    builder_state::BuilderState,
    service::{
        run_builder_service, BroadcastSenders, BuilderHooks, GlobalState, NoHooks, ProxyGlobalState,
    },
};

const BUILDER_CHANNEL_CAPACITY: usize = 1024;

/// Testing configuration for marketplace builder
/// Stores hooks that will be used in the builder in a type-erased manner,
/// allowing for runtime configuration of hooks to use in tests.
struct TestMarketplaceBuilderConfig<Types>
where
    Types: NodeType,
{
    hooks: Arc<Box<dyn BuilderHooks<Types>>>,
}

impl<Types> Default for TestMarketplaceBuilderConfig<Types>
where
    Types: NodeType,
{
    fn default() -> Self {
        Self {
            hooks: Arc::new(Box::new(NoHooks(PhantomData))),
        }
    }
}

/// [`TestBuilderImplementation`] for marketplace builder.
/// Passed as a generic parameter to [`TestRunner::run_test`], it is be used
/// to instantiate builder API and builder task.
struct MarketplaceBuilderImpl {}

#[async_trait]
impl<Types> TestBuilderImplementation<Types> for MarketplaceBuilderImpl
where
    Types: NodeType<View = ViewNumber>,
    Types::InstanceState: Default,
    for<'a> <<Types::SignatureKey as SignatureKey>::PureAssembledSignatureType as TryFrom<
        &'a TaggedBase64,
    >>::Error: Display,
    for<'a> <Types::SignatureKey as TryFrom<&'a TaggedBase64>>::Error: Display,
{
    type Config = TestMarketplaceBuilderConfig<Types>;

    /// This is mostly boilerplate to instantiate and start [`ProxyGlobalState`] APIs and initial [`BuilderState`]'s event loop.
    /// [`BuilderTask`] it returns will be injected into consensus runtime by HotShot testing harness and
    /// will forward transactions from hotshot event stream to the builder.
    async fn start(
        n_nodes: usize,
        url: Url,
        config: Self::Config,
        _changes: HashMap<u64, BuilderChange>,
    ) -> Box<dyn BuilderTask<Types>> {
        let instance_state = Types::InstanceState::default();
        let (validated_state, _) = Types::ValidatedState::genesis(&instance_state);

        let builder_key_pair = Types::BuilderSignatureKey::generated_from_seed_indexed([0; 32], 0);

        let (senders, receivers) = crate::service::broadcast_channels(BUILDER_CHANNEL_CAPACITY);

        // builder api request channel
        let (req_sender, req_receiver) = async_broadcast::broadcast::<_>(BUILDER_CHANNEL_CAPACITY);

        let (genesis_payload, genesis_ns_table) =
            Types::BlockPayload::from_transactions([], &validated_state, &instance_state)
                .await
                .expect("genesis payload construction failed");

        let builder_commitment = genesis_payload.builder_commitment(&genesis_ns_table);

        let vid_commitment = {
            let payload_bytes = genesis_payload.encode();
            vid_commitment(&payload_bytes, n_nodes)
        };

        // create the global state
        let global_state: GlobalState<Types> = GlobalState::<Types>::new(
            req_sender,
            senders.transactions.clone(),
            vid_commitment,
            Types::View::genesis(),
        );

        let global_state = Arc::new(RwLock::new(global_state));

        let leaf = Leaf::genesis(&validated_state, &instance_state).await;

        let builder_state = BuilderState::<Types>::new(
            ParentBlockReferences {
                view_number: Types::View::genesis(),
                vid_commitment,
                leaf_commit: leaf
                    .commit(&UpgradeLock::<Types, TestVersions>::new())
                    .await,
                builder_commitment,
            },
            &receivers,
            req_receiver,
            Vec::new(), /* tx_queue */
            Arc::clone(&global_state),
            Duration::from_millis(1),
            10,
            Arc::new(instance_state),
            Duration::from_secs(60),
            Arc::new(validated_state),
        );

        builder_state.event_loop();

        let hooks = Arc::new(NoHooks(PhantomData));

        // create the proxy global state it will server the builder apis
        let app = ProxyGlobalState::new(
            global_state.clone(),
            Arc::clone(&hooks),
            builder_key_pair,
            Duration::from_millis(500),
        )
        .into_app()
        .expect("Failed to create builder tide-disco app");

        let url_clone = url.clone();
        spawn(async move {
            tracing::error!("Starting builder app on {url_clone}");
            if let Err(e) = app.serve(url_clone, StaticVersion::<0, 1> {}).await {
                tracing::error!(?e, "Builder API App exited with error");
            } else {
                tracing::error!("Builder API App exited");
            }
        });

        Box::new(MarketplaceBuilderTask {
            hooks: config.hooks,
            senders,
        })
    }
}

/// Marketplace builder task. Stores all the necessary information to run builder service
struct MarketplaceBuilderTask<Types>
where
    Types: NodeType,
{
    hooks: Arc<Box<dyn BuilderHooks<Types>>>,
    senders: BroadcastSenders<Types>,
}

impl<Types> BuilderTask<Types> for MarketplaceBuilderTask<Types>
where
    Types: NodeType<View = ViewNumber>,
{
    fn start(
        self: Box<Self>,
        stream: Box<
            dyn futures::prelude::Stream<Item = hotshot::types::Event<Types>>
                + std::marker::Unpin
                + Send
                + 'static,
        >,
    ) {
        spawn(run_builder_service(self.hooks, self.senders, stream));
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::testing::integration::MarketplaceBuilderImpl;
    use marketplace_builder_shared::testing::{
        generation::{self, TransactionGenerationConfig},
        run_test,
        validation::BuilderValidationConfig,
    };

    use hotshot_example_types::node_types::MarketplaceTestVersions;
    use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
    use hotshot_macros::cross_tests;
    use hotshot_testing::{
        completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
        overall_safety_task::OverallSafetyPropertiesDescription,
        test_builder::TestDescription,
    };

    #[tokio::test(flavor = "multi_thread")]
    #[tracing::instrument]
    #[ignore = "slow"]
    async fn example_test() {
        let num_successful_views = 45;
        let min_txns_per_view = 5;

        run_test::<MarketplaceTestVersions, MarketplaceBuilderImpl>(
            TestDescription {
                completion_task_description:
                    CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                        TimeBasedCompletionTaskDescription {
                            duration: Duration::from_secs(60),
                        },
                    ),
                overall_safety_properties: OverallSafetyPropertiesDescription {
                    num_successful_views,
                    num_failed_views: 5,
                    ..Default::default()
                },
                ..TestDescription::default()
            },
            BuilderValidationConfig {
                expected_txn_num: num_successful_views * min_txns_per_view,
            },
            TransactionGenerationConfig {
                strategy: generation::GenerationStrategy::Random {
                    min_per_view: min_txns_per_view,
                    max_per_view: 10,
                    min_tx_size: 128,
                    max_tx_size: 1280,
                },
                endpoints: vec![],
            },
        )
        .await
    }

    #[tokio::test(flavor = "multi_thread")]
    #[tracing::instrument]
    #[ignore = "slow"]
    async fn stress_test() {
        run_test::<MarketplaceTestVersions, MarketplaceBuilderImpl>(
            TestDescription {
                completion_task_description:
                    CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                        TimeBasedCompletionTaskDescription {
                            duration: Duration::from_secs(60),
                        },
                    ),
                overall_safety_properties: OverallSafetyPropertiesDescription {
                    num_successful_views: 50,
                    num_failed_views: 5,
                    ..Default::default()
                },
                ..TestDescription::default()
            },
            BuilderValidationConfig {
                expected_txn_num: num_cpus::get() * 50,
            },
            TransactionGenerationConfig {
                strategy: generation::GenerationStrategy::Flood {
                    min_tx_size: 16,
                    max_tx_size: 128,
                },
                endpoints: vec![],
            },
        )
        .await
    }

    cross_tests!(
        TestName: example_cross_test,
        Impls: [MemoryImpl],
        BuilderImpls: [MarketplaceBuilderImpl],
        Types: [TestTypes],
        Versions: [MarketplaceTestVersions],
        Ignore: true,
        Metadata: {
            TestDescription {
                validate_transactions : hotshot_testing::test_builder::nonempty_block_threshold((90,100)),
                txn_description : hotshot_testing::txn_task::TxnTaskDescription::RoundRobinTimeBased(Duration::from_millis(50)),
                completion_task_description : CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                            TimeBasedCompletionTaskDescription {
                                duration: Duration::from_secs(120),
                            },
                ),
                overall_safety_properties: OverallSafetyPropertiesDescription {
                    num_successful_views: 50,
                    num_failed_views: 5,
                    ..Default::default()
                },
                ..Default::default()
            }
        },
    );
}
