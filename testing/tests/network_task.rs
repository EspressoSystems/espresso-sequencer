use commit::Committable;
use hotshot::HotShotSequencingConsensusApi;
use hotshot_task_impls::events::SequencingHotShotEvent;
use hotshot_testing::{
    node_types::{SequencingMemoryImpl, SequencingTestTypes},
    task_helpers::build_quorum_proposal,
};
use hotshot_types::{
    data::{DAProposal, ViewNumber},
    traits::{
        consensus_api::ConsensusSharedApi, node_implementation::ExchangesType, state::ConsensusTime,
    },
};
use std::collections::HashMap;

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_network_task() {
    use hotshot::demos::sdemo::{SDemoBlock, SDemoNormalBlock};
    use hotshot_task_impls::harness::run_harness;
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_types::{message::Proposal, traits::election::CommitteeExchangeType};

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 2.
    let (handle, event_stream) = build_system_handle(2).await;
    let api: HotShotSequencingConsensusApi<SequencingTestTypes, SequencingMemoryImpl> =
        HotShotSequencingConsensusApi {
            inner: handle.hotshot.inner.clone(),
        };
    let committee_exchange = api.inner.exchanges.committee_exchange().clone();
    let pub_key = *api.public_key();
    let priv_key = api.private_key();
    let block = SDemoBlock::Normal(SDemoNormalBlock {
        previous_state: (),
        transactions: Vec::new(),
    });
    let block_commitment = block.commit();
    let signature = committee_exchange.sign_da_proposal(&block_commitment);
    let da_proposal = Proposal {
        data: DAProposal {
            deltas: block.clone(),
            view_number: ViewNumber::new(2),
        },
        signature,
    };
    let quorum_proposal = build_quorum_proposal(&handle, priv_key, 2).await;

    // Every event input is seen on the event stream in the output.
    let mut input = Vec::new();
    let mut output = HashMap::new();

    input.push(SequencingHotShotEvent::ViewChange(ViewNumber::new(1)));
    input.push(SequencingHotShotEvent::DAProposalSend(
        da_proposal.clone(),
        pub_key,
    ));
    input.push(SequencingHotShotEvent::QuorumProposalSend(
        quorum_proposal.clone(),
        pub_key,
    ));
    input.push(SequencingHotShotEvent::ViewChange(ViewNumber::new(2)));
    input.push(SequencingHotShotEvent::Shutdown);

    output.insert(SequencingHotShotEvent::ViewChange(ViewNumber::new(1)), 2);
    // One output from the input, the other from the DA task.
    output.insert(
        SequencingHotShotEvent::DAProposalSend(da_proposal.clone(), pub_key),
        2,
    );
    // Only one output from the input.
    // The consensus task will fail to send a second proposal, like the DA task does, due to the
    // view number check in `publish_proposal_if_able` in consensus.rs, and we will see an error in
    // logging, but that is fine for testing as long as the network task is correctly handling
    // events.
    output.insert(
        SequencingHotShotEvent::QuorumProposalSend(quorum_proposal.clone(), pub_key),
        1,
    );
    output.insert(SequencingHotShotEvent::SendDABlockData(block), 1);
    output.insert(
        SequencingHotShotEvent::DAProposalRecv(da_proposal, pub_key),
        1,
    );
    output.insert(
        SequencingHotShotEvent::QuorumProposalRecv(quorum_proposal, pub_key),
        1,
    );
    output.insert(SequencingHotShotEvent::ViewChange(ViewNumber::new(2)), 2);
    output.insert(SequencingHotShotEvent::Shutdown, 1);

    let build_fn = |task_runner, _| async { task_runner };
    run_harness(input, output, Some(event_stream), build_fn).await;
}
