use hotshot::{types::SignatureKey, HotShotConsensusApi};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_testing::{
    node_types::{MemoryImpl, TestTypes},
    task_helpers::{build_quorum_proposal, vid_init},
};
use hotshot_types::{
    data::{DAProposal, VidSchemeTrait, ViewNumber},
    traits::{consensus_api::ConsensusSharedApi, state::ConsensusTime},
};
use std::{collections::HashMap, marker::PhantomData};

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
        block_impl::VIDTransaction, data::VidDisperse, message::Proposal,
        traits::node_implementation::NodeType,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 2.
    let (handle, event_stream) = build_system_handle(2).await;
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let pub_key = *api.public_key();
    let priv_key = api.private_key();
    let vid = vid_init();
    let transactions = vec![VIDTransaction(vec![0])];
    let encoded_transactions = VIDTransaction::encode(transactions.clone()).unwrap();
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();
    let payload_commitment = vid_disperse.commit;
    let signature =
        <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey::sign(
            api.private_key(),
            payload_commitment.as_ref(),
        )
        .expect("Failed to sign block commitment");
    let da_proposal = Proposal {
        data: DAProposal {
            encoded_transactions: encoded_transactions.clone(),
            metadata: (),
            view_number: ViewNumber::new(2),
        },
        signature,
        _pd: PhantomData,
    };
    let quorum_proposal = build_quorum_proposal(&handle, priv_key, 2).await;
    let da_vid_disperse_inner = VidDisperse {
        view_number: da_proposal.data.view_number,
        payload_commitment,
        shares: vid_disperse.shares,
        common: vid_disperse.common,
    };
    // TODO for now reuse the same block payload commitment and signature as DA committee
    // https://github.com/EspressoSystems/jellyfish/issues/369
    let da_vid_disperse = Proposal {
        data: da_vid_disperse_inner.clone(),
        signature: da_proposal.signature.clone(),
        _pd: PhantomData,
    };

    // Every event input is seen on the event stream in the output.
    let mut input = Vec::new();
    let mut output = HashMap::new();

    input.push(HotShotEvent::ViewChange(ViewNumber::new(1)));
    input.push(HotShotEvent::TransactionsSequenced(
        encoded_transactions.clone(),
        (),
        ViewNumber::new(2),
    ));
    input.push(HotShotEvent::BlockReady(
        da_vid_disperse_inner.clone(),
        ViewNumber::new(2),
    ));
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
        HotShotEvent::TransactionsSequenced(encoded_transactions, (), ViewNumber::new(2)),
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
    output.insert(
        HotShotEvent::SendPayloadCommitmentAndMetadata(payload_commitment, ()),
        1,
    );
    output.insert(
        HotShotEvent::BlockReady(da_vid_disperse_inner, ViewNumber::new(2)),
        2,
    );
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
