use hotshot::tasks::{inject_consensus_polls, task_state::CreateTaskState};
use hotshot::types::SystemContextHandle;
use hotshot_example_types::{
    block_types::TestTransaction,
    node_types::{MemoryImpl, TestTypes},
};
use hotshot_macros::test_scripts;
use hotshot_task_impls::{
    consensus::ConsensusTaskState, events::HotShotEvent::*, upgrade::UpgradeTaskState,
};
use hotshot_testing::{
    predicates::*,
    script::{Expectations, TaskScript},
    view_generator::TestViewGenerator,
};
use hotshot_types::{
    data::ViewNumber,
    simple_vote::UpgradeProposalData,
    traits::{election::Membership, node_implementation::ConsensusTime},
};
use std::time::Duration;
use vbs::version::Version;

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
/// Tests that we correctly update our internal consensus state when reaching a decided upgrade certificate.
async fn test_consensus_task_upgrade() {
    use hotshot_testing::script::{run_test_script, TestScriptStage};
    use hotshot_testing::task_helpers::build_system_handle;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(1).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let old_version = Version { major: 0, minor: 1 };
    let new_version = Version { major: 0, minor: 2 };

    let upgrade_data: UpgradeProposalData<TestTypes> = UpgradeProposalData {
        old_version,
        new_version,
        decide_by: ViewNumber::new(5),
        new_version_hash: [0u8; 12].to_vec(),
        old_version_last_view: ViewNumber::new(5),
        new_version_first_view: ViewNumber::new(7),
    };

    let mut proposals = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    let mut leaders = Vec::new();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
    }

    generator.add_upgrade(upgrade_data);

    for view in generator.take(4) {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
    }

    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            VidDisperseRecv(vids[0].0[0].clone()),
            DACertificateRecv(dacs[0].clone()),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(1))),
            exact(QuorumProposalValidated(proposals[0].data.clone())),
            exact(QuorumVoteSend(votes[0].clone())),
        ],
        asserts: vec![],
    };

    let view_2 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            VidDisperseRecv(vids[1].0[0].clone()),
            DACertificateRecv(dacs[1].clone()),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(2))),
            exact(QuorumProposalValidated(proposals[1].data.clone())),
            exact(QuorumVoteSend(votes[1].clone())),
        ],
        asserts: vec![no_decided_upgrade_cert()],
    };

    let view_3 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[2].clone(), leaders[2]),
            DACertificateRecv(dacs[2].clone()),
            VidDisperseRecv(vids[2].0[0].clone()),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(3))),
            exact(QuorumProposalValidated(proposals[2].data.clone())),
            leaf_decided(),
            exact(QuorumVoteSend(votes[2].clone())),
        ],
        asserts: vec![no_decided_upgrade_cert()],
    };

    let view_4 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[3].clone(), leaders[3]),
            DACertificateRecv(dacs[3].clone()),
            VidDisperseRecv(vids[3].0[0].clone()),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(4))),
            exact(QuorumProposalValidated(proposals[3].data.clone())),
            leaf_decided(),
            exact(QuorumVoteSend(votes[3].clone())),
        ],
        asserts: vec![no_decided_upgrade_cert()],
    };

    let view_5 = TestScriptStage {
        inputs: vec![QuorumProposalRecv(proposals[4].clone(), leaders[4])],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(5))),
            exact(QuorumProposalValidated(proposals[4].data.clone())),
            leaf_decided(),
        ],
        asserts: vec![decided_upgrade_cert()],
    };

    let script = vec![view_1, view_2, view_3, view_4, view_5];

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;

    inject_consensus_polls(&consensus_state).await;

    run_test_script(script, consensus_state).await;
}

#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
/// Test that we correctly form and include an `UpgradeCertificate` when receiving votes.
async fn test_upgrade_and_consensus_task() {
    use hotshot_testing::task_helpers::build_system_handle;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let other_handles = futures::future::join_all((0..=9).map(build_system_handle)).await;

    let old_version = Version { major: 0, minor: 1 };
    let new_version = Version { major: 0, minor: 2 };

    let upgrade_data: UpgradeProposalData<TestTypes> = UpgradeProposalData {
        old_version,
        new_version,
        decide_by: ViewNumber::new(4),
        new_version_hash: [0u8; 12].to_vec(),
        old_version_last_view: ViewNumber::new(5),
        new_version_first_view: ViewNumber::new(7),
    };

    let mut proposals = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    let mut leaders = Vec::new();
    let mut views = Vec::new();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    generator.add_upgrade(upgrade_data.clone());

    for view in generator.take(4) {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    let upgrade_votes = other_handles
        .iter()
        .map(|h| views[1].create_upgrade_vote(upgrade_data.clone(), &h.0));

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;
    let mut upgrade_state = UpgradeTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;

    upgrade_state.should_vote = |_| true;

    inject_consensus_polls(&consensus_state).await;

    let upgrade_vote_recvs: Vec<_> = upgrade_votes.map(UpgradeVoteRecv).collect();

    let inputs = vec![
        vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            VidDisperseRecv(vids[0].0[0].clone()),
            DACertificateRecv(dacs[0].clone()),
        ],
        upgrade_vote_recvs,
        vec![QuorumProposalRecv(proposals[1].clone(), leaders[1])],
        vec![
            DACertificateRecv(dacs[1].clone()),
            SendPayloadCommitmentAndMetadata(
                vids[2].0[0].data.payload_commitment,
                (),
                ViewNumber::new(2),
            ),
            QCFormed(either::Either::Left(proposals[1].data.justify_qc.clone())),
        ],
    ];

    let consensus_script = TaskScript {
        state: consensus_state,
        expectations: vec![
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(1))),
                    quorum_proposal_validated(),
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(2))),
                    quorum_proposal_validated(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![quorum_proposal_send_with_upgrade_certificate()],
                task_state_asserts: vec![],
            },
        ],
    };

    let upgrade_script = TaskScript {
        state: upgrade_state,
        expectations: vec![
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![upgrade_certificate_formed()],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
        ],
    };

    test_scripts![inputs, consensus_script, upgrade_script];
}

#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
/// Test that we correctly handle blank blocks between versions.
/// Specifically, this test schedules an upgrade between views 4 and 8,
/// and ensures that:
///   - we correctly vote affirmatively on a QuorumProposal with a null block payload in view 5
///   - we correctly propose with a null block payload in view 6, even if we have indications to do otherwise (via SendPayloadCommitmentAndMetadata, VID etc).
///   - we correctly reject a QuorumProposal with a non-null block payload in view 7.
async fn test_upgrade_and_consensus_task_blank_blocks() {
    use hotshot_testing::task_helpers::build_system_handle;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(6).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let old_version = Version { major: 0, minor: 1 };
    let new_version = Version { major: 0, minor: 2 };

    let upgrade_data: UpgradeProposalData<TestTypes> = UpgradeProposalData {
        old_version,
        new_version,
        decide_by: ViewNumber::new(4),
        new_version_hash: [0u8; 12].to_vec(),
        old_version_last_view: ViewNumber::new(4),
        new_version_first_view: ViewNumber::new(8),
    };

    let mut proposals = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    let mut leaders = Vec::new();
    let mut views = Vec::new();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    generator.add_upgrade(upgrade_data.clone());

    for view in (&mut generator).take(3) {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    // We are now in the upgrade period, and set the transactions to null for the QuorumProposalRecv in view 5.
    // Our node should vote affirmatively on this.
    generator.add_transactions(vec![]);

    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    // We set the transactions to something not null for view 6, but we expect the node to emit a quorum proposal where they are still null.
    generator.add_transactions(vec![TestTransaction(vec![0])]);

    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    // For view 7, we set the transactions to something not null. The node should fail to vote on this.
    generator.add_transactions(vec![TestTransaction(vec![0])]);

    for view in generator.take(1) {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;
    let mut upgrade_state = UpgradeTaskState::<
        TestTypes,
        MemoryImpl,
        SystemContextHandle<TestTypes, MemoryImpl>,
    >::create_from(&handle)
    .await;

    upgrade_state.should_vote = |_| true;

    inject_consensus_polls(&consensus_state).await;

    let inputs = vec![
        vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            VidDisperseRecv(vids[0].0[0].clone()),
            DACertificateRecv(dacs[0].clone()),
        ],
        vec![
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            VidDisperseRecv(vids[1].0[0].clone()),
            DACertificateRecv(dacs[1].clone()),
            SendPayloadCommitmentAndMetadata(
                vids[1].0[0].data.payload_commitment,
                (),
                ViewNumber::new(2),
            ),
        ],
        vec![
            DACertificateRecv(dacs[2].clone()),
            VidDisperseRecv(vids[2].0[0].clone()),
            SendPayloadCommitmentAndMetadata(
                vids[2].0[0].data.payload_commitment,
                (),
                ViewNumber::new(3),
            ),
            QuorumProposalRecv(proposals[2].clone(), leaders[2]),
        ],
        vec![
            DACertificateRecv(dacs[3].clone()),
            VidDisperseRecv(vids[3].0[0].clone()),
            SendPayloadCommitmentAndMetadata(
                vids[3].0[0].data.payload_commitment,
                (),
                ViewNumber::new(4),
            ),
            QuorumProposalRecv(proposals[3].clone(), leaders[3]),
        ],
        vec![
            DACertificateRecv(dacs[4].clone()),
            VidDisperseRecv(vids[4].0[0].clone()),
            SendPayloadCommitmentAndMetadata(
                vids[4].0[0].data.payload_commitment,
                (),
                ViewNumber::new(5),
            ),
            QuorumProposalRecv(proposals[4].clone(), leaders[4]),
        ],
        vec![
            DACertificateRecv(dacs[5].clone()),
            SendPayloadCommitmentAndMetadata(
                vids[5].0[0].data.payload_commitment,
                (),
                ViewNumber::new(6),
            ),
            QCFormed(either::Either::Left(proposals[5].data.justify_qc.clone())),
        ],
        vec![
            DACertificateRecv(dacs[6].clone()),
            VidDisperseRecv(vids[6].0[0].clone()),
            SendPayloadCommitmentAndMetadata(
                vids[6].0[0].data.payload_commitment,
                (),
                ViewNumber::new(7),
            ),
            QuorumProposalRecv(proposals[6].clone(), leaders[6]),
        ],
    ];

    let consensus_script = TaskScript {
        state: consensus_state,
        expectations: vec![
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(1))),
                    quorum_proposal_validated(),
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(2))),
                    quorum_proposal_validated(),
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(3))),
                    quorum_proposal_validated(),
                    leaf_decided(),
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(4))),
                    quorum_proposal_validated(),
                    leaf_decided(),
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(5))),
                    quorum_proposal_validated(),
                    leaf_decided(),
                    // This is between versions, but we are receiving a null block and hence should vote affirmatively on it.
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![quorum_proposal_send_with_null_block(
                    quorum_membership.total_nodes(),
                )],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(7))),
                    // We do NOT expect a quorum_vote_send() because we have set the block to be non-null in this view.
                ],
                task_state_asserts: vec![],
            },
        ],
    };

    let upgrade_script = TaskScript {
        state: upgrade_state,
        expectations: vec![
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
        ],
    };

    test_scripts![inputs, consensus_script, upgrade_script];
}
