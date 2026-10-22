//! The enum and forwarding macro shared by the SQL data source and the SQL persistence.

/// A value of either SQL backend: a data source, a persistence, or a transaction of one.
#[derive(Clone, Debug)]
pub enum Sql<P, L> {
    Postgres(P),
    Sqlite(L),
}

/// Evaluate `$e` with `$x` bound to the value inside whichever backend `$self` holds.
macro_rules! dispatch {
    ($self:expr, $x:ident => $e:expr) => {
        match $self {
            $crate::sql_dispatch::Sql::Postgres($x) => $e,
            $crate::sql_dispatch::Sql::Sqlite($x) => $e,
        }
    };
}

pub(crate) use dispatch;
