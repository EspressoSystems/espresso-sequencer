use super::*;
use crate::data_source::sql::sqlite as sql;

#[expect(
    clippy::duplicate_mod,
    reason = "each backend runs the same tests against its own `Db`"
)]
#[path = "tests.rs"]
mod tests;
