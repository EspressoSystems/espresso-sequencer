use hotshot_example_types::node_types::{Libp2pImpl, MemoryImpl, PushCdnImpl, WebImpl};
use hotshot_example_types::state_types::TestTypes;
use hotshot_macros::cross_tests;
use hotshot_testing::spinning_task::ChangeNode;
use hotshot_testing::spinning_task::SpinningTaskDescription;
use hotshot_testing::spinning_task::UpDown;
use hotshot_testing::test_builder::TestMetadata;

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
