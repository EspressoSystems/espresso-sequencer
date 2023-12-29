use hotshot_testing_macros::cross_tests;

cross_tests!(
     DemoType: [ (hotshot::demos::vdemo::VDemoState) ],
     SignatureKey: [ hotshot_types::traits::signature_key::bn254::BLSPubKey ],
     CommChannel: [ hotshot::traits::implementations::MemoryCommChannel ],
     Storage: [ hotshot::traits::implementations::MemoryStorage ],
     Time: [ hotshot_types::data::ViewNumber ],
     TestName: single_permanent_failure_fast,
     TestBuilder: hotshot_testing::test_builder::TestBuilder {
         metadata: hotshot_testing::test_builder::TestMetadata {
             total_nodes: 7,
             start_nodes: 7,
             num_succeeds: 10,
             timing_data: hotshot_testing::test_builder::TimingData {
                 next_view_timeout: 1000,
                 ..hotshot_testing::test_builder::TimingData::default()
             },
             failure_threshold: 20,
             ..hotshot_testing::test_builder::TestMetadata::default()
         },
         setup:
             Some(hotshot_testing::round_builder::RoundSetupBuilder {
                 scheduled_changes: vec![
                     hotshot_testing::round_builder::ChangeNode {
                         idx: 5,
                         view: 1,
                         updown: hotshot_testing::round_builder::UpDown::Down
                     },
                 ],
                 ..Default::default()
             }),
         check: None
     },
    Slow: false,
);

cross_tests!(
     DemoType: [ (hotshot::demos::vdemo::VDemoState) ],
     SignatureKey: [ hotshot_types::traits::signature_key::bn254::BLSPubKey ],
     CommChannel: [ hotshot::traits::implementations::MemoryCommChannel ],
     Storage: [ hotshot::traits::implementations::MemoryStorage ],
     Time: [ hotshot_types::data::ViewNumber ],
     TestName: double_permanent_failure_fast,
     TestBuilder: hotshot_testing::test_builder::TestBuilder {
         metadata: hotshot_testing::test_builder::TestMetadata {
             total_nodes: 7,
             start_nodes: 7,
             num_succeeds: 10,
             failure_threshold: 20,
             timing_data: hotshot_testing::test_builder::TimingData {
                 next_view_timeout: 1000,
                 ..hotshot_testing::test_builder::TimingData::default()
             },
             ..hotshot_testing::test_builder::TestMetadata::default()
         },
         setup:
             Some(hotshot_testing::round_builder::RoundSetupBuilder {
                 scheduled_changes: vec![
                     hotshot_testing::round_builder::ChangeNode {
                         idx: 5,
                         view: 1,
                         updown: hotshot_testing::round_builder::UpDown::Down
                     },
                     hotshot_testing::round_builder::ChangeNode {
                         idx: 6,
                         view: 1,
                         updown: hotshot_testing::round_builder::UpDown::Down },
                 ],
                 ..Default::default()
             }),
         check: None
     },
    Slow: false,
);
