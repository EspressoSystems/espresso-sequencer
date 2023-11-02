use commit::Committable;
use hotshot::HotShotConsensusApi;
use hotshot_task_impls::events::HotShotEvent;
use hotshot_testing::{
    node_types::{MemoryImpl, TestTypes},
    task_helpers::vid_init,
};
use hotshot_types::{
    block_impl::VIDTransaction,
    data::{DAProposal, VidSchemeTrait, ViewNumber},
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
async fn test_da_task() {
    use hotshot::tasks::add_da_task;
    use hotshot_task_impls::harness::run_harness;
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_types::{
        block_impl::VIDBlockPayload, message::Proposal, traits::election::CommitteeExchangeType,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 2.
    let handle = build_system_handle(2).await.0;
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let committee_exchange = api.inner.exchanges.committee_exchange().clone();
    let pub_key = *api.public_key();
    let vid = vid_init();
    let txn = vec![0u8];
    let vid_disperse = vid.disperse(&txn).unwrap();
    let payload_commitment = vid_disperse.commit;
    let block = VIDBlockPayload {
        transactions: vec![VIDTransaction(txn)],
        payload_commitment,
    };

    let signature = committee_exchange.sign_da_proposal(&block.commit());
    let proposal = DAProposal {
        block_payload: block.clone(),
        view_number: ViewNumber::new(2),
    };
    let message = Proposal {
        data: proposal,
        signature,
    };

    // TODO for now reuse the same block payload commitment and signature as DA committee
    // https://github.com/EspressoSystems/jellyfish/issues/369

    // Every event input is seen on the event stream in the output.
    let mut input = Vec::new();
    let mut output = HashMap::new();

    // In view 1, node 2 is the next leader.
    input.push(HotShotEvent::ViewChange(ViewNumber::new(1)));
    input.push(HotShotEvent::ViewChange(ViewNumber::new(2)));
    input.push(HotShotEvent::BlockReady(block.clone(), ViewNumber::new(2)));
    input.push(HotShotEvent::DAProposalRecv(message.clone(), pub_key));

    input.push(HotShotEvent::Shutdown);

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(1)), 1);
    output.insert(
        HotShotEvent::BlockReady(block.clone(), ViewNumber::new(2)),
        1,
    );
    output.insert(HotShotEvent::SendPayloadCommitment(block.commit()), 1);
    output.insert(HotShotEvent::DAProposalSend(message.clone(), pub_key), 1);
    let vote_token = committee_exchange
        .make_vote_token(ViewNumber::new(2))
        .unwrap()
        .unwrap();
    let da_vote =
        committee_exchange.create_da_message(block.commit(), ViewNumber::new(2), vote_token);
    output.insert(HotShotEvent::DAVoteSend(da_vote), 1);

    output.insert(HotShotEvent::DAProposalRecv(message, pub_key), 1);

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(2)), 1);
    output.insert(HotShotEvent::Shutdown, 1);

    let build_fn = |task_runner, event_stream| {
        add_da_task(task_runner, event_stream, committee_exchange, handle)
    };

    run_harness(input, output, None, build_fn).await;
}
