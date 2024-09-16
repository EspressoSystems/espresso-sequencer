/// The concrete database backing a SQL data source.
///
/// Currently only Postgres is supported. In the future we can support SQLite as well by making this
/// an enum with variants for each (we'll then need to create enums and trait implementations for
/// all the associated types as well; it will be messy).
///
/// The reason for taking this approach over sqlx's `Any` database is that we can support SQL types
/// which are implemented for the two backends we care about (Postgres and SQLite) but not for _any_
/// SQL database, such as MySQL. Crucially, JSON types fall in this category.
///
/// The reason for taking this approach rather than writing all of our code to be generic over the
/// `Database` implementation is that `sqlx` does not have the necessary trait bounds on all of the
/// associated types (e.g. `Database::Connection` does not implement `Executor` for all possible
/// databases, the `Executor` impl lives on each concrete connection type) and Rust does not provide
/// a good way of encapsulating a collection of trait bounds on associated types. Thus, our function
/// signatures become untenably messy with bounds like
///
/// ```
/// where
///     for<'a> &'a mut DB::Connection: Executor<'a>,
///     for<'q> DB::Arguments<'q>: IntoArguments<'q, DB>,
///     for<'a> i64: Type<DB> + Encode<'a, DB>,
/// ```
/// etc.
pub type Db = sqlx::Postgres;
