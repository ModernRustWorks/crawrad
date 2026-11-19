//! Columnar in-memory store for large-volume grids: sort, filter, group and locate over row ids.

pub mod bitmap;
pub mod column;
pub mod cursor;
#[cfg(any(test, feature = "test-sources"))]
pub mod cursor_contract;
pub mod filter;
pub mod flat;
pub mod group;
pub mod live;
pub mod locate;
pub(crate) mod ordinal;
pub mod query;
pub mod sort;
pub mod source;
pub mod store;
pub mod view;

pub use bitmap::Bitmap;
pub use column::{Column, Value};
pub use cursor::{
    Bookmark, Cursor, CursorEvent, CursorLine, Date, Field, LinesIter, ListenerId, LocateOptions,
    RowsIter,
};
pub use filter::{Filter, Owned};
pub use flat::{FlatView, GroupedFlat, Line, LineKey, LineKind, WindowedRows};
pub use group::{Aggregate, Group, Total};
pub use live::{Busy, LiveRows, Screen, ScreenLine, Write, Written};
pub use locate::Locator;
pub use query::{GroupLevel, Query};
pub use sort::{Nulls, Order, PACKED_MAX, SMALL_SORT, SortKey};
pub use source::{
    Capabilities, Change, ColumnInfo, ColumnKind, DisplayHint, Distinct, LookupSource, RowSource,
    Rows, SourceError, SourceGroup, VecSource,
};
#[cfg(any(test, feature = "test-sources"))]
pub use source::{FakeSlowSource, FetchStats};
pub use store::{Store, StoreError};
pub use view::{GroupNode, ReuseStats, View};

#[cfg(test)]
#[path = "test/oracle/test.rs"]
mod oracle_test;
