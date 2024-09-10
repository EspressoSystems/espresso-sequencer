use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use async_lock::RwLock;
use hotshot_example_types::{
    node_types::{Libp2pImpl, MarketplaceTestVersions, MemoryImpl, PushCdnImpl, TestVersions},
    state_types::TestTypes,
};
use hotshot_macros::cross_tests;
use hotshot_testing::{
    block_builder::SimpleBuilderImplementation,
    byzantine::byzantine_behaviour::{
        BadProposalViewDos, DishonestDa, DishonestLeader, DishonestVoter, DishonestVoting,
        DoubleProposeVote, ViewDelay,
    },
    completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
    test_builder::{Behaviour, TestDescription},
};
use hotshot_types::{
    data::ViewNumber,
    message::{GeneralConsensusMessage, MessageKind, SequencingMessage},
    traits::{
        election::Membership,
        network::TransmitType,
        node_implementation::{ConsensusTime, NodeType},
    },
    vote::HasViewNumber,
};
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
    TestName: dishonest_leader,
    Impls: [MemoryImpl],
    Types: [TestTypes],
    Versions: [TestVersions],
    Ignore: false,
    Metadata: {
        let behaviour = Rc::new(|node_id| {
                let dishonest_leader = DishonestLeader {
                    dishonest_at_proposal_numbers: HashSet::from([2, 3]),
                    validated_proposals: Vec::new(),
                    total_proposals_from_node: 0,
                    view_look_back: 1,
                    dishonest_proposal_view_numbers: Arc::new(RwLock::new(HashSet::new())),
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
    TestName: dishonest_da,
    Impls: [MemoryImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Versions: [MarketplaceTestVersions],
    Ignore: false,
    Metadata: {
        let behaviour = Rc::new(|node_id| {
                let dishonest_da = DishonestDa {
                    dishonest_at_da_cert_sent_numbers: HashSet::from([2]),
                    total_da_certs_sent_from_node: 0,
                    total_views_add_to_cert: 4
                };
                match node_id {
                    2 => Behaviour::Byzantine(Box::new(dishonest_da)),
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

        metadata.num_nodes_with_stake = 10;
        metadata
    },
);

cross_tests!(
    TestName: view_delay,
    Impls: [MemoryImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Versions: [MarketplaceTestVersions],
    Ignore: false,
    Metadata: {

        let behaviour = Rc::new(|node_id| {
                let view_delay = ViewDelay {
                    number_of_views_to_delay: node_id/3,
                    events_for_view: HashMap::new(),
                    stop_view_delay_at_view_number: 25,
                };
                match node_id {
                    6|10|14 => Behaviour::Byzantine(Box::new(view_delay)),
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

        let num_nodes_with_stake = 15;
        metadata.num_nodes_with_stake = num_nodes_with_stake;
        metadata.da_staked_committee_size = num_nodes_with_stake;
        metadata.overall_safety_properties.num_failed_views = 20;
        metadata.overall_safety_properties.num_successful_views = 20;
        metadata.overall_safety_properties.expected_views_to_fail = HashMap::from([
            (ViewNumber::new(6), false),
            (ViewNumber::new(10), false),
            (ViewNumber::new(14), false),
            (ViewNumber::new(21), false),
            (ViewNumber::new(25), false),
        ]);
        metadata
    },
);

cross_tests!(
    TestName: dishonest_voting,
    Impls: [MemoryImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Versions: [MarketplaceTestVersions],
    Ignore: false,
    Metadata: {
        let nodes_count: usize = 10;
        let behaviour = Rc::new(move |node_id| {
            let dishonest_voting = DishonestVoting {
                view_increment: nodes_count as u64,
                modifier: Arc::new(move |_pk, message_kind, transmit_type: &mut TransmitType<TestTypes>, membership: &<TestTypes as NodeType>::Membership| {
                    if let MessageKind::Consensus(SequencingMessage::General(GeneralConsensusMessage::Vote(vote))) = message_kind {
                        *transmit_type = TransmitType::Direct(membership.leader(vote.view_number() + 1 - nodes_count as u64));
                    } else {
                        {}
                    }
                })
            };
            match node_id {
                5 => Behaviour::Byzantine(Box::new(dishonest_voting)),
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

        metadata.num_nodes_with_stake = nodes_count;
        metadata
    },
);

cross_tests!(
    TestName: coordination_attack,
    Impls: [MemoryImpl],
    Types: [TestTypes],
    Versions: [MarketplaceTestVersions],
    Ignore: false,
    Metadata: {
        let dishonest_proposal_view_numbers = Arc::new(RwLock::new(HashSet::new()));
        let behaviour = Rc::new(move |node_id| {
            match node_id {
                4 => Behaviour::Byzantine(Box::new(DishonestLeader {
                    // On second proposal send a dishonest qc
                    dishonest_at_proposal_numbers: HashSet::from([2]),
                    validated_proposals: Vec::new(),
                    total_proposals_from_node: 0,
                    view_look_back: 1,
                    dishonest_proposal_view_numbers: Arc::clone(&dishonest_proposal_view_numbers),
                })),
                5 | 6 => Behaviour::Byzantine(Box::new(DishonestVoter {
                    votes_sent: Vec::new(),
                    dishonest_proposal_view_numbers: Arc::clone(&dishonest_proposal_view_numbers),
                })),
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

        metadata.overall_safety_properties.num_failed_views = 1;
        metadata.num_nodes_with_stake = 10;
        metadata.overall_safety_properties.expected_views_to_fail = HashMap::from([
            (ViewNumber::new(14), false),
        ]);
        metadata
    },
);
