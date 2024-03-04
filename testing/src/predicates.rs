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
