// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

use std::{marker::PhantomData, sync::Arc};

use hotshot::{tasks::task_state::CreateTaskState, types::SignatureKey};
use hotshot_example_types::{
    block_types::{TestBlockPayload, TestMetadata, TestTransaction},
    node_types::{MemoryImpl, TestTypes, TestVersions},
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
    data::{null_block, DaProposal, PackedBundle, VidDisperse, ViewNumber},
    traits::{
        consensus_api::ConsensusApi,
        node_implementation::{ConsensusTime, NodeType, Versions},
        BlockPayload,
    },
};
use jf_vid::VidScheme;
use vbs::version::{StaticVersionType, Version};
use vec1::vec1;

#[tokio::test(flavor = "multi_thread")]
async fn test_vid_task() {
    use hotshot_types::message::Proposal;

    hotshot::helpers::initialize_logging();

    // Build the API for node 2.
    let handle = build_system_handle::<TestTypes, MemoryImpl, TestVersions>(2)
        .await
        .0;
    let pub_key = handle.public_key();

    let membership = handle.hotshot.membership_coordinator.clone();

    let default_version = Version { major: 0, minor: 0 };

    let mut vid = vid_scheme_from_view_number::<TestTypes, TestVersions>(
        &membership.membership_for_epoch(None).await,
        ViewNumber::new(0),
        default_version,
    )
    .await;
    let transactions = vec![TestTransaction::new(vec![0])];

    let (payload, metadata) = <TestBlockPayload as BlockPayload<TestTypes>>::from_transactions(
        transactions.clone(),
        &TestValidatedState::default(),
        &TestInstanceState::default(),
    )
    .await
    .unwrap();
    let builder_commitment =
        <TestBlockPayload as BlockPayload<TestTypes>>::builder_commitment(&payload, &metadata);
    let encoded_transactions = Arc::from(TestTransaction::encode(&transactions));
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();
    let payload_commitment = vid_disperse.commit;

    let signature = <TestTypes as NodeType>::SignatureKey::sign(
        handle.private_key(),
        payload_commitment.as_ref(),
    )
    .expect("Failed to sign block payload!");
    let proposal: DaProposal<TestTypes> = DaProposal {
        encoded_transactions: encoded_transactions.clone(),
        metadata: TestMetadata {
            num_transactions: encoded_transactions.len() as u64,
        },
        view_number: ViewNumber::new(2),
    };
    let message = Proposal {
        data: proposal.clone(),
        signature,
        _pd: PhantomData,
    };

    let vid_disperse = VidDisperse::from_membership(
        message.data.view_number,
        vid_disperse,
        &membership,
        None,
        None,
        None,
    )
    .await;

    let vid_proposal = Proposal {
        data: vid_disperse.clone(),
        signature: message.signature.clone(),
        _pd: PhantomData,
    };
    let mem = membership.membership_for_epoch(None).await;
    let inputs = vec![
        serial![ViewChange(ViewNumber::new(1), None)],
        serial![
            ViewChange(ViewNumber::new(2), None),
            BlockRecv(PackedBundle::new(
                encoded_transactions.clone(),
                TestMetadata {
                    num_transactions: transactions.len() as u64
                },
                ViewNumber::new(2),
                None,
                vec1::vec1![null_block::builder_fee::<TestTypes, TestVersions>(
                    mem.total_nodes().await,
                    <TestVersions as Versions>::Base::VERSION,
                    *ViewNumber::new(2),
                )
                .unwrap()],
                None,
            )),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(vec![]),
        Expectations::from_outputs(vec![
            exact(SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata {
                    num_transactions: transactions.len() as u64,
                },
                ViewNumber::new(2),
                vec1![null_block::builder_fee::<TestTypes, TestVersions>(
                    mem.total_nodes().await,
                    <TestVersions as Versions>::Base::VERSION,
                    *ViewNumber::new(2),
                )
                .unwrap()],
                None,
            )),
            exact(VidDisperseSend(vid_proposal.clone(), pub_key)),
        ]),
    ];

    let vid_state = VidTaskState::<TestTypes, MemoryImpl, TestVersions>::create_from(&handle).await;
    let mut script = TaskScript {
        timeout: std::time::Duration::from_millis(35),
        state: vid_state,
        expectations,
    };

    run_test![inputs, script].await;
}
