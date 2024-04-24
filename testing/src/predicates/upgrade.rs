use std::sync::Arc;

use async_trait::async_trait;
use hotshot_example_types::node_types::{MemoryImpl, TestTypes};
use hotshot_task_impls::consensus::ConsensusTaskState;
use hotshot_types::simple_certificate::UpgradeCertificate;

use crate::predicates::{Predicate, PredicateResult};

type ConsensusTaskTestState = ConsensusTaskState<TestTypes, MemoryImpl>;

type UpgradeCertCallback =
    Arc<dyn Fn(Arc<Option<UpgradeCertificate<TestTypes>>>) -> bool + Send + Sync>;

pub struct UpgradeCertPredicate {
    check: UpgradeCertCallback,
    info: String,
}

impl std::fmt::Debug for UpgradeCertPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

#[async_trait]
impl Predicate<ConsensusTaskTestState> for UpgradeCertPredicate {
    async fn evaluate(&self, input: &ConsensusTaskTestState) -> PredicateResult {
        let upgrade_cert = input.decided_upgrade_cert.clone();
        PredicateResult::from((self.check)(upgrade_cert.into()))
    }

    async fn info(&self) -> String {
        self.info.clone()
    }
}

pub fn no_decided_upgrade_cert() -> Box<UpgradeCertPredicate> {
    let info = "expected decided_upgrade_cert to be None".to_string();
    let check: UpgradeCertCallback = Arc::new(move |s| s.is_none());
    Box::new(UpgradeCertPredicate { info, check })
}

pub fn decided_upgrade_cert() -> Box<UpgradeCertPredicate> {
    let info = "expected decided_upgrade_cert to be Some(_)".to_string();
    let check: UpgradeCertCallback = Arc::new(move |s| s.is_some());
    Box::new(UpgradeCertPredicate { info, check })
}
