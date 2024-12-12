// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{sync::Arc, time::Duration};

use hotshot_example_types::{
    node_types::{
        EpochsTestVersions, Libp2pImpl, MemoryImpl, PushCdnImpl, TestConsecutiveLeaderTypes,
        TestTwoStakeTablesTypes, TestTypes, TestTypesRandomizedLeader, TestVersions,
    },
    testable_delay::{DelayConfig, DelayOptions, DelaySettings, SupportedTraitTypesForAsyncDelay},
};
use hotshot_macros::cross_tests;
use hotshot_testing::{
    block_builder::SimpleBuilderImplementation,
    completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
    overall_safety_task::OverallSafetyPropertiesDescription,
    spinning_task::{ChangeNode, NodeAction, SpinningTaskDescription},
    test_builder::TestDescription,
    view_sync_task::ViewSyncTaskDescription,
};

cross_tests!(
    TestName: test_success,
    Impls: [MemoryImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes, TestTypesRandomizedLeader],
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

// cross_tests!(
//     TestName: test_epoch_success,
//     Impls: [MemoryImpl, Libp2pImpl, PushCdnImpl],
//     Types: [TestTypes, TestTypesRandomizedLeader, TestTypesRandomizedCommitteeMembers<StableQuorumFilterConfig<123, 2>>, TestTypesRandomizedCommitteeMembers<RandomOverlapQuorumFilterConfig<123, 4, 5, 0, 2>>],
//     Versions: [EpochsTestVersions],
//     Ignore: false,
//     Metadata: {
//         TestDescription {
//             // allow more time to pass in CI
//             completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
//                                              TimeBasedCompletionTaskDescription {
//                                                  duration: Duration::from_secs(60),
//                                              },
//                                          ),
//             ..TestDescription::default()
//         }
//     },
// );

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

cross_tests!(
    TestName: test_epoch_end,
    Impls: [PushCdnImpl],
    Types: [TestTwoStakeTablesTypes],
    Versions: [EpochsTestVersions],
    Ignore: false,
    Metadata: {
        TestDescription {
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                TimeBasedCompletionTaskDescription {
                    duration: Duration::from_millis(100000),
                },
            ),
            epoch_height: 10,
            num_nodes_with_stake: 10,
            start_nodes: 10,
            num_bootstrap_nodes: 10,
            da_staked_committee_size: 10,
            overall_safety_properties: OverallSafetyPropertiesDescription {
                // Explicitly show that we use normal threshold, i.e. 2 nodes_len / 3 + 1
                // but we divide by two because only half of the nodes are active in each epoch
                threshold_calculator: Arc::new(|_, nodes_len| 2 * nodes_len / 2 / 3 + 1),
                ..OverallSafetyPropertiesDescription::default()
            },

            ..TestDescription::default()
        }
    },
);

// Test to make sure we can decide in just 3 views
// This test fails with the old decide rule
cross_tests!(
    TestName: test_shorter_decide,
    Impls: [MemoryImpl],
    Types: [TestTypes],
    Versions: [EpochsTestVersions],
    Ignore: false,
    Metadata: {
        let mut metadata = TestDescription {
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                TimeBasedCompletionTaskDescription {
                    duration: Duration::from_millis(100000),
                },
            ),
            ..TestDescription::default()
        };
        // after the first 3 leaders the next leader is down. It's a hack to make sure we decide in
        // 3 views or else we get a timeout
        let dead_nodes = vec![
            ChangeNode {
                idx: 4,
                updown: NodeAction::Down,
            },

        ];
        metadata.spinning_properties = SpinningTaskDescription {
            node_changes: vec![(1, dead_nodes)]
        };
        metadata.overall_safety_properties.num_successful_views = 1;
        metadata.overall_safety_properties.num_failed_views = 0;
        metadata
    },
);
