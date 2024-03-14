use hotshot::tasks::{inject_consensus_polls, task_state::CreateTaskState};
use hotshot::types::SystemContextHandle;
use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
use hotshot_macros::test_scripts;
use hotshot_task_impls::{
    consensus::ConsensusTaskState, events::HotShotEvent::*, upgrade::UpgradeTaskState,
};
use hotshot_testing::{
    predicates::*,
    script::{Expectations, TaskScript},
    view_generator::TestViewGenerator,
};
use hotshot_types::constants::Version;
use hotshot_types::{
    data::ViewNumber, simple_vote::UpgradeProposalData, traits::node_implementation::ConsensusTime,
};

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_upgrade_task() {
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
        new_version_hash: [0u8; 12].to_vec(),
        old_version_last_block: ViewNumber::new(5),
        new_version_first_block: ViewNumber::new(7),
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
            VidDisperseRecv(vids[0].0.clone(), vids[0].1),
            DACRecv(dacs[0].clone()),
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
            VidDisperseRecv(vids[1].0.clone(), vids[1].1),
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            DACRecv(dacs[1].clone()),
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
            DACRecv(dacs[2].clone()),
            VidDisperseRecv(vids[2].0.clone(), vids[2].1),
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
            DACRecv(dacs[3].clone()),
            VidDisperseRecv(vids[3].0.clone(), vids[3].1),
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
        new_version_hash: [0u8; 12].to_vec(),
        old_version_last_block: ViewNumber::new(5),
        new_version_first_block: ViewNumber::new(7),
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
            VidDisperseRecv(vids[0].0.clone(), vids[0].1),
            DACRecv(dacs[0].clone()),
        ],
        upgrade_vote_recvs,
        vec![QuorumProposalRecv(proposals[1].clone(), leaders[1])],
        vec![
            DACRecv(dacs[1].clone()),
            SendPayloadCommitmentAndMetadata(
                vids[2].0.data.payload_commitment,
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
