use std::sync::Arc;

use async_trait::async_trait;
use hotshot_task_impls::events::{HotShotEvent, HotShotEvent::*};
use hotshot_types::{
    data::null_block,
    traits::{block_contents::BlockHeader, node_implementation::NodeType},
};

use crate::predicates::{Predicate, PredicateResult};

type EventCallback<TYPES> = Arc<dyn Fn(Arc<HotShotEvent<TYPES>>) -> bool + Send + Sync>;

pub struct EventPredicate<TYPES>
where
    TYPES: NodeType + Send + Sync,
{
    check: EventCallback<TYPES>,
    info: String,
}

impl<TYPES: NodeType> std::fmt::Debug for EventPredicate<TYPES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

#[async_trait]
impl<TYPES> Predicate<Arc<HotShotEvent<TYPES>>> for EventPredicate<TYPES>
where
    TYPES: NodeType + Send + Sync + 'static,
{
    async fn evaluate(&self, input: &Arc<HotShotEvent<TYPES>>) -> PredicateResult {
        PredicateResult::from((self.check)(input.clone()))
    }

    async fn info(&self) -> String {
        self.info.clone()
    }
}

pub fn exact<TYPES>(event: HotShotEvent<TYPES>) -> Box<EventPredicate<TYPES>>
where
    TYPES: NodeType,
{
    let info = format!("{:?}", event);
    let event = Arc::new(event);

    let check: EventCallback<TYPES> = Arc::new(move |e: Arc<HotShotEvent<TYPES>>| {
        let event_clone = event.clone();
        *e == *event_clone
    });

    Box::new(EventPredicate { check, info })
}

pub fn leaf_decided<TYPES>() -> Box<EventPredicate<TYPES>>
where
    TYPES: NodeType,
{
    let info = "LeafDecided".to_string();
    let check: EventCallback<TYPES> =
        Arc::new(move |e: Arc<HotShotEvent<TYPES>>| matches!(e.as_ref(), LeafDecided(_)));

    Box::new(EventPredicate { check, info })
}

pub fn quorum_vote_send<TYPES>() -> Box<EventPredicate<TYPES>>
where
    TYPES: NodeType,
{
    let info = "QuorumVoteSend".to_string();
    let check: EventCallback<TYPES> =
        Arc::new(move |e: Arc<HotShotEvent<TYPES>>| matches!(e.as_ref(), QuorumVoteSend(_)));

    Box::new(EventPredicate { check, info })
}

pub fn view_change<TYPES>() -> Box<EventPredicate<TYPES>>
where
    TYPES: NodeType,
{
    let info = "ViewChange".to_string();
    let check: EventCallback<TYPES> =
        Arc::new(move |e: Arc<HotShotEvent<TYPES>>| matches!(e.as_ref(), ViewChange(_)));
    Box::new(EventPredicate { check, info })
}

pub fn upgrade_certificate_formed<TYPES>() -> Box<EventPredicate<TYPES>>
where
    TYPES: NodeType,
{
    let info = "UpgradeCertificateFormed".to_string();
    let check: EventCallback<TYPES> = Arc::new(move |e: Arc<HotShotEvent<TYPES>>| {
        matches!(e.as_ref(), UpgradeCertificateFormed(_))
    });
    Box::new(EventPredicate { check, info })
}

pub fn quorum_proposal_send_with_upgrade_certificate<TYPES>() -> Box<EventPredicate<TYPES>>
where
    TYPES: NodeType,
{
    let info = "QuorumProposalSend with UpgradeCertificate attached".to_string();
    let check: EventCallback<TYPES> =
        Arc::new(move |e: Arc<HotShotEvent<TYPES>>| match e.as_ref() {
            QuorumProposalSend(proposal, _) => proposal.data.upgrade_certificate.is_some(),
            _ => false,
        });
    Box::new(EventPredicate { info, check })
}

pub fn quorum_proposal_validated<TYPES>() -> Box<EventPredicate<TYPES>>
where
    TYPES: NodeType,
{
    let info = "QuorumProposalValidated".to_string();
    let check: EventCallback<TYPES> = Arc::new(move |e: Arc<HotShotEvent<TYPES>>| {
        matches!(*e.clone(), QuorumProposalValidated(..))
    });
    Box::new(EventPredicate { check, info })
}

pub fn quorum_proposal_send<TYPES>() -> Box<EventPredicate<TYPES>>
where
    TYPES: NodeType,
{
    let info = "QuorumProposalSend".to_string();
    let check: EventCallback<TYPES> =
        Arc::new(move |e: Arc<HotShotEvent<TYPES>>| matches!(e.as_ref(), QuorumProposalSend(..)));
    Box::new(EventPredicate { check, info })
}

pub fn quorum_proposal_send_with_null_block<TYPES>(
    num_storage_nodes: usize,
) -> Box<EventPredicate<TYPES>>
where
    TYPES: NodeType,
{
    let info = "QuorumProposalSend with null block payload".to_string();
    let check: EventCallback<TYPES> =
        Arc::new(move |e: Arc<HotShotEvent<TYPES>>| match e.as_ref() {
            QuorumProposalSend(proposal, _) => {
                Some(proposal.data.block_header.payload_commitment())
                    == null_block::commitment(num_storage_nodes)
            }
            _ => false,
        });
    Box::new(EventPredicate { check, info })
}

pub fn timeout_vote_send<TYPES>() -> Box<EventPredicate<TYPES>>
where
    TYPES: NodeType,
{
    let info = "TimeoutVoteSend".to_string();
    let check: EventCallback<TYPES> =
        Arc::new(move |e: Arc<HotShotEvent<TYPES>>| matches!(e.as_ref(), TimeoutVoteSend(..)));
    Box::new(EventPredicate { check, info })
}
