use std::{marker::PhantomData, sync::Arc};

use hotshot::{tasks::task_state::CreateTaskState, types::SignatureKey};
use hotshot_example_types::{
    block_types::{TestBlockPayload, TestMetadata, TestTransaction},
    node_types::{MemoryImpl, TestTypes},
    state_types::{TestInstanceState, TestValidatedState},
};
use hotshot_macros::{run_test, test_scripts};
use hotshot_task_impls::{events::HotShotEvent::*, vid::VidTaskState};
use hotshot_testing::{
    helpers::{build_system_handle, vid_scheme_from_view_number},
    predicates::event::exact,
    script::{Expectations, InputOrder, TaskScript},
    serial,
};
use hotshot_types::{
    constants::BaseVersion,
    data::{null_block, DaProposal, PackedBundle, VidDisperse, ViewNumber},
    traits::{
        consensus_api::ConsensusApi,
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
        BlockPayload,
    },
};
use jf_vid::{precomputable::Precomputable, VidScheme};
use vbs::version::StaticVersionType;

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_vid_task() {
    use hotshot_types::message::Proposal;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // Build the API for node 2.
    let handle = build_system_handle(2).await.0;
    let pub_key = handle.public_key();

    // quorum membership for VID share distribution
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let mut vid = vid_scheme_from_view_number::<TestTypes>(&quorum_membership, ViewNumber::new(0));
    let transactions = vec![TestTransaction::new(vec![0])];

    let (payload, metadata) = <TestBlockPayload as BlockPayload<TestTypes>>::from_transactions(
        transactions.clone(),
        &TestValidatedState::default(),
        &TestInstanceState {},
    )
    .await
    .unwrap();
    let builder_commitment =
        <TestBlockPayload as BlockPayload<TestTypes>>::builder_commitment(&payload, &metadata);
    let encoded_transactions = Arc::from(TestTransaction::encode(&transactions));
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();
    let (_, vid_precompute) = vid.commit_only_precompute(&encoded_transactions).unwrap();
    let payload_commitment = vid_disperse.commit;

    let signature = <TestTypes as NodeType>::SignatureKey::sign(
        handle.private_key(),
        payload_commitment.as_ref(),
    )
    .expect("Failed to sign block payload!");
    let proposal: DaProposal<TestTypes> = DaProposal {
        encoded_transactions: encoded_transactions.clone(),
        metadata: TestMetadata,
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
    let inputs = vec![
        serial![ViewChange(ViewNumber::new(1))],
        serial![
            ViewChange(ViewNumber::new(2)),
            BlockRecv(PackedBundle::new(
                encoded_transactions,
                TestMetadata,
                ViewNumber::new(2),
                vec1::vec1![null_block::builder_fee(
                    quorum_membership.total_nodes(),
                    BaseVersion::version()
                )
                .unwrap()],
                vec1::vec1![null_block::builder_fee(
                    quorum_membership.total_nodes(),
                    BaseVersion::version()
                )
                .unwrap()],
                Some(vid_precompute),
            )),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(vec![]),
        Expectations::from_outputs(vec![
            exact(SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(2),
                null_block::builder_fee(quorum_membership.total_nodes(), BaseVersion::version())
                    .unwrap(),
            )),
            exact(BlockReady(vid_disperse, ViewNumber::new(2))),
            exact(VidDisperseSend(vid_proposal.clone(), pub_key)),
        ]),
    ];

    let vid_state = VidTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    let mut script = TaskScript {
        timeout: std::time::Duration::from_millis(35),
        state: vid_state,
        expectations,
    };

    run_test![inputs, script].await;
}
