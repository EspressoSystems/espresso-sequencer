use hotshot_example_types::node_types::{Libp2pImpl, MemoryImpl, PushCdnImpl, WebImpl};
use hotshot_example_types::state_types::TestTypes;
use hotshot_testing::completion_task::{
    CompletionTaskDescription, TimeBasedCompletionTaskDescription,
};
use hotshot_testing::spinning_task::ChangeNode;
use hotshot_testing::spinning_task::SpinningTaskDescription;
use hotshot_testing::spinning_task::UpDown;
use hotshot_testing::test_builder::TestMetadata;
use hotshot_testing_macros::cross_tests;
use std::time::Duration;

cross_tests!(
    TestName: test_success,
    Impls: [MemoryImpl, WebImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Ignore: false,
    Metadata: {
        TestMetadata {
            // allow more time to pass in CI
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                                             TimeBasedCompletionTaskDescription {
                                                 duration: Duration::from_secs(60),
                                             },
                                         ),
            ..TestMetadata::default()
        }
    },
);

// Test one node leaving the network.
cross_tests!(
    TestName: test_with_failures_one,
    Impls: [MemoryImpl, WebImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Ignore: false,
    Metadata: {
        let mut metadata = TestMetadata::default_more_nodes();
        metadata.num_bootstrap_nodes = 19;
        // The first 14 (i.e., 20 - f) nodes are in the DA committee and we may shutdown the
        // remaining 6 (i.e., f) nodes. We could remove this restriction after fixing the
        // following issue.
        // TODO: Update message broadcasting to avoid hanging
        // <https://github.com/EspressoSystems/HotShot/issues/1567>
        let dead_nodes = vec![ChangeNode {
            idx: 19,
            updown: UpDown::Down,
        }];

        metadata.spinning_properties = SpinningTaskDescription {
            node_changes: vec![(5, dead_nodes)]
        };
        metadata.overall_safety_properties.num_failed_views = 3;
        metadata.overall_safety_properties.num_successful_views = 25;
        metadata
    }
);

// Test f/2 nodes leaving the network.
cross_tests!(
    TestName: test_with_failures_half_f,
    Impls: [MemoryImpl, WebImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Ignore: false,
    Metadata: {
        let mut metadata = TestMetadata::default_more_nodes();
        metadata.num_bootstrap_nodes = 17;
        // The first 14 (i.e., 20 - f) nodes are in the DA committee and we may shutdown the
        // remaining 6 (i.e., f) nodes. We could remove this restriction after fixing the
        // following issue.
        // TODO: Update message broadcasting to avoid hanging
        // <https://github.com/EspressoSystems/HotShot/issues/1567>
        let dead_nodes = vec![
            ChangeNode {
                idx: 17,
                updown: UpDown::Down,
            },
            ChangeNode {
                idx: 18,
                updown: UpDown::Down,
            },
            ChangeNode {
                idx: 19,
                updown: UpDown::Down,
            },
        ];

        metadata.spinning_properties = SpinningTaskDescription {
            node_changes: vec![(5, dead_nodes)]
        };

        metadata.overall_safety_properties.num_failed_views = 3;
        // Make sure we keep committing rounds after the bad leaders, but not the full 50 because of the numerous timeouts
        metadata.overall_safety_properties.num_successful_views = 22;
        metadata
    }
);

// Test f nodes leaving the network.
cross_tests!(
    TestName: test_with_failures_f,
    Impls: [MemoryImpl, WebImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Ignore: false,
    Metadata: {

        let mut metadata = TestMetadata::default_more_nodes();
        metadata.overall_safety_properties.num_failed_views = 6;
        // Make sure we keep committing rounds after the bad leaders, but not the full 50 because of the numerous timeouts
        metadata.overall_safety_properties.num_successful_views = 22;
        metadata.num_bootstrap_nodes = 14;
        // The first 14 (i.e., 20 - f) nodes are in the DA committee and we may shutdown the
        // remaining 6 (i.e., f) nodes. We could remove this restriction after fixing the
        // following issue.
        // TODO: Update message broadcasting to avoid hanging
        // <https://github.com/EspressoSystems/HotShot/issues/1567>
        let dead_nodes = vec![
            ChangeNode {
                idx: 14,
                updown: UpDown::Down,
            },
            ChangeNode {
                idx: 15,
                updown: UpDown::Down,
            },
            ChangeNode {
                idx: 16,
                updown: UpDown::Down,
            },
            ChangeNode {
                idx: 17,
                updown: UpDown::Down,
            },
            ChangeNode {
                idx: 18,
                updown: UpDown::Down,
            },
            ChangeNode {
                idx: 19,
                updown: UpDown::Down,
            },
            ];

        metadata.spinning_properties = SpinningTaskDescription {
            node_changes: vec![(5, dead_nodes)]
        };

        metadata
    }
);

// Test that a good leader can succeed in the view directly after view sync
cross_tests!(
    TestName: test_with_failures_2,
    Impls: [MemoryImpl, WebImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Ignore: false,
    Metadata: {
        let mut metadata = TestMetadata::default_more_nodes();
        metadata.num_bootstrap_nodes = 10;
        metadata.num_nodes_with_stake = 12;
        metadata.da_staked_committee_size = 12;
        metadata.start_nodes = 12;
        // The first 14 (i.e., 20 - f) nodes are in the DA committee and we may shutdown the
        // remaining 6 (i.e., f) nodes. We could remove this restriction after fixing the
        // following issue.
        // TODO: Update message broadcasting to avoid hanging
        // <https://github.com/EspressoSystems/HotShot/issues/1567>
        let dead_nodes = vec![
            ChangeNode {
                idx: 10,
                updown: UpDown::Down,
            },
            ChangeNode {
                idx: 11,
                updown: UpDown::Down,
            },
        ];

        metadata.spinning_properties = SpinningTaskDescription {
            node_changes: vec![(5, dead_nodes)]
        };

        // 2 nodes fail triggering view sync, expect no other timeouts
        metadata.overall_safety_properties.num_failed_views = 2;
        // Make sure we keep committing rounds after the bad leaders, but not the full 50 because of the numerous timeouts
        metadata.overall_safety_properties.num_successful_views = 15;

        metadata
    }
);
