use hotshot::{
    tasks::{inject_consensus_polls, task_state::CreateTaskState},
};
use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
use hotshot_task_impls::{consensus::ConsensusTaskState, events::HotShotEvent::*};
use hotshot_testing::{
    predicates::event::{
        exact, quorum_proposal_send, quorum_proposal_validated, quorum_vote_send, timeout_vote_send,
    },
    script::{run_test_script, TestScriptStage},
    task_helpers::{
        build_system_handle, get_vid_share, key_pair_for_id, vid_scheme_from_view_number,
    },
    test_helpers::permute_input_with_index_order,
    view_generator::TestViewGenerator,
};
use hotshot_types::{
    data::{ViewChangeEvidence, ViewNumber},
    simple_vote::{TimeoutData, TimeoutVote, ViewSyncFinalizeData},
    traits::{election::Membership, node_implementation::ConsensusTime},
    utils::BuilderCommitment,
};
use jf_primitives::vid::VidScheme;
use sha2::Digest;

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_task() {
    use hotshot_example_types::block_types::TestMetadata;
    use hotshot_types::data::null_block;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    // Make some empty encoded transactions, we just care about having a commitment handy for the
    // later calls. We need the VID commitment to be able to propose later.
    let mut vid = vid_scheme_from_view_number::<TestTypes>(&quorum_membership, ViewNumber::new(2));
    let encoded_transactions = Vec::new();
    let vid_disperse = vid.disperse(&encoded_transactions).unwrap();
    let payload_commitment = vid_disperse.commit;

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Run view 1 (the genesis stage).
    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DACertificateRecv(dacs[0].clone()),
            VIDShareRecv(get_vid_share(&vids[0].0, handle.get_public_key())),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(1))),
            quorum_proposal_validated(),
            exact(QuorumVoteSend(votes[0].clone())),
        ],
        asserts: vec![],
    };

    let cert = proposals[1].data.justify_qc.clone();
    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());

    // Run view 2 and propose.
    let view_2 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            QCFormed(either::Left(cert)),
            // We must have a payload commitment and metadata to propose.
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(2),
                null_block::builder_fee(quorum_membership.total_nodes()).unwrap(),
            ),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(2))),
            quorum_proposal_validated(),
            quorum_proposal_send(),
        ],
        asserts: vec![],
    };

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
    >::create_from(&handle)
    .await;

    inject_consensus_polls(&consensus_state).await;

    run_test_script(vec![view_1, view_2], consensus_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_vote() {
    use hotshot::tasks::{inject_consensus_polls, task_state::CreateTaskState};
    use hotshot_task_impls::{consensus::ConsensusTaskState, events::HotShotEvent::*};
    use hotshot_testing::{
        script::{run_test_script, TestScriptStage},
        task_helpers::build_system_handle,
        view_generator::TestViewGenerator,
    };

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Send a proposal, vote on said proposal, update view based on proposal QC, receive vote as next leader
    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DACertificateRecv(dacs[0].clone()),
            VIDShareRecv(get_vid_share(&vids[0].0, handle.get_public_key())),
            QuorumVoteRecv(votes[0].clone()),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(1))),
            quorum_proposal_validated(),
            exact(QuorumVoteSend(votes[0].clone())),
        ],
        asserts: vec![],
    };

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
    >::create_from(&handle)
    .await;

    inject_consensus_polls(&consensus_state).await;
    run_test_script(vec![view_1], consensus_state).await;
}

/// Tests the voting behavior by allowing the input to be permuted in any order desired. This
/// assures that, no matter what, a vote is indeed sent no matter what order the precipitating
/// events occur. The permutation is specified as `input_permutation` and is a vector of indices.
async fn test_vote_with_specific_order(input_permutation: Vec<usize>) {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(2) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Get out of the genesis view first
    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DACertificateRecv(dacs[0].clone()),
            VIDShareRecv(get_vid_share(&vids[0].0, handle.get_public_key())),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(1))),
            quorum_proposal_validated(),
            exact(QuorumVoteSend(votes[0].clone())),
        ],
        asserts: vec![],
    };

    let inputs = vec![
        // We need a VID share for view 2 otherwise we cannot vote at view 2 (as node 2).
        VIDShareRecv(get_vid_share(&vids[1].0, handle.get_public_key())),
        DACertificateRecv(dacs[1].clone()),
        QuorumProposalRecv(proposals[1].clone(), leaders[1]),
    ];
    let view_2_inputs = permute_input_with_index_order(inputs, input_permutation);

    // Use the permuted inputs for view 2 depending on the provided index ordering.
    let view_2 = TestScriptStage {
        inputs: view_2_inputs,
        outputs: vec![
            exact(ViewChange(ViewNumber::new(2))),
            quorum_proposal_validated(),
            exact(QuorumVoteSend(votes[1].clone())),
        ],
        asserts: vec![],
    };

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
    >::create_from(&handle)
    .await;

    inject_consensus_polls(&consensus_state).await;
    run_test_script(vec![view_1, view_2], consensus_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_consensus_vote_with_permuted_dac() {
    // These tests verify that a vote is indeed sent no matter when it receives a DACertificateRecv
    // event. In particular, we want to verify that receiving events in an unexpected (but still
    // valid) order allows the system to proceed as it normally would.
    test_vote_with_specific_order(vec![0, 1, 2]).await;
    test_vote_with_specific_order(vec![0, 2, 1]).await;
    test_vote_with_specific_order(vec![1, 0, 2]).await;
    test_vote_with_specific_order(vec![2, 0, 1]).await;
    test_vote_with_specific_order(vec![1, 2, 0]).await;
    test_vote_with_specific_order(vec![2, 1, 0]).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_view_sync_finalize_propose() {
    use hotshot_example_types::block_types::TestMetadata;
    use hotshot_types::data::null_block;

    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(4).await.0;
    let (priv_key, pub_key) = key_pair_for_id(4);
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
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

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());
    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut vids = Vec::new();
    let mut dacs = Vec::new();

    generator.next();
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
    generator.next_from_anscestor_view(view.clone());
    let view = generator.current_view.unwrap();
    proposals.push(view.quorum_proposal.clone());
    leaders.push(view.leader_public_key);
    votes.push(view.create_quorum_vote(&handle));

    // This is a bog standard view and covers the situation where everything is going normally.
    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DACertificateRecv(dacs[0].clone()),
            VIDShareRecv(get_vid_share(&vids[0].0, handle.get_public_key())),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(1))),
            quorum_proposal_validated(),
            exact(QuorumVoteSend(votes[0].clone())),
        ],
        asserts: vec![],
    };

    // Fail twice here to "trigger" a view sync event. This is accomplished above by advancing the
    // view number in the generator.
    let view_2_3 = TestScriptStage {
        inputs: vec![Timeout(ViewNumber::new(2)), Timeout(ViewNumber::new(3))],
        outputs: vec![timeout_vote_send(), timeout_vote_send()],
        // Times out, so we now have a delayed view
        asserts: vec![],
    };

    // Handle the view sync finalize cert, get the requisite data, propose.
    let cert = match proposals[1].data.proposal_certificate.clone().unwrap() {
        ViewChangeEvidence::ViewSync(vsc) => vsc,
        _ => panic!("Found a TC when there should have been a view sync cert"),
    };

    // Generate the timeout votes for the timeouts that just occurred.
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

    let builder_commitment = BuilderCommitment::from_raw_digest(sha2::Sha256::new().finalize());
    let view_4 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[1].clone(), leaders[1]),
            TimeoutVoteRecv(timeout_vote_view_2),
            TimeoutVoteRecv(timeout_vote_view_3),
            ViewSyncFinalizeCertificate2Recv(cert),
            SendPayloadCommitmentAndMetadata(
                payload_commitment,
                builder_commitment,
                TestMetadata,
                ViewNumber::new(4),
                null_block::builder_fee(4).unwrap(),
            ),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(4))),
            quorum_proposal_validated(),
            quorum_proposal_send(),
        ],
        asserts: vec![],
    };

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
    >::create_from(&handle)
    .await;

    let stages = vec![view_1, view_2_3, view_4];

    inject_consensus_polls(&consensus_state).await;
    run_test_script(stages, consensus_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
/// Makes sure that, when a valid ViewSyncFinalize certificate is available, the consensus task
/// will indeed vote if the cert is valid and matches the correct view number.
async fn test_view_sync_finalize_vote() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(5).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let view_sync_finalize_data: ViewSyncFinalizeData<TestTypes> = ViewSyncFinalizeData {
        relay: 4,
        round: ViewNumber::new(5),
    };

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());
    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut vids = Vec::new();
    let mut dacs = Vec::new();
    for view in (&mut generator).take(3) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        vids.push(view.vid_proposal.clone());
        dacs.push(view.da_certificate.clone());
    }

    // Each call to `take` moves us to the next generated view. We advance to view
    // 3 and then add the finalize cert for checking there.
    generator.add_view_sync_finalize(view_sync_finalize_data);
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        vids.push(view.vid_proposal.clone());
        dacs.push(view.da_certificate.clone());
    }

    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DACertificateRecv(dacs[0].clone()),
            VIDShareRecv(get_vid_share(&vids[0].0, handle.get_public_key())),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(1))),
            quorum_proposal_validated(),
            exact(QuorumVoteSend(votes[0].clone())),
        ],
        asserts: vec![],
    };

    let view_2 = TestScriptStage {
        inputs: vec![Timeout(ViewNumber::new(2)), Timeout(ViewNumber::new(3))],
        outputs: vec![timeout_vote_send(), timeout_vote_send()],
        // Times out, so we now have a delayed view
        asserts: vec![],
    };

    // Now we're on the latest view. We want to set the quorum
    // certificate to be the previous highest QC (before the timeouts). This will be distinct from
    // the view sync cert, which is saying "hey, I'm _actually_ at view 4, but my highest QC is
    // only for view 1." This forces the QC to be for view 1, and we can move on under this
    // assumption.

    // Try to view sync at view 4.
    let cert = match proposals[3].data.proposal_certificate.clone().unwrap() {
        ViewChangeEvidence::ViewSync(vsc) => vsc,
        _ => panic!("Found a TC when there should have been a view sync cert"),
    };

    // Now at view 3 we receive the proposal received response.
    let view_3 = TestScriptStage {
        inputs: vec![
            // Receive a proposal for view 4, but with the highest qc being from view 1.
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            // Multiple timeouts in a row, so we call for a view sync
            ViewSyncFinalizeCertificate2Recv(cert),
        ],
        outputs: vec![quorum_proposal_validated(), quorum_vote_send()],
        asserts: vec![],
    };

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
    >::create_from(&handle)
    .await;

    let stages = vec![view_1, view_2, view_3];

    inject_consensus_polls(&consensus_state).await;
    run_test_script(stages, consensus_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
/// Makes sure that, when a valid ViewSyncFinalize certificate is available, the consensus task
/// will NOT vote when the certificate matches a different view number.
async fn test_view_sync_finalize_vote_fail_view_number() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(5).await.0;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();

    let view_sync_finalize_data: ViewSyncFinalizeData<TestTypes> = ViewSyncFinalizeData {
        relay: 4,
        round: ViewNumber::new(10),
    };

    let mut generator = TestViewGenerator::generate(quorum_membership.clone());
    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut vids = Vec::new();
    let mut dacs = Vec::new();
    for view in (&mut generator).take(3) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        vids.push(view.vid_proposal.clone());
        dacs.push(view.da_certificate.clone());
    }

    // Each call to `take` moves us to the next generated view. We advance to view
    // 3 and then add the finalize cert for checking there.
    generator.add_view_sync_finalize(view_sync_finalize_data);
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        vids.push(view.vid_proposal.clone());
        dacs.push(view.da_certificate.clone());
    }

    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DACertificateRecv(dacs[0].clone()),
            VIDShareRecv(get_vid_share(&vids[0].0, handle.get_public_key())),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(1))),
            quorum_proposal_validated(),
            exact(QuorumVoteSend(votes[0].clone())),
        ],
        asserts: vec![],
    };

    let view_2 = TestScriptStage {
        inputs: vec![Timeout(ViewNumber::new(2)), Timeout(ViewNumber::new(3))],
        outputs: vec![timeout_vote_send(), timeout_vote_send()],
        // Times out, so we now have a delayed view
        asserts: vec![],
    };

    // Now we're on the latest view. We want to set the quorum
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

    // We introduce an error by setting a different view number as well, this makes the task check
    // for a view sync or timeout cert. This value could be anything as long as it is not the
    // previous view number.
    proposals[0].data.justify_qc.view_number = proposals[3].data.justify_qc.view_number;

    // Now at view 3 we receive the proposal received response.
    let view_3 = TestScriptStage {
        inputs: vec![
            // Multiple timeouts in a row, so we call for a view sync
            ViewSyncFinalizeCertificate2Recv(cert),
            // Receive a proposal for view 4, but with the highest qc being from view 1.
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
        ],
        outputs: vec![
            /* No outputs make it through. We never got a valid proposal, so we never vote */
        ],
        asserts: vec![],
    };

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
    >::create_from(&handle)
    .await;

    let stages = vec![view_1, view_2, view_3];

    inject_consensus_polls(&consensus_state).await;
    run_test_script(stages, consensus_state).await;
}

#[cfg(test)]
#[cfg_attr(async_executor_impl = "tokio", tokio::test(flavor = "multi_thread"))]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_vid_disperse_storage_failure() {
    async_compatibility_layer::logging::setup_logging();
    async_compatibility_layer::logging::setup_backtrace();

    let handle = build_system_handle(2).await.0;

    // Set the error flag here for the system handle. This causes it to emit an error on append.
    handle.get_storage().write().await.should_return_err = true;
    let quorum_membership = handle.hotshot.memberships.quorum_membership.clone();
    let mut generator = TestViewGenerator::generate(quorum_membership.clone());

    let mut proposals = Vec::new();
    let mut leaders = Vec::new();
    let mut votes = Vec::new();
    let mut dacs = Vec::new();
    let mut vids = Vec::new();
    for view in (&mut generator).take(1) {
        proposals.push(view.quorum_proposal.clone());
        leaders.push(view.leader_public_key);
        votes.push(view.create_quorum_vote(&handle));
        dacs.push(view.da_certificate.clone());
        vids.push(view.vid_proposal.clone());
    }

    // Run view 1 (the genesis stage).
    let view_1 = TestScriptStage {
        inputs: vec![
            QuorumProposalRecv(proposals[0].clone(), leaders[0]),
            DACertificateRecv(dacs[0].clone()),
            VIDShareRecv(get_vid_share(&vids[0].0, handle.get_public_key())),
        ],
        outputs: vec![
            exact(ViewChange(ViewNumber::new(1))),
            quorum_proposal_validated(),
            /* Does not vote */
        ],
        asserts: vec![],
    };

    let consensus_state = ConsensusTaskState::<
        TestTypes,
        MemoryImpl,
    >::create_from(&handle)
    .await;

    inject_consensus_polls(&consensus_state).await;

    run_test_script(vec![view_1], consensus_state).await;
}
