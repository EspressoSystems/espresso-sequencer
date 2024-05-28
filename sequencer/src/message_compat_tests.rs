#![cfg(test)]
//! Serialization compatibility tests for consensus messages.
//!
//! This test generates a test vector containing one variant of each type of consensus message,
//! instantiated with sequencer types. A serialized version of this vector is written to a file and
//! checked into the repo under `data/messages.json`. If the serialization of the generated test
//! vector does not match the committed file, the test fails, indicating a potential API
//! incompatibility.
//!
//! If this test fails and you intended to change the consensus API, you may simply replace the
//! serialized file as indicated in the test output. Note however that this may break compatibility
//! with older releases, and your pull request should explain why this is OK.
//!
//! If this test is failing and you did not intend to change the consensus API, figure out what
//! code changed caused the serialization change and revert it.

use crate::{Leaf, NodeState, Payload, PubKey, SeqTypes, Transaction, ValidatedState};
use async_compatibility_layer::art::async_test;
use committable::Committable;
use es_version::SequencerVersion;
use hotshot::traits::election::static_committee::GeneralStaticCommittee;
use hotshot_types::{
    data::{
        DaProposal, QuorumProposal, UpgradeProposal, VidDisperse, VidDisperseShare,
        ViewChangeEvidence, ViewNumber,
    },
    message::{
        DaConsensusMessage, DataMessage, GeneralConsensusMessage, Message, MessageKind, Proposal,
        SequencingMessage,
    },
    simple_certificate::{
        DaCertificate, QuorumCertificate, TimeoutCertificate, UpgradeCertificate,
        ViewSyncCommitCertificate2, ViewSyncFinalizeCertificate2, ViewSyncPreCommitCertificate2,
    },
    simple_vote::{
        DaData, DaVote, QuorumData, QuorumVote, TimeoutData, TimeoutVote, UpgradeProposalData,
        UpgradeVote, ViewSyncCommitData, ViewSyncCommitVote, ViewSyncFinalizeData,
        ViewSyncFinalizeVote, ViewSyncPreCommitData, ViewSyncPreCommitVote,
    },
    traits::{
        node_implementation::ConsensusTime, signature_key::SignatureKey, BlockPayload, EncodeBytes,
    },
    vid::vid_scheme,
};
use jf_vid::VidScheme;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::path::Path;
use vbs::{version::Version, BinarySerializer};

type Serializer = vbs::Serializer<SequencerVersion>;

#[async_test]
async fn test_message_compat() {
    let (sender, priv_key) = PubKey::generated_from_seed_indexed(Default::default(), 0);
    let signature = PubKey::sign(&priv_key, &[]).unwrap();
    let membership = GeneralStaticCommittee::new(&[], vec![sender.stake_table_entry(1)], vec![], 0);
    let upgrade_data = UpgradeProposalData {
        old_version: Version { major: 0, minor: 1 },
        new_version: Version { major: 1, minor: 0 },
        decide_by: ViewNumber::genesis(),
        new_version_hash: Default::default(),
        old_version_last_view: ViewNumber::genesis(),
        new_version_first_view: ViewNumber::genesis(),
    };
    let leaf = Leaf::genesis(&ValidatedState::default(), &NodeState::mock()).await;
    let block_header = leaf.block_header().clone();
    let transaction = Transaction::new(1.into(), vec![1, 2, 3]);
    let (payload, metadata) = Payload::from_transactions(
        [transaction.clone()],
        &ValidatedState::default(),
        &NodeState::mock(),
    )
    .await
    .unwrap();
    let view_sync_pre_commit_data = ViewSyncPreCommitData {
        relay: 0,
        round: ViewNumber::genesis(),
    };
    let view_sync_commit_data = ViewSyncCommitData {
        relay: 0,
        round: ViewNumber::genesis(),
    };
    let view_sync_finalize_data = ViewSyncFinalizeData {
        relay: 0,
        round: ViewNumber::genesis(),
    };
    let timeout_data = TimeoutData {
        view: ViewNumber::genesis(),
    };
    let da_data = DaData {
        payload_commit: block_header.payload_commitment,
    };

    let consensus_messages = [
        GeneralConsensusMessage::Proposal(Proposal {
            data: QuorumProposal {
                block_header,
                view_number: ViewNumber::genesis(),
                justify_qc: QuorumCertificate::genesis(
                    &ValidatedState::default(),
                    &NodeState::mock(),
                )
                .await,
                upgrade_certificate: Some(UpgradeCertificate {
                    data: upgrade_data.clone(),
                    vote_commitment: upgrade_data.commit(),
                    view_number: ViewNumber::genesis(),
                    signatures: Default::default(),
                    _pd: Default::default(),
                }),
                proposal_certificate: Some(ViewChangeEvidence::Timeout(TimeoutCertificate {
                    data: timeout_data.clone(),
                    vote_commitment: timeout_data.commit(),
                    view_number: ViewNumber::genesis(),
                    signatures: Default::default(),
                    _pd: Default::default(),
                })),
            },
            signature: signature.clone(),
            _pd: Default::default(),
        }),
        GeneralConsensusMessage::Vote(QuorumVote {
            signature: (sender, signature.clone()),
            data: QuorumData {
                leaf_commit: leaf.commit(),
            },
            view_number: ViewNumber::genesis(),
        }),
        GeneralConsensusMessage::ViewSyncPreCommitVote(ViewSyncPreCommitVote {
            signature: (sender, signature.clone()),
            data: view_sync_pre_commit_data.clone(),
            view_number: ViewNumber::genesis(),
        }),
        GeneralConsensusMessage::ViewSyncCommitVote(ViewSyncCommitVote {
            signature: (sender, signature.clone()),
            data: view_sync_commit_data.clone(),
            view_number: ViewNumber::genesis(),
        }),
        GeneralConsensusMessage::ViewSyncFinalizeVote(ViewSyncFinalizeVote {
            signature: (sender, signature.clone()),
            data: view_sync_finalize_data.clone(),
            view_number: ViewNumber::genesis(),
        }),
        GeneralConsensusMessage::ViewSyncPreCommitCertificate(ViewSyncPreCommitCertificate2 {
            data: view_sync_pre_commit_data.clone(),
            vote_commitment: view_sync_pre_commit_data.commit(),
            view_number: ViewNumber::genesis(),
            signatures: Default::default(),
            _pd: Default::default(),
        }),
        GeneralConsensusMessage::ViewSyncCommitCertificate(ViewSyncCommitCertificate2 {
            data: view_sync_commit_data.clone(),
            vote_commitment: view_sync_commit_data.commit(),
            view_number: ViewNumber::genesis(),
            signatures: Default::default(),
            _pd: Default::default(),
        }),
        GeneralConsensusMessage::ViewSyncFinalizeCertificate(ViewSyncFinalizeCertificate2 {
            data: view_sync_finalize_data.clone(),
            vote_commitment: view_sync_finalize_data.commit(),
            view_number: ViewNumber::genesis(),
            signatures: Default::default(),
            _pd: Default::default(),
        }),
        GeneralConsensusMessage::TimeoutVote(TimeoutVote {
            signature: (sender, signature.clone()),
            data: timeout_data,
            view_number: ViewNumber::genesis(),
        }),
        GeneralConsensusMessage::UpgradeProposal(Proposal {
            data: UpgradeProposal {
                upgrade_proposal: upgrade_data.clone(),
                view_number: ViewNumber::genesis(),
            },
            signature: signature.clone(),
            _pd: Default::default(),
        }),
        GeneralConsensusMessage::UpgradeVote(UpgradeVote {
            signature: (sender, signature.clone()),
            data: upgrade_data,
            view_number: ViewNumber::genesis(),
        }),
    ];
    let da_messages = [
        DaConsensusMessage::DaProposal(Proposal {
            data: DaProposal {
                encoded_transactions: payload.encode(),
                metadata,
                view_number: ViewNumber::genesis(),
            },
            signature: signature.clone(),
            _pd: Default::default(),
        }),
        DaConsensusMessage::DaVote(DaVote {
            signature: (sender, signature.clone()),
            data: da_data.clone(),
            view_number: ViewNumber::genesis(),
        }),
        DaConsensusMessage::DaCertificate(DaCertificate {
            data: da_data.clone(),
            vote_commitment: da_data.commit(),
            view_number: ViewNumber::genesis(),
            signatures: Default::default(),
            _pd: Default::default(),
        }),
        DaConsensusMessage::VidDisperseMsg(Proposal {
            data: VidDisperseShare::from_vid_disperse(VidDisperse::from_membership(
                ViewNumber::genesis(),
                vid_scheme(1).disperse(payload.encode()).unwrap(),
                &membership,
            ))
            .remove(0),
            signature: signature.clone(),
            _pd: Default::default(),
        }),
    ];
    let data_messages = [DataMessage::SubmitTransaction(
        transaction,
        ViewNumber::genesis(),
    )];

    let seq_messages = consensus_messages
        .into_iter()
        .map(SequencingMessage::General)
        .chain(da_messages.into_iter().map(SequencingMessage::Da));
    let messages = seq_messages
        .map(MessageKind::Consensus)
        .chain(data_messages.into_iter().map(MessageKind::Data))
        .map(|kind| Message { kind, sender })
        .collect::<Vec<Message<SeqTypes>>>();

    // Load the expected serialization from the repo.
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../data");
    let expected_bytes = std::fs::read(data_dir.join("messages.json")).unwrap();
    let expected: Value = serde_json::from_slice(&expected_bytes).unwrap();

    // Ensure the current serialization implementation generates the same JSON as the committed
    // reference.
    let actual = serde_json::to_value(&messages).unwrap();
    if actual != expected {
        let actual_pretty = serde_json::to_string_pretty(&actual).unwrap();
        let expected_pretty = serde_json::to_string_pretty(&expected).unwrap();

        // Write the actual output to a file to make it easier to compare with/replace the expected
        // file if the serialization change was actually intended.
        let actual_path = data_dir.join("messages-actual.json");
        std::fs::write(&actual_path, actual_pretty.as_bytes()).unwrap();

        // Fail the test with an assertion that outputs a nice diff between the prettified JSON
        // objects.
        assert_eq!(
            expected_pretty,
            actual_pretty,
            r#"
    Serialized messages do not match expected JSON. The actual serialization has been written to {}.
    If you intended to make a breaking change to the API you may replace the reference JSON file
    /data/messages.json with /data/messages-actual.json. Otherwise, revert your changes which have
    caused a change in the serialization of HotShot messages.
    "#,
            actual_path.display()
        );
    }

    // Ensure the current `Message` type can be parsed from the committed reference JSON.
    let parsed: Vec<Message<SeqTypes>> = serde_json::from_value(expected).unwrap();
    assert_eq!(parsed, messages);

    // Ensure the current serialization implementation generates the same binary output as the
    // committed reference.
    let expected = std::fs::read(data_dir.join("messages.bin")).unwrap();
    let actual = Serializer::serialize(&messages).unwrap();
    if actual != expected {
        // Write the actual output to a file to make it easier to compare with/replace the expected
        // file if the serialization change was actually intended.
        let actual_path = data_dir.join("messages-actual.bin");
        std::fs::write(&actual_path, &actual).unwrap();

        // Fail the test with an assertion that outputs a diff.
        assert_eq!(
            expected,
            actual,
            r#"
    Serialized messages do not match expected binary. The actual serialization has been written to
    {}. If you intended to make a breaking change to the API you may replace the reference binary
    file /data/messages.bin with /data/messages-actual.bin. Otherwise, revert your changes which
    have caused a change in the serialization of HotShot messages.
    "#,
            actual_path.display()
        );
    }

    // Ensure the current `Message` type can be parsed from the committed reference binary.
    let parsed: Vec<Message<SeqTypes>> = Serializer::deserialize(&expected).unwrap();
    assert_eq!(parsed, messages);
}
