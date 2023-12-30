use hotshot_testing::completion_task::CompletionTaskDescription;
use hotshot_testing::completion_task::TimeBasedCompletionTaskDescription;
use hotshot_testing::node_types::TestTypes;
use hotshot_testing::node_types::{Libp2pImpl, MemoryImpl};
use hotshot_testing::test_builder::TestMetadata;
use hotshot_testing_macros::cross_tests;

cross_tests!(
     Metadata:
         TestMetadata {
            // allow more time to pass in CI
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                TimeBasedCompletionTaskDescription {
                    duration: std::time::Duration::from_secs(60),
                },
            ),
            ..TestMetadata::default()
        },
    Ignore: false,
    TestName: single_permanent_failure_fast,
    // types that implement nodetype
    Types: [TestTypes],
    // forall impl in Impls, forall type in Types, impl : NodeImplementation<type>
    Impls: [MemoryImpl, Libp2pImpl],
);
