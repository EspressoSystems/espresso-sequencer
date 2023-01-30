use contract_bindings::{polygon_zk_evm::PolygonZkEVM, shared_types::BatchData, TestHermezSystem};
use ethers::{providers::Middleware as _, types::BlockNumber};

#[async_std::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let TestHermezSystem {
        provider,
        rollup,
        clients,
        ..
    } = TestHermezSystem::deploy().await;

    // Try to sequence a batch
    let l2_tx_data = hex::decode("1234").unwrap();

    let current_timestamp = provider
        .get_block(BlockNumber::Latest)
        .await
        .unwrap()
        .unwrap()
        .timestamp;

    let batch = BatchData {
        transactions: l2_tx_data.into(),
        global_exit_root: [0u8; 32],
        timestamp: current_timestamp.as_u64(),
        min_forced_timestamp: 0u64,
    };

    // Sent a batch
    let rollup_trusted: PolygonZkEVM<_> = rollup.connect(clients.trusted_sequencer).into();
    let receipt = rollup_trusted
        .sequence_batches(vec![batch])
        .send()
        .await
        .unwrap()
        .await
        .unwrap();

    assert_eq!(receipt.unwrap().status, Some(1u64.into()));
}
