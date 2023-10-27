use commit::Committable;
use hotshot::HotShotConsensusApi;
use hotshot_task_impls::events::HotShotEvent;
use hotshot_testing::{
    node_types::{MemoryImpl, TestTypes},
    task_helpers::{build_quorum_proposal, vid_init},
};
use hotshot_types::{
    data::{DAProposal, VidSchemeTrait, ViewNumber},
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
#[ignore]
async fn test_network_task() {
    use hotshot_task_impls::harness::run_harness;
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_types::{
        block_impl::{VIDBlockPayload, VIDTransaction},
        data::VidDisperse,
        message::Proposal,
        traits::election::CommitteeExchangeType,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 2.
    let (handle, event_stream) = build_system_handle(2).await;
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let committee_exchange = api.inner.exchanges.committee_exchange().clone();
    let pub_key = *api.public_key();
    let priv_key = api.private_key();
    let vid = vid_init();
    let txn = vec![0u8];
    let vid_disperse = vid.disperse(&txn).unwrap();
    let payload_commitment = vid_disperse.commit;
    let block = VIDBlockPayload {
        transactions: vec![VIDTransaction(txn)],
        payload_commitment,
    };
    let signature = committee_exchange.sign_da_proposal(&block.commit());
    let da_proposal = Proposal {
        data: DAProposal {
            deltas: block.clone(),
            view_number: ViewNumber::new(2),
        },
        signature,
    };
    let quorum_proposal = build_quorum_proposal(&handle, priv_key, 2).await;
    // TODO for now reuse the same block payload commitment and signature as DA committee
    // https://github.com/EspressoSystems/jellyfish/issues/369
    let da_vid_disperse = Proposal {
        data: VidDisperse {
            view_number: da_proposal.data.view_number,
            payload_commitment: block.commit(),
            shares: vid_disperse.shares,
            common: vid_disperse.common,
        },
        signature: da_proposal.signature.clone(),
    };

    // Every event input is seen on the event stream in the output.
    let mut input = Vec::new();
    let mut output = HashMap::new();

    input.push(HotShotEvent::ViewChange(ViewNumber::new(1)));
    input.push(HotShotEvent::BlockReady(block.clone(), ViewNumber::new(2)));
    input.push(HotShotEvent::DAProposalSend(da_proposal.clone(), pub_key));
    input.push(HotShotEvent::VidDisperseSend(
        da_vid_disperse.clone(),
        pub_key,
    ));
    input.push(HotShotEvent::QuorumProposalSend(
        quorum_proposal.clone(),
        pub_key,
    ));
    input.push(HotShotEvent::ViewChange(ViewNumber::new(2)));
    input.push(HotShotEvent::Shutdown);

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(1)), 2);
    output.insert(
        HotShotEvent::DAProposalSend(da_proposal.clone(), pub_key),
        2, // 2 occurrences: 1 from `input`, 1 from the DA task
    );
    output.insert(
        HotShotEvent::BlockReady(block.clone(), ViewNumber::new(2)),
        2,
    );
    output.insert(
        HotShotEvent::VidDisperseRecv(da_vid_disperse.clone(), pub_key),
        1,
    );
    output.insert(
        HotShotEvent::VidDisperseSend(da_vid_disperse, pub_key),
        2, // 2 occurrences: 1 from `input`, 1 from the DA task
    );
    output.insert(HotShotEvent::Timeout(ViewNumber::new(1)), 1);
    output.insert(HotShotEvent::Timeout(ViewNumber::new(2)), 1);

    // Only one output from the input.
    // The consensus task will fail to send a second proposal, like the DA task does, due to the
    // view number check in `publish_proposal_if_able` in consensus.rs, and we will see an error in
    // logging, but that is fine for testing as long as the network task is correctly handling
    // events.
    output.insert(
        HotShotEvent::QuorumProposalSend(quorum_proposal.clone(), pub_key),
        1,
    );
    output.insert(HotShotEvent::SendDABlockData(block), 1);
    output.insert(HotShotEvent::DAProposalRecv(da_proposal, pub_key), 1);
    output.insert(
        HotShotEvent::QuorumProposalRecv(quorum_proposal, pub_key),
        1,
    );
    output.insert(HotShotEvent::ViewChange(ViewNumber::new(2)), 2);
    output.insert(HotShotEvent::Shutdown, 1);

    let build_fn = |task_runner, _| async { task_runner };
    run_harness(input, output, Some(event_stream), build_fn).await;
}
