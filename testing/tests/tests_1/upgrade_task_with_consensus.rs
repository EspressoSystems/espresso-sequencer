#![cfg(not(feature = "dependency-tasks"))]
// TODO: Remove after integration of dependency-tasks
#![cfg(not(feature = "dependency-tasks"))]
#![allow(unused_imports)]

use std::time::Duration;

use futures::StreamExt;
use hotshot::{tasks::task_state::CreateTaskState, types::SystemContextHandle};
use hotshot_example_types::{
    block_types::{TestMetadata, TestTransaction},
    node_types::{MemoryImpl, TestTypes},
    state_types::TestInstanceState,
};
use hotshot_macros::test_scripts;
use hotshot_task_impls::{
    consensus::ConsensusTaskState, events::HotShotEvent::*, upgrade::UpgradeTaskState,
};
use hotshot_testing::{
    helpers::{build_fake_view_with_leaf, vid_share},
    predicates::{event::*, upgrade_with_consensus::*},
    script::{Expectations, TaskScript},
    view_generator::TestViewGenerator,
};
use hotshot_types::{
    constants::BaseVersion,
    data::{null_block, ViewNumber},
    simple_vote::UpgradeProposalData,
    traits::{election::Membership, node_implementation::ConsensusTime},
    vote::HasViewNumber,
};
use vbs::version::{StaticVersionType, Version};

#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
/// Tests that we correctly update our internal consensus state when reaching a decided upgrade certificate.
async fn test_upgrade_task_vote() {
    use hotshot_testing::helpers::build_system_handle;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(1).await.0;
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

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    for view in (&mut generator).take(2).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
    }

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
        vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            VidShareRecv(vid_share(&vids[0].0, handle.public_key())),
            DaCertificateRecv(dacs[0].clone()),
        ],
        vec![
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            VidShareRecv(vid_share(&vids[1].0, handle.public_key())),
            DaCertificateRecv(dacs[1].clone()),
        ],
        vec![
            QuorumProposalRecv(proposals[2].clone(), leaders[2]),
            DaCertificateRecv(dacs[2].clone()),
            VidShareRecv(vid_share(&vids[2].0, handle.public_key())),
        ],
        vec![
            QuorumProposalRecv(proposals[3].clone(), leaders[3]),
            DaCertificateRecv(dacs[3].clone()),
            VidShareRecv(vid_share(&vids[3].0, handle.public_key())),
        ],
        vec![
            QuorumProposalRecv(proposals[4].clone(), leaders[4]),
            DaCertificateRecv(dacs[4].clone()),
            VidShareRecv(vid_share(&vids[4].0, handle.public_key())),
        ],
        vec![QuorumProposalRecv(proposals[5].clone(), leaders[5])],
    ];

    let expectations = vec![
        Expectations {
            output_asserts: vec![
                exact(ViewChange(ViewNumber::new(1))),
                validated_state_updated(),
                quorum_proposal_validated(),
                exact(QuorumVoteSend(votes[0].clone())),
            ],
            task_state_asserts: vec![],
        },
        Expectations {
            output_asserts: vec![
                exact(ViewChange(ViewNumber::new(2))),
                validated_state_updated(),
                quorum_proposal_validated(),
                exact(QuorumVoteSend(votes[1].clone())),
            ],
            task_state_asserts: vec![no_decided_upgrade_certificate()],
        },
        Expectations {
            output_asserts: vec![
                exact(ViewChange(ViewNumber::new(3))),
                validated_state_updated(),
                quorum_proposal_validated(),
                exact(QuorumVoteSend(votes[2].clone())),
            ],
            task_state_asserts: vec![no_decided_upgrade_certificate()],
        },
        Expectations {
            output_asserts: vec![
                exact(ViewChange(ViewNumber::new(4))),
                validated_state_updated(),
                quorum_proposal_validated(),
                leaf_decided(),
                exact(QuorumVoteSend(votes[3].clone())),
            ],
            task_state_asserts: vec![no_decided_upgrade_certificate()],
        },
        Expectations {
            output_asserts: vec![
                exact(ViewChange(ViewNumber::new(5))),
                validated_state_updated(),
                quorum_proposal_validated(),
                leaf_decided(),
                exact(QuorumVoteSend(votes[4].clone())),
            ],
            task_state_asserts: vec![no_decided_upgrade_certificate()],
        },
        Expectations {
            output_asserts: vec![
                exact(ViewChange(ViewNumber::new(6))),
                validated_state_updated(),
                quorum_proposal_validated(),
                upgrade_decided(),
                leaf_decided(),
            ],
            task_state_asserts: vec![decided_upgrade_certificate()],
        },
    ];

    let consensus_state = ConsensusTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    let mut consensus_script = TaskScript {
        timeout: Duration::from_millis(65),
        state: consensus_state,
        expectations,
    };

    test_scripts![inputs, consensus_script].await;
}

#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
/// Test that we correctly form and include an `UpgradeCertificate` when receiving votes.
async fn test_upgrade_task_propose() {
    use std::sync::Arc;

    use hotshot_testing::helpers::build_system_handle;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(3).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

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

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    generator.add_upgrade(upgrade_data.clone());

    for view in generator.take(4).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    let upgrade_votes = other_handles
        .iter()
        .map(|h| views[2].create_upgrade_vote(upgrade_data.clone(), &h.0));

    let consensus_state = ConsensusTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    let upgrade_state = UpgradeTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let upgrade_vote_recvs: Vec<_> = upgrade_votes.map(UpgradeVoteRecv).collect();

    let inputs = vec![
        vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            VidShareRecv(vid_share(&vids[0].0, handle.public_key())),
            DaCertificateRecv(dacs[0].clone()),
        ],
        upgrade_vote_recvs,
        vec![
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            DaCertificateRecv(dacs[1].clone()),
            VidShareRecv(vid_share(&vids[1].0, handle.public_key())),
        ],
        vec![
            VidShareRecv(vid_share(&vids[2].0, handle.public_key())),
            SendPayloadCommitmentAndMetadata(
                vids[2].0[0].data.payload_commitment,
                proposals[2].data.block_header.builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(3),
                null_block::builder_fee(quorum_membership.total_nodes(), BaseVersion::version())
                    .unwrap(),
            ),
            QcFormed(either::Either::Left(proposals[2].data.justify_qc.clone())),
        ],
    ];

    let mut consensus_script = TaskScript {
        timeout: Duration::from_millis(35),
        state: consensus_state,
        expectations: vec![
            Expectations {
                output_asserts: vec![
                    exact::<TestTypes>(ViewChange(ViewNumber::new(1))),
                    validated_state_updated(),
                    quorum_proposal_validated::<TestTypes>(),
                    quorum_vote_send::<TestTypes>(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact::<TestTypes>(ViewChange(ViewNumber::new(2))),
                    validated_state_updated(),
                    quorum_proposal_validated::<TestTypes>(),
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![quorum_proposal_send_with_upgrade_certificate::<TestTypes>()],
                task_state_asserts: vec![],
            },
        ],
    };

    let mut upgrade_script = TaskScript {
        timeout: Duration::from_millis(35),
        state: upgrade_state,
        expectations: vec![
            Expectations {
                output_asserts: vec![],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![upgrade_certificate_formed::<TestTypes>()],
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

    test_scripts![inputs, consensus_script, upgrade_script].await;
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
async fn test_upgrade_task_blank_blocks() {
    use hotshot_testing::helpers::build_system_handle;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(6).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let old_version = Version { major: 0, minor: 1 };
    let new_version = Version { major: 0, minor: 2 };

    let builder_fee =
        null_block::builder_fee(quorum_membership.total_nodes(), BaseVersion::version()).unwrap();

    let upgrade_data: UpgradeProposalData<TestTypes> = UpgradeProposalData {
        old_version,
        new_version,
        decide_by: ViewNumber::new(7),
        new_version_hash: [0u8; 12].to_vec(),
        old_version_last_view: ViewNumber::new(6),
        new_version_first_view: ViewNumber::new(8),
    };

    let mut proposals = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    let mut leaders = Vec::new();
    let mut views = Vec::new();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone(), da_membership);

    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    generator.add_upgrade(upgrade_data.clone());

    for view in (&mut generator).take(3).collect::<Vec<_>>().await {
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

    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    // The transactions task generates an empty transaction set in this view,
    // because we are proposing between versions.
    generator.add_transactions(vec![]);

    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    // For view 7, we set the transactions to something not null. The node should fail to vote on this.
    generator.add_transactions(vec![TestTransaction::new(vec![0])]);

    for view in generator.take(1).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
        leaders.push(view.leader_public_key);
        views.push(view.clone());
    }

    let consensus_state = ConsensusTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    let upgrade_state = UpgradeTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;

    let inputs = vec![
        vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            VidShareRecv(vid_share(&vids[0].0, handle.public_key())),
            DaCertificateRecv(dacs[0].clone()),
        ],
        vec![
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            VidShareRecv(vid_share(&vids[1].0, handle.public_key())),
            DaCertificateRecv(dacs[1].clone()),
            SendPayloadCommitmentAndMetadata(
                vids[1].0[0].data.payload_commitment,
                proposals[1].data.block_header.builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(2),
                builder_fee.clone(),
            ),
        ],
        vec![
            DaCertificateRecv(dacs[2].clone()),
            VidShareRecv(vid_share(&vids[2].0, handle.public_key())),
            SendPayloadCommitmentAndMetadata(
                vids[2].0[0].data.payload_commitment,
                proposals[2].data.block_header.builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(3),
                builder_fee.clone(),
            ),
            QuorumProposalRecv(proposals[2].clone(), leaders[2]),
        ],
        vec![
            DaCertificateRecv(dacs[3].clone()),
            VidShareRecv(vid_share(&vids[3].0, handle.public_key())),
            SendPayloadCommitmentAndMetadata(
                vids[3].0[0].data.payload_commitment,
                proposals[3].data.block_header.builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(4),
                builder_fee.clone(),
            ),
            QuorumProposalRecv(proposals[3].clone(), leaders[3]),
        ],
        vec![
            DaCertificateRecv(dacs[4].clone()),
            VidShareRecv(vid_share(&vids[4].0, handle.public_key())),
            SendPayloadCommitmentAndMetadata(
                vids[4].0[0].data.payload_commitment,
                proposals[4].data.block_header.builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(5),
                builder_fee.clone(),
            ),
            QuorumProposalRecv(proposals[4].clone(), leaders[4]),
        ],
        vec![
            DaCertificateRecv(dacs[5].clone()),
            VidShareRecv(vid_share(&vids[5].0, handle.public_key())),
            SendPayloadCommitmentAndMetadata(
                vids[5].0[0].data.payload_commitment,
                proposals[5].data.block_header.builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(6),
                builder_fee.clone(),
            ),
            QuorumProposalRecv(proposals[5].clone(), leaders[5]),
            QcFormed(either::Either::Left(proposals[5].data.justify_qc.clone())),
        ],
        vec![
            DaCertificateRecv(dacs[6].clone()),
            VidShareRecv(vid_share(&vids[6].0, handle.public_key())),
            SendPayloadCommitmentAndMetadata(
                vids[6].0[0].data.payload_commitment,
                proposals[6].data.block_header.builder_commitment.clone(),
                TestMetadata,
                ViewNumber::new(7),
                builder_fee,
            ),
            QuorumProposalRecv(proposals[6].clone(), leaders[6]),
        ],
    ];

    let mut consensus_script = TaskScript {
        timeout: Duration::from_millis(35),
        state: consensus_state,
        expectations: vec![
            Expectations {
                output_asserts: vec![
                    exact::<TestTypes>(ViewChange(ViewNumber::new(1))),
                    validated_state_updated(),
                    quorum_proposal_validated(),
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(2))),
                    validated_state_updated(),
                    quorum_proposal_validated(),
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(3))),
                    validated_state_updated(),
                    quorum_proposal_validated(),
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(4))),
                    validated_state_updated(),
                    quorum_proposal_validated(),
                    leaf_decided(),
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(5))),
                    validated_state_updated(),
                    quorum_proposal_validated(),
                    upgrade_decided(),
                    leaf_decided(),
                    // This is between versions, but we are receiving a null block and hence should vote affirmatively on it.
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(6))),
                    validated_state_updated(),
                    quorum_proposal_validated(),
                    quorum_proposal_send_with_null_block(quorum_membership.total_nodes()),
                    leaf_decided(),
                    quorum_vote_send(),
                ],
                task_state_asserts: vec![],
            },
            Expectations {
                output_asserts: vec![
                    exact(ViewChange(ViewNumber::new(7))),
                    validated_state_updated(),
                    quorum_proposal_validated(),
                    leaf_decided(),
                    // We do NOT expect a quorum_vote_send() because we have set the block to be non-null in this view.
                ],
                task_state_asserts: vec![],
            },
        ],
    };

    let mut upgrade_script = TaskScript {
        timeout: Duration::from_millis(35),
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

    test_scripts![inputs, consensus_script, upgrade_script].await;
}
