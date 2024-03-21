use hotshot_example_types::{
    block_types::{TestBlockPayload, TestTransaction},
    node_types::TestTypes,
};
use hotshot_task_impls::builder::{BuilderClient, BuilderClientError};
use hotshot_testing::block_builder::run_random_builder;
use hotshot_types::traits::BlockPayload;
use hotshot_types::traits::{
    block_contents::vid_commitment, node_implementation::NodeType, signature_key::SignatureKey,
};
use std::time::Duration;
use tide_disco::Url;

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_block_builder() {
    use hotshot_types::constants::Version01;

    let port = portpicker::pick_unused_port().expect("Could not find an open port");
    let api_url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

    run_random_builder(api_url.clone());

    let client: BuilderClient<TestTypes, Version01> = BuilderClient::new(api_url);
    assert!(client.connect(Duration::from_millis(100)).await);

    // Test getting blocks
    let mut blocks = client
        .get_avaliable_blocks(vid_commitment(&vec![], 1))
        .await
        .expect("Failed to get avaliable blocks");

    assert!(!blocks.is_empty());

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
