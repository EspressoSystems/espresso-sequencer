use async_compatibility_layer::art::async_sleep;
use hotshot_example_types::{
    block_types::{TestBlockPayload, TestTransaction},
    node_types::TestTypes,
};
use hotshot_task_impls::{
    builder::{BuilderClient, BuilderClientError},
    events::HotShotEvent,
};
use hotshot_testing::{
    block_builder::{make_simple_builder, run_random_builder},
    script::{run_test_script, TestScriptStage},
    task_helpers::build_system_handle,
};
use hotshot_types::{
    constants::Version01,
    traits::{
        block_contents::vid_commitment, node_implementation::NodeType, signature_key::SignatureKey,
        BlockPayload,
    },
};
use std::time::Duration;
use tide_disco::Url;

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_random_block_builder() {
    let port = portpicker::pick_unused_port().expect("Could not find an open port");
    let api_url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

    run_random_builder(api_url.clone());

    let client: BuilderClient<TestTypes, Version01> = BuilderClient::new(api_url);
    assert!(client.connect(Duration::from_millis(100)).await);

    // Wait for at least one block to be built
    async_sleep(Duration::from_millis(30)).await;

    // Test getting blocks
    let mut blocks = client
        .get_available_blocks(vid_commitment(&vec![], 1))
        .await
        .expect("Failed to get available blocks");

    {
        let mut attempt = 0;

        while blocks.is_empty() && attempt < 50 {
            blocks = client
                .get_available_blocks(vid_commitment(&vec![], 1))
                .await
                .expect("Failed to get available blocks");
            attempt += 1;
            async_sleep(Duration::from_millis(100)).await;
        }

        assert!(!blocks.is_empty());
    }

    // Test claiming available block
    let signature = {
        let (_key, private_key) =
            <TestTypes as NodeType>::SignatureKey::generated_from_seed_indexed([0_u8; 32], 0);
        <TestTypes as NodeType>::SignatureKey::sign(&private_key, &[0_u8; 32])
            .expect("Failed to create dummy signature")
    };

    let _: TestBlockPayload = client
        .claim_block(blocks.pop().unwrap(), &signature)
        .await
        .expect("Failed to claim block");

    // Test claiming non-existent block
    let commitment_for_non_existent_block = TestBlockPayload {
        transactions: vec![TestTransaction(vec![0; 1])],
    }
    .builder_commitment(&());
    let result = client
        .claim_block(commitment_for_non_existent_block, &signature)
        .await;
    assert!(matches!(result, Err(BuilderClientError::NotFound)));
}

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_simple_block_builder() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let (source, task) = make_simple_builder(quorum_membership.into()).await;

    let port = portpicker::pick_unused_port().expect("Could not find an open port");
    let api_url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

    source.run(api_url.clone()).await;

    let client: BuilderClient<TestTypes, Version01> = BuilderClient::new(api_url);
    assert!(client.connect(Duration::from_millis(100)).await);

    // Before block-building task is spun up, should only have an empty block
    {
        let mut blocks = client
            .get_available_blocks(vid_commitment(&vec![], 1))
            .await
            .expect("Failed to get available blocks");

        assert_eq!(blocks.len(), 1);

        let signature = {
            let (_key, private_key) =
                <TestTypes as NodeType>::SignatureKey::generated_from_seed_indexed([0_u8; 32], 0);
            <TestTypes as NodeType>::SignatureKey::sign(&private_key, &[0_u8; 32])
                .expect("Failed to create dummy signature")
        };

        let payload: TestBlockPayload = client
            .claim_block(blocks.pop().unwrap(), &signature)
            .await
            .expect("Failed to claim block");

        assert_eq!(payload.transactions.len(), 0);
    }

    {
        let stage_1 = TestScriptStage {
            inputs: vec![
                HotShotEvent::TransactionsRecv(vec![
                    TestTransaction(vec![1]),
                    TestTransaction(vec![2]),
                ]),
                HotShotEvent::TransactionsRecv(vec![TestTransaction(vec![3])]),
            ],
            outputs: vec![],
            asserts: vec![],
        };

        let stages = vec![stage_1];

        run_test_script(stages, task).await;

        // Test getting blocks
        let mut blocks = client
            .get_available_blocks(vid_commitment(&vec![], 1))
            .await
            .expect("Failed to get available blocks");

        assert!(!blocks.is_empty());

        let signature = {
            let (_key, private_key) =
                <TestTypes as NodeType>::SignatureKey::generated_from_seed_indexed([0_u8; 32], 0);
            <TestTypes as NodeType>::SignatureKey::sign(&private_key, &[0_u8; 32])
                .expect("Failed to create dummy signature")
        };

        let payload: TestBlockPayload = client
            .claim_block(blocks.pop().unwrap(), &signature)
            .await
            .expect("Failed to claim block");

        assert_eq!(payload.transactions.len(), 3);
    }
}
