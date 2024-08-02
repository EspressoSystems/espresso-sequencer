use hotshot::tasks::task_state::CreateTaskState;
use hotshot_example_types::{
    block_types::TestMetadata,
    node_types::{MemoryImpl, TestConsecutiveLeaderTypes},
};
use hotshot_task_impls::{
    events::HotShotEvent, harness::run_harness, transactions::TransactionTaskState,
};
use hotshot_testing::helpers::build_system_handle;
use hotshot_types::{
    constants::BaseVersion,
    data::{null_block, PackedBundle, ViewNumber},
    traits::{
        block_contents::precompute_vid_commitment, election::Membership,
        node_implementation::ConsensusTime,
    },
};
use vbs::version::StaticVersionType;

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_transaction_task_leader_two_views_in_a_row() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 2.
    let node_id = 2;
    let handle = build_system_handle::<TestConsecutiveLeaderTypes, MemoryImpl>(node_id)
        .await
        .0;

    let mut input = Vec::new();
    let mut output = Vec::new();

    let current_view = ViewNumber::new(4);
    input.push(HotShotEvent::ViewChange(current_view));
    input.push(HotShotEvent::Shutdown);
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let (_, precompute_data) = precompute_vid_commitment(&[], quorum_membership.total_nodes());

    // current view
    let mut exp_packed_bundle = PackedBundle::new(
        vec![].into(),
        TestMetadata,
        current_view,
        vec1::vec1![null_block::builder_fee(
            quorum_membership.total_nodes(),
            BaseVersion::version()
        )
        .unwrap()],
        Some(precompute_data.clone()),
        None,
    );
    output.push(HotShotEvent::BlockRecv(exp_packed_bundle.clone()));

    // next view
    exp_packed_bundle.view_number = current_view + 1;
    output.push(HotShotEvent::BlockRecv(exp_packed_bundle));

    let transaction_state =
        TransactionTaskState::<TestConsecutiveLeaderTypes, MemoryImpl>::create_from(&handle).await;
    run_harness(input, output, transaction_state, false).await;
}
