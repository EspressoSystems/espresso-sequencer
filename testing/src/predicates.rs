use hotshot_task_impls::{
    consensus::ConsensusTaskState, events::HotShotEvent, events::HotShotEvent::*,
};
use hotshot_types::traits::node_implementation::NodeType;

use hotshot::types::SystemContextHandle;

use hotshot_example_types::node_types::{MemoryImpl, TestTypes};

pub struct Predicate<INPUT> {
    pub function: Box<dyn Fn(&INPUT) -> bool>,
    pub info: String,
}

impl<INPUT> std::fmt::Debug for Predicate<INPUT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.info)
    }
}

pub fn exact<TYPES>(event: HotShotEvent<TYPES>) -> Predicate<HotShotEvent<TYPES>>
where
    TYPES: NodeType,
{
    let info = format!("{:?}", event);

    Predicate {
        function: Box::new(move |e| e == &event),
        info,
    }
}

pub fn leaf_decided<TYPES>() -> Predicate<HotShotEvent<TYPES>>
where
    TYPES: NodeType,
{
    let info = "LeafDecided".to_string();
    let function = |e: &_| matches!(e, LeafDecided(_));

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn quorum_vote_send<TYPES>() -> Predicate<HotShotEvent<TYPES>>
where
    TYPES: NodeType,
{
    let info = "QuorumVoteSend".to_string();
    let function = |e: &_| matches!(e, QuorumVoteSend(_));

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn view_change<TYPES>() -> Predicate<HotShotEvent<TYPES>>
where
    TYPES: NodeType,
{
    let info = "ViewChange".to_string();
    let function = |e: &_| matches!(e, ViewChange(_));

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn upgrade_certificate_formed<TYPES>() -> Predicate<HotShotEvent<TYPES>>
where
    TYPES: NodeType,
{
    let info = "UpgradeCertificateFormed".to_string();
    let function = |e: &_| matches!(e, UpgradeCertificateFormed(_));

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn quorum_proposal_send_with_upgrade_certificate<TYPES>() -> Predicate<HotShotEvent<TYPES>>
where
    TYPES: NodeType,
{
    let info = "QuorumProposalSend with UpgradeCertificate attached".to_string();
    let function = |e: &_| match e {
        QuorumProposalSend(proposal, _) => proposal.data.upgrade_certificate.is_some(),
        _ => false,
    };

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn quorum_proposal_validated<TYPES>() -> Predicate<HotShotEvent<TYPES>>
where
    TYPES: NodeType,
{
    let info = "QuorumProposalValidated".to_string();
    let function = |e: &_| matches!(e, QuorumProposalValidated(_));

    Predicate {
        function: Box::new(function),
        info,
    }
}

pub fn quorum_proposal_send<TYPES>() -> Predicate<HotShotEvent<TYPES>>
where
    TYPES: NodeType,
{
    let info = "QuorumProposalSend".to_string();
    let function = |e: &_| matches!(e, QuorumProposalSend(_, _));

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
    Predicate {
        function,
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
