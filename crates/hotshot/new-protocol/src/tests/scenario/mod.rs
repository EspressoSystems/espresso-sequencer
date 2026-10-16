//! Scenarios from the Lean simulator, replayed to the Rust node.
//!
//! `lake exe sim --scenarios <dir>` runs the reference machine on simulated
//! networks and writes, for each node, the inputs it was given in order
//! (`format`). Replaying one gives the Rust node a run in the shape the
//! specification describes: real certificates, timeouts, re-votes and epoch
//! changes, arriving as a network that meets the specification's delivery
//! assumptions would deliver them. The test asserts nothing about what the node
//! does. Its recorded trace is checked against the specification afterwards
//! (`just test-lean-check`), as every other test's is.
//!
//! The scenario is the node's environment, not a script of what it must do: the
//! simulated node ran the reference machine, and the Rust node may act
//! differently where the rules let it. Where it proposes, its own proposal
//! stands for the simulated one if it extends the same parent (`World::adopt_own`).
//! A simulated proposal of the node's that the Rust node did not make is not
//! delivered to it: no network could deliver a proposal under a node's key that
//! the node never signed. Certificates over that block still arrive, as they
//! would from the nodes that voted for it.
//!
//! Inputs the Rust node gets from its own modules are answered by the harness as
//! in the other tests: the header a leader asks for, the validity report, storage
//! confirmations, DRB results. The scenario's `headerBuilt` and `blockValidated`
//! inputs are therefore skipped. A `Cert1` over an epoch root arrives with its
//! light-client state certificate, as the node's state-vote tally delivers it.

mod format;
mod world;

use std::path::{Path, PathBuf};

use hotshot_types::{
    data::{EpochNumber, ViewNumber},
    simple_certificate::TimeoutEvidence,
};

use self::{
    format::{Input, Scenario},
    world::World,
};
use crate::{
    cert_verifier::ValidCert,
    consensus::{ConsensusInput, ConsensusOutput},
    helpers::test_timeout_epoch_lock,
    message::{EpochChangeMessage, ProposalMessage, ReVote, ReVoteMessage},
    tests::common::{
        coordinator_builder::{build_genesis_cert1, build_genesis_proposal},
        utils::ConsensusHarness,
    },
};

/// The nodes and epoch height of the harness's test membership.
const NODES: u64 = 10;
const EPOCH_HEIGHT: u64 = 10;

/// Replay every scenario in the directory `NP_SCENARIOS` names.
///
/// Does nothing when it is unset, so a plain `cargo test` passes over it.
#[tokio::test]
async fn scenarios_replay() {
    let Ok(dir) = std::env::var("NP_SCENARIOS") else {
        return;
    };
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {dir}: {e}"))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "jsonl"))
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no scenarios in {dir}");
    for path in &paths {
        replay(path).await;
    }
}

/// Replay one scenario to a fresh node, recording its trace under the file's name.
async fn replay(path: &Path) {
    crate::logging::init_test_logging();
    let scenario = Scenario::read(path);
    let h = &scenario.header;
    assert_eq!(
        (h.nodes, h.epoch_height),
        (NODES, EPOCH_HEIGHT),
        "{}: the harness has {NODES} nodes and epochs of {EPOCH_HEIGHT}",
        path.display()
    );
    let label = format!(
        "scenario-{}",
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unnamed")
    );
    let me = h.node;
    let (my_key, _) = World::key(me);
    let mut harness =
        ConsensusHarness::new_with_upgrade_lock(me - 1, EPOCH_HEIGHT, test_timeout_epoch_lock())
            .await
            .record_as(&label);
    // Started as `Coordinator::maker` starts a node: locked on the anchor.
    let genesis_leaf = harness.consensus.last_decided_leaf().clone();
    harness.consensus.seed_parent(
        build_genesis_cert1(&genesis_leaf),
        build_genesis_proposal(&genesis_leaf),
        std::iter::empty(),
    );
    let mut world = World::new(NODES, EPOCH_HEIGHT, &scenario.blocks).await;
    let mut seen_outputs = 0;
    for step in &scenario.steps {
        // The node's own proposals since the last step stand for the simulated ones.
        let own: Vec<_> = harness
            .outputs()
            .iter()
            .skip(seen_outputs)
            .filter_map(|o| match o {
                ConsensusOutput::SendProposal(p) => Some(p.clone()),
                _ => None,
            })
            .collect();
        seen_outputs = harness.outputs().len();
        for p in own {
            if let Some(id) = scenario
                .blocks
                .iter()
                .find(|b| b.view == *p.data.view_number && b.epoch == *p.data.epoch)
                .filter(|b| world.leader(b.epoch, b.view).0 == my_key)
                .map(|b| b.id)
            {
                world.adopt_own(id, p).await;
            }
        }
        if let Some(input) = consensus_input(&mut world, &my_key, &step.input).await {
            for i in input {
                harness.apply(i).await;
            }
        }
    }
}

/// The inputs the Rust node takes for one of the scenario's.
async fn consensus_input(
    world: &mut World,
    me: &hotshot::types::BLSPubKey,
    input: &Input,
) -> Option<Vec<ConsensusInput<hotshot_example_types::node_types::TestTypes>>> {
    let inputs = match *input {
        Input::Proposal {
            sender,
            block,
            share,
        } => {
            if world.node_of(me) == Some(sender) && !world.is_own(block.id) {
                return None;
            }
            let built = world.block(block.id).await;
            assert_eq!(
                world.node_of(&built.leader),
                Some(sender),
                "the scenario's leader of view {} is the harness's",
                block.view
            );
            let mut v = vec![ConsensusInput::Proposal(
                built.leader,
                ProposalMessage::validated(built.proposal.clone()),
            )];
            if share {
                v.push(ConsensusInput::VidShare(built.share_for(me)));
            }
            v
        },
        Input::Certificate1(c) => {
            let cert1 = ValidCert::new(world.cert1(c).await, EpochNumber::new(c.epoch));
            match world.state_cert(c).await {
                Some(state_cert) => {
                    vec![ConsensusInput::EpochRootCertificates { cert1, state_cert }]
                },
                None => vec![ConsensusInput::Certificate1(cert1)],
            }
        },
        Input::Certificate2(c) => {
            world.saw_cert2(c);
            vec![ConsensusInput::Certificate2(ValidCert::new(
                world.cert2(c).await,
                EpochNumber::new(c.epoch),
            ))]
        },
        Input::EpochChange {
            cert1,
            cert2,
            block,
        } => {
            world.saw_cert2(cert2);
            let c1 = world.cert1(cert1).await;
            let c2 = world.cert2(cert2).await;
            let built = world.block(block.id).await;
            vec![ConsensusInput::EpochChange(EpochChangeMessage::validated(
                c1,
                c2,
                built.proposal.data,
            ))]
        },
        Input::BlockReconstructed { view, payload } => {
            let id = world
                .block_with_payload(view, payload)
                .unwrap_or_else(|| panic!("a block at view {view} with payload {payload}"));
            let built = world.block(id).await;
            vec![ConsensusInput::BlockReconstructed {
                view: ViewNumber::new(view),
                payload_commitment: built.vid.payload_commitment,
                payload: built.payload,
            }]
        },
        Input::Revote {
            sender,
            cert,
            view,
            evidence,
        } => {
            let cert1 = world.cert1(cert).await;
            let timeout = match evidence {
                Some(tc) => Some(world.timeout_cert(tc).await),
                None => None,
            };
            let revote = ReVote {
                view: ViewNumber::new(view),
                epoch: EpochNumber::new(cert.epoch),
                cert1,
                timeout,
            };
            let (_, key) = World::key(sender);
            vec![ConsensusInput::ReVote(
                ReVoteMessage::new(revote, &key).expect("the leader signs the request"),
            )]
        },
        Input::Timeout(v) => vec![ConsensusInput::Timeout(ViewNumber::new(v))],
        Input::TimeoutOneHonest(v) => vec![ConsensusInput::TimeoutOneHonest(ViewNumber::new(v))],
        Input::TimeoutCertificate(tc) => vec![ConsensusInput::TimeoutCertificate(ValidCert::new(
            TimeoutEvidence::V3(world.timeout_cert(tc).await),
            EpochNumber::new(tc.epoch),
        ))],
        Input::HeaderBuilt(_) | Input::BlockValidated(_) => return None,
    };
    Some(inputs)
}
