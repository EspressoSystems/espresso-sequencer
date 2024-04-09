use hotshot_task_impls::{
    consensus::ConsensusTaskState, events::HotShotEvent, events::HotShotEvent::*,
};
use hotshot_types::data::null_block;
use hotshot_types::traits::{block_contents::BlockHeader, node_implementation::NodeType};
use std::collections::HashSet;
use std::sync::Arc;

use hotshot::types::SystemContextHandle;

use hotshot_example_types::node_types::{MemoryImpl, TestTypes};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PredicateResult {
    Pass,

    Fail,

    Incomplete,
}

impl From<bool> for PredicateResult {
    fn from(boolean: bool) -> Self {
        match boolean {
            true => PredicateResult::Pass,
            false => PredicateResult::Fail,
        }
    }
}

pub struct Predicate<INPUT> {
    pub function: Box<dyn FnMut(&INPUT) -> PredicateResult>,
    pub info: String,
}

pub fn all<TYPES>(events: Vec<HotShotEvent<TYPES>>) -> Predicate<Arc<HotShotEvent<TYPES>>>
where
    TYPES: NodeType,
{
    let info = format!("{:?}", events);
    let mut set: HashSet<_> = events.into_iter().collect();

    let function = move |e: &Arc<HotShotEvent<TYPES>>| match set.take(e.as_ref()) {
        Some(_) => {
            if set.is_empty() {
                PredicateResult::Pass
            } else {
                PredicateResult::Incomplete
            }
        }
        None => PredicateResult::Fail,
    };

    Predicate {
        function: Box::new(function),
        info,
    }
}

impl<INPUT> std::fmt::Debug for Predicate<INPUT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.info)
    }
}

pub fn exact<TYPES>(event: HotShotEvent<TYPES>) -> Predicate<Arc<HotShotEvent<TYPES>>>
where
    TYPES: NodeType,
{
    let info = format!("{:?}", event);
    let event = Arc::new(event);

    Predicate {
        function: Box::new(move |e| PredicateResult::from(e == &event)),
        info,
    }
}

pub fn multi_exact<TYPES>(
    events: Vec<HotShotEvent<TYPES>>,
) -> Vec<Predicate<Arc<HotShotEvent<TYPES>>>>
where
    TYPES: NodeType,
{
    events
        .into_iter()
        .map(|event| {
            let event = Arc::new(event);
            let info = format!("{:?}", event);
            Predicate {
                function: Box::new(move |e| PredicateResult::from(e == &event)),
                info,
            }
        })
        .collect()
}

pub fn leaf_decided<TYPES>() -> Predicate<Arc<HotShotEvent<TYPES>>>
where
    TYPES: NodeType,
{
    let info = "LeafDecided".to_string();
    let function =
        |e: &Arc<HotShotEvent<TYPES>>| PredicateResult::from(matches!(e.as_ref(), LeafDecided(_)));

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn quorum_vote_send<TYPES>() -> Predicate<Arc<HotShotEvent<TYPES>>>
where
    TYPES: NodeType,
{
    let info = "QuorumVoteSend".to_string();
    let function = |e: &Arc<HotShotEvent<TYPES>>| {
        PredicateResult::from(matches!(e.as_ref(), QuorumVoteSend(_)))
    };

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn view_change<TYPES>() -> Predicate<Arc<HotShotEvent<TYPES>>>
where
    TYPES: NodeType,
{
    let info = "ViewChange".to_string();
    let function =
        |e: &Arc<HotShotEvent<TYPES>>| PredicateResult::from(matches!(e.as_ref(), ViewChange(_)));

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn upgrade_certificate_formed<TYPES>() -> Predicate<Arc<HotShotEvent<TYPES>>>
where
    TYPES: NodeType,
{
    let info = "UpgradeCertificateFormed".to_string();
    let function = |e: &Arc<HotShotEvent<TYPES>>| {
        PredicateResult::from(matches!(e.as_ref(), UpgradeCertificateFormed(_)))
    };

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn quorum_proposal_send_with_upgrade_certificate<TYPES>() -> Predicate<Arc<HotShotEvent<TYPES>>>
where
    TYPES: NodeType,
{
    let info = "QuorumProposalSend with UpgradeCertificate attached".to_string();
    let function = |e: &Arc<HotShotEvent<TYPES>>| match e.as_ref() {
        QuorumProposalSend(proposal, _) => {
            PredicateResult::from(proposal.data.upgrade_certificate.is_some())
        }
        _ => PredicateResult::Fail,
    };

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn quorum_proposal_validated<TYPES>() -> Predicate<Arc<HotShotEvent<TYPES>>>
where
    TYPES: NodeType,
{
    let info = "QuorumProposalValidated".to_string();
    let function = |e: &Arc<HotShotEvent<TYPES>>| {
        PredicateResult::from(matches!(e.as_ref(), QuorumProposalValidated(..)))
    };

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn quorum_proposal_send<TYPES>() -> Predicate<Arc<HotShotEvent<TYPES>>>
where
    TYPES: NodeType,
{
    let info = "QuorumProposalSend".to_string();
    let function = |e: &Arc<HotShotEvent<TYPES>>| {
        PredicateResult::from(matches!(e.as_ref(), QuorumProposalSend(_, _)))
    };

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn quorum_proposal_send_with_null_block<TYPES>(
    num_storage_nodes: usize,
) -> Predicate<Arc<HotShotEvent<TYPES>>>
where
    TYPES: NodeType,
{
    let info = "QuorumProposalSend with null block payload".to_string();
    let function = move |e: &Arc<HotShotEvent<TYPES>>| match e.as_ref() {
        QuorumProposalSend(proposal, _) => PredicateResult::from(
            Some(proposal.data.block_header.payload_commitment())
                == null_block::commitment(num_storage_nodes),
        ),
        _ => PredicateResult::Fail,
    };

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn timeout_vote_send<TYPES>() -> Predicate<Arc<HotShotEvent<TYPES>>>
where
    TYPES: NodeType,
{
    let info = "TimeoutVoteSend".to_string();
    let function = |e: &Arc<HotShotEvent<TYPES>>| {
        PredicateResult::from(matches!(e.as_ref(), TimeoutVoteSend(_)))
    };

    Predicate {
        function: Box::new(function),
        info,
    }
}

type ConsensusTaskTestState =
    ConsensusTaskState<TestTypes, MemoryImpl, SystemContextHandle<TestTypes, MemoryImpl>>;

pub fn consensus_predicate(
    function: Box<dyn for<'a> Fn(&'a ConsensusTaskTestState) -> bool>,
    info: &str,
) -> Predicate<ConsensusTaskTestState> {
    let wrapped_function = move |e: &ConsensusTaskTestState| PredicateResult::from(function(e));

    Predicate {
        function: Box::new(wrapped_function),
        info: info.to_string(),
    }
}

pub fn no_decided_upgrade_cert() -> Predicate<ConsensusTaskTestState> {
    consensus_predicate(
        Box::new(|state| state.decided_upgrade_cert.is_none()),
        "expected decided_upgrade_cert to be None",
    )
}

pub fn decided_upgrade_cert() -> Predicate<ConsensusTaskTestState> {
    consensus_predicate(
        Box::new(|state| state.decided_upgrade_cert.is_some()),
        "expected decided_upgrade_cert to be Some(_)",
    )
}

pub fn is_at_view_number(n: u64) -> Predicate<ConsensusTaskTestState> {
    consensus_predicate(
        Box::new(move |state| *state.cur_view == n),
        format!("expected cur view to be {}", n).as_str(),
    )
}
