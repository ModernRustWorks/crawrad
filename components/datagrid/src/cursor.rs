//! Dataset-style cursor over any [`RowSource`].
//!
//! # Example
//! ```
//! use datagrid::column::Column;
//! use datagrid::cursor::Cursor;
//! use datagrid::query::Query;
//! use datagrid::source::{ColumnInfo, ColumnKind, DisplayHint, VecSource};
//! use datagrid::store::Store;
//! use std::sync::Arc;
//!
//! let mut store = Store::new();
//! store.add("id", Column::ints(vec![Some(1), Some(2), Some(3)])).unwrap();
//! store.add(
//!     "name",
//!     Column::texts(vec![
//!         Some("Alice".into()),
//!         Some("Bob".into()),
//!         Some("Charlie".into()),
//!     ]),
//! )
//! .unwrap();
//!
//! let schema = vec![
//!     ColumnInfo {
//!         name: "id".into(),
//!         kind: ColumnKind::Int,
//!         nullable: false,
//!         hint: DisplayHint::Plain,
//!     },
//!     ColumnInfo {
//!         name: "name".into(),
//!         kind: ColumnKind::Text,
//!         nullable: false,
//!         hint: DisplayHint::Plain,
//!     },
//! ];
//!
//! let source = Arc::new(VecSource::new(store, schema).unwrap());
//! let mut cursor = Cursor::open(source, Query::default()).unwrap();
//!
//! let mut ids = Vec::new();
//! while !cursor.eof() {
//!     let id = cursor.field("id").unwrap().as_i64().unwrap();
//!     ids.push(id);
//!     cursor.next().unwrap();
//! }
//! assert_eq!(ids, vec![1, 2, 3]);
//!
//! // find a row (case-insensitive) and read it
//! use datagrid::cursor::LocateOptions;
//! use datagrid::filter::Owned;
//! let options = LocateOptions { case_insensitive: true, ..Default::default() };
//! assert!(cursor.locate(&[("name", Owned::from("bob"))], options).unwrap());
//! assert_eq!(cursor.field("id").unwrap().as_i64(), Some(2));
//! ```

use crate::column::Value;
use crate::filter::{Filter, Owned};
use crate::flat::WindowedRows;
use crate::query::Query;
use crate::sort::{Nulls, Order, SortKey};
use crate::source::{
    ColumnInfo, ColumnKind, DisplayHint, Distinct, RowSource, Rows, SourceError, SourceGroup,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Options controlling cursor locate operations.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LocateOptions {
    pub case_insensitive: bool,
    pub partial_key: bool,
}

/// A stable row identifier bookmark from a cursor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Bookmark(u64);

impl Bookmark {
    /// Constructs a new bookmark wrapping a row id.
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the underlying row id.
    pub fn id(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Bookmark {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

/// Civil calendar date (year, month, day).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl Date {
    /// Constructs a new `Date`.
    pub const fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Converts days since Unix epoch (1970-01-01) to a civil date using Howard Hinnant's algorithm.
    pub fn from_days(days: i64) -> Self {
        let z = days + 719468;
        let era = z.div_euclid(146097);
        let doe = (z - era * 146097) as u64;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe as i64 + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let year = if m <= 2 { y + 1 } else { y };
        Self {
            year: year as i32,
            month: m as u8,
            day: d as u8,
        }
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.year < 0 {
            write!(
                f,
                "-{:04}-{:02}-{:02}",
                self.year.unsigned_abs(),
                self.month,
                self.day
            )
        } else {
            write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
        }
    }
}

/// Notification events emitted by a [`Cursor`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CursorEvent {
    /// Emitted after the current row index changes to `rec_no`.
    Scrolled { rec_no: usize },
    /// Emitted after the active query has been replaced and reset to row 0.
    QueryChanged,
    /// Emitted right before the cursor calls the source (a count or a page it does not hold).
    /// Moves served from cached pages emit nothing — a busy indicator shows only real waits.
    Busy,
    /// Emitted when the call announced by `Busy` returns, successfully or not.
    Idle,
}

/// Unique identifier for a registered cursor listener.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListenerId(u64);

/// A borrowed scalar cell value and associated column metadata.
#[derive(Clone, Copy, Debug)]
pub struct Field<'a> {
    info: &'a ColumnInfo,
    value: Value<'a>,
}

impl<'a> Field<'a> {
    /// Returns the column name.
    #[inline]
    pub fn name(&self) -> &str {
        &self.info.name
    }

    /// Returns the column metadata.
    #[inline]
    pub fn info(&self) -> &ColumnInfo {
        self.info
    }

    /// Returns `true` if this field's value is null.
    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self.value, Value::Null)
    }

    /// Returns the borrowed cell value.
    #[inline]
    pub fn value(&self) -> Value<'a> {
        self.value
    }

    /// Returns the integer value, or `None` if null or non-integer.
    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self.value {
            Value::Int(v) => Some(v),
            _ => None,
        }
    }

    /// Returns the float value, converting `Int` variants if present.
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self.value {
            Value::Float(v) => Some(v),
            Value::Int(v) => Some(v as f64),
            _ => None,
        }
    }

    /// Returns the boolean value, or `None` if null or non-boolean.
    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        match self.value {
            Value::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Returns the text slice if this field contains text.
    #[inline]
    pub fn as_str(&self) -> Option<&'a str> {
        match self.value {
            Value::Text(s) => Some(s),
            _ => None,
        }
    }

    /// Converts an integer date or datetime to a [`Date`].
    pub fn as_date(&self) -> Option<Date> {
        match self.value {
            Value::Int(v) => match self.info.hint {
                DisplayHint::Date => Some(Date::from_days(v)),
                DisplayHint::DateTime => Some(Date::from_days(v.div_euclid(86_400_000))),
                _ => None,
            },
            _ => None,
        }
    }

    /// Formats the field for display.
    pub fn to_string_lossy(&self) -> String {
        if let Some(date) = self.as_date() {
            return date.to_string();
        }
        match self.value {
            Value::Null => String::new(),
            Value::Int(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::Bool(v) => v.to_string(),
            Value::Text(s) => s.to_string(),
        }
    }
}

type Listener = Box<dyn FnMut(&CursorEvent) + Send + 'static>;

/// Dataset-style navigation cursor over any [`RowSource`].
pub struct Cursor {
    window: WindowedRows,
    schema: Vec<ColumnInfo>,
    field_indices: HashMap<String, usize>,
    current_rec: usize,
    bof: bool,
    eof: bool,
    listeners: Vec<(ListenerId, Listener)>,
    next_listener_id: u64,
    user_query: Query,
    group_cache: HashMap<Vec<Option<Owned>>, Vec<SourceGroup>>,
}

impl Cursor {
    /// Opens `query` and positions on its first row, like a classic dataset's `Open`.
    pub fn open(source: Arc<dyn RowSource>, query: Query) -> Result<Self, SourceError> {
        let mut cursor = Self::new(source, query);
        cursor.first()?;
        Ok(cursor)
    }

    /// Constructs a `Cursor` with default page cache parameters (page_size 256, max_pages 4).
    ///
    /// Nothing is fetched until the first move; reading a field before it is
    /// `SourceError::NotPositioned`. Use [`Cursor::open`] to start on row 0.
    pub fn new(source: Arc<dyn RowSource>, query: Query) -> Self {
        Self::with_window(source, query, 256, 4)
    }

    /// Constructs a `Cursor` with explicit page size and cache capacity.
    pub fn with_window(
        source: Arc<dyn RowSource>,
        query: Query,
        page_size: usize,
        max_pages: usize,
    ) -> Self {
        let schema = source.schema().to_vec();
        let mut field_indices = HashMap::with_capacity(schema.len());
        for (i, col) in schema.iter().enumerate() {
            field_indices.insert(col.name.clone(), i);
        }
        let derived = derive_query(&query);
        Self {
            window: WindowedRows::new(source, derived, page_size, max_pages),
            schema,
            field_indices,
            current_rec: 0,
            bof: true,
            eof: false,
            listeners: Vec::new(),
            next_listener_id: 1,
            user_query: query,
            group_cache: HashMap::new(),
        }
    }

    /// The underlying row source.
    #[inline]
    pub fn source(&self) -> &Arc<dyn RowSource> {
        self.window.source()
    }

    /// The active query.
    #[inline]
    pub fn query(&self) -> &Query {
        &self.user_query
    }

    /// Positions on the first row (row 0). Sets `bof = true` and `eof = is_empty`.
    pub fn first(&mut self) -> Result<(), SourceError> {
        let count = self.count()?;
        if count == 0 {
            self.bof = true;
            self.eof = true;
            return Ok(());
        }
        let old_rec = self.current_rec;
        self.current_rec = 0;
        self.bof = true;
        self.eof = false;
        self.load(0)?;
        if old_rec != 0 {
            self.notify_scrolled(0);
        }
        Ok(())
    }

    /// Positions on the last row (count - 1). Sets `eof = true` and `bof = is_empty`.
    pub fn last(&mut self) -> Result<(), SourceError> {
        let count = self.count()?;
        if count == 0 {
            self.bof = true;
            self.eof = true;
            return Ok(());
        }
        let target = count - 1;
        let old_rec = self.current_rec;
        self.current_rec = target;
        self.eof = true;
        self.bof = false;
        self.load(target)?;
        if old_rec != target {
            self.notify_scrolled(target);
        }
        Ok(())
    }

    /// Advances to the next row. If on the last row, sets `eof = true` and remains there.
    // The classic dataset vocabulary (`next`/`prior`/`eof`) is the point of this type.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<(), SourceError> {
        let count = self.count()?;
        if count == 0 {
            self.bof = true;
            self.eof = true;
            return Ok(());
        }
        if self.current_rec >= count - 1 {
            self.eof = true;
            return Ok(());
        }
        self.current_rec += 1;
        self.bof = false;
        self.eof = false;
        self.load(self.current_rec)?;
        self.notify_scrolled(self.current_rec);
        Ok(())
    }

    /// Moves to the prior row. If on the first row, sets `bof = true` and remains there.
    pub fn prior(&mut self) -> Result<(), SourceError> {
        let count = self.count()?;
        if count == 0 {
            self.bof = true;
            self.eof = true;
            return Ok(());
        }
        if self.current_rec == 0 {
            self.bof = true;
            return Ok(());
        }
        self.current_rec -= 1;
        self.bof = false;
        self.eof = false;
        self.load(self.current_rec)?;
        self.notify_scrolled(self.current_rec);
        Ok(())
    }

    /// Moves the cursor by `n` rows, clamping at the edges and setting `bof`/`eof`.
    ///
    /// Returns the distance actually moved.
    pub fn move_by(&mut self, n: isize) -> Result<isize, SourceError> {
        let count = self.count()?;
        if count == 0 {
            self.bof = true;
            self.eof = true;
            return Ok(0);
        }
        if n == 0 {
            return Ok(0);
        }
        let old_rec = self.current_rec;
        if n > 0 {
            let max_rec = count - 1;
            let target = (self.current_rec as isize).saturating_add(n);
            if target >= max_rec as isize {
                let actual = max_rec as isize - old_rec as isize;
                self.current_rec = max_rec;
                self.eof = true;
                self.bof = false;
                self.load(max_rec)?;
                if actual != 0 {
                    self.notify_scrolled(max_rec);
                }
                Ok(actual)
            } else {
                let new_rec = target as usize;
                self.current_rec = new_rec;
                self.bof = false;
                self.eof = false;
                self.load(new_rec)?;
                self.notify_scrolled(new_rec);
                Ok(n)
            }
        } else {
            let target = (self.current_rec as isize).saturating_add(n);
            if target <= 0 {
                let actual = -(old_rec as isize);
                self.current_rec = 0;
                self.bof = true;
                self.eof = false;
                self.load(0)?;
                if actual != 0 {
                    self.notify_scrolled(0);
                }
                Ok(actual)
            } else {
                let new_rec = target as usize;
                self.current_rec = new_rec;
                self.bof = false;
                self.eof = false;
                self.load(new_rec)?;
                self.notify_scrolled(new_rec);
                Ok(n)
            }
        }
    }

    /// Returns the current record number (0-based), or `None` if the result set is empty.
    pub fn rec_no(&self) -> Option<usize> {
        let count = self.record_count().ok()?;
        if count == 0 {
            None
        } else {
            Some(self.current_rec.min(count - 1))
        }
    }

    /// Explicitly positions the cursor on record `i` (0-based).
    ///
    /// If `i` is out of bounds, returns `Err(SourceError::OutOfRange)` and does not move.
    pub fn set_rec_no(&mut self, i: usize) -> Result<(), SourceError> {
        let count = self.count()?;
        if i >= count {
            return Err(SourceError::OutOfRange {
                index: i,
                len: count,
            });
        }
        self.move_to(i, count)
    }

    /// Moves to row `target` (< `count`) like a dataset's `RecNo :=`: the row is loaded
    /// first, so a failing source leaves the cursor where it was.
    fn move_to(&mut self, target: usize, count: usize) -> Result<(), SourceError> {
        self.load(target)?;
        let old_rec = self.current_rec;
        self.current_rec = target;
        self.bof = target == 0;
        self.eof = target + 1 == count;
        if old_rec != target {
            self.notify_scrolled(target);
        }
        Ok(())
    }

    /// Total matching record count.
    #[inline]
    pub fn record_count(&self) -> Result<usize, SourceError> {
        self.window.len()
    }

    /// Returns `true` if at the beginning of the dataset.
    pub fn bof(&self) -> bool {
        if let Ok(count) = self.record_count()
            && count == 0
        {
            return true;
        }
        self.bof
    }

    /// Returns `true` if past the end of the dataset.
    pub fn eof(&self) -> bool {
        if let Ok(count) = self.record_count()
            && count == 0
        {
            return true;
        }
        self.eof
    }

    /// Returns `true` if the dataset contains 0 records.
    #[inline]
    pub fn is_empty(&self) -> Result<bool, SourceError> {
        self.window.is_empty()
    }

    /// Borrows a field on the current row by column name.
    pub fn field(&self, name: &str) -> Result<Field<'_>, SourceError> {
        let &col_idx = self
            .field_indices
            .get(name)
            .ok_or_else(|| SourceError::UnknownField(name.to_string()))?;
        self.field_by_index(col_idx)
    }

    /// Borrows a field on the current row by column ordinal index.
    pub fn field_by_index(&self, index: usize) -> Result<Field<'_>, SourceError> {
        if index >= self.schema.len() {
            return Err(SourceError::OutOfRange {
                index,
                len: self.schema.len(),
            });
        }
        // Reads never call the source: the current page was loaded by the last move.
        if self.window.known_len() == Some(0) {
            return Err(SourceError::OutOfRange { index: 0, len: 0 });
        }
        let (_, rows, offset) = self
            .window
            .peek(self.current_rec)
            .ok_or(SourceError::NotPositioned)?;
        Ok(Field {
            info: &self.schema[index],
            value: rows.columns[index].get(offset),
        })
    }

    /// Iterates over all fields of the current row in schema order (none when not positioned).
    pub fn fields(&self) -> impl Iterator<Item = Field<'_>> {
        let current = self
            .window
            .peek(self.current_rec)
            .map(|(_, rows, offset)| (rows, offset));
        let schema = &self.schema;
        (0..schema.len()).filter_map(move |i| {
            let (rows, offset) = current?;
            Some(Field {
                info: &schema[i],
                value: rows.columns[i].get(offset),
            })
        })
    }

    /// Returns the stable row ID from the underlying source for the current row.
    pub fn row_id(&self) -> Option<u64> {
        let rec = self.rec_no()?;
        let (_, rows, offset) = self.window.peek(rec)?;
        rows.ids.get(offset).copied()
    }

    /// Returns a bookmark for the current cursor position, or `None` if not positioned.
    pub fn bookmark(&self) -> Option<Bookmark> {
        self.row_id().map(Bookmark)
    }

    /// Moves the cursor to the record identified by `bookmark`.
    ///
    /// Returns `true` if found and moved; returns `false` with no move if the row is excluded.
    pub fn goto_bookmark(&mut self, bookmark: Bookmark) -> Result<bool, SourceError> {
        self.notify(CursorEvent::Busy);
        let pos_res = self
            .window
            .source()
            .position_of(self.window.query(), bookmark.0);
        self.notify(CursorEvent::Idle);

        let Some(target_rec) = pos_res? else {
            return Ok(false);
        };
        let count = self.count()?;
        self.move_to(target_rec, count)?;
        Ok(true)
    }

    fn build_locate_filter(
        &self,
        keys: &[(&str, Owned)],
        options: LocateOptions,
    ) -> Result<Filter, SourceError> {
        let mut filters = Vec::with_capacity(keys.len());
        for (col_name, val) in keys {
            let &idx = self
                .field_indices
                .get(*col_name)
                .ok_or_else(|| SourceError::UnknownField((*col_name).to_string()))?;
            let info = &self.schema[idx];
            let f = match info.kind {
                ColumnKind::Text => match val {
                    Owned::Text(s) => {
                        if options.partial_key {
                            Filter::StartsWith {
                                column: (*col_name).to_string(),
                                text: s.clone(),
                                ignore_case: options.case_insensitive,
                            }
                        } else if options.case_insensitive {
                            Filter::EqIgnoreCase {
                                column: (*col_name).to_string(),
                                text: s.clone(),
                            }
                        } else {
                            Filter::Eq((*col_name).to_string(), val.clone())
                        }
                    }
                    _ => Filter::Eq((*col_name).to_string(), val.clone()),
                },
                _ => Filter::Eq((*col_name).to_string(), val.clone()),
            };
            filters.push(f);
        }

        if filters.is_empty() {
            Ok(Filter::And(Vec::new()))
        } else if filters.len() == 1 {
            Ok(filters.into_iter().next().expect("single filter"))
        } else {
            Ok(Filter::And(filters))
        }
    }

    /// Locates the first matching record in query order, positioning the cursor there if found.
    ///
    /// Returns `true` if found; if not found, the cursor does not move and returns `false`.
    pub fn locate(
        &mut self,
        keys: &[(&str, Owned)],
        options: LocateOptions,
    ) -> Result<bool, SourceError> {
        let filter = self.build_locate_filter(keys, options)?;

        self.notify(CursorEvent::Busy);
        let locate_res = self.window.source().locate(self.window.query(), &filter);
        self.notify(CursorEvent::Idle);

        let Some((target_rec, _)) = locate_res? else {
            return Ok(false);
        };
        let count = self.count()?;
        self.move_to(target_rec, count)?;
        Ok(true)
    }

    /// Looks up a field value on the first matching record without moving the cursor.
    pub fn lookup(
        &mut self,
        keys: &[(&str, Owned)],
        options: LocateOptions,
        result_field: &str,
    ) -> Result<Option<Owned>, SourceError> {
        let &result_col_idx = self
            .field_indices
            .get(result_field)
            .ok_or_else(|| SourceError::UnknownField(result_field.to_string()))?;

        let filter = self.build_locate_filter(keys, options)?;
        let mut lookup_query = self.window.query().clone();
        lookup_query.filter = match lookup_query.filter {
            Some(existing) => Some(existing.and(filter)),
            None => Some(filter),
        };

        self.notify(CursorEvent::Busy);
        let rows_res = self.window.source().fetch(&lookup_query, 0..1);
        self.notify(CursorEvent::Idle);

        let rows = rows_res?;
        if rows.len == 0 {
            return Ok(None);
        }

        let val = rows.columns[result_col_idx].get(0);
        let owned = match val {
            Value::Null => None,
            Value::Int(v) => Some(Owned::Int(v)),
            Value::Float(v) => Some(Owned::Float(v)),
            Value::Bool(b) => Some(Owned::Bool(b)),
            Value::Text(s) => Some(Owned::Text(s.to_string())),
        };
        Ok(owned)
    }

    /// Returns a lending iterator that walks the entire logical result from row 0.
    pub fn rows(&mut self) -> RowsIter<'_> {
        RowsIter {
            cursor: self,
            next_idx: 0,
            total: None,
        }
    }

    /// Registers an event listener callback, returning a stable [`ListenerId`].
    pub fn on_event(&mut self, f: impl FnMut(&CursorEvent) + Send + 'static) -> ListenerId {
        let id = ListenerId(self.next_listener_id);
        self.next_listener_id += 1;
        self.listeners.push((id, Box::new(f)));
        id
    }

    /// Unregisters a previously added listener.
    pub fn remove_listener(&mut self, id: ListenerId) {
        self.listeners.retain(|(lid, _)| *lid != id);
    }

    /// Changes the active query, resetting the position to row 0 and notifying listeners.
    pub fn set_query(&mut self, query: Query) {
        let derived = derive_query(&query);
        self.window.set_query(derived);
        self.user_query = query;
        self.group_cache.clear();
        self.current_rec = 0;
        self.bof = true;
        self.eof = false;
        self.notify(CursorEvent::QueryChanged);
    }

    /// Values of the group-by columns on the current row, outermost first (None when ungrouped or not positioned).
    pub fn group_path(&self) -> Option<Vec<Option<Owned>>> {
        if self.user_query.group_by.is_empty() {
            return None;
        }
        if self.window.known_len() == Some(0) {
            return None;
        }
        let (_, rows, offset) = self.window.peek(self.current_rec)?;
        let mut path = Vec::with_capacity(self.user_query.group_by.len());
        for level in &self.user_query.group_by {
            let &col_idx = self.field_indices.get(&level.column)?;
            let col = rows.columns.get(col_idx)?;
            let val = col.get(offset);
            path.push(value_to_owned(val));
        }
        Some(path)
    }

    /// The current row's group at `level` (0 = outermost): key, row count and that level's totals.
    pub fn group(&mut self, level: usize) -> Result<Option<SourceGroup>, SourceError> {
        if level >= self.user_query.group_by.len() {
            return Ok(None);
        }
        if self.window.known_len() == Some(0) {
            return Err(SourceError::OutOfRange { index: 0, len: 0 });
        }
        let path = self.group_path().ok_or(SourceError::NotPositioned)?;
        let parent_path = &path[..level];
        let key = &path[level];
        self.cached_or_fetch_group(parent_path, key)
    }

    /// Grand totals of columns requested in `query.summary`, computed across the entire filtered dataset.
    pub fn summary(&mut self) -> Result<Vec<Option<f64>>, SourceError> {
        self.notify(CursorEvent::Busy);
        let res = self.window.source().summary(&self.user_query);
        self.notify(CursorEvent::Idle);
        res
    }

    /// Distinct values for `column` across rows matching `query.filter`, up to `limit`.
    pub fn distinct(
        &mut self,
        query: &Query,
        column: &str,
        limit: usize,
    ) -> Result<Distinct, SourceError> {
        self.notify(CursorEvent::Busy);
        let res = self.window.source().distinct(query, column, limit);
        self.notify(CursorEvent::Idle);
        res
    }

    /// Returns a lending iterator that walks the entire logical result from row 0 as lines.
    pub fn lines(&mut self) -> LinesIter<'_> {
        let group_cols = self
            .user_query
            .group_by
            .iter()
            .map(|level| {
                self.field_indices
                    .get(&level.column)
                    .copied()
                    .ok_or_else(|| SourceError::UnknownField(level.column.clone()))
            })
            .collect();
        LinesIter {
            cursor: self,
            group_cols,
            next_idx: 0,
            total: None,
            path: None,
            pending: std::collections::VecDeque::new(),
        }
    }

    fn cached_or_fetch_group(
        &mut self,
        parent_path: &[Option<Owned>],
        key: &Option<Owned>,
    ) -> Result<Option<SourceGroup>, SourceError> {
        let parent_key = parent_path.to_vec();
        if !self.group_cache.contains_key(&parent_key) {
            if self.group_cache.len() >= 256 {
                self.group_cache.clear();
            }
            self.notify(CursorEvent::Busy);
            let res = self.window.source().groups(&self.user_query, &parent_key);
            self.notify(CursorEvent::Idle);
            let groups = res?;
            self.group_cache.insert(parent_key.clone(), groups);
        }
        let found = self
            .group_cache
            .get(&parent_key)
            .and_then(|groups| groups.iter().find(|g| g.key.first() == Some(key)).cloned());
        Ok(found)
    }

    /// Number of fetch calls issued to the underlying source.
    #[inline]
    pub fn fetch_calls(&self) -> usize {
        self.window.fetch_calls()
    }

    /// Number of pages currently cached in memory.
    #[inline]
    pub fn cached_pages(&self) -> usize {
        self.window.cached_pages_count()
    }

    /// Row count, announcing `Busy`/`Idle` when it has to ask the source.
    pub(crate) fn count(&mut self) -> Result<usize, SourceError> {
        if let Some(count) = self.window.known_len() {
            return Ok(count);
        }
        self.notify(CursorEvent::Busy);
        let result = self.window.len();
        self.notify(CursorEvent::Idle);
        result
    }

    /// Makes row `i` readable, announcing `Busy`/`Idle` only when a page is really fetched.
    fn load(&mut self, i: usize) -> Result<(), SourceError> {
        if !self.window.needs_fetch(i..i + 1) {
            return self.window.ensure(i..i + 1);
        }
        self.notify(CursorEvent::Busy);
        let result = self.window.ensure(i..i + 1);
        self.notify(CursorEvent::Idle);
        result
    }

    fn notify_scrolled(&mut self, rec_no: usize) {
        let event = CursorEvent::Scrolled { rec_no };
        for (_, listener) in &mut self.listeners {
            listener(&event);
        }
    }

    fn notify(&mut self, event: CursorEvent) {
        for (_, listener) in &mut self.listeners {
            listener(&event);
        }
    }
}

/// A lending iterator over all logical rows in a [`Cursor`], starting from row 0.
///
/// # Why a lending iterator?
/// A standard [`Iterator`] cannot borrow from `&mut self` across iterations without
/// per-row heap allocations. By using a lending pattern with [`RowsIter::next_row`],
/// each [`RowRef`] borrows directly from the cached page in [`WindowedRows`] with
/// zero allocations.
pub struct RowsIter<'a> {
    cursor: &'a mut Cursor,
    next_idx: usize,
    total: Option<usize>,
}

impl<'a> RowsIter<'a> {
    /// Returns the next row, or `None` if the end of the result set has been reached.
    pub fn next_row(&mut self) -> Result<Option<RowRef<'_>>, SourceError> {
        let total = match self.total {
            Some(t) => t,
            None => {
                let t = self.cursor.count()?;
                self.total = Some(t);
                t
            }
        };
        if self.next_idx >= total {
            return Ok(None);
        }
        let idx = self.next_idx;
        self.next_idx += 1;
        self.cursor.load(idx)?;
        let (_, rows, offset) = self
            .cursor
            .window
            .peek(idx)
            .ok_or(SourceError::OutOfRange {
                index: idx,
                len: total,
            })?;
        Ok(Some(RowRef {
            cursor: self.cursor,
            rows,
            offset,
            rec_no: idx,
        }))
    }

    /// Number of pages currently cached in memory.
    #[inline]
    pub fn cached_pages(&self) -> usize {
        self.cursor.cached_pages()
    }

    /// Number of fetch calls issued to the underlying source.
    #[inline]
    pub fn fetch_calls(&self) -> usize {
        self.cursor.fetch_calls()
    }
}

/// A borrowed reference to a single row during iteration via [`RowsIter`].
pub struct RowRef<'a> {
    cursor: &'a Cursor,
    rows: &'a Rows,
    offset: usize,
    rec_no: usize,
}

impl<'a> RowRef<'a> {
    /// Logical record index (0-based) of this row.
    #[inline]
    pub fn rec_no(&self) -> usize {
        self.rec_no
    }

    /// Stable row ID if provided by the source.
    #[inline]
    pub fn row_id(&self) -> Option<u64> {
        self.rows.ids.get(self.offset).copied()
    }

    /// Borrows a field by column name.
    pub fn field(&self, name: &str) -> Result<Field<'_>, SourceError> {
        let &col_idx = self
            .cursor
            .field_indices
            .get(name)
            .ok_or_else(|| SourceError::UnknownField(name.to_string()))?;
        self.field_by_index(col_idx)
    }

    /// Borrows a field by column ordinal index.
    pub fn field_by_index(&self, index: usize) -> Result<Field<'_>, SourceError> {
        if index >= self.cursor.schema.len() {
            return Err(SourceError::OutOfRange {
                index,
                len: self.cursor.schema.len(),
            });
        }
        let val = self.rows.columns[index].get(self.offset);
        Ok(Field {
            info: &self.cursor.schema[index],
            value: val,
        })
    }

    /// Iterates over all fields of this row in schema order.
    pub fn fields(&self) -> impl Iterator<Item = Field<'_>> {
        let rows = self.rows;
        let offset = self.offset;
        let schema = &self.cursor.schema;
        (0..schema.len()).map(move |i| Field {
            info: &schema[i],
            value: rows.columns[i].get(offset),
        })
    }
}

impl<'a> std::fmt::Debug for RowRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RowRef")
            .field("rec_no", &self.rec_no)
            .field("row_id", &self.row_id())
            .finish()
    }
}

/// A visible line yielded during iteration via [`LinesIter`].
#[derive(Debug)]
pub enum CursorLine<'a> {
    /// First line of a group: emitted before its first row, outer levels first.
    Header { level: usize, group: SourceGroup },
    /// A single data row.
    Row(RowRef<'a>),
}

/// A lending iterator over visible lines (group headers and rows) in a [`Cursor`], starting from row 0.
///
/// # Why a lending iterator?
/// A standard [`Iterator`] cannot borrow from `&mut self` across iterations without
/// per-row heap allocations. By using a lending pattern with [`LinesIter::next_line`],
/// each [`CursorLine::Row`] borrows directly from the cached page in [`WindowedRows`] with
/// zero allocations.
pub struct LinesIter<'a> {
    cursor: &'a mut Cursor,
    /// Ordinals of the group-by columns, outermost first (an unknown column is reported on
    /// the first `next_line`).
    group_cols: Result<Vec<usize>, SourceError>,
    next_idx: usize,
    total: Option<usize>,
    /// Group path of the last row whose headers were emitted.
    path: Option<Vec<Option<Owned>>>,
    /// Headers still to emit before row `next_idx`.
    pending: std::collections::VecDeque<(usize, SourceGroup)>,
}

impl<'a> LinesIter<'a> {
    /// Number of pages currently cached in memory.
    #[inline]
    pub fn cached_pages(&self) -> usize {
        self.cursor.cached_pages()
    }

    /// Number of fetch calls issued to the underlying source.
    #[inline]
    pub fn fetch_calls(&self) -> usize {
        self.cursor.fetch_calls()
    }

    /// Returns the next line — a group header or a row — or `None` past the last row.
    pub fn next_line(&mut self) -> Result<Option<CursorLine<'_>>, SourceError> {
        if let Some((level, group)) = self.pending.pop_front() {
            return Ok(Some(CursorLine::Header { level, group }));
        }
        let total = match self.total {
            Some(total) => total,
            None => *self.total.insert(self.cursor.count()?),
        };
        let idx = self.next_idx;
        if idx >= total {
            return Ok(None);
        }
        self.cursor.load(idx)?;

        let grouped = match &self.group_cols {
            Ok(cols) => !cols.is_empty(),
            Err(err) => return Err(err.clone()),
        };
        // once a row's headers are queued its path is the current one, so the same row
        // compares equal on the next call and is emitted
        if grouped && let Some(level) = self.first_changed_level(idx)? {
            self.queue_headers(idx, level)?;
            if let Some((level, group)) = self.pending.pop_front() {
                return Ok(Some(CursorLine::Header { level, group }));
            }
        }

        self.next_idx += 1;
        let (_, rows, offset) = self
            .cursor
            .window
            .peek(idx)
            .ok_or(SourceError::NotPositioned)?;
        Ok(Some(CursorLine::Row(RowRef {
            cursor: self.cursor,
            rows,
            offset,
            rec_no: idx,
        })))
    }

    /// The outermost level whose key differs from the previous row's (every level for the
    /// first row), comparing the cells in place — nothing is allocated while the path holds.
    fn first_changed_level(&self, idx: usize) -> Result<Option<usize>, SourceError> {
        let (Ok(group_cols), Some(path)) = (&self.group_cols, &self.path) else {
            return Ok(Some(0));
        };
        let (_, rows, offset) = self
            .cursor
            .window
            .peek(idx)
            .ok_or(SourceError::NotPositioned)?;
        Ok(group_cols
            .iter()
            .zip(path)
            .position(|(&col, key)| !cell_matches(&rows.columns[col].get(offset), key)))
    }

    /// Takes row `idx`'s group path and queues one header per level from `level` inward.
    fn queue_headers(&mut self, idx: usize, level: usize) -> Result<(), SourceError> {
        let group_cols = self.group_cols.as_ref().map_err(Clone::clone)?;
        let path: Vec<Option<Owned>> = {
            let (_, rows, offset) = self
                .cursor
                .window
                .peek(idx)
                .ok_or(SourceError::NotPositioned)?;
            group_cols
                .iter()
                .map(|&col| value_to_owned(rows.columns[col].get(offset)))
                .collect()
        };
        for lvl in level..path.len() {
            let group = self
                .cursor
                .cached_or_fetch_group(&path[..lvl], &path[lvl])?
                .ok_or_else(|| {
                    SourceError::Backend(format!(
                        "row {idx}: its group at level {lvl} is not in the source's groups"
                    ))
                })?;
            self.pending.push_back((lvl, group));
        }
        self.path = Some(path);
        Ok(())
    }
}

fn derive_query(query: &Query) -> Query {
    if query.group_by.is_empty() {
        return query.clone();
    }
    let mut sort = Vec::with_capacity(query.group_by.len() + query.sort.len());
    let mut seen_cols = std::collections::HashSet::new();
    for g in &query.group_by {
        if seen_cols.insert(g.column.clone()) {
            sort.push(SortKey {
                column: g.column.clone(),
                order: Order::Asc,
                nulls: Nulls::First,
            });
        }
    }
    for key in &query.sort {
        if seen_cols.insert(key.column.clone()) {
            sort.push(key.clone());
        }
    }
    Query {
        filter: query.filter.clone(),
        sort,
        group_by: query.group_by.clone(),
        summary: query.summary.clone(),
    }
}

fn value_to_owned(val: Value<'_>) -> Option<Owned> {
    match val {
        Value::Null => None,
        Value::Int(v) => Some(Owned::Int(v)),
        Value::Float(v) => Some(Owned::Float(v)),
        Value::Bool(b) => Some(Owned::Bool(b)),
        Value::Text(s) => Some(Owned::Text(s.to_string())),
    }
}

fn cell_matches(val: &Value<'_>, owned: &Option<Owned>) -> bool {
    match (val, owned) {
        (Value::Null, None) => true,
        (Value::Int(v), Some(Owned::Int(o))) => v == o,
        (Value::Float(v), Some(Owned::Float(o))) => {
            canonical_float(*v).to_bits() == canonical_float(*o).to_bits()
        }
        (Value::Bool(v), Some(Owned::Bool(o))) => v == o,
        (Value::Text(v), Some(Owned::Text(o))) => *v == o.as_str(),
        _ => false,
    }
}

#[inline]
fn canonical_float(v: f64) -> f64 {
    if v.is_nan() {
        f64::NAN
    } else if v == 0.0 {
        0.0
    } else {
        v
    }
}

#[cfg(test)]
#[path = "test/cursor/test.rs"]
mod test;
