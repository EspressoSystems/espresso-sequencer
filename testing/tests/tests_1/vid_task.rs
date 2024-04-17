use std::{collections::HashMap, marker::PhantomData};

use hotshot::types::SignatureKey;
use hotshot_example_types::{
    block_types::{TestBlockPayload, TestTransaction},
    node_types::TestTypes,
};
use hotshot_task_impls::{events::HotShotEvent, vid::VIDTaskState};
use hotshot_testing::task_helpers::{build_system_handle, vid_scheme_from_view_number};
use hotshot_types::{
    data::{null_block, DAProposal, VidDisperse, VidDisperseShare, ViewNumber},
    traits::{
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
        BlockPayload,
    },
};
use jf_primitives::vid::VidScheme;

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_vid_task() {
    use hotshot_task_impls::harness::run_harness;
    use hotshot_types::message::Proposal;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 2.
    let handle = build_system_handle(2).await.0;
    let pub_key = *handle.public_key();

    // quorum membership for VID share distribution
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let mut vid = vid_scheme_from_view_number::<TestTypes>(&quorum_membership, ViewNumber::new(0));
    let transactions = vec![TestTransaction(vec![0])];
    let (payload, metadata) = TestBlockPayload::from_transactions(transactions.clone()).unwrap();
    let builder_commitment = payload.builder_commitment(&metadata);
    let encoded_transactions = TestTransaction::encode(transactions.clone()).unwrap();
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();
    let payload_commitment = vid_disperse.commit;

    let signature = <TestTypes as NodeType>::SignatureKey::sign(
        handle.private_key(),
        payload_commitment.as_ref(),
    )
    .expect("Failed to sign block payload!");
    let proposal: DAProposal<TestTypes> = DAProposal {
        encoded_transactions: encoded_transactions.clone(),
        metadata: (),
        view_number: ViewNumber::new(2),
    };
    let message = Proposal {
        data: proposal.clone(),
        signature,
        _pd: PhantomData,
    };

    let vid_disperse =
        VidDisperse::from_membership(message.data.view_number, vid_disperse, &quorum_membership);

    let vid_proposal = Proposal {
        data: vid_disperse.clone(),
        signature: message.signature.clone(),
        _pd: PhantomData,
    };
    let vid_share_proposals: Vec<_> = VidDisperseShare::from_vid_disperse(vid_disperse.clone())
        .into_iter()
        .map(|vid_disperse_share| {
            vid_disperse_share
                .to_proposal(handle.private_key())
                .expect("Failed to sign block payload!")
        })
        .collect();
    let vid_share_proposal = vid_share_proposals[0].clone();

    let mut input = Vec::new();
    let mut output = HashMap::new();

    // In view 1, node 2 is the next leader.
    input.push(HotShotEvent::ViewChange(ViewNumber::new(1)));
    input.push(HotShotEvent::ViewChange(ViewNumber::new(2)));
    input.push(HotShotEvent::BlockRecv(
        encoded_transactions.clone(),
        (),
        ViewNumber::new(2),
        null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
    ));
    input.push(HotShotEvent::BlockReady(
        vid_disperse.clone(),
        ViewNumber::new(2),
    ));

    input.push(HotShotEvent::VidDisperseSend(vid_proposal.clone(), pub_key));
    input.push(HotShotEvent::VIDShareRecv(vid_share_proposal.clone()));
    input.push(HotShotEvent::Shutdown);

    output.insert(
        HotShotEvent::BlockReady(vid_disperse, ViewNumber::new(2)),
        1,
    );

    output.insert(
        HotShotEvent::SendPayloadCommitmentAndMetadata(
            payload_commitment,
            builder_commitment,
            (),
            ViewNumber::new(2),
            null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
        ),
        1,
    );
    output.insert(
        HotShotEvent::VidDisperseSend(vid_proposal.clone(), pub_key),
        1,
    );

    let vid_state = VIDTaskState {
        api: handle.clone(),
        consensus: handle.hotshot.get_consensus(),
        cur_view: ViewNumber::new(0),
        vote_collector: None,
        network: handle.hotshot.networks.quorum_network.clone(),
        membership: handle.hotshot.memberships.vid_membership.clone().into(),
        public_key: *handle.public_key(),
        private_key: handle.private_key().clone(),
        id: handle.hotshot.id,
    };
    run_harness(input, output, vid_state, false).await;
}
