use std::sync::Arc;

use async_std::sync::RwLock;
use contract_bindings::HotShot;
use ethers::types::U256;
use futures::stream::StreamExt;
use hotshot_query_service::availability::BlockQueryData;
use sequencer::{hotshot_commitment::connect_l1, Options, SeqTypes};
use surf_disco::Url;

use crate::state::State;

type HotShotClient = surf_disco::Client<hotshot_query_service::Error>;

pub fn execute_block(_block: &BlockQueryData<SeqTypes>, _state: Arc<RwLock<State>>) {
    dbg!("executing block");
}

pub async fn execute(opt: &Options, state: Arc<RwLock<State>>) {
    // Connect to the HotShot query service to stream sequenced blocks.
    let mut query_service_url = opt.query_service_url.clone().unwrap_or(
        format!("http://localhost:{}", opt.port)
            .parse::<Url>()
            .unwrap(),
    );
    query_service_url = query_service_url.join("availability").unwrap();
    let hotshot = HotShotClient::new(query_service_url.clone());
    hotshot.connect(None).await;

    // Connect to the layer one HotShot contract.
    let Some(l1) = connect_l1(opt).await else {
        tracing::error!("unable to connect to L1, hotshot commitment task exiting");
        return;
    };
    let contract = HotShot::new(opt.hotshot_address.unwrap(), l1.clone());

    // Get commitments
    let block_height = contract.block_height().await.unwrap().as_u64();
    for i in 0..block_height {
        // TODO fetch by commitment
        let _commitment = contract.commitments(U256::from(i)).await;
        let block = hotshot
            .get::<BlockQueryData<SeqTypes>>("block/0")
            .send()
            .await
            .unwrap();
        execute_block(&block, state.clone());
    }

    let mut blocks = match hotshot
        .socket(&format!("stream/blocks/{block_height}"))
        .subscribe()
        .await
    {
        Ok(leaves) => Box::pin(leaves.peekable()),
        Err(err) => {
            tracing::error!("unable to subscribe to HotShot query service: {}", err);
            tracing::error!("hotshot commitment task will exit");
            dbg!("error here");
            dbg!(err);
            return;
        }
    };
    loop {
        // Wait for HotShot to sequence a block.
        let block: BlockQueryData<SeqTypes> = match blocks.next().await {
            Some(Ok(block)) => block,
            Some(Err(err)) => {
                tracing::error!("error from HotShot, retrying: {}", err);
                continue;
            }
            None => {
                tracing::error!("HotShot leaf stream ended, hotshot commitment task will exit");
                return;
            }
        };

        execute_block(&block, state.clone());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use async_std::task::spawn;
    use contract_bindings::TestHermezContracts;
    use ethers::signers::{LocalWallet, Signer};
    use futures::future::ready;
    use futures::FutureExt;
    use hotshot_query_service::data_source::QueryData;
    use portpicker::pick_unused_port;
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use sequencer::api::SequencerNode;
    use sequencer::hotshot_commitment::run_hotshot_commitment_task;
    use sequencer::Transaction;
    use sequencer_utils::Anvil;
    use std::path::Path;
    use surf_disco::Client;
    use tempfile::TempDir;
    use tide_disco::error::ServerError;

    const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";

    #[async_std::test]
    async fn test_execute() {
        // Startup test contracts
        let anvil = Anvil::spawn(None).await;
        let l1 = TestHermezContracts::deploy(&anvil.url(), "http://dummy".to_string()).await;
        let initial_batch_num = l1.hotshot.block_height().call().await.unwrap();
        dbg!(initial_batch_num);

        // Start a test HotShot configuration
        let sequencer_port = pick_unused_port().unwrap();
        let nodes = sequencer::testing::init_hotshot_handles().await;
        let api_node = nodes[0].clone();
        let tmp_dir = TempDir::new().unwrap();
        let storage_path: &Path = &(tmp_dir.path().join("tmp_storage"));
        let init_handle = Box::new(move |_| (ready((api_node, 0)).boxed()));
        let query_data = QueryData::create(storage_path, ()).unwrap();
        let SequencerNode { .. } = sequencer::api::serve(query_data, init_handle, sequencer_port)
            .await
            .unwrap();
        for node in &nodes {
            node.start().await;
        }
        let sequencer_url: Url = format!("http://localhost:{sequencer_port}")
            .parse()
            .unwrap();

        let sequencer_opt = sequencer::Options {
            l1_provider: Some(anvil.url()),
            sequencer_mnemonic: Some(TEST_MNEMONIC.to_string()),
            hotshot_address: Some(l1.hotshot.address()),
            l1_chain_id: None,
            chain_id: Default::default(),
            port: Default::default(),
            storage_path: Default::default(),
            cdn_url: "https:///dummy.com".parse::<Url>().unwrap(),
            reset_store: Default::default(),
            query_service_url: Some(sequencer_url.clone()),
        };

        // Submit transaction to sequencer
        let txn = Transaction::new(1.into(), vec![1, 2, 3, 4]);
        let client: Client<ServerError> = Client::new(sequencer_url);
        client.connect(None).await;
        client
            .post::<()>("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();

        let genesis_wallet = LocalWallet::new(&mut ChaChaRng::seed_from_u64(0));
        let genesis_address = genesis_wallet.address();
        let state = Arc::new(RwLock::new(State::from_initial_balances([(
            genesis_address,
            9999,
        )])));

        let options = sequencer_opt.clone();
        spawn(async move { run_hotshot_commitment_task(&sequencer_opt).await });
        execute(&options, state.clone()).await;
    }
}
