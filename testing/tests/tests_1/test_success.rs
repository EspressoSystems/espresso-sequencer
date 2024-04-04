use hotshot_example_types::node_types::{Libp2pImpl, MemoryImpl, PushCdnImpl, WebImpl};
use hotshot_example_types::state_types::TestTypes;
use hotshot_macros::cross_tests;
use hotshot_testing::completion_task::{
    CompletionTaskDescription, TimeBasedCompletionTaskDescription,
};
use hotshot_testing::test_builder::TestMetadata;
use std::time::Duration;
use hotshot_testing::block_builder::SimpleBuilderImplementation;
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

