// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

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
/// # use sqlx::{Database, Encode, Executor, IntoArguments, Type};
/// fn foo<DB: Database>()
/// where
///     for<'a> &'a mut DB::Connection: Executor<'a>,
///     for<'q> DB::Arguments<'q>: IntoArguments<'q, DB>,
///     for<'a> i64: Type<DB> + Encode<'a, DB>,
/// {}
/// ```
/// etc.
pub type Db = sqlx::Postgres;
