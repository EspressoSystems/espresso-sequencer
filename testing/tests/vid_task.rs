use commit::Committable;
use hotshot::{tasks::add_vid_task, HotShotConsensusApi};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_testing::{
    node_types::{MemoryImpl, TestTypes},
    task_helpers::vid_init,
};
use hotshot_types::traits::election::VIDExchangeType;
use hotshot_types::{
    block_impl::VIDTransaction,
    data::{DAProposal, VidDisperse, VidSchemeTrait, ViewNumber},
    traits::{
        consensus_api::ConsensusSharedApi, election::ConsensusExchange,
        node_implementation::ExchangesType, state::ConsensusTime,
    },
};
use std::collections::HashMap;

#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_vid_task() {
    use hotshot_task_impls::harness::run_harness;
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_types::{block_impl::VIDBlockPayload, message::Proposal};

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 2.
    let handle = build_system_handle(2).await.0;
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let vid_exchange = api.inner.exchanges.vid_exchange().clone();
    let pub_key = *api.public_key();

    let vid = vid_init();
    let txn = vec![0u8];
    let vid_disperse = vid.disperse(&txn).unwrap();
    let block_commitment = vid_disperse.commit;
    let block = VIDBlockPayload {
        transactions: vec![VIDTransaction(txn)],
        commitment: block_commitment,
    };

    let signature = vid_exchange.sign_vid_disperse(&block.commit());
    let proposal: DAProposal<TestTypes> = DAProposal {
        deltas: block.clone(),
        view_number: ViewNumber::new(2),
    };
    let message = Proposal {
        data: proposal,
        signature,
    };
    let vid_proposal = Proposal {
        data: VidDisperse {
            view_number: message.data.view_number,
            commitment: block.commit(),
            shares: vid_disperse.shares,
            common: vid_disperse.common,
        },
        signature: message.signature.clone(),
    };

    // Every event input is seen on the event stream in the output.
    let mut input = Vec::new();
    let mut output = HashMap::new();

    // In view 1, node 2 is the next leader.
    input.push(HotShotEvent::ViewChange(ViewNumber::new(1)));
    input.push(HotShotEvent::ViewChange(ViewNumber::new(2)));
    input.push(HotShotEvent::BlockReady(block.clone(), ViewNumber::new(2)));

    input.push(HotShotEvent::VidDisperseRecv(vid_proposal.clone(), pub_key));
    input.push(HotShotEvent::Shutdown);

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(1)), 1);
    output.insert(
        HotShotEvent::BlockReady(block.clone(), ViewNumber::new(2)),
        1,
    );

    let vote_token = vid_exchange
        .make_vote_token(ViewNumber::new(2))
        .unwrap()
        .unwrap();
    let vid_vote = vid_exchange.create_vid_message(block.commit(), ViewNumber::new(2), vote_token);
    output.insert(HotShotEvent::VidVoteSend(vid_vote), 1);

    output.insert(HotShotEvent::VidDisperseRecv(vid_proposal, pub_key), 1);
    output.insert(HotShotEvent::ViewChange(ViewNumber::new(2)), 1);
    output.insert(HotShotEvent::Shutdown, 1);

    let build_fn =
        |task_runner, event_stream| add_vid_task(task_runner, event_stream, vid_exchange, handle);

    run_harness(input, output, None, build_fn).await;
}
