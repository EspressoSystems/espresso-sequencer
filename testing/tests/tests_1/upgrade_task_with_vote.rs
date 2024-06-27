#![cfg(feature = "dependency-tasks")]

// TODO: Remove after integration of dependency-tasks
#![allow(unused_imports)]

use std::time::Duration;

use futures::StreamExt;
use hotshot::{tasks::task_state::CreateTaskState, types::SystemContextHandle};
use hotshot_example_types::{
    block_types::{TestMetadata, TestTransaction},
    node_types::{MemoryImpl, TestTypes},
    state_types::TestInstanceState,
};
use hotshot_macros::{run_test, test_scripts};
use hotshot_task_impls::{
    consensus::ConsensusTaskState, consensus2::Consensus2TaskState, events::HotShotEvent::*, quorum_vote::QuorumVoteTaskState, upgrade::UpgradeTaskState
};
use hotshot_testing::{
    helpers::{build_fake_view_with_leaf, vid_share},
    predicates::{event::*, upgrade_with_vote::*},
    script::{Expectations, TaskScript, InputOrder},
    view_generator::TestViewGenerator,
    random, all_predicates
};
use hotshot_types::{
    data::{null_block, ViewNumber},
    simple_vote::UpgradeProposalData,
    traits::{election::Membership, node_implementation::ConsensusTime},
    vote::HasViewNumber,
};
use vbs::version::Version;

const TIMEOUT: Duration = Duration::from_millis(65);

#[cfg(feature = "dependency-tasks")]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
/// Tests that we correctly update our internal quorum vote state when reaching a decided upgrade
/// certificate.
async fn test_upgrade_task_with_vote() {
    use hotshot_testing::helpers::build_system_handle;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let old_version = Version { major: 0, minor: 1 };
    let new_version = Version { major: 0, minor: 2 };

    let upgrade_data: UpgradeProposalData<TestTypes> = UpgradeProposalData {
        old_version,
        new_version,
        decide_by: ViewNumber::new(6),
        new_version_hash: [0u8; 12].to_vec(),
        old_version_last_view: ViewNumber::new(6),
        new_version_first_view: ViewNumber::new(7),
    };

    let mut proposals = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    let consensus = handle.hotshot.consensus().clone();
    let mut consensus_writer = consensus.write().await;

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);
    for view in (&mut generator).take(2).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        consensus_writer.update_validated_state_map(
            view.quorum_proposal.data.view_number(),
            build_fake_view_with_leaf(view.leaf.clone()),
        ).unwrap();
        consensus_writer.update_saved_leaves(view.leaf.clone());
    }
    drop(consensus_writer);

    generator.add_upgrade(upgrade_data);

    for view in generator.take(4).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
    }

    let inputs = vec![
        random![
            QuorumProposalValidated(proposals[1].data.clone(), leaves[0].clone()),
            DaCertificateRecv(dacs[1].clone()),
            VidShareRecv(vids[1].0[0].clone()),
        ],
        random![
            QuorumProposalValidated(proposals[2].data.clone(), leaves[1].clone()),
            DaCertificateRecv(dacs[2].clone()),
            VidShareRecv(vids[2].0[0].clone()),
        ],
        random![
            QuorumProposalValidated(proposals[3].data.clone(), leaves[2].clone()),
            DaCertificateRecv(dacs[3].clone()),
            VidShareRecv(vids[3].0[0].clone()),
        ],
        random![
            QuorumProposalValidated(proposals[4].data.clone(), leaves[3].clone()),
            DaCertificateRecv(dacs[4].clone()),
            VidShareRecv(vids[4].0[0].clone()),
        ],
        random![
            QuorumProposalValidated(proposals[5].data.clone(), leaves[5].clone()),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(all_predicates![
            exact(DaCertificateValidated(dacs[1].clone())),
            exact(VidShareValidated(vids[1].0[0].clone())),
            exact(QuorumVoteDependenciesValidated(ViewNumber::new(2))),
            validated_state_updated(),
            quorum_vote_send(),
        ]),
        Expectations::from_outputs_and_task_states(
            all_predicates![
                exact(LockedViewUpdated(ViewNumber::new(1))),
                exact(DaCertificateValidated(dacs[2].clone())),
                exact(VidShareValidated(vids[2].0[0].clone())),
                exact(QuorumVoteDependenciesValidated(ViewNumber::new(3))),
                validated_state_updated(),
                quorum_vote_send(),
            ],
            vec![no_decided_upgrade_cert()],
        ),
        Expectations::from_outputs_and_task_states(
            all_predicates![
                exact(LockedViewUpdated(ViewNumber::new(2))),
                exact(LastDecidedViewUpdated(ViewNumber::new(1))),
                leaf_decided(),
                exact(DaCertificateValidated(dacs[3].clone())),
                exact(VidShareValidated(vids[3].0[0].clone())),
                exact(QuorumVoteDependenciesValidated(ViewNumber::new(4))),
                validated_state_updated(),
                quorum_vote_send(),
            ],
            vec![no_decided_upgrade_cert()],
        ),
        Expectations::from_outputs_and_task_states(
            all_predicates![
                exact(LockedViewUpdated(ViewNumber::new(3))),
                exact(LastDecidedViewUpdated(ViewNumber::new(2))),
                leaf_decided(),
                exact(DaCertificateValidated(dacs[4].clone())),
                exact(VidShareValidated(vids[4].0[0].clone())),
                exact(QuorumVoteDependenciesValidated(ViewNumber::new(5))),
                validated_state_updated(),
                quorum_vote_send(),
            ],
            vec![no_decided_upgrade_cert()],
        ),
        Expectations::from_outputs_and_task_states(
            all_predicates![
                upgrade_decided(),
                exact(LockedViewUpdated(ViewNumber::new(4))),
                exact(LastDecidedViewUpdated(ViewNumber::new(3))),
                leaf_decided(),
            ],
            vec![decided_upgrade_cert()],
        ),
    ];

    let vote_state = QuorumVoteTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    let mut vote_script = TaskScript {
        timeout: TIMEOUT,
        state: vote_state,
        expectations,
    };


    run_test![inputs, vote_script].await;
}
