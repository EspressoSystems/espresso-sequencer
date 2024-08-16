// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{rc::Rc, time::Duration};

use hotshot::tasks::{BadProposalViewDos, DoubleProposeVote};
use hotshot_example_types::{
    node_types::{
        Libp2pImpl, MarketplaceUpgradeTestVersions, MemoryImpl, PushCdnImpl,
        TestConsecutiveLeaderTypes, TestVersions,
    },
    state_types::TestTypes,
    testable_delay::{DelayConfig, DelayOptions, DelaySettings, SupportedTraitTypesForAsyncDelay},
};
use hotshot_macros::cross_tests;
use hotshot_testing::{
    block_builder::SimpleBuilderImplementation,
    completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
    test_builder::{Behaviour, TestDescription},
    view_sync_task::ViewSyncTaskDescription,
};

cross_tests!(
    TestName: test_success,
    Impls: [MemoryImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Versions: [TestVersions],
    Ignore: false,
    Metadata: {
        TestDescription {
            // allow more time to pass in CI
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                                             TimeBasedCompletionTaskDescription {
                                                 duration: Duration::from_secs(60),
                                             },
                                         ),
            ..TestDescription::default()
        }
    },
);

cross_tests!(
    TestName: test_success_marketplace,
    Impls: [MemoryImpl],
    Types: [TestTypes],
    Versions: [MarketplaceUpgradeTestVersions],
    Ignore: false,
    Metadata: {
        TestDescription {
            // allow more time to pass in CI
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                                             TimeBasedCompletionTaskDescription {
                                                 duration: Duration::from_secs(60),
                                             },
                                         ),
            upgrade_view: Some(5),
            ..TestDescription::default()
        }
    },
);

cross_tests!(
    TestName: test_success_with_async_delay,
    Impls: [MemoryImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Versions: [TestVersions],
    Ignore: false,
    Metadata: {
        let mut metadata = TestDescription {
            // allow more time to pass in CI
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                                             TimeBasedCompletionTaskDescription {
                                                 duration: Duration::from_secs(60),
                                             },
                                         ),
            ..TestDescription::default()
        };

        metadata.overall_safety_properties.num_failed_views = 0;
        metadata.overall_safety_properties.num_successful_views = 0;
        let mut config = DelayConfig::default();
        let delay_settings = DelaySettings {
            delay_option: DelayOptions::Random,
            min_time_in_milliseconds: 10,
            max_time_in_milliseconds: 100,
            fixed_time_in_milliseconds: 0,
        };
        config.add_settings_for_all_types(delay_settings);
        metadata.async_delay_config = config;
        metadata
    },
);

cross_tests!(
    TestName: test_success_with_async_delay_2,
    Impls: [MemoryImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Versions: [TestVersions],
    Ignore: false,
    Metadata: {
        let mut metadata = TestDescription {
            // allow more time to pass in CI
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                                             TimeBasedCompletionTaskDescription {
                                                 duration: Duration::from_secs(60),
                                             },
                                         ),
            ..TestDescription::default()
        };

        metadata.overall_safety_properties.num_failed_views = 0;
        metadata.overall_safety_properties.num_successful_views = 10;
        let mut config = DelayConfig::default();
        let mut delay_settings = DelaySettings {
            delay_option: DelayOptions::Random,
            min_time_in_milliseconds: 10,
            max_time_in_milliseconds: 100,
            fixed_time_in_milliseconds: 15,
        };
        config.add_setting(SupportedTraitTypesForAsyncDelay::Storage, &delay_settings);

        delay_settings.delay_option = DelayOptions::Fixed;
        config.add_setting(SupportedTraitTypesForAsyncDelay::BlockHeader, &delay_settings);

        delay_settings.delay_option = DelayOptions::Random;
        delay_settings.min_time_in_milliseconds = 5;
        delay_settings.max_time_in_milliseconds = 20;
        config.add_setting(SupportedTraitTypesForAsyncDelay::ValidatedState, &delay_settings);
        metadata.async_delay_config = config;
        metadata
    },
);

cross_tests!(
    TestName: double_propose_vote,
    Impls: [MemoryImpl],
    Types: [TestTypes],
    Versions: [TestVersions],
    Ignore: false,
    Metadata: {
        let behaviour = Rc::new(|node_id| { match node_id {
          1 => Behaviour::Byzantine(Box::new(DoubleProposeVote)),
          _ => Behaviour::Standard,
          } });

        TestDescription {
            // allow more time to pass in CI
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                                             TimeBasedCompletionTaskDescription {
                                                 duration: Duration::from_secs(60),
                                             },
                                         ),
            behaviour,
            ..TestDescription::default()
        }
    },
);

// Test where node 4 sends out the correct quorum proposal and additionally spams the network with an extra 99 malformed proposals
cross_tests!(
    TestName: multiple_bad_proposals,
    Impls: [MemoryImpl],
    Types: [TestTypes],
    Versions: [TestVersions],
    Ignore: false,
    Metadata: {
        let behaviour = Rc::new(|node_id| { match node_id {
          4 => Behaviour::Byzantine(Box::new(BadProposalViewDos { multiplier: 100, increment: 1 })),
          _ => Behaviour::Standard,
          } });

        let mut metadata = TestDescription {
            // allow more time to pass in CI
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                                             TimeBasedCompletionTaskDescription {
                                                 duration: Duration::from_secs(60),
                                             },
                                         ),
            behaviour,
            ..TestDescription::default()
        };

        metadata.overall_safety_properties.num_failed_views = 0;

        metadata
    },
);

cross_tests!(
    TestName: test_with_double_leader_no_failures,
    Impls: [MemoryImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestConsecutiveLeaderTypes],
    Versions: [TestVersions],
    Ignore: false,
    Metadata: {
        let mut metadata = TestDescription::default_more_nodes();
        metadata.num_bootstrap_nodes = 10;
        metadata.num_nodes_with_stake = 12;
        metadata.da_staked_committee_size = 12;
        metadata.start_nodes = 12;

        metadata.overall_safety_properties.num_failed_views = 0;

        metadata.view_sync_properties = ViewSyncTaskDescription::Threshold(0, 0);

        metadata
    }
);
