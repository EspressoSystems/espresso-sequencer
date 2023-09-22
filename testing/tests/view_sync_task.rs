use commit::Committable;
use hotshot::{types::SignatureKey, HotShotSequencingConsensusApi};
use hotshot_task_impls::events::SequencingHotShotEvent;
use hotshot_testing::node_types::{SequencingMemoryImpl, SequencingTestTypes};
use hotshot_types::{
    data::ViewNumber,
    traits::{
        consensus_api::ConsensusSharedApi,
        election::{ConsensusExchange, ViewSyncExchangeType},
        node_implementation::ExchangesType,
        state::ConsensusTime,
    },
};
use std::collections::HashMap;

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_view_sync_task() {
    use core::panic;

    use hotshot::tasks::add_view_sync_task;
    use hotshot_task_impls::harness::run_harness;
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_types::{
        traits::election::VoteData,
        vote::{ViewSyncData, ViewSyncVote, ViewSyncVoteInternal},
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 3.
    let handle = build_system_handle(5).await.0;
    let api: HotShotSequencingConsensusApi<SequencingTestTypes, SequencingMemoryImpl> =
        HotShotSequencingConsensusApi {
            inner: handle.hotshot.inner.clone(),
        };
    let view_sync_exchange = api.inner.exchanges.view_sync_exchange().clone();
    let relay_pub_key = api.public_key().to_bytes();
    let vote_token = view_sync_exchange
        .make_vote_token(ViewNumber::new(5))
        .unwrap_or_else(|_| panic!("Error making vote token"))
        .unwrap_or_else(|| panic!("Not chosen for the committee"));
    let vote_data_internal: ViewSyncData<SequencingTestTypes> = ViewSyncData {
        relay: relay_pub_key.clone(),
        round: ViewNumber::new(5),
    };
    let vote_data_internal_commitment = vote_data_internal.commit();
    let signature = view_sync_exchange.sign_precommit_message(vote_data_internal_commitment);
    let vote = ViewSyncVote::PreCommit(ViewSyncVoteInternal {
        relay_pub_key,
        relay: 0,
        round: ViewNumber::new(5),
        signature,
        vote_token,
        vote_data: VoteData::ViewSyncPreCommit(vote_data_internal_commitment),
    });

    // Every event input is seen on the event stream in the output.
    let mut input = Vec::new();
    let mut output = HashMap::new();

    input.push(SequencingHotShotEvent::Timeout(ViewNumber::new(2)));
    input.push(SequencingHotShotEvent::Timeout(ViewNumber::new(3)));
    input.push(SequencingHotShotEvent::Timeout(ViewNumber::new(4)));

    input.push(SequencingHotShotEvent::Shutdown);

    output.insert(SequencingHotShotEvent::Timeout(ViewNumber::new(2)), 1);
    output.insert(SequencingHotShotEvent::Timeout(ViewNumber::new(3)), 1);
    output.insert(SequencingHotShotEvent::Timeout(ViewNumber::new(4)), 1);

    output.insert(SequencingHotShotEvent::ViewSyncVoteSend(vote.clone()), 1);
    output.insert(SequencingHotShotEvent::ViewChange(ViewNumber::new(3)), 1);
    output.insert(SequencingHotShotEvent::ViewChange(ViewNumber::new(4)), 1);

    output.insert(SequencingHotShotEvent::Shutdown, 1);

    let build_fn =
        |task_runner, event_stream| add_view_sync_task(task_runner, event_stream, handle);

    run_harness(input, output, None, build_fn).await;
}
