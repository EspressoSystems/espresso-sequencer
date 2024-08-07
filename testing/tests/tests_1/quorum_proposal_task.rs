// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

#![cfg(feature = "dependency-tasks")]

use std::time::Duration;

use futures::StreamExt;
use hotshot::{tasks::task_state::CreateTaskState, traits::ValidatedState};
use hotshot_example_types::{
    block_types::TestMetadata,
    node_types::{MemoryImpl, TestTypes},
    state_types::TestValidatedState,
};
use hotshot_macros::{run_test, test_scripts};
use hotshot_task_impls::{events::HotShotEvent::*, quorum_proposal::QuorumProposalTaskState};
use hotshot_testing::{
    all_predicates,
    helpers::{build_fake_view_with_leaf, build_payload_commitment, build_system_handle},
    predicates::event::{all_predicates, exact, quorum_proposal_send},
    random,
    script::{Expectations, InputOrder, TaskScript},
    serial,
    view_generator::TestViewGenerator,
};
use hotshot_types::{
    data::{null_block, Leaf, ViewChangeEvidence, ViewNumber},
    simple_vote::{TimeoutData, ViewSyncFinalizeData},
    traits::{election::Membership, node_implementation::ConsensusTime},
    utils::BuilderCommitment,
    vote::HasViewNumber,
};
use sha2::Digest;
use vec1::vec1;

const TIMEOUT: Duration = Duration::from_millis(35);

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_quorum_proposal_view_1() {
    use hotshot_testing::script::{Expectations, TaskScript};
    use hotshot_types::constants::BaseVersion;
    use vbs::version::StaticVersionType;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 1;
    let handle = build_system_handle::<TestTypes, MemoryImpl>(node_id)
        .await
        .0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = build_payload_commitment(&quorum_membership, ViewNumber::new(node_id));

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    let mut vids = Vec::new();
    let mut vid_dispersals = Vec::new();
    let consensus = handle.hotshot.consensus();
    let mut consensus_writer = consensus.write().await;
    for view in (&mut generator).take(2).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        vids.push(view.vid_proposal.clone());
        vid_dispersals.push(view.vid_disperse.clone());

        // We don't have a `QuorumProposalRecv` task handler, so we'll just manually insert the proposals
        // to make sure they show up during tests.
        consensus_writer
            .update_saved_leaves(Leaf::from_quorum_proposal(&view.quorum_proposal.data));
    }

    // We must send the genesis cert here to initialize hotshot successfully.
    let genesis_cert = proposals[0].data.justify_qc.clone();
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());
    let builder_fee =
        null_block::builder_fee(quorum_membership.total_nodes(), BaseVersion::version()).unwrap();
    drop(consensus_writer);

    let inputs = vec![
        serial![VidDisperseSend(
            vid_dispersals[0].clone(),
            handle.public_key()
        )],
        random![
            QcFormed(either::Left(genesis_cert.clone())),
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(1),
                vec1![builder_fee.clone()],
                None,
            ),
            ValidatedStateUpdated(
                proposals[0].data.view_number(),
                build_fake_view_with_leaf(leaves[0].clone()),
            ),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(vec![]),
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(genesis_cert.clone())),
            exact(HighQcUpdated(genesis_cert.clone())),
            quorum_proposal_send(),
        ]),
    ];

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let mut script = TaskScript {
        timeout: TIMEOUT,
        state: quorum_proposal_task_state,
        expectations,
    };
    run_test![inputs, script].await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_quorum_proposal_view_gt_1() {
    use hotshot_types::constants::BaseVersion;
    use vbs::version::StaticVersionType;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 3;
    let handle = build_system_handle::<TestTypes, MemoryImpl>(node_id)
        .await
        .0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    let mut vids = Vec::new();
    let mut vid_dispersals = Vec::new();
    let consensus = handle.hotshot.consensus();
    let mut consensus_writer = consensus.write().await;
    for view in (&mut generator).take(5).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        vids.push(view.vid_proposal.clone());
        vid_dispersals.push(view.vid_disperse.clone());

        // We don't have a `QuorumProposalRecv` task handler, so we'll just manually insert the proposals
        // to make sure they show up during tests.
        consensus_writer
            .update_saved_leaves(Leaf::from_quorum_proposal(&view.quorum_proposal.data));

        consensus_writer
            .update_validated_state_map(
                view.quorum_proposal.data.view_number(),
                build_fake_view_with_leaf(view.leaf.clone()),
            )
            .unwrap();
    }

    // We need to handle the views where we aren't the leader to ensure that the states are
    // updated properly.

    let (validated_state, _ /* state delta */) = <TestValidatedState as ValidatedState<
        TestTypes,
    >>::genesis(&*handle.hotshot.instance_state());
    let genesis_cert = proposals[0].data.justify_qc.clone();
    let genesis_leaf = Leaf::genesis(&validated_state, &*handle.hotshot.instance_state()).await;

    drop(consensus_writer);

    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());
    let builder_fee =
        null_block::builder_fee(quorum_membership.total_nodes(), BaseVersion::version()).unwrap();

    let inputs = vec![
        random![
            QcFormed(either::Left(genesis_cert.clone())),
            SendPayloadCommitmentAndMetadata(
                build_payload_commitment(&quorum_membership, ViewNumber::new(1)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(1),
                vec1![builder_fee.clone()],
                None,
            ),
            VidDisperseSend(vid_dispersals[0].clone(), handle.public_key()),
            ValidatedStateUpdated(
                genesis_cert.view_number(),
                build_fake_view_with_leaf(genesis_leaf.clone()),
            ),
        ],
        random![
            QuorumProposalPreliminarilyValidated(proposals[0].clone()),
            QcFormed(either::Left(proposals[1].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                build_payload_commitment(&quorum_membership, ViewNumber::new(2)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(2),
                vec1![builder_fee.clone()],
                None,
            ),
            VidDisperseSend(vid_dispersals[1].clone(), handle.public_key()),
            ValidatedStateUpdated(
                proposals[0].data.view_number(),
                build_fake_view_with_leaf(leaves[0].clone()),
            ),
        ],
        random![
            QuorumProposalPreliminarilyValidated(proposals[1].clone()),
            QcFormed(either::Left(proposals[2].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                build_payload_commitment(&quorum_membership, ViewNumber::new(3)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(3),
                vec1![builder_fee.clone()],
                None,
            ),
            VidDisperseSend(vid_dispersals[2].clone(), handle.public_key()),
            ValidatedStateUpdated(
                proposals[1].data.view_number(),
                build_fake_view_with_leaf(leaves[1].clone()),
            ),
        ],
        random![
            QuorumProposalPreliminarilyValidated(proposals[2].clone()),
            QcFormed(either::Left(proposals[3].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                build_payload_commitment(&quorum_membership, ViewNumber::new(4)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(4),
                vec1![builder_fee.clone()],
                None,
            ),
            VidDisperseSend(vid_dispersals[3].clone(), handle.public_key()),
            ValidatedStateUpdated(
                proposals[2].data.view_number(),
                build_fake_view_with_leaf(leaves[2].clone()),
            ),
        ],
        random![
            QuorumProposalPreliminarilyValidated(proposals[3].clone()),
            QcFormed(either::Left(proposals[4].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                build_payload_commitment(&quorum_membership, ViewNumber::new(5)),
                builder_commitment,
                TestMetadata,
                ViewNumber::new(5),
                vec1![builder_fee.clone()],
                None,
            ),
            VidDisperseSend(vid_dispersals[4].clone(), handle.public_key()),
            ValidatedStateUpdated(
                proposals[3].data.view_number(),
                build_fake_view_with_leaf(leaves[3].clone()),
            ),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(genesis_cert.clone())),
            exact(HighQcUpdated(genesis_cert.clone())),
        ]),
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(proposals[1].data.justify_qc.clone())),
            exact(HighQcUpdated(proposals[1].data.justify_qc.clone())),
        ]),
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(proposals[2].data.justify_qc.clone())),
            exact(HighQcUpdated(proposals[2].data.justify_qc.clone())),
            quorum_proposal_send(),
        ]),
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(proposals[3].data.justify_qc.clone())),
            exact(HighQcUpdated(proposals[3].data.justify_qc.clone())),
        ]),
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(proposals[4].data.justify_qc.clone())),
            exact(HighQcUpdated(proposals[4].data.justify_qc.clone())),
        ]),
    ];

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let mut script = TaskScript {
        timeout: TIMEOUT,
        state: quorum_proposal_task_state,
        expectations,
    };

    run_test![inputs, script].await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_qc_timeout() {
    use hotshot_types::constants::BaseVersion;
    use vbs::version::StaticVersionType;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 3;
    let handle = build_system_handle::<TestTypes, MemoryImpl>(node_id)
        .await
        .0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = build_payload_commitment(&quorum_membership, ViewNumber::new(node_id));
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut vids = Vec::new();
    let mut vid_dispersals = Vec::new();
    let mut leaves = Vec::new();
    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
        vid_dispersals.push(view.vid_disperse.clone());
        leaves.push(view.leaf.clone());
    }
    let timeout_data = TimeoutData {
        view: ViewNumber::new(1),
    };
    generator.add_timeout(timeout_data);
    for view in (&mut generator).take(2).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
        vid_dispersals.push(view.vid_disperse.clone());
        leaves.push(view.leaf.clone());
    }

    // Get the proposal cert out for the view sync input
    let cert = match proposals[1].data.proposal_certificate.clone().unwrap() {
        ViewChangeEvidence::Timeout(tc) => tc,
        _ => panic!("Found a View Sync Cert when there should have been a Timeout cert"),
    };

    let inputs = vec![random![
        QcFormed(either::Right(cert.clone())),
        SendPayloadCommitmentAndMetadata(
            payload_commitment,
            builder_commitment,
            TestMetadata,
            ViewNumber::new(3),
            vec1![
                null_block::builder_fee(quorum_membership.total_nodes(), BaseVersion::version())
                    .unwrap()
            ],
            None,
        ),
        VidDisperseSend(vid_dispersals[2].clone(), handle.public_key()),
        ValidatedStateUpdated(
            proposals[1].data.view_number(),
            build_fake_view_with_leaf(leaves[1].clone()),
        ),
    ]];

    let expectations = vec![Expectations::from_outputs(vec![quorum_proposal_send()])];

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let mut script = TaskScript {
        timeout: TIMEOUT,
        state: quorum_proposal_task_state,
        expectations,
    };
    run_test![inputs, script].await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_view_sync() {
    use hotshot_example_types::block_types::TestMetadata;
    use hotshot_types::{constants::BaseVersion, data::null_block};
    use vbs::version::StaticVersionType;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 2;
    let handle = build_system_handle::<TestTypes, MemoryImpl>(node_id)
        .await
        .0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = build_payload_commitment(&quorum_membership, ViewNumber::new(node_id));
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut vids = Vec::new();
    let mut vid_dispersals = Vec::new();
    let mut leaves = Vec::new();
    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
        vid_dispersals.push(view.vid_disperse.clone());
        leaves.push(view.leaf.clone());
    }

    let view_sync_finalize_data = ViewSyncFinalizeData {
        relay: 2,
        round: ViewNumber::new(node_id),
    };
    generator.add_view_sync_finalize(view_sync_finalize_data);
    for view in (&mut generator).take(2).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
        vid_dispersals.push(view.vid_disperse.clone());
        leaves.push(view.leaf.clone());
    }

    // Get the proposal cert out for the view sync input
    let cert = match proposals[1].data.proposal_certificate.clone().unwrap() {
        ViewChangeEvidence::ViewSync(vsc) => vsc,
        _ => panic!("Found a TC when there should have been a view sync cert"),
    };

    let inputs = vec![random![
        ViewSyncFinalizeCertificate2Recv(cert.clone()),
        SendPayloadCommitmentAndMetadata(
            payload_commitment,
            builder_commitment,
            TestMetadata,
            ViewNumber::new(2),
            vec1![
                null_block::builder_fee(quorum_membership.total_nodes(), BaseVersion::version())
                    .unwrap()
            ],
            None,
        ),
        VidDisperseSend(vid_dispersals[1].clone(), handle.public_key()),
        ValidatedStateUpdated(
            proposals[0].data.view_number(),
            build_fake_view_with_leaf(leaves[0].clone()),
        ),
    ]];

    let expectations = vec![Expectations::from_outputs(vec![quorum_proposal_send()])];

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let mut script = TaskScript {
        timeout: TIMEOUT,
        state: quorum_proposal_task_state,
        expectations,
    };
    run_test![inputs, script].await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_liveness_check() {
    use hotshot_types::constants::BaseVersion;
    use vbs::version::StaticVersionType;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 3;
    let handle = build_system_handle::<TestTypes, MemoryImpl>(node_id)
        .await
        .0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    let mut vids = Vec::new();
    let mut vid_dispersals = Vec::new();
    let consensus = handle.hotshot.consensus();
    let mut consensus_writer = consensus.write().await;
    for view in (&mut generator).take(5).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        vids.push(view.vid_proposal.clone());
        vid_dispersals.push(view.vid_disperse.clone());

        // We don't have a `QuorumProposalRecv` task handler, so we'll just manually insert the proposals
        // to make sure they show up during tests.
        consensus_writer
            .update_saved_leaves(Leaf::from_quorum_proposal(&view.quorum_proposal.data));
        consensus_writer
            .update_validated_state_map(
                view.quorum_proposal.data.view_number(),
                build_fake_view_with_leaf(view.leaf.clone()),
            )
            .unwrap();
    }
    drop(consensus_writer);

    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());
    let builder_fee =
        null_block::builder_fee(quorum_membership.total_nodes(), BaseVersion::version()).unwrap();

    // We need to handle the views where we aren't the leader to ensure that the states are
    // updated properly.

    let (validated_state, _ /* state delta */) = <TestValidatedState as ValidatedState<
        TestTypes,
    >>::genesis(&*handle.hotshot.instance_state());
    let genesis_cert = proposals[0].data.justify_qc.clone();
    let genesis_leaf = Leaf::genesis(&validated_state, &*handle.hotshot.instance_state()).await;

    let inputs = vec![
        random![
            QcFormed(either::Left(genesis_cert.clone())),
            SendPayloadCommitmentAndMetadata(
                build_payload_commitment(&quorum_membership, ViewNumber::new(1)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(1),
                vec1![builder_fee.clone()],
                None,
            ),
            VidDisperseSend(vid_dispersals[0].clone(), handle.public_key()),
            ValidatedStateUpdated(
                genesis_cert.view_number(),
                build_fake_view_with_leaf(genesis_leaf.clone()),
            ),
        ],
        random![
            QuorumProposalPreliminarilyValidated(proposals[0].clone()),
            QcFormed(either::Left(proposals[1].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                build_payload_commitment(&quorum_membership, ViewNumber::new(2)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(2),
                vec1![builder_fee.clone()],
                None,
            ),
            VidDisperseSend(vid_dispersals[1].clone(), handle.public_key()),
            ValidatedStateUpdated(
                proposals[0].data.view_number(),
                build_fake_view_with_leaf(leaves[0].clone()),
            ),
        ],
        random![
            QuorumProposalPreliminarilyValidated(proposals[1].clone()),
            QcFormed(either::Left(proposals[2].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                build_payload_commitment(&quorum_membership, ViewNumber::new(3)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(3),
                vec1![builder_fee.clone()],
                None,
            ),
            VidDisperseSend(vid_dispersals[2].clone(), handle.public_key()),
            ValidatedStateUpdated(
                proposals[1].data.view_number(),
                build_fake_view_with_leaf(leaves[1].clone()),
            ),
        ],
        random![
            QuorumProposalPreliminarilyValidated(proposals[2].clone()),
            QcFormed(either::Left(proposals[3].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                build_payload_commitment(&quorum_membership, ViewNumber::new(4)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(4),
                vec1![builder_fee.clone()],
                None,
            ),
            VidDisperseSend(vid_dispersals[3].clone(), handle.public_key()),
            ValidatedStateUpdated(
                proposals[2].data.view_number(),
                build_fake_view_with_leaf(leaves[2].clone()),
            ),
        ],
        random![
            QuorumProposalPreliminarilyValidated(proposals[3].clone()),
            QcFormed(either::Left(proposals[4].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                build_payload_commitment(&quorum_membership, ViewNumber::new(5)),
                builder_commitment,
                TestMetadata,
                ViewNumber::new(5),
                vec1![builder_fee.clone()],
                None,
            ),
            VidDisperseSend(vid_dispersals[4].clone(), handle.public_key()),
            ValidatedStateUpdated(
                proposals[3].data.view_number(),
                build_fake_view_with_leaf(leaves[3].clone()),
            ),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(genesis_cert.clone())),
            exact(HighQcUpdated(genesis_cert.clone())),
        ]),
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(proposals[1].data.justify_qc.clone())),
            exact(HighQcUpdated(proposals[1].data.justify_qc.clone())),
        ]),
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(proposals[2].data.justify_qc.clone())),
            exact(HighQcUpdated(proposals[2].data.justify_qc.clone())),
            quorum_proposal_send(),
        ]),
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(proposals[3].data.justify_qc.clone())),
            exact(HighQcUpdated(proposals[3].data.justify_qc.clone())),
        ]),
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(proposals[4].data.justify_qc.clone())),
            exact(HighQcUpdated(proposals[4].data.justify_qc.clone())),
        ]),
    ];

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let mut script = TaskScript {
        timeout: TIMEOUT,
        state: quorum_proposal_task_state,
        expectations,
    };
    run_test![inputs, script].await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_with_incomplete_events() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle::<TestTypes, MemoryImpl>(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    for view in (&mut generator).take(2).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
    }

    // We run the task here at view 2, but this time we ignore the crucial piece of evidence: the
    // payload commitment and metadata. Instead we send only one of the three "OR" required fields.
    // This should result in the proposal failing to be sent.
    let inputs = vec![serial![QuorumProposalRecv(
        proposals[1].clone(),
        leaders[1]
    )]];

    let expectations = vec![Expectations::from_outputs(vec![])];

    let quorum_proposal_task_state =
        QuorumProposalTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let mut script = TaskScript {
        timeout: TIMEOUT,
        state: quorum_proposal_task_state,
        expectations,
    };
    run_test![inputs, script].await;
}
