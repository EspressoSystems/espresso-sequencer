#[cfg(test)]
use std::marker::PhantomData;

use committable::Committable;
use hotshot_example_types::node_types::TestTypes;
use hotshot_types::{
    message::{GeneralConsensusMessage, Message, MessageKind, SequencingMessage},
    signature_key::BLSPubKey,
    simple_certificate::SimpleCertificate,
    simple_vote::ViewSyncCommitData,
    traits::{node_implementation::ConsensusTime, signature_key::SignatureKey},
};
use vbs::{
    version::{StaticVersion, Version},
    BinarySerializer, Serializer,
};

#[test]
// Checks that the current program protocol version
// correctly appears at the start of a serialized messaged.
fn version_number_at_start_of_serialization() {
    let sender = BLSPubKey::generated_from_seed_indexed([0u8; 32], 0).0;
    let view_number = ConsensusTime::new(17);
    // The version we set for the message
    const MAJOR: u16 = 37;
    const MINOR: u16 = 17;
    let version = Version {
        major: MAJOR,
        minor: MINOR,
    };
    type TestVersion = StaticVersion<MAJOR, MINOR>;
    // The specific data we attach to our message shouldn't affect the serialization,
    // we're using ViewSyncCommitData for simplicity.
    let data: ViewSyncCommitData<TestTypes> = ViewSyncCommitData {
        relay: 37,
        round: view_number,
    };
    let simple_certificate = SimpleCertificate {
        data: data.clone(),
        vote_commitment: data.commit(),
        view_number,
        signatures: None,
        _pd: PhantomData,
    };
    let message = Message {
        sender,
        kind: MessageKind::Consensus(SequencingMessage::General(
            GeneralConsensusMessage::ViewSyncCommitCertificate(simple_certificate),
        )),
    };
    let serialized_message: Vec<u8> = Serializer::<TestVersion>::serialize(&message).unwrap();
    // The versions we've read from the message

    let version_read = Version::deserialize(&serialized_message).unwrap().0;

    assert_eq!(version.major, version_read.major);
    assert_eq!(version.minor, version_read.minor);
}
