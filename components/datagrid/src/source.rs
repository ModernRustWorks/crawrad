//! Pluggable row source abstraction for windowed data access.
//!
//! # Async Decision
//! The [`RowSource`] trait is synchronous and `Send + Sync`. Calls may block,
//! and the grid layer runs them off the UI thread. No asynchronous
//! executor or runtime is forced on the caller.

use crate::column::Column;
use crate::filter::{Filter, Owned};
use crate::sort::SortKey;
use crate::store::{Store, StoreError};
use crate::view::Query;

/// Column metadata declared by a row source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColumnInfo {
    pub name: String,
    pub kind: ColumnKind,
    pub nullable: bool,
    pub hint: DisplayHint,
}

/// Primitive column data types recognized across row sources.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnKind {
    Int,
    Float,
    Bool,
    Text,
}

/// Presentation hint for cell rendering.
///
/// `Date` and `DateTime` hints annotate `Int` columns holding days or milliseconds
/// since the Unix epoch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DisplayHint {
    #[default]
    Plain,
    Money,
    Percent,
    Date,
    DateTime,
}

/// What a source can do by itself. Anything false is done by the in-memory engine above it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Capabilities {
    pub filter: bool,
    pub sort: bool,
    pub group: bool,
    pub totals: bool,
    pub keyset: bool,
}

/// A columnar window of rows: `columns[c]` has exactly `len` cells, in the order of `schema()`.
#[derive(Clone, Debug, PartialEq)]
pub struct Rows {
    pub start: usize,
    pub len: usize,
    pub columns: Vec<Column>,
    pub ids: Vec<u64>,
}

/// A group of rows sharing the same key values and aggregated totals across row sources.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceGroup {
    pub key: Vec<Option<Owned>>,
    pub row_count: usize,
    pub totals: Vec<Option<f64>>,
}

/// Errors occurring during row source query execution or capability resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceError {
    Store(StoreError),
    Unsupported(&'static str),
    Backend(String),
    OutOfRange {
        index: usize,
        len: usize,
    },
    UnknownField(String),
    /// A cursor field was read before any move loaded its row.
    NotPositioned,
}

impl std::fmt::Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceError::Store(err) => write!(f, "store error: {}", err),
            SourceError::Unsupported(feature) => {
                write!(f, "unsupported source capability: {}", feature)
            }
            SourceError::Backend(msg) => write!(f, "backend error: {}", msg),
            SourceError::OutOfRange { index, len } => {
                write!(f, "index {} out of range (len {})", index, len)
            }
            SourceError::UnknownField(name) => write!(f, "unknown field: {}", name),
            SourceError::NotPositioned => {
                write!(
                    f,
                    "cursor is not positioned: move (or open) before reading fields"
                )
            }
        }
    }
}

impl std::error::Error for SourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SourceError::Store(err) => Some(err),
            _ => None,
        }
    }
}

impl From<StoreError> for SourceError {
    fn from(err: StoreError) -> Self {
        SourceError::Store(err)
    }
}

/// The distinct values of a column (`None` = NULL), as `RowSource::distinct` answers them.
pub type Distinct = Vec<Option<String>>;

/// One value to write: the column's name and its new value (`None` writes NULL).
pub type Change = (String, Option<Owned>);

/// Trait for resolving display labels and searching lookup datasets.
pub trait LookupSource: Send + Sync {
    /// Resolves display texts for a batch of lookup keys.
    fn labels(&self, keys: &[Owned]) -> Result<Vec<(Owned, Vec<String>)>, SourceError>;

    /// Searches the lookup dataset by prefix on the first display column, returning up to `limit` rows.
    fn search(&self, prefix: &str, limit: usize) -> Result<Vec<(Owned, Vec<String>)>, SourceError>;
}

/// Uniform row source trait implemented by all data backends.
///
/// Consumers request only a window of rows (`fetch`) and matching row counts (`count`).
pub trait RowSource: Send + Sync {
    /// Column metadata in ordinal order.
    fn schema(&self) -> &[ColumnInfo];

    /// Capabilities natively supported by the backend.
    fn capabilities(&self) -> Capabilities;

    /// Rows matching `query.filter` (grouping does not change the count).
    fn count(&self, query: &Query) -> Result<usize, SourceError>;

    /// Rows `range` of the filtered + sorted result. `range.end` beyond the count is clamped.
    fn fetch(&self, query: &Query, range: std::ops::Range<usize>) -> Result<Rows, SourceError>;

    /// Position of row `id` in the filtered + sorted result, or None if the filter excludes it.
    fn position_of(&self, query: &Query, id: u64) -> Result<Option<usize>, SourceError> {
        const PAGE_SIZE: usize = 4096;
        let total = self.count(query)?;
        let mut offset = 0;
        while offset < total {
            let end = (offset + PAGE_SIZE).min(total);
            let rows = self.fetch(query, offset..end)?;
            if let Some(pos) = rows.ids.iter().position(|&row_id| row_id == id) {
                return Ok(Some(offset + pos));
            }
            if rows.len == 0 {
                break;
            }
            offset += rows.len;
        }
        Ok(None)
    }

    /// Position of the first row (in query order) that also matches `filter`, with its id.
    fn locate(&self, query: &Query, filter: &Filter) -> Result<Option<(usize, u64)>, SourceError> {
        const PAGE_SIZE: usize = 4096;
        let total = self.count(query)?;
        let schema = self.schema();
        let mut offset = 0;
        while offset < total {
            let end = (offset + PAGE_SIZE).min(total);
            let rows = self.fetch(query, offset..end)?;
            if rows.len == 0 {
                break;
            }
            let mut store = Store::new();
            for (info, col) in schema.iter().zip(rows.columns) {
                store.add(&info.name, col)?;
            }
            let bm = store.filter(filter)?;
            if let Some(pos) = bm.ones().next() {
                let pos = pos as usize;
                return Ok(Some((offset + pos, rows.ids[pos])));
            }
            offset += rows.len;
        }
        Ok(None)
    }

    /// Groups rows matching `query` at hierarchy level `parent.len()` under `parent`.
    ///
    /// Default implementation returns `Err(SourceError::Unsupported("group"))`.
    fn groups(
        &self,
        query: &Query,
        parent: &[Option<Owned>],
    ) -> Result<Vec<SourceGroup>, SourceError> {
        let _ = (query, parent);
        Err(SourceError::Unsupported("group"))
    }

    /// Grand totals across all rows matching `query.filter` for each aggregate in `query.summary`.
    ///
    /// Default implementation returns `Err(SourceError::Unsupported("summary"))`.
    fn summary(&self, query: &Query) -> Result<Vec<Option<f64>>, SourceError> {
        let _ = query;
        Err(SourceError::Unsupported("summary"))
    }

    /// Distinct values for `column` across rows matching `query.filter`, up to `limit`.
    ///
    /// Default implementation returns `Err(SourceError::Unsupported("distinct"))`.
    fn distinct(&self, query: &Query, column: &str, limit: usize) -> Result<Distinct, SourceError> {
        let _ = (query, column, limit);
        Err(SourceError::Unsupported("distinct"))
    }

    /// Whether [`RowSource::update`] and [`RowSource::insert`] write through (#936 P2).
    fn can_write(&self) -> bool {
        false
    }

    /// The column that identifies a row (the id `update` takes). The grid never edits it:
    /// the database owns it, like an auto-increment field.
    fn key_column(&self) -> Option<&str> {
        None
    }

    /// Writes `changes` into the row whose id is `id`, all in one statement.
    ///
    /// Default implementation returns `Err(SourceError::Unsupported("update"))`.
    fn update(&self, id: u64, changes: &[Change]) -> Result<(), SourceError> {
        let _ = (id, changes);
        Err(SourceError::Unsupported("update"))
    }

    /// Adds a row holding `values` (columns left out take their database default)
    /// and returns its id.
    ///
    /// Default implementation returns `Err(SourceError::Unsupported("insert"))`.
    fn insert(&self, values: &[Change]) -> Result<u64, SourceError> {
        let _ = values;
        Err(SourceError::Unsupported("insert"))
    }
}

/// The last query's filtered + sorted row order, keyed by what decides it.
type OrderCache = Option<(RowOrderKey, Vec<u32>)>;

/// An in-memory columnar row source backed by a [`Store`].
///
/// Query execution uses the in-memory engine (`Store::filter`, `Store::sort`)
/// and caches the latest query's sorted permutation so that scrolling through
/// successive windows of the same query avoids redundant filter/sort passes.
pub struct VecSource {
    store: Store,
    schema: Vec<ColumnInfo>,
    capabilities: Capabilities,
    /// Keyed by filter + sort only: grouping and totals do not change the row order.
    cache: std::sync::Mutex<OrderCache>,
    #[cfg(test)]
    pub(crate) execution_count: std::sync::atomic::AtomicUsize,
}

impl VecSource {
    /// Constructs a `VecSource` from an existing `Store` and explicit `schema`.
    ///
    /// Validates that schema names and kinds match the store's columns.
    pub fn new(store: Store, schema: Vec<ColumnInfo>) -> Result<Self, SourceError> {
        let mut seen = std::collections::HashSet::with_capacity(schema.len());
        for info in &schema {
            if !seen.insert(&info.name) {
                return Err(SourceError::Store(StoreError::DuplicateColumn(
                    info.name.clone(),
                )));
            }
            let col = store
                .column(&info.name)
                .ok_or_else(|| SourceError::Store(StoreError::ColumnNotFound(info.name.clone())))?;
            let actual_kind = match col {
                Column::Int { .. } => ColumnKind::Int,
                Column::Float { .. } => ColumnKind::Float,
                Column::Bool { .. } => ColumnKind::Bool,
                Column::Text { .. } | Column::Dict { .. } => ColumnKind::Text,
            };
            if info.kind != actual_kind {
                let expected = match actual_kind {
                    ColumnKind::Int => "Int",
                    ColumnKind::Float => "Float",
                    ColumnKind::Bool => "Bool",
                    ColumnKind::Text => "Text",
                };
                return Err(SourceError::Store(StoreError::TypeMismatch {
                    column: info.name.clone(),
                    expected,
                }));
            }
        }

        if schema.len() != store.columns_count() {
            return Err(SourceError::Store(StoreError::LengthMismatch {
                expected: store.columns_count(),
                got: schema.len(),
            }));
        }

        Ok(Self {
            store,
            schema,
            capabilities: Capabilities {
                filter: true,
                sort: true,
                group: true,
                totals: true,
                keyset: false,
            },
            cache: std::sync::Mutex::new(None),
            #[cfg(test)]
            execution_count: std::sync::atomic::AtomicUsize::new(0),
        })
    }

    /// Constructs a `VecSource` by deriving the schema directly from `store`.
    ///
    /// Mappings: `Dict` -> `ColumnKind::Text`, `nullable` = has a validity bitmap, hint `DisplayHint::Plain`.
    pub fn from_store(store: Store) -> Self {
        let mut schema = Vec::with_capacity(store.columns_count());
        for (name, col) in store.column_names().iter().zip(store.columns().iter()) {
            let kind = match col {
                Column::Int { .. } => ColumnKind::Int,
                Column::Float { .. } => ColumnKind::Float,
                Column::Bool { .. } => ColumnKind::Bool,
                Column::Text { .. } | Column::Dict { .. } => ColumnKind::Text,
            };
            let nullable = col.validity().is_some();
            schema.push(ColumnInfo {
                name: name.clone(),
                kind,
                nullable,
                hint: DisplayHint::Plain,
            });
        }
        Self {
            store,
            schema,
            capabilities: Capabilities {
                filter: true,
                sort: true,
                group: true,
                totals: true,
                keyset: false,
            },
            cache: std::sync::Mutex::new(None),
            #[cfg(test)]
            execution_count: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Overrides the declared capabilities of this source.
    pub fn with_capabilities(mut self, capabilities: Capabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Returns a reference to the underlying store.
    #[inline]
    pub fn store(&self) -> &Store {
        &self.store
    }

    /// Returns the number of filter/sort runs performed.
    #[cfg(test)]
    pub fn execution_count(&self) -> usize {
        self.execution_count
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    fn ensure_cache(
        &self,
        query: &Query,
    ) -> Result<std::sync::MutexGuard<'_, OrderCache>, SourceError> {
        let mut cache_guard = self
            .cache
            .lock()
            .map_err(|_| SourceError::Backend("cache mutex poisoned".to_string()))?;

        let need_compute = match cache_guard.as_ref() {
            Some((cached_key, _)) => !cached_key.matches(query),
            None => true,
        };

        if need_compute {
            #[cfg(test)]
            self.execution_count
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            let filtered_rows: Vec<u32> = match &query.filter {
                Some(f) => {
                    let bm = self.store.filter(f)?;
                    bm.ones().collect()
                }
                None => (0..self.store.rows() as u32).collect(),
            };

            let sorted_rows = self.store.sort(&filtered_rows, &query.sort)?;
            *cache_guard = Some((RowOrderKey::of(query), sorted_rows));
        }

        Ok(cache_guard)
    }
}

/// The part of a query that decides which rows a window holds and in what order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RowOrderKey {
    filter: Option<Filter>,
    sort: Vec<SortKey>,
}

impl RowOrderKey {
    fn of(query: &Query) -> Self {
        Self {
            filter: query.filter.clone(),
            sort: query.sort.clone(),
        }
    }

    fn matches(&self, query: &Query) -> bool {
        self.filter == query.filter && self.sort == query.sort
    }
}

impl RowSource for VecSource {
    fn schema(&self) -> &[ColumnInfo] {
        &self.schema
    }

    fn capabilities(&self) -> Capabilities {
        self.capabilities
    }

    fn count(&self, query: &Query) -> Result<usize, SourceError> {
        if query.filter.is_some() && !self.capabilities.filter {
            return Err(SourceError::Unsupported("filter"));
        }

        let cache_guard = self
            .cache
            .lock()
            .map_err(|_| SourceError::Backend("cache mutex poisoned".to_string()))?;

        if let Some((cached_key, sorted_rows)) = cache_guard.as_ref()
            && cached_key.matches(query)
        {
            return Ok(sorted_rows.len());
        }
        drop(cache_guard);

        let n = match &query.filter {
            Some(f) => {
                let bm = self.store.filter(f)?;
                bm.count_ones()
            }
            None => self.store.rows(),
        };
        Ok(n)
    }

    fn fetch(&self, query: &Query, range: std::ops::Range<usize>) -> Result<Rows, SourceError> {
        if query.filter.is_some() && !self.capabilities.filter {
            return Err(SourceError::Unsupported("filter"));
        }
        if !query.sort.is_empty() && !self.capabilities.sort {
            return Err(SourceError::Unsupported("sort"));
        }
        if !query.group_by.is_empty() && !self.capabilities.group {
            return Err(SourceError::Unsupported("group"));
        }
        if query.group_by.iter().any(|g| !g.totals.is_empty()) && !self.capabilities.totals {
            return Err(SourceError::Unsupported("totals"));
        }

        let cache_guard = self.ensure_cache(query)?;
        let (_, sorted_rows) = cache_guard.as_ref().expect("cache populated");
        let total = sorted_rows.len();
        let start = range.start.min(total);
        let end = range.end.min(total);
        let (start, end) = if start <= end {
            (start, end)
        } else {
            (start, start)
        };
        let len = end - start;
        let window_rows = &sorted_rows[start..end];
        let ids: Vec<u64> = window_rows.iter().map(|&id| id as u64).collect();

        let mut columns = Vec::with_capacity(self.schema.len());
        for info in &self.schema {
            let col = self
                .store
                .column(&info.name)
                .ok_or_else(|| SourceError::Store(StoreError::ColumnNotFound(info.name.clone())))?;
            columns.push(col.gather(window_rows));
        }

        Ok(Rows {
            start,
            len,
            columns,
            ids,
        })
    }

    fn position_of(&self, query: &Query, id: u64) -> Result<Option<usize>, SourceError> {
        if query.filter.is_some() && !self.capabilities.filter {
            return Err(SourceError::Unsupported("filter"));
        }
        if !query.sort.is_empty() && !self.capabilities.sort {
            return Err(SourceError::Unsupported("sort"));
        }

        if id >= self.store.rows() as u64 {
            return Ok(None);
        }

        let cache_guard = self.ensure_cache(query)?;
        let (_, sorted_rows) = cache_guard.as_ref().expect("cache populated");
        let target = id as u32;
        let pos = sorted_rows.iter().position(|&r| r == target);
        Ok(pos)
    }

    fn locate(&self, query: &Query, filter: &Filter) -> Result<Option<(usize, u64)>, SourceError> {
        if !self.capabilities.filter {
            return Err(SourceError::Unsupported("filter"));
        }
        if !query.sort.is_empty() && !self.capabilities.sort {
            return Err(SourceError::Unsupported("sort"));
        }

        let bm = self.store.filter(filter)?;
        let cache_guard = self.ensure_cache(query)?;
        let (_, sorted_rows) = cache_guard.as_ref().expect("cache populated");

        for (pos, &row_idx) in sorted_rows.iter().enumerate() {
            if bm.get(row_idx as usize) {
                return Ok(Some((pos, row_idx as u64)));
            }
        }
        Ok(None)
    }

    fn groups(
        &self,
        query: &Query,
        parent: &[Option<Owned>],
    ) -> Result<Vec<SourceGroup>, SourceError> {
        if query.filter.is_some() && !self.capabilities.filter {
            return Err(SourceError::Unsupported("filter"));
        }
        if !self.capabilities.group {
            return Err(SourceError::Unsupported("group"));
        }

        let level = parent.len();
        if level >= query.group_by.len() {
            return Ok(Vec::new());
        }

        let group_level = &query.group_by[level];
        if !group_level.totals.is_empty() && !self.capabilities.totals {
            return Err(SourceError::Unsupported("totals"));
        }

        let mut parent_pairs = Vec::with_capacity(level);
        for (i, p_val) in parent.iter().enumerate() {
            parent_pairs.push((query.group_by[i].column.as_str(), p_val.clone()));
        }
        let scoped_query = query.within_group(&parent_pairs);

        let filtered_rows: Vec<u32> = match &scoped_query.filter {
            Some(f) => {
                let bm = self.store.filter(f)?;
                bm.ones().collect()
            }
            None => (0..self.store.rows() as u32).collect(),
        };

        let groups = self.store.group(
            &filtered_rows,
            std::slice::from_ref(&group_level.column),
            &group_level.totals,
        )?;

        let mut source_groups = Vec::with_capacity(groups.len());
        for grp in groups {
            let ord = grp.key.first().copied().flatten();
            let val = match self.store.key_value(&group_level.column, ord)? {
                crate::column::Value::Null => None,
                crate::column::Value::Int(v) => Some(Owned::Int(v)),
                crate::column::Value::Float(v) => Some(Owned::Float(v)),
                crate::column::Value::Bool(v) => Some(Owned::Bool(v)),
                crate::column::Value::Text(v) => Some(Owned::Text(v.to_string())),
            };
            source_groups.push(SourceGroup {
                key: vec![val],
                row_count: grp.rows.len(),
                totals: grp.totals,
            });
        }

        Ok(source_groups)
    }

    fn summary(&self, query: &Query) -> Result<Vec<Option<f64>>, SourceError> {
        if query.summary.is_empty() {
            return Ok(Vec::new());
        }
        if query.filter.is_some() && !self.capabilities.filter {
            return Err(SourceError::Unsupported("filter"));
        }
        if !self.capabilities.totals {
            return Err(SourceError::Unsupported("totals"));
        }

        let filtered_rows: Vec<u32> = match &query.filter {
            Some(f) => {
                let bm = self.store.filter(f)?;
                bm.ones().collect()
            }
            None => (0..self.store.rows() as u32).collect(),
        };

        let groups = self.store.group(&filtered_rows, &[], &query.summary)?;
        if let Some(first) = groups.into_iter().next() {
            Ok(first.totals)
        } else {
            Ok(vec![None; query.summary.len()])
        }
    }

    fn distinct(&self, query: &Query, column: &str, limit: usize) -> Result<Distinct, SourceError> {
        if query.filter.is_some() && !self.capabilities.filter {
            return Err(SourceError::Unsupported("filter"));
        }

        let col = self
            .store
            .column(column)
            .ok_or_else(|| SourceError::Store(StoreError::ColumnNotFound(column.to_string())))?;

        let matching_rows: Vec<u32> = match &query.filter {
            Some(f) => {
                let bm = self.store.filter(f)?;
                bm.ones().collect()
            }
            None => (0..self.store.rows() as u32).collect(),
        };

        let mut values: Vec<Option<String>> = matching_rows
            .into_iter()
            .map(|r| match col.get(r as usize) {
                crate::column::Value::Null => None,
                crate::column::Value::Int(v) => Some(v.to_string()),
                crate::column::Value::Float(v) => Some(v.to_string()),
                crate::column::Value::Bool(v) => Some(v.to_string()),
                crate::column::Value::Text(v) => Some(v.to_string()),
            })
            .collect();

        values.sort();
        values.dedup();
        values.truncate(limit);
        Ok(values)
    }
}

/// Telemetry statistics recorded by [`FakeSlowSource`].
#[cfg(any(test, feature = "test-sources"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FetchStats {
    pub count_calls: usize,
    pub fetch_calls: usize,
    pub total_rows_returned: usize,
    pub max_window_requested: usize,
    pub position_calls: usize,
    pub locate_calls: usize,
}

/// A test source that wraps another [`RowSource`] with simulated latency and telemetry.
#[cfg(any(test, feature = "test-sources"))]
pub struct FakeSlowSource {
    inner: Box<dyn RowSource>,
    delay: std::time::Duration,
    custom_capabilities: Option<Capabilities>,
    count_calls: std::sync::atomic::AtomicUsize,
    fetch_calls: std::sync::atomic::AtomicUsize,
    total_rows_returned: std::sync::atomic::AtomicUsize,
    max_window_requested: std::sync::atomic::AtomicUsize,
    position_calls: std::sync::atomic::AtomicUsize,
    locate_calls: std::sync::atomic::AtomicUsize,
}

#[cfg(any(test, feature = "test-sources"))]
impl FakeSlowSource {
    /// Wraps `inner` with a simulated call delay.
    pub fn new(inner: Box<dyn RowSource>, delay: std::time::Duration) -> Self {
        Self {
            inner,
            delay,
            custom_capabilities: None,
            count_calls: std::sync::atomic::AtomicUsize::new(0),
            fetch_calls: std::sync::atomic::AtomicUsize::new(0),
            total_rows_returned: std::sync::atomic::AtomicUsize::new(0),
            max_window_requested: std::sync::atomic::AtomicUsize::new(0),
            position_calls: std::sync::atomic::AtomicUsize::new(0),
            locate_calls: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Wraps `inner` with a simulated call delay and overridden capabilities.
    pub fn with_capabilities(
        inner: Box<dyn RowSource>,
        delay: std::time::Duration,
        capabilities: Capabilities,
    ) -> Self {
        Self {
            inner,
            delay,
            custom_capabilities: Some(capabilities),
            count_calls: std::sync::atomic::AtomicUsize::new(0),
            fetch_calls: std::sync::atomic::AtomicUsize::new(0),
            total_rows_returned: std::sync::atomic::AtomicUsize::new(0),
            max_window_requested: std::sync::atomic::AtomicUsize::new(0),
            position_calls: std::sync::atomic::AtomicUsize::new(0),
            locate_calls: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Returns the accumulated fetch telemetry.
    pub fn stats(&self) -> FetchStats {
        FetchStats {
            count_calls: self.count_calls.load(std::sync::atomic::Ordering::Relaxed),
            fetch_calls: self.fetch_calls.load(std::sync::atomic::Ordering::Relaxed),
            total_rows_returned: self
                .total_rows_returned
                .load(std::sync::atomic::Ordering::Relaxed),
            max_window_requested: self
                .max_window_requested
                .load(std::sync::atomic::Ordering::Relaxed),
            position_calls: self
                .position_calls
                .load(std::sync::atomic::Ordering::Relaxed),
            locate_calls: self.locate_calls.load(std::sync::atomic::Ordering::Relaxed),
        }
    }
}

#[cfg(any(test, feature = "test-sources"))]
impl FakeSlowSource {
    /// The delay, then the filter capability check every call shares.
    fn admit(&self, query: &Query) -> Result<Capabilities, SourceError> {
        if !self.delay.is_zero() {
            std::thread::sleep(self.delay);
        }
        let caps = self.capabilities();
        if query.filter.is_some() && !caps.filter {
            return Err(SourceError::Unsupported("filter"));
        }
        Ok(caps)
    }
}

#[cfg(any(test, feature = "test-sources"))]
impl RowSource for FakeSlowSource {
    fn schema(&self) -> &[ColumnInfo] {
        self.inner.schema()
    }

    fn capabilities(&self) -> Capabilities {
        self.custom_capabilities
            .unwrap_or_else(|| self.inner.capabilities())
    }

    fn count(&self, query: &Query) -> Result<usize, SourceError> {
        if !self.delay.is_zero() {
            std::thread::sleep(self.delay);
        }
        self.count_calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let caps = self.capabilities();
        if query.filter.is_some() && !caps.filter {
            return Err(SourceError::Unsupported("filter"));
        }

        self.inner.count(query)
    }

    fn fetch(&self, query: &Query, range: std::ops::Range<usize>) -> Result<Rows, SourceError> {
        if !self.delay.is_zero() {
            std::thread::sleep(self.delay);
        }
        self.fetch_calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let requested_len = range.end.saturating_sub(range.start);
        self.max_window_requested
            .fetch_max(requested_len, std::sync::atomic::Ordering::Relaxed);

        let caps = self.capabilities();
        if query.filter.is_some() && !caps.filter {
            return Err(SourceError::Unsupported("filter"));
        }
        if !query.sort.is_empty() && !caps.sort {
            return Err(SourceError::Unsupported("sort"));
        }
        if !query.group_by.is_empty() && !caps.group {
            return Err(SourceError::Unsupported("group"));
        }
        if query.group_by.iter().any(|g| !g.totals.is_empty()) && !caps.totals {
            return Err(SourceError::Unsupported("totals"));
        }

        let rows = self.inner.fetch(query, range)?;
        self.total_rows_returned
            .fetch_add(rows.len, std::sync::atomic::Ordering::Relaxed);
        Ok(rows)
    }

    fn position_of(&self, query: &Query, id: u64) -> Result<Option<usize>, SourceError> {
        if !self.delay.is_zero() {
            std::thread::sleep(self.delay);
        }
        self.position_calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let caps = self.capabilities();
        if query.filter.is_some() && !caps.filter {
            return Err(SourceError::Unsupported("filter"));
        }
        if !query.sort.is_empty() && !caps.sort {
            return Err(SourceError::Unsupported("sort"));
        }

        self.inner.position_of(query, id)
    }

    fn locate(&self, query: &Query, filter: &Filter) -> Result<Option<(usize, u64)>, SourceError> {
        if !self.delay.is_zero() {
            std::thread::sleep(self.delay);
        }
        self.locate_calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let caps = self.capabilities();
        if !caps.filter {
            return Err(SourceError::Unsupported("filter"));
        }
        if !query.sort.is_empty() && !caps.sort {
            return Err(SourceError::Unsupported("sort"));
        }

        self.inner.locate(query, filter)
    }

    fn groups(
        &self,
        query: &Query,
        parent: &[Option<Owned>],
    ) -> Result<Vec<SourceGroup>, SourceError> {
        let caps = self.admit(query)?;
        if !caps.group {
            return Err(SourceError::Unsupported("group"));
        }
        let level = parent.len();
        if level < query.group_by.len() && !query.group_by[level].totals.is_empty() && !caps.totals
        {
            return Err(SourceError::Unsupported("totals"));
        }
        self.inner.groups(query, parent)
    }

    fn summary(&self, query: &Query) -> Result<Vec<Option<f64>>, SourceError> {
        let caps = self.admit(query)?;
        if !query.summary.is_empty() && !caps.totals {
            return Err(SourceError::Unsupported("totals"));
        }
        self.inner.summary(query)
    }

    fn distinct(&self, query: &Query, column: &str, limit: usize) -> Result<Distinct, SourceError> {
        self.admit(query)?;
        self.inner.distinct(query, column, limit)
    }
}

#[cfg(test)]
#[path = "test/source/test.rs"]
mod test;
