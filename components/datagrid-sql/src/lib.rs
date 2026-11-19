//! SQL backend row source bridge for datagrid through portadb.
//!
//! Provides [`SqlSource`] implementing [`datagrid::RowSource`] to execute
//! grid queries natively inside SQL databases (such as SQLite via `portadb`).

#![forbid(unsafe_code)]

pub mod lookup;
pub mod source;
pub mod translate;

pub use lookup::SqlLookup;
pub use source::{SqlSource, sqlite_kind};
pub use translate::Table;
