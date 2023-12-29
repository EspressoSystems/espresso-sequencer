// #[cfg(feature = "slow-tests")]
// use either::Either::Right;
// #[cfg(feature = "slow-tests")]
// use hotshot_testing::test_builder::{get_tolerance, TestMetadata};
// use hotshot_testing_macros::cross_all_types;
// #[cfg(feature = "slow-tests")]
// use std::{collections::HashSet, iter::FromIterator};

// TODO these need to be fixed. But the slow runs fail anyway. We should re-enable when we decide
// to debug and fix them.

// cross_all_types!(
//     TestName: test_fail_first_node_regression_small,
//     TestDescription: GeneralTestDescriptionBuilder {
//         total_nodes: 10,
//         start_nodes: 10,
//         num_succeeds: 40,
//         txn_ids: Right(30),
//         ids_to_shut_down: vec![vec![0].into_iter().collect::<HashSet<_>>()],
//         failure_threshold: 5,
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     Slow: true
// );
//
// cross_all_types!(
//     TestName: test_fifty_nodes_regression,
//     TestDescription: GeneralTestDescriptionBuilder {
//         total_nodes: 50,
//         start_nodes: 50,
//         num_succeeds: 40,
//         txn_ids: Right(30),
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     Slow: true
// );
//
// cross_all_types!(
//     TestName: test_ninety_nodes_regression,
//     TestDescription: GeneralTestDescriptionBuilder {
//         total_nodes: 90,
//         start_nodes: 90,
//         num_succeeds: 40,
//         txn_ids: Right(30),
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     Slow: true
// );
//
// cross_all_types!(
//     TestName: test_large_num_txns_regression,
//     TestDescription: GeneralTestDescriptionBuilder {
//         total_nodes: 10,
//         start_nodes: 10,
//         num_succeeds: 40,
//         txn_ids: Right(500),
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     Slow: true
// );
//
// cross_all_types!(
//     TestName: test_fail_last_node_regression,
//     TestDescription: GeneralTestDescriptionBuilder {
//         total_nodes: 53,
//         start_nodes: 53,
//         num_succeeds: 40,
//         txn_ids: Right(30),
//         ids_to_shut_down: vec![vec![52].into_iter().collect::<HashSet<_>>()],
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     Slow: true
// );
//
// cross_all_types!(
//     TestName: test_fail_first_node_regression,
//     TestDescription: GeneralTestDescriptionBuilder {
//         total_nodes: 76,
//         start_nodes: 76,
//         num_succeeds: 40,
//         txn_ids: Right(30),
//         ids_to_shut_down: vec![vec![0].into_iter().collect::<HashSet<_>>()],
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     Slow: true
// );
//
// cross_all_types!(
//     TestName: test_fail_last_f_nodes_regression,
//     TestDescription: GeneralTestDescriptionBuilder {
//         total_nodes: 75,
//         start_nodes: 75,
//         num_succeeds: 40,
//         txn_ids: Right(30),
//         ids_to_shut_down: vec![HashSet::<u64>::from_iter(
//             (0..get_tolerance(75)).map(|x| 74 - x),
//         )],
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     Slow: true
// );
//
// cross_all_types!(
//     TestName: test_fail_last_f_plus_one_nodes_regression,
//     TestDescription: GeneralTestDescriptionBuilder {
//         total_nodes: 15,
//         start_nodes: 15,
//         txn_ids: Right(30),
//         ids_to_shut_down: vec![HashSet::<u64>::from_iter(
//             (0..get_tolerance(15) + 1).map(|x| 14 - x),
//         )],
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     Slow: true
// );
//
// cross_all_types!(
//     TestName: test_mul_txns_regression,
//     TestDescription: GeneralTestDescriptionBuilder {
//         total_nodes: 30,
//         start_nodes: 30,
//         txn_ids: Right(30),
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     Slow: true
// );

// TODO re-enable these tests if we decide to use proptest
//
// cross_all_types_proptes!(
//     test_large_num_nodes_random,
//     GeneralTestDescriptionBuilder {
//         total_nodes: num_nodes,
//         start_nodes: num_nodes,
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     keep: true,
//     slow: true,
//     args: num_nodes in 50..100usize
// );
//
// cross_all_types_proptest!(
//     test_fail_last_node_random,
//     GeneralTestDescriptionBuilder {
//         total_nodes: num_nodes,
//         start_nodes: num_nodes,
//         txn_ids: Right(30),
//         ids_to_shut_down: vec![vec![(num_nodes - 1) as u64].into_iter().collect()],
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     keep: true,
//     slow: true,
//     args: num_nodes in 30..100usize
// );
//
// cross_all_types_proptest!(
//     test_fail_first_node_random,
//     GeneralTestDescriptionBuilder {
//         total_nodes: num_nodes,
//         start_nodes: num_nodes,
//         txn_ids: Right(30),
//         ids_to_shut_down: vec![vec![0].into_iter().collect()],
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     keep: true,
//     slow: true,
//     args: num_nodes in 30..100usize
// );
//
// cross_all_types_proptest!(
//     test_fail_last_f_nodes_random,
//     GeneralTestDescriptionBuilder {
//         total_nodes: num_nodes,
//         start_nodes: num_nodes,
//         num_succeeds: 40,
//         txn_ids: Right(30),
//         ids_to_shut_down: vec![HashSet::<u64>::from_iter((0..get_tolerance(num_nodes as u64)).map(|x| (num_nodes as u64) - x - 1))],
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     keep: true,
//     slow: true,
//     args: num_nodes in 30..100usize
// );
//
// cross_all_types_proptest!(
//     test_fail_first_f_nodes_random,
//     GeneralTestDescriptionBuilder {
//         total_nodes: num_nodes,
//         start_nodes: num_nodes,
//         num_succeeds: 40,
//         txn_ids: Right(30),
//         ids_to_shut_down: vec![HashSet::<u64>::from_iter(0..get_tolerance(num_nodes as u64))],
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     keep: true,
//     slow: true,
//     args: num_nodes in 30..100usize
// );
//
// cross_all_types_proptest!(
//     test_mul_txns_random,
//     GeneralTestDescriptionBuilder {
//         total_nodes: 30,
//         start_nodes: 30,
//         txn_ids: Left(vec![vec![txn_proposer_1, txn_proposer_2]]),
//         ..GeneralTestDescriptionBuilder::default()
//     },
//     keep: true,
//     slow: true,
//     args: txn_proposer_1 in 0..15u64, txn_proposer_2 in 15..30u64
// );
