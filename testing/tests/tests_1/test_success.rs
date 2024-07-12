use std::time::Duration;

use hotshot_example_types::{
    node_types::{Libp2pImpl, MemoryImpl, PushCdnImpl},
    state_types::TestTypes,
};
use hotshot_macros::cross_tests;
use hotshot_testing::{
    block_builder::SimpleBuilderImplementation,
    completion_task::{CompletionTaskDescription, TimeBasedCompletionTaskDescription},
    test_builder::TestDescription,
};
#[cfg(async_executor_impl = "async-std")]
use {hotshot::tasks::DoubleProposeVote, hotshot_testing::test_builder::Behaviour, std::rc::Rc};

cross_tests!(
    TestName: test_success,
    Impls: [MemoryImpl, Libp2pImpl, PushCdnImpl],
    Types: [TestTypes],
    Ignore: false,
    Metadata: {
        TestDescription {
            // allow more time to pass in CI
            completion_task_description: CompletionTaskDescription::TimeBasedCompletionTaskBuilder(
                                             TimeBasedCompletionTaskDescription {
                                                 duration: Duration::from_secs(60),
                                             },
                                         ),
            ..TestDescription::default()
        }
    },
);

#[cfg(async_executor_impl = "async-std")]
cross_tests!(
    TestName: twins_test_success,
    Impls: [MemoryImpl],
    Types: [TestTypes],
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
