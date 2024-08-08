// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

// TODO: Remove this after integration
#![allow(unused_imports)]
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use hotshot_example_types::{
    node_types::{Libp2pImpl, MemoryImpl, PushCdnImpl, TestConsecutiveLeaderTypes},
    state_types::TestTypes,
};
use hotshot_macros::cross_tests;
use hotshot_testing::{
    block_builder::SimpleBuilderImplementation,
    completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
    spinning_task::{ChangeNode, SpinningTaskDescription, UpDown},
    test_builder::TestDescription,
    view_sync_task::ViewSyncTaskDescription,
};
use hotshot_types::{data::ViewNumber, traits::node_implementation::ConsensusTime};
#[cfg(async_executor_impl = "async-std")]
use {hotshot::tasks::DishonestLeader, hotshot_testing::test_builder::Behaviour, std::rc::Rc};
// Test that a good leader can succeed in the view directly after view sync
cross_tests!(
    TestName: test_with_failures_2,
    Impls: [MemoryImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Ignore: false,
    Metadata: {
        let mut metadata = TestDescription::default_more_nodes();
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
        metadata.overall_safety_properties.num_successful_views = 13;

        metadata
    }
);

#[cfg(async_executor_impl = "async-std")]
cross_tests!(
    TestName: dishonest_leader,
    Impls: [MemoryImpl],
    Types: [TestTypes],
    Ignore: false,
    Metadata: {
        let behaviour = Rc::new(|node_id| {
                let dishonest_leader = DishonestLeader::<TestTypes, MemoryImpl> {
                    dishonest_at_proposal_numbers: HashSet::from([2, 3]),
                    validated_proposals: Vec::new(),
                    total_proposals_from_node: 0,
                    view_look_back: 1,
                    _phantom: std::marker::PhantomData
                };
                match node_id {
                    2 => Behaviour::Byzantine(Box::new(dishonest_leader)),
                    _ => Behaviour::Standard,
                }
            });

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

        metadata.overall_safety_properties.num_failed_views = 2;
        metadata.num_nodes_with_stake = 5;
        metadata.overall_safety_properties.expected_views_to_fail = HashMap::from([
            (ViewNumber::new(7), false),
            (ViewNumber::new(12), false)
        ]);
        metadata
    },
);

cross_tests!(
    TestName: test_with_double_leader_failures,
    Impls: [MemoryImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestConsecutiveLeaderTypes],
    Ignore: false,
    Metadata: {
        let mut metadata = TestDescription::default_more_nodes();
        metadata.num_bootstrap_nodes = 10;
        metadata.num_nodes_with_stake = 12;
        metadata.da_staked_committee_size = 12;
        metadata.start_nodes = 12;
        let dead_nodes = vec![
            ChangeNode {
                idx: 3,
                updown: UpDown::Down,
            },
        ];

        // shutdown while node 3 is leader
        // we want to trigger `ViewSyncTrigger`
        // then ensure we do not fail again as next leader will be leader 2 views also
        let view_spin_node_down = 5;
        metadata.spinning_properties = SpinningTaskDescription {
            node_changes: vec![(view_spin_node_down, dead_nodes)]
        };

        // node 3 is leader twice when we shut down
        metadata.overall_safety_properties.num_failed_views = 2;
        metadata.overall_safety_properties.expected_views_to_fail = HashMap::from([
            // next views after turning node off
            (ViewNumber::new(view_spin_node_down + 1), false),
            (ViewNumber::new(view_spin_node_down + 2), false)
        ]);
        // Make sure we keep committing rounds after the bad leaders, but not the full 50 because of the numerous timeouts
        metadata.overall_safety_properties.num_successful_views = 13;

        // only turning off 1 node, so expected should be num_nodes_with_stake - 1
        let expected_nodes_in_view_sync = metadata.num_nodes_with_stake - 1;
        metadata.view_sync_properties = ViewSyncTaskDescription::Threshold(expected_nodes_in_view_sync, expected_nodes_in_view_sync);

        metadata
    }
);
