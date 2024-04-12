pub mod event;
pub mod upgrade;

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
