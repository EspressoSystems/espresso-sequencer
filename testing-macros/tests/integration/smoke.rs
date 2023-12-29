use hotshot_testing_macros::cross_tests;

cross_tests!(
    DemoType: [(hotshot::demos::vdemo::VDemoState)],
    SignatureKey: [ hotshot_types::traits::signature_key::bn254::BLSPubKey ],
    CommChannel: [ hotshot::traits::implementations::MemoryCommChannel ],
    Storage: [ hotshot::traits::implementations::MemoryStorage ],
    Time: [ hotshot_types::data::ViewNumber ],
    TestName: ten_tx_five_nodes_fast,
    TestBuilder: hotshot_testing::test_builder::TestBuilder {
        metadata: hotshot_testing::test_builder::TestMetadata {
            total_nodes: 5,
            start_nodes: 5,
            num_succeeds: 10,
            ..Default::default()
        },
        ..Default::default()
    },
    Slow: false,
);

cross_tests!(
    DemoType: [(hotshot::demos::vdemo::VDemoState) ],
    SignatureKey: [ hotshot_types::traits::signature_key::bn254::BLSPubKey ],
    CommChannel: [ hotshot::traits::implementations::MemoryCommChannel ],
    Storage: [ hotshot::traits::implementations::MemoryStorage ],
    Time: [ hotshot_types::data::ViewNumber ],
    TestName: ten_tx_seven_nodes_fast,
    TestBuilder: hotshot_testing::test_builder::TestBuilder {
        metadata: hotshot_testing::test_builder::TestMetadata {
            total_nodes: 7,
            start_nodes: 7,
            num_succeeds: 10,
            ..Default::default()
        },
        ..Default::default()
    },
    Slow: false,
);
