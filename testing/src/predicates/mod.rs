pub mod event;
#[cfg(not(feature = "dependency-tasks"))]
pub mod upgrade_with_consensus;
#[cfg(feature = "dependency-tasks")]
pub mod upgrade_with_proposal;
#[cfg(feature = "dependency-tasks")]
pub mod upgrade_with_vote;

use async_trait::async_trait;

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

#[async_trait]
pub trait Predicate<INPUT>: std::fmt::Debug {
    async fn evaluate(&self, input: &INPUT) -> PredicateResult;
    async fn info(&self) -> String;
}
