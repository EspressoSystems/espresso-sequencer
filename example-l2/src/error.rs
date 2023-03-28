use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub struct RollupError(pub String);

impl std::error::Error for RollupError {}

impl Display for RollupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
