//! The query service on a Postgres server.

use hotshot_query_service::data_source::sql::postgres as qs;

use crate::persistence::sql::postgres as persistence;

#[path = "imp.rs"]
mod imp;

pub use imp::*;
