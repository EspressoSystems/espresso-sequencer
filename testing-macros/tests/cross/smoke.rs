use hotshot_testing_macros::cross_all_types;

cross_all_types!(
    TestName: ten_tx_five_nodes_slow,
    TestBuilder: hotshot_testing::test_builder::TestBuilder {
        metadata: hotshot_testing::test_builder::TestMetadata {
            total_nodes: 5,
            start_nodes: 5,
            num_succeeds: 10,
            ..Default::default()
        },
        ..Default::default()
    },
    Slow: true
);

cross_all_types!(
    TestName: ten_tx_seven_nodes_slow,
    TestBuilder: hotshot_testing::test_builder::TestBuilder {
        metadata: hotshot_testing::test_builder::TestMetadata {
            total_nodes: 7,
            start_nodes: 7,
            num_succeeds: 10,
            ..Default::default()
        },
        ..Default::default()
    },
    Slow: true
);
