#![cfg(not(feature = "dependency-tasks"))]
// TODO: Remove after integration of dependency-tasks
#![allow(unused_imports)]

use std::{sync::Arc, time::Duration};

use futures::StreamExt;
use hotshot::tasks::task_state::CreateTaskState;
use hotshot_example_types::{
    block_types::TestMetadata,
    node_types::{MemoryImpl, TestTypes},
    state_types::TestInstanceState,
};
use hotshot_macros::{run_test, test_scripts};
use hotshot_task_impls::{consensus::ConsensusTaskState, events::HotShotEvent::*};
use hotshot_testing::{
    all_predicates,
    helpers::{
        build_fake_view_with_leaf, build_system_handle, key_pair_for_id,
        permute_input_with_index_order, vid_scheme_from_view_number, vid_share,
    },
    predicates::event::{
        all_predicates, exact, quorum_proposal_send, quorum_proposal_validated, quorum_vote_send,
        timeout_vote_send, validated_state_updated,
    },
    random,
    script::{Expectations, InputOrder, TaskScript},
    serial,
    view_generator::TestViewGenerator,
};
use hotshot_types::{
    data::{null_block, ViewChangeEvidence, ViewNumber},
    simple_vote::{TimeoutData, TimeoutVote, ViewSyncFinalizeData},
    traits::{election::Membership, node_implementation::ConsensusTime},
    utils::BuilderCommitment,
    vote::HasViewNumber,
};
use jf_vid::VidScheme;
use sha2::Digest;
use vec1::vec1;

const TIMEOUT: Duration = Duration::from_millis(35);

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_task() {
    use hotshot_types::constants::BaseVersion;
    use vbs::version::StaticVersionType;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle::<TestTypes, MemoryImpl>(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    // Make some empty encoded transactions, we just care about having a commitment handy for the
    // later calls. We need the VID commitment to be able to propose later.
    let mut vid = vid_scheme_from_view_number::<TestTypes>(&quorum_membership, ViewNumber::new(2));
    let encoded_transactions = Vec::new();
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();
    let payload_commitment = vid_disperse.commit;

    let mut generator =
        TestViewGenerator::generate(quorum_membership.clone(), da_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(2).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    let cert = proposals[1].data.justify_qc.clone();
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    let inputs = vec![
        random![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DaCertificateRecv(dacs[0].clone()),
            VidShareRecv(vid_share(&vids[0].0, handle.public_key())),
        ],
        serial![
            VidShareRecv(vid_share(&vids[1].0, handle.public_key())),
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            QcFormed(either::Left(cert)),
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(2),
                vec1![null_block::builder_fee(
                    quorum_membership.total_nodes(),
                    BaseVersion::version()
                )
                .unwrap()],
                None,
            ),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(all_predicates![
            validated_state_updated(),
            exact(ViewChange(ViewNumber::new(1))),
            quorum_proposal_validated(),
            exact(QuorumVoteSend(votes[0].clone())),
        ]),
        Expectations::from_outputs(all_predicates![
            validated_state_updated(),
            exact(ViewChange(ViewNumber::new(2))),
            quorum_proposal_validated(),
            quorum_proposal_send(),
        ]),
    ];

    let consensus_state = ConsensusTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    let mut consensus_script = TaskScript {
        timeout: TIMEOUT,
        state: consensus_state,
        expectations,
    };

    run_test![inputs, consensus_script].await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_vote() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle::<TestTypes, MemoryImpl>(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator =
        TestViewGenerator::generate(quorum_membership.clone(), da_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut leaves = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(2).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        leaves.push(view.leaf.clone());
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Send a proposal, vote on said proposal, update view based on proposal QC, receive vote as next leader
    let inputs = vec![random![
        QuorumProposalRecv(proposals[0].clone(), leaders[0]),
        DaCertificateRecv(dacs[0].clone()),
        VidShareRecv(vid_share(&vids[0].0, handle.public_key())),
        QuorumVoteRecv(votes[0].clone()),
    ]];

    let expectations = vec![Expectations::from_outputs(all_predicates![
        validated_state_updated(),
        exact(ViewChange(ViewNumber::new(1))),
        quorum_proposal_validated(),
        exact(QuorumVoteSend(votes[0].clone())),
    ])];

    let consensus_state = ConsensusTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    let mut consensus_script = TaskScript {
        timeout: TIMEOUT,
        state: consensus_state,
        expectations,
    };

    run_test![inputs, consensus_script].await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_view_sync_finalize_propose() {
    use hotshot_example_types::{block_types::TestMetadata, state_types::TestValidatedState};
    use hotshot_types::{constants::BaseVersion, data::null_block};
    use vbs::version::StaticVersionType;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle::<TestTypes, MemoryImpl>(4).await.0;
    let (priv_key, pub_key) = key_pair_for_id(4);
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    // Make some empty encoded transactions, we just care about having a commitment handy for the
    // later calls. We need the VID commitment to be able to propose later.
    let mut vid = vid_scheme_from_view_number::<TestTypes>(&quorum_membership, ViewNumber::new(4));
    let encoded_transactions = Vec::new();
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();
    let payload_commitment = vid_disperse.commit;

    let view_sync_finalize_data: ViewSyncFinalizeData<TestTypes> = ViewSyncFinalizeData {
        relay: 4,
        round: ViewNumber::new(4),
    };

    let mut generator =
        TestViewGenerator::generate(quorum_membership.clone(), da_membership.clone());
    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut vids = Vec::new();
    let mut dacs = Vec::new();

    generator.next().await;
    let view = generator.current_view.clone().unwrap();
    proposals.push(view.quorum_proposal.clone());
    leaders.push(view.leader_public_key);
    votes.push(view.create_quorum_vote(&handle));
    vids.push(view.vid_proposal.clone());
    dacs.push(view.da_certificate.clone());

    // Skip two views
    generator.advance_view_number_by(2);

    // Initiate a view sync finalize
    generator.add_view_sync_finalize(view_sync_finalize_data);

    // Build the next proposal from view 1
    generator.next_from_anscestor_view(view.clone()).await;
    let view = generator.current_view.unwrap();
    proposals.push(view.quorum_proposal.clone());
    leaders.push(view.leader_public_key);
    votes.push(view.create_quorum_vote(&handle));
    vids.push(view.vid_proposal);

    // Handle the view sync finalize cert, get the requisite data, propose.
    let cert = match proposals[1].data.proposal_certificate.clone().unwrap() {
        ViewChangeEvidence::ViewSync(vsc) => vsc,
        _ => panic!("Found a TC when there should have been a view sync cert"),
    };

    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());
    let timeout_vote_view_2 = TimeoutVote::create_signed_vote(
        TimeoutData {
            view: ViewNumber::new(2),
        },
        ViewNumber::new(2),
        &pub_key,
        &priv_key,
    )
    .unwrap();

    let timeout_vote_view_3 = TimeoutVote::create_signed_vote(
        TimeoutData {
            view: ViewNumber::new(3),
        },
        ViewNumber::new(3),
        &pub_key,
        &priv_key,
    )
    .unwrap();

    let inputs = vec![
        serial![VidShareRecv(vid_share(&vids[0].0, handle.public_key()))],
        random![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DaCertificateRecv(dacs[0].clone()),
        ],
        serial![Timeout(ViewNumber::new(2)), Timeout(ViewNumber::new(3))],
        serial![VidShareRecv(vid_share(&vids[1].0, handle.public_key()))],
        random![
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            TimeoutVoteRecv(timeout_vote_view_2),
            TimeoutVoteRecv(timeout_vote_view_3),
            ViewSyncFinalizeCertificate2Recv(cert),
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(4),
                vec1![null_block::builder_fee(4, BaseVersion::version()).unwrap()],
                None,
            ),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(vec![]),
        Expectations::from_outputs(all_predicates![
            validated_state_updated(),
            exact(ViewChange(ViewNumber::new(1))),
            quorum_proposal_validated(),
            exact(QuorumVoteSend(votes[0].clone())),
        ]),
        Expectations::from_outputs(vec![timeout_vote_send(), timeout_vote_send()]),
        Expectations::from_outputs(vec![]),
        Expectations::from_outputs(all_predicates![
            validated_state_updated(),
            exact(ViewChange(ViewNumber::new(4))),
            quorum_proposal_validated(),
            quorum_proposal_send(),
        ]),
    ];

    let consensus_state = ConsensusTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    let mut consensus_script = TaskScript {
        timeout: TIMEOUT,
        state: consensus_state,
        expectations,
    };

    run_test![inputs, consensus_script].await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
/// Makes sure that, when a valid ViewSyncFinalize certificate is available, the consensus task
/// will indeed vote if the cert is valid and matches the correct view number.
async fn test_view_sync_finalize_vote() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle::<TestTypes, MemoryImpl>(5).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let view_sync_finalize_data: ViewSyncFinalizeData<TestTypes> = ViewSyncFinalizeData {
        relay: 4,
        round: ViewNumber::new(5),
    };

    let mut generator =
        TestViewGenerator::generate(quorum_membership.clone(), da_membership.clone());
    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut vids = Vec::new();
    let mut dacs = Vec::new();
    for view in (&mut generator).take(3).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        vids.push(view.vid_proposal.clone());
        dacs.push(view.da_certificate.clone());
    }

    // Each call to `take` moves us to the next generated view. We advance to view
    // 3 and then add the finalize cert for checking there.
    generator.add_view_sync_finalize(view_sync_finalize_data);
    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        vids.push(view.vid_proposal.clone());
        dacs.push(view.da_certificate.clone());
    }

    // When we're on the latest view. We want to set the quorum
    // certificate to be the previous highest QC (before the timeouts). This will be distinct from
    // the view sync cert, which is saying "hey, I'm _actually_ at view 4, but my highest QC is
    // only for view 1." This forces the QC to be for view 1, and we can move on under this
    // assumption.

    // Try to view sync at view 4.
    let cert = match proposals[3].data.proposal_certificate.clone().unwrap() {
        ViewChangeEvidence::ViewSync(vsc) => vsc,
        _ => panic!("Found a TC when there should have been a view sync cert"),
    };

    let inputs = vec![
        serial![VidShareRecv(vid_share(&vids[0].0, handle.public_key()))],
        random![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DaCertificateRecv(dacs[0].clone()),
        ],
        serial![Timeout(ViewNumber::new(2)), Timeout(ViewNumber::new(3))],
        random![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            ViewSyncFinalizeCertificate2Recv(cert),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(vec![]),
        Expectations::from_outputs(all_predicates![
            validated_state_updated(),
            exact(ViewChange(ViewNumber::new(1))),
            quorum_proposal_validated(),
            exact(QuorumVoteSend(votes[0].clone()))
        ]),
        Expectations::from_outputs(vec![timeout_vote_send(), timeout_vote_send()]),
        Expectations::from_outputs(all_predicates![
            validated_state_updated(),
            quorum_proposal_validated(),
            quorum_vote_send()
        ]),
    ];

    let consensus_state = ConsensusTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    let mut consensus_script = TaskScript {
        timeout: TIMEOUT,
        state: consensus_state,
        expectations,
    };

    run_test![inputs, consensus_script].await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
/// Makes sure that, when a valid ViewSyncFinalize certificate is available, the consensus task
/// will NOT vote when the certificate matches a different view number.
async fn test_view_sync_finalize_vote_fail_view_number() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle::<TestTypes, MemoryImpl>(5).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let view_sync_finalize_data: ViewSyncFinalizeData<TestTypes> = ViewSyncFinalizeData {
        relay: 4,
        round: ViewNumber::new(10),
    };

    let mut generator =
        TestViewGenerator::generate(quorum_membership.clone(), da_membership.clone());
    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut vids = Vec::new();
    let mut dacs = Vec::new();
    for view in (&mut generator).take(3).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        vids.push(view.vid_proposal.clone());
        dacs.push(view.da_certificate.clone());
    }

    // Each call to `take` moves us to the next generated view. We advance to view
    // 3 and then add the finalize cert for checking there.
    generator.add_view_sync_finalize(view_sync_finalize_data);
    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        vids.push(view.vid_proposal.clone());
        dacs.push(view.da_certificate.clone());
    }

    // When we're on the latest view. We want to set the quorum
    // certificate to be the previous highest QC (before the timeouts). This will be distinct from
    // the view sync cert, which is saying "hey, I'm _actually_ at view 4, but my highest QC is
    // only for view 1." This forces the QC to be for view 1, and we can move on under this
    // assumption.

    let mut cert = match proposals[3].data.proposal_certificate.clone().unwrap() {
        ViewChangeEvidence::ViewSync(vsc) => vsc,
        _ => panic!("Found a TC when there should have been a view sync cert"),
    };

    // Force this to fail by making the cert happen for a view we've never seen. This will
    // intentionally skip the proposal for this node so we can get the proposal and fail to vote.
    cert.view_number = ViewNumber::new(10);

    // Get a good proposal first.
    let good_proposal = proposals[0].clone();

    // Now We introduce an error by setting a different view number as well, this makes the task check
    // for a view sync or timeout cert. This value could be anything as long as it is not the
    // previous view number.
    proposals[0].data.justify_qc.view_number = proposals[3].data.justify_qc.view_number;

    let inputs = vec![
        random![
            QuorumProposalRecv(good_proposal, leaders[0]),
            DaCertificateRecv(dacs[0].clone()),
        ],
        serial![VidShareRecv(vid_share(&vids[0].0, handle.public_key()))],
        serial![Timeout(ViewNumber::new(2)), Timeout(ViewNumber::new(3))],
        random![
            ViewSyncFinalizeCertificate2Recv(cert),
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
        ],
    ];

    let expectations = vec![
        Expectations::from_outputs(all_predicates![
            quorum_proposal_validated(),
            validated_state_updated(),
            exact(ViewChange(ViewNumber::new(1))),
        ]),
        Expectations::from_outputs(vec![exact(QuorumVoteSend(votes[0].clone()))]),
        Expectations::from_outputs(vec![timeout_vote_send(), timeout_vote_send()]),
        // We get no output here due to the invalid view number.
        Expectations::from_outputs(vec![]),
    ];

    let consensus_state = ConsensusTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    let mut consensus_script = TaskScript {
        timeout: TIMEOUT,
        state: consensus_state,
        expectations,
    };

    run_test![inputs, consensus_script].await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_vid_disperse_storage_failure() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle::<TestTypes, MemoryImpl>(2).await.0;

    // Set the error flag here for the system handle. This causes it to emit an error on append.
    handle.storage().write().await.should_return_err = true;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let da_membership = handle.hotshot.memberships.da_membership.clone();

    let mut generator =
        TestViewGenerator::generate(quorum_membership.clone(), da_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(1).collect::<Vec<_>>().await {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    let inputs = vec![random![
        QuorumProposalRecv(proposals[0].clone(), leaders[0]),
        DaCertificateRecv(dacs[0].clone()),
        VidShareRecv(vid_share(&vids[0].0, handle.public_key())),
    ]];

    let expectations = vec![Expectations::from_outputs(all_predicates![
        validated_state_updated(),
        exact(ViewChange(ViewNumber::new(1))),
        quorum_proposal_validated(),
    ])];

    let consensus_state = ConsensusTaskState::<TestTypes, MemoryImpl>::create_from(&handle).await;
    let mut consensus_script = TaskScript {
        timeout: TIMEOUT,
        state: consensus_state,
        expectations,
    };

    run_test![inputs, consensus_script].await;
}
