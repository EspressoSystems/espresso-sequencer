#![cfg(feature = "dependency-tasks")]

use std::time::Duration;
#[cfg(not(feature = "dependency-tasks"))]
use committable::Committable;
use futures::StreamExt;
use hotshot::tasks::task_state::CreateTaskState;
use hotshot::traits::ValidatedState;
use hotshot_example_types::state_types::TestValidatedState;
use hotshot_example_types::{
    block_types::TestMetadata,
    node_types::{MemoryImpl, TestTypes},

};
#[cfg(not(feature = "dependency-tasks"))]
use hotshot_example_types::{state_types::TestInstanceState,};
#[cfg(not(feature = "dependency-tasks"))]
use hotshot_testing::{
    all_predicates,
    helpers::{
        build_cert, key_pair_for_id
    }};
use hotshot_macros::{run_test, test_scripts};
use hotshot_task_impls::{
    events::HotShotEvent::*,
    quorum_proposal::QuorumProposalTaskState,
};
use hotshot_testing::{
    all_predicates,
    helpers::{
        build_fake_view_with_leaf, build_system_handle,
        vid_scheme_from_view_number, vid_share,
    },
    predicates::{
        event::{all_predicates, exact, quorum_proposal_send},
    },
    random,
    script::{Expectations, InputOrder, TaskScript},
    serial,
    view_generator::TestViewGenerator,
};
#[cfg(not(feature = "dependency-tasks"))]
use hotshot_types::{simple_certificate::QuorumCertificate,};
use hotshot_types::{
    data::{null_block, Leaf, ViewChangeEvidence, ViewNumber},
    simple_vote::{TimeoutData, ViewSyncFinalizeData},
    traits::{
        election::Membership,
        node_implementation::{ConsensusTime, NodeType},
    },
    utils::BuilderCommitment,
    vid::VidSchemeType,
    vote::HasViewNumber,
};
use jf_vid::VidScheme;
use sha2::Digest;

const TIMEOUT: Duration = Duration::from_millis(35);

fn make_payload_commitment(
    membership: &<TestTypes as NodeType>::Membership,
    view: ViewNumber,
) -> <VidSchemeType as VidScheme>::Commit {
    // Make some empty encoded transactions, we just care about having a commitment handy for the
    // later calls. We need the VID commitment to be able to propose later.
    let mut vid = vid_scheme_from_view_number::<TestTypes>(membership, view);
    let encoded_transactions = Vec::new();
    vid.commit_only(&encoded_transactions).unwrap()
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_quorum_proposal_task_quorum_proposal_view_1() {
    use hotshot_testing::script::{Expectations, TaskScript};

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 1;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(node_id));

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    let mut vids = Vec::new();
    let consensus = handle.hotshot.consensus();
    let mut consensus_writer = consensus.write().await;
    for view in (&mut generator).take(2).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        vids.push(view.vid_proposal.clone());

        // We don't have a `QuorumProposalRecv` task handler, so we'll just manually insert the proposals
        // to make sure they show up during tests.
        consensus_writer
            .update_saved_leaves(Leaf::from_quorum_proposal(&view.quorum_proposal.data));
    }

    // We must send the genesis cert here to initialize hotshot successfully.
    let (validated_state, _ /* state delta */) = <TestValidatedState as ValidatedState<
        TestTypes,
    >>::genesis(&*handle.hotshot.instance_state());
    let genesis_cert = proposals[0].data.justify_qc.clone();
    let genesis_leaf = Leaf::genesis(&validated_state, &*handle.hotshot.instance_state()).await;
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());
    // Special case: the genesis validated state is already
    // present
    consensus_writer.update_validated_state_map(
        ViewNumber::new(0),
        build_fake_view_with_leaf(genesis_leaf.clone()),
    );
    drop(consensus_writer);

    let inputs = vec![
        serial![VidShareValidated(vid_share(
            &vids[0].0,
            handle.public_key()
        )),],
        random![
            QcFormed(either::Left(genesis_cert.clone())),
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(1),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
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
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 3;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    let mut vids = Vec::new();
    let consensus = handle.hotshot.consensus();
    let mut consensus_writer = consensus.write().await;
    for view in (&mut generator).take(5).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        vids.push(view.vid_proposal.clone());

        // We don't have a `QuorumProposalRecv` task handler, so we'll just manually insert the proposals
        // to make sure they show up during tests.
        consensus_writer
            .update_saved_leaves(Leaf::from_quorum_proposal(&view.quorum_proposal.data));
    }

    // We need to handle the views where we aren't the leader to ensure that the states are
    // updated properly.

    let (validated_state, _ /* state delta */) = <TestValidatedState as ValidatedState<
        TestTypes,
    >>::genesis(&*handle.hotshot.instance_state());
    let genesis_cert = proposals[0].data.justify_qc.clone();
    let genesis_leaf = Leaf::genesis(&validated_state, &*handle.hotshot.instance_state()).await;

    // Special case: the genesis validated state is already
    // present
    consensus_writer.update_validated_state_map(
        ViewNumber::new(0),
        build_fake_view_with_leaf(genesis_leaf.clone()),
    );

    drop(consensus_writer);

    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let inputs = vec![
        random![
            QcFormed(either::Left(genesis_cert.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(1)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(1),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
            ),
            VidShareValidated(vid_share(&vids[0].0, handle.public_key())),
            ValidatedStateUpdated(
                genesis_cert.view_number(),
                build_fake_view_with_leaf(genesis_leaf.clone()),
            ),
        ],
        random![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            QcFormed(either::Left(proposals[1].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(2)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(2),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
            ),
            VidShareValidated(vid_share(&vids[1].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[0].data.view_number(),
                build_fake_view_with_leaf(leaves[0].clone()),
            ),
        ],
        random![
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            QcFormed(either::Left(proposals[2].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(3)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(3),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
            ),
            VidShareValidated(vid_share(&vids[2].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[1].data.view_number(),
                build_fake_view_with_leaf(leaves[1].clone()),
            ),
        ],
        random![
            QuorumProposalRecv(proposals[2].clone(), leaders[2]),
            QcFormed(either::Left(proposals[3].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(4)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(4),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
            ),
            VidShareValidated(vid_share(&vids[3].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[2].data.view_number(),
                build_fake_view_with_leaf(leaves[2].clone()),
            ),
        ],
        random![
            QuorumProposalRecv(proposals[3].clone(), leaders[3]),
            QcFormed(either::Left(proposals[4].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(5)),
                builder_commitment,
                TestMetadata,
                ViewNumber::new(5),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
            ),
            VidShareValidated(vid_share(&vids[4].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[3].data.view_number(),
                build_fake_view_with_leaf(leaves[3].clone()),
            ),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(all_predicates![exact(UpdateHighQc(genesis_cert.clone()))]),
        Expectations::from_outputs(all_predicates![exact(UpdateHighQc(
            proposals[1].data.justify_qc.clone(),
        ))]),
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(proposals[2].data.justify_qc.clone())),
            quorum_proposal_send(),
        ]),
        Expectations::from_outputs(all_predicates![exact(UpdateHighQc(
            proposals[3].data.justify_qc.clone(),
        ))]),
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(proposals[4].data.justify_qc.clone())),
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
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 3;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(node_id));
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut vids = Vec::new();
    let mut leaves = Vec::new();
    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
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
            null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
        ),
        VidShareValidated(vid_share(&vids[2].0.clone(), handle.public_key())),
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
    use hotshot_types::data::null_block;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    // We need to propose as the leader for view 2, otherwise we get caught up with the special
    // case in the genesis view.
    let node_id = 2;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let payload_commitment = make_payload_commitment(&quorum_membership, ViewNumber::new(node_id));
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut vids = Vec::new();
    let mut leaves = Vec::new();
    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        vids.push(view.vid_proposal.clone());
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
            null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
        ),
        VidShareValidated(vid_share(&vids[1].0.clone(), handle.public_key())),
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
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let node_id = 3;
    let handle = build_system_handle(node_id).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    let mut vids = Vec::new();
    let consensus = handle.hotshot.consensus();
    let mut consensus_writer = consensus.write().await;
    for view in (&mut generator).take(5).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        vids.push(view.vid_proposal.clone());

        // We don't have a `QuorumProposalRecv` task handler, so we'll just manually insert the proposals
        // to make sure they show up during tests.
        consensus_writer
            .update_saved_leaves(Leaf::from_quorum_proposal(&view.quorum_proposal.data));
    }
    drop(consensus_writer);

    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

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
                make_payload_commitment(&quorum_membership, ViewNumber::new(1)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(1),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
            ),
            VidShareValidated(vid_share(&vids[0].0, handle.public_key())),
            ValidatedStateUpdated(
                genesis_cert.view_number(),
                build_fake_view_with_leaf(genesis_leaf.clone()),
            ),
        ],
        random![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            QcFormed(either::Left(proposals[1].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(2)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(2),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
            ),
            VidShareValidated(vid_share(&vids[1].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[0].data.view_number(),
                build_fake_view_with_leaf(leaves[0].clone()),
            ),
        ],
        random![
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            QcFormed(either::Left(proposals[2].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(3)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(3),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
            ),
            VidShareValidated(vid_share(&vids[2].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[1].data.view_number(),
                build_fake_view_with_leaf(leaves[1].clone()),
            ),
        ],

        random![
            QuorumProposalRecv(proposals[2].clone(), leaders[2]),
            QcFormed(either::Left(proposals[3].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(4)),
                builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(4),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
            ),
            VidShareValidated(vid_share(&vids[3].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[2].data.view_number(),
                build_fake_view_with_leaf(leaves[2].clone()),
            ),
        ],
        random![
            QuorumProposalRecv(proposals[3].clone(), leaders[3]),
            QcFormed(either::Left(proposals[4].data.justify_qc.clone())),
            SendPayloadCommitmentAndMetadata(
                make_payload_commitment(&quorum_membership, ViewNumber::new(5)),
                builder_commitment,
                TestMetadata,
                ViewNumber::new(5),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
            ),
            VidShareValidated(vid_share(&vids[4].0, handle.public_key())),
            ValidatedStateUpdated(
                proposals[3].data.view_number(),
                build_fake_view_with_leaf(leaves[3].clone()),
            ),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(vec![exact(UpdateHighQc(genesis_cert.clone()))]),
        Expectations::from_outputs(vec![exact(UpdateHighQc(
            proposals[1].data.justify_qc.clone(),
        ))]),
        Expectations::from_outputs(all_predicates![
            exact(UpdateHighQc(proposals[2].data.justify_qc.clone())),
            quorum_proposal_send(),
        ]),
        Expectations::from_outputs(vec![exact(UpdateHighQc(
            proposals[3].data.justify_qc.clone(),
        ))]),
        Expectations::from_outputs(vec![
            exact(UpdateHighQc(proposals[4].data.justify_qc.clone())),
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

    // We need to propose as the leader for view 2, otherwise we get caught up with the special
    // case in the genesis view.
    let handle = build_system_handle(2).await.0;
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
    let inputs = vec![serial![            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
    ]];

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
