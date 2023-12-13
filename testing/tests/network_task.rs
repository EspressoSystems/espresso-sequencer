use hotshot::{types::SignatureKey, HotShotConsensusApi};
use hotshot_task_impls::events::HotShotEvent;
use hotshot_testing::{
    node_types::{MemoryImpl, TestTypes},
    task_helpers::{build_quorum_proposal, vid_init},
};
use hotshot_types::{
    data::{DAProposal, VidSchemeTrait, ViewNumber},
    traits::{consensus_api::ConsensusApi, state::ConsensusTime},
};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, marker::PhantomData};

#[cfg(test)]
#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_network_task() {
    use hotshot_task_impls::harness::run_harness;
    use hotshot_testing::task_helpers::build_system_handle;
    use hotshot_types::{data::VidDisperse, message::Proposal};

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 2.
    let (handle, event_stream) = build_system_handle(2).await;
    let api: HotShotConsensusApi<TestTypes, MemoryImpl> = HotShotConsensusApi {
        inner: handle.hotshot.inner.clone(),
    };
    let pub_key = *api.public_key();
    let priv_key = api.private_key();
    // quorum membership for VID share distribution
    let quorum_membership = handle.hotshot.inner.memberships.quorum_membership.clone();

    let encoded_transactions = Vec::new();
    let encoded_transactions_hash = Sha256::digest(&encoded_transactions);
    let da_signature =
        <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey::sign(
            api.private_key(),
            &encoded_transactions_hash,
        );
    let vid = vid_init::<TestTypes>(quorum_membership.clone(), ViewNumber::new(2));
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();
    let payload_commitment = vid_disperse.commit;
    let vid_signature =
        <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey::sign(
            api.private_key(),
            payload_commitment.as_ref(),
        );

    let da_proposal = Proposal {
        data: DAProposal {
            encoded_transactions: encoded_transactions.clone(),
            metadata: (),
            view_number: ViewNumber::new(2),
        },
        signature: da_signature,
        _pd: PhantomData,
    };
    let quorum_proposal = build_quorum_proposal(&handle, priv_key, 2).await;

    let vid_disperse_inner = VidDisperse::from_membership(
        da_proposal.data.view_number,
        vid_disperse,
        &quorum_membership.clone().into(),
    );

    // TODO for now reuse the same block payload commitment and signature as DA committee
    // https://github.com/EspressoSystems/jellyfish/issues/369
    let vid_proposal = Proposal {
        data: vid_disperse_inner.clone(),
        signature: vid_signature,
        _pd: PhantomData,
    };

    // Every event input is seen on the event stream in the output.
    let mut input = Vec::new();
    let mut output = HashMap::new();

    input.push(HotShotEvent::ViewChange(ViewNumber::new(1)));
    input.push(HotShotEvent::ViewChange(ViewNumber::new(2)));
    input.push(HotShotEvent::TransactionsSequenced(
        encoded_transactions.clone(),
        (),
        ViewNumber::new(2),
    ));
    input.push(HotShotEvent::BlockReady(
        vid_disperse_inner.clone(),
        ViewNumber::new(2),
    ));
    input.push(HotShotEvent::DAProposalSend(da_proposal.clone(), pub_key));
    input.push(HotShotEvent::VidDisperseSend(vid_proposal.clone(), pub_key));
    input.push(HotShotEvent::QuorumProposalSend(
        quorum_proposal.clone(),
        pub_key,
    ));
    input.push(HotShotEvent::Shutdown);

    output.insert(HotShotEvent::ViewChange(ViewNumber::new(1)), 2);
    output.insert(
        HotShotEvent::TransactionsSequenced(encoded_transactions, (), ViewNumber::new(2)),
        2, // 2 occurrences: 1 from `input`, 1 from the transactions task
    );
    output.insert(
        HotShotEvent::BlockReady(vid_disperse_inner, ViewNumber::new(2)),
        2, // 2 occurrences: 1 from `input`, 1 from the VID task
    );
    output.insert(
        HotShotEvent::DAProposalSend(da_proposal.clone(), pub_key),
        2, // 2 occurrences: 1 from `input`, 1 from the DA task
    );
    output.insert(
        HotShotEvent::VidDisperseSend(vid_proposal.clone(), pub_key),
        2, // 2 occurrences: 1 from `input`, 1 from the VID task
    );
    // Only one output from the input.
    // The consensus task will fail to send a second proposal, like the DA task does, due to the
    // view number check in `publish_proposal_if_able` in consensus.rs, and we will see an error in
    // logging, but that is fine for testing as long as the network task is correctly handling
    // events.
    output.insert(
        HotShotEvent::QuorumProposalSend(quorum_proposal.clone(), pub_key),
        1,
    );
    output.insert(HotShotEvent::ViewChange(ViewNumber::new(2)), 2);
    output.insert(
        HotShotEvent::SendPayloadCommitmentAndMetadata(payload_commitment, ()),
        1,
    );
    output.insert(
        HotShotEvent::QuorumProposalRecv(quorum_proposal, pub_key),
        1,
    );
    output.insert(HotShotEvent::VidDisperseRecv(vid_proposal, pub_key), 1);
    output.insert(HotShotEvent::DAProposalRecv(da_proposal, pub_key), 1);
    output.insert(HotShotEvent::Shutdown, 1);

    let build_fn = |task_runner, _| async { task_runner };
    run_harness(input, output, Some(event_stream), build_fn).await;
}
