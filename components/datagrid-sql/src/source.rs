//! SQL-backed [`RowSource`] implementation via `portadb`.
//!
//! # Synchronous by Contract
//!
//! As decided in #669, the [`RowSource`] trait is synchronous and
//! `Send + Sync`. Data operations may block, and callers (such as grid and virtual viewport layers)
//! execute them off the UI thread.
//!
//! `SqlSource` constructs a dedicated single-threaded [`tokio::runtime::Runtime`] upon instantiation
//! and blocks synchronously (`runtime.block_on(...)`) on database operations.

use std::collections::{BTreeMap, HashMap};
use std::ops::ControlFlow;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use datagrid::bitmap::Bitmap;
use datagrid::column::Column;
use datagrid::filter::{Filter, Owned};
use datagrid::group::Aggregate;
use datagrid::query::Query;
use datagrid::sort::SortKey;
use datagrid::source::{
    Capabilities, Change, ColumnInfo, ColumnKind, DisplayHint, Distinct, RowSource, Rows,
    SourceError, SourceGroup,
};
use datagrid::store::StoreError;
use portadb::connection::Connection;
use portadb::dialect::SqliteDialect;
use portadb::drivers::RusqliteDriver;
use portadb::options::ConnectOptions;
use portadb::params::NamedParams;
use portadb::row_ref::{RowRef, RowVisitor, ValueRef};

use crate::translate::reversed;
use crate::translate::{
    Table, effective_sort_keys, translate_checkpoints, translate_count, translate_count_before,
    translate_distinct, translate_fetch_keyset, translate_fetch_offset, translate_groups,
    translate_insert, translate_locate, translate_sort_tuple_of, translate_summary,
    translate_update,
};

/// The grid kind of a column declared with SQLite type `declared`.
///
/// SQLite's own affinity rules, in the order SQLite applies them: `INT`
/// anywhere is an integer, then text, then real. A type SQLite would give
/// NUMERIC affinity (`DECIMAL`, `DATE`…) is shown as text, which loses nothing.
pub fn sqlite_kind(declared: &str) -> ColumnKind {
    let upper = declared.to_ascii_uppercase();
    if upper.contains("INT") {
        ColumnKind::Int
    } else if upper.contains("CHAR") || upper.contains("TEXT") || upper.contains("CLOB") {
        ColumnKind::Text
    } else if upper.contains("REAL") || upper.contains("FLOA") || upper.contains("DOUB") {
        ColumnKind::Float
    } else if upper.contains("BOOL") {
        ColumnKind::Bool
    } else {
        ColumnKind::Text
    }
}

/// Refuses a SQLite url whose file is not there: opening it would CREATE an
/// empty database, and a grid reading it would only say "no such table".
fn refuse_missing_file(url: &str) -> Result<(), SourceError> {
    let url = url.trim();
    let file = url
        .strip_prefix("sqlite://")
        .or_else(|| url.strip_prefix("sqlite:"))
        .filter(|p| !p.is_empty() && *p != ":memory:" && !p.starts_with("file:"));
    match file {
        Some(path) if !std::path::Path::new(path).is_file() => {
            let full = std::path::absolute(path).unwrap_or_else(|_| path.into());
            Err(SourceError::Backend(format!(
                "database file not found: {}",
                full.display()
            )))
        }
        _ => Ok(()),
    }
}

/// `[A-Za-z_][A-Za-z0-9_]*`: a table name that is safe to splice into SQL.
fn is_plain_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Normalized query signature identifying unique filter and sort order configurations for bookmark caching.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct NormalizedQueryKey {
    filter: Option<Filter>,
    sort: Vec<SortKey>,
}

impl NormalizedQueryKey {
    fn of(query: &Query) -> Self {
        let norm = query.normalized();
        Self {
            filter: norm.filter,
            sort: norm.sort,
        }
    }
}

/// Per query, the key tuple of the row just before each remembered position.
/// Per query, the stride and the tuple of the row just before each multiple of it.
type Checkpoints = HashMap<NormalizedQueryKey, (usize, Vec<Tuple>)>;

type Bookmarks = HashMap<NormalizedQueryKey, BTreeMap<usize, Vec<Option<Owned>>>>;

/// An active SQL-backed row source implementation.
///
/// Executes queries against an underlying database using `portadb` and translates
/// datagrid [`Query`] structures into native SQL expressions.
pub struct SqlSource {
    table: Table,
    /// The column that is the table's own key when `table.key` is SQLite's `rowid`: its
    /// single `INTEGER PRIMARY KEY`, which is an alias of `rowid`.
    rowid_alias: Option<String>,
    /// A query's rows (a Dataset's `sql`) are read-only: they are a snapshot, not a table.
    read_only: bool,
    conn: Mutex<Connection>,
    rt: tokio::runtime::Runtime,
    capabilities: Capabilities,
    bookmarks: Mutex<Bookmarks>,
    checkpoints: Mutex<Checkpoints>,
    /// Row count per normalized filter, for the paging decisions (read-only data, like the bookmarks).
    counts: Mutex<HashMap<Option<Filter>, usize>>,
    tail_fetches: AtomicUsize,
    keyset_fetches: AtomicUsize,
    offset_fetches: AtomicUsize,
    checkpoint_builds: AtomicUsize,
}

impl SqlSource {
    /// Opens an SQLite database connection at `url` and binds to `table`.
    ///
    /// Natively supports filter, sort, group, totals, and keyset capabilities.
    pub fn open_sqlite(url: impl AsRef<str>, table: Table) -> Result<Self, SourceError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| {
                SourceError::Backend(format!("failed to initialize tokio runtime: {e}"))
            })?;

        let options = ConnectOptions::new(url.as_ref());
        let conn = rt
            .block_on(async { Connection::open(RusqliteDriver::new(), &options).await })
            .map_err(|e| SourceError::Backend(format!("database connection failed: {e}")))?;

        Ok(Self {
            table,
            rowid_alias: None,
            read_only: false,
            conn: Mutex::new(conn),
            rt,
            capabilities: Capabilities {
                filter: true,
                sort: true,
                group: true,
                totals: true,
                keyset: true,
            },
            bookmarks: Mutex::new(HashMap::new()),
            checkpoints: Mutex::new(HashMap::new()),
            keyset_fetches: AtomicUsize::new(0),
            offset_fetches: AtomicUsize::new(0),
            checkpoint_builds: AtomicUsize::new(0),
            counts: Mutex::new(HashMap::new()),
            tail_fetches: AtomicUsize::new(0),
        })
    }

    /// Opens the SQLite table `table` at `url`, reading its columns from the database.
    ///
    /// A form names a table, not its columns: the schema the grid shows is the
    /// one the database has now, so a column added later appears without
    /// touching the form. The key is SQLite's `rowid`.
    pub fn open_sqlite_table(url: impl AsRef<str>, table: &str) -> Result<Self, SourceError> {
        // The name is spliced into the PRAGMA (it cannot be a parameter), so
        // anything that is not a plain identifier is refused before any SQL.
        if !is_plain_identifier(table) {
            return Err(SourceError::Backend(format!("invalid table name: {table}")));
        }
        refuse_missing_file(url.as_ref())?;
        let mut source = Self::open_sqlite(url, Table::sqlite(table, Vec::new()))?;
        let (columns, rowid_alias) = source.read_columns(table)?;
        if columns.is_empty() {
            return Err(SourceError::Backend(format!("table not found: {table}")));
        }
        source.table.columns = columns;
        source.rowid_alias = rowid_alias;
        Ok(source)
    }

    /// The name of the temporary table a query's rows are read into.
    pub const QUERY_TABLE: &'static str = "crawrad_dataset_query";

    /// Opens the rows of the query `sql` at `url`, read-only (a Dataset's `sql`).
    ///
    /// The rows are read once into a temporary table on this source's own
    /// connection, in the query's order, so filter, sort, group and keyset
    /// paging work as on a table. The query is wrapped as a subquery: it is
    /// read, never spliced into anything that writes.
    pub fn open_sqlite_query(url: impl AsRef<str>, sql: &str) -> Result<Self, SourceError> {
        let sql = sql.trim().trim_end_matches(';').trim();
        if sql.is_empty() {
            return Err(SourceError::Backend("the dataset has no SQL".to_string()));
        }
        refuse_missing_file(url.as_ref())?;
        let mut source = Self::open_sqlite(url, Table::sqlite(Self::QUERY_TABLE, Vec::new()))?;
        source.execute(
            &format!(
                "CREATE TEMP TABLE \"{}\" AS SELECT * FROM ({sql})",
                Self::QUERY_TABLE
            ),
            &NamedParams::new(),
        )?;
        let (columns, _) = source.read_columns(Self::QUERY_TABLE)?;
        source.table.columns = columns;
        source.read_only = true;
        Ok(source)
    }

    /// The columns of `table` in ordinal order, as `PRAGMA table_info` lists them, and its
    /// `rowid` alias (a lone `INTEGER PRIMARY KEY` column).
    fn read_columns(&self, table: &str) -> Result<(Vec<ColumnInfo>, Option<String>), SourceError> {
        struct SchemaVisitor {
            out: Vec<ColumnInfo>,
            primary: Vec<(String, String)>,
        }
        impl RowVisitor for SchemaVisitor {
            fn visit<R: RowRef>(&mut self, row: &R) -> Result<ControlFlow<()>, portadb::Error> {
                // PRAGMA table_info: cid, name, type, notnull, dflt_value, pk.
                let name = match row.value(1)? {
                    ValueRef::Text(s) => s.to_string(),
                    _ => return Ok(ControlFlow::Continue(())),
                };
                let declared = match row.value(2)? {
                    ValueRef::Text(s) => s.to_string(),
                    _ => String::new(),
                };
                let not_null = matches!(row.value(3)?, ValueRef::Integer(1));
                if matches!(row.value(5)?, ValueRef::Integer(n) if n > 0) {
                    self.primary.push((name.clone(), declared.clone()));
                }
                self.out.push(ColumnInfo {
                    name,
                    kind: sqlite_kind(&declared),
                    nullable: !not_null,
                    hint: DisplayHint::Plain,
                });
                Ok(ControlFlow::Continue(()))
            }
        }
        let sql = format!("PRAGMA table_info(\"{table}\")");
        let mut visitor = SchemaVisitor {
            out: Vec::new(),
            primary: Vec::new(),
        };
        let mut guard = self
            .conn
            .lock()
            .map_err(|_| SourceError::Backend("connection lock poisoned".to_string()))?;
        self.rt
            .block_on(async {
                guard
                    .query_each_ref(&sql, &NamedParams::new(), &mut visitor)
                    .await
            })
            .map_err(|e| SourceError::Backend(e.to_string()))?;
        let alias = match visitor.primary.as_slice() {
            [(name, declared)] if declared.eq_ignore_ascii_case("INTEGER") => Some(name.clone()),
            _ => None,
        };
        Ok((visitor.out, alias))
    }

    /// Wraps an existing [`Connection`] and metadata [`Table`] into a `SqlSource`.
    pub fn from_connection(conn: Connection, table: Table) -> Result<Self, SourceError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| {
                SourceError::Backend(format!("failed to initialize tokio runtime: {e}"))
            })?;

        Ok(Self {
            table,
            rowid_alias: None,
            read_only: false,
            conn: Mutex::new(conn),
            rt,
            capabilities: Capabilities {
                filter: true,
                sort: true,
                group: true,
                totals: true,
                keyset: true,
            },
            bookmarks: Mutex::new(HashMap::new()),
            checkpoints: Mutex::new(HashMap::new()),
            keyset_fetches: AtomicUsize::new(0),
            offset_fetches: AtomicUsize::new(0),
            checkpoint_builds: AtomicUsize::new(0),
            counts: Mutex::new(HashMap::new()),
            tail_fetches: AtomicUsize::new(0),
        })
    }

    /// Returns the number of fetch operations performed using keyset pagination.
    pub fn keyset_fetches(&self) -> usize {
        self.keyset_fetches.load(Ordering::Relaxed)
    }

    /// Returns the number of fetch operations performed using offset pagination.
    pub fn offset_fetches(&self) -> usize {
        self.offset_fetches.load(Ordering::Relaxed)
    }

    /// Returns the number of checkpoint build operations performed.
    pub fn checkpoint_builds(&self) -> usize {
        self.checkpoint_builds.load(Ordering::Relaxed)
    }

    /// Resets the keyset, offset, and checkpoint build telemetry counters.
    pub fn reset_counters(&self) {
        self.keyset_fetches.store(0, Ordering::Relaxed);
        self.offset_fetches.store(0, Ordering::Relaxed);
        self.checkpoint_builds.store(0, Ordering::Relaxed);
        self.tail_fetches.store(0, Ordering::Relaxed);
    }

    /// Clears the bookmark and checkpoint caches.
    pub fn clear_cache(&self) {
        if let Ok(mut bm) = self.bookmarks.lock() {
            bm.clear();
        }
        if let Ok(mut cp) = self.checkpoints.lock() {
            cp.clear();
        }
        if let Ok(mut counts) = self.counts.lock() {
            counts.clear();
        }
    }

    /// Returns the number of windows read backwards from the end of the result.
    pub fn tail_fetches(&self) -> usize {
        self.tail_fetches.load(Ordering::Relaxed)
    }

    /// Helper to execute an arbitrary SQL statement with named parameters.
    pub fn execute(
        &self,
        sql: &str,
        params: &portadb::params::NamedParams,
    ) -> Result<u64, SourceError> {
        let mut guard = self
            .conn
            .lock()
            .map_err(|_| SourceError::Backend("connection mutex poisoned".to_string()))?;
        self.rt
            .block_on(async { guard.execute(sql, params).await })
            .map_err(|e| SourceError::Backend(e.to_string()))
    }

    /// Runs `sql` on the connection and feeds every row to `visitor`.
    fn visit_rows<V: RowVisitor + Send>(
        &self,
        sql: &str,
        params: &NamedParams,
        visitor: &mut V,
    ) -> Result<(), SourceError> {
        let mut guard = self
            .conn
            .lock()
            .map_err(|_| SourceError::Backend("connection lock poisoned".to_string()))?;
        self.rt
            .block_on(async { guard.query_each_ref(sql, params, visitor).await })
            .map_err(|e| SourceError::Backend(e.to_string()))
            .map(|_| ())
    }

    /// Rejects a write naming the key, an unknown column, NULL in a NOT NULL column, or a
    /// value of the wrong kind (an Int fits a Float column) before it reaches the database.
    fn validate_changes(&self, changes: &[Change]) -> Result<(), SourceError> {
        for (column, value) in changes {
            if *column == self.table.key {
                return Err(SourceError::Unsupported("writing the key column"));
            }
            let info = self
                .table
                .columns
                .iter()
                .find(|c| c.name == *column)
                .ok_or_else(|| SourceError::Store(StoreError::ColumnNotFound(column.clone())))?;
            match value {
                None if !info.nullable => {
                    return Err(SourceError::Backend(format!("{column} is required")));
                }
                Some(Owned::Int(_)) if info.kind == ColumnKind::Float => {}
                Some(Owned::Float(_)) if info.kind == ColumnKind::Int => {
                    return Err(SourceError::Store(StoreError::TypeMismatch {
                        column: column.clone(),
                        expected: "Int",
                    }));
                }
                Some(v) => self.check_type_match(info, v)?,
                None => {}
            }
        }
        Ok(())
    }

    /// Validates that all columns referenced in the query exist in this source's schema
    /// and have compatible data types, ensuring error equivalence with `VecSource`.
    fn validate_query(&self, query: &Query) -> Result<(), SourceError> {
        if let Some(ref filter) = query.filter {
            self.validate_filter(filter)?;
        }

        for k in &query.sort {
            if !self.table.columns.iter().any(|c| c.name == k.column) {
                return Err(SourceError::Store(StoreError::ColumnNotFound(
                    k.column.clone(),
                )));
            }
        }

        for level in &query.group_by {
            // Any column groups in SQL, text included (the in-memory engine groups
            // only dictionary text, a storage detail the database does not have).
            if !self.table.columns.iter().any(|c| c.name == level.column) {
                return Err(SourceError::Store(StoreError::ColumnNotFound(
                    level.column.clone(),
                )));
            }

            for tot in &level.totals {
                let tot_col = self
                    .table
                    .columns
                    .iter()
                    .find(|c| c.name == tot.column)
                    .ok_or_else(|| {
                        SourceError::Store(StoreError::ColumnNotFound(tot.column.clone()))
                    })?;

                if tot.aggregate != Aggregate::Count
                    && tot_col.kind != ColumnKind::Int
                    && tot_col.kind != ColumnKind::Float
                {
                    return Err(SourceError::Store(StoreError::TypeMismatch {
                        column: tot.column.clone(),
                        expected: "Int or Float",
                    }));
                }
            }
        }

        for tot in &query.summary {
            let tot_col = self
                .table
                .columns
                .iter()
                .find(|c| c.name == tot.column)
                .ok_or_else(|| {
                    SourceError::Store(StoreError::ColumnNotFound(tot.column.clone()))
                })?;

            if tot.aggregate != Aggregate::Count
                && tot_col.kind != ColumnKind::Int
                && tot_col.kind != ColumnKind::Float
            {
                return Err(SourceError::Store(StoreError::TypeMismatch {
                    column: tot.column.clone(),
                    expected: "Int or Float",
                }));
            }
        }

        Ok(())
    }

    fn validate_filter(&self, filter: &Filter) -> Result<(), SourceError> {
        match filter {
            Filter::And(list) | Filter::Or(list) => {
                for child in list {
                    self.validate_filter(child)?;
                }
            }
            Filter::Not(child) => {
                self.validate_filter(child)?;
            }
            Filter::IsNull(col) => {
                if !self.table.columns.iter().any(|c| &c.name == col) {
                    return Err(SourceError::Store(StoreError::ColumnNotFound(col.clone())));
                }
            }
            Filter::Eq(col, val)
            | Filter::Lt(col, val)
            | Filter::Le(col, val)
            | Filter::Gt(col, val)
            | Filter::Ge(col, val) => {
                let info = self
                    .table
                    .columns
                    .iter()
                    .find(|c| &c.name == col)
                    .ok_or_else(|| SourceError::Store(StoreError::ColumnNotFound(col.clone())))?;
                self.check_type_match(info, val)?;
            }
            Filter::Between(col, lo, hi) => {
                let info = self
                    .table
                    .columns
                    .iter()
                    .find(|c| &c.name == col)
                    .ok_or_else(|| SourceError::Store(StoreError::ColumnNotFound(col.clone())))?;
                self.check_type_match(info, lo)?;
                self.check_type_match(info, hi)?;
            }
            Filter::In(col, list) => {
                let info = self
                    .table
                    .columns
                    .iter()
                    .find(|c| &c.name == col)
                    .ok_or_else(|| SourceError::Store(StoreError::ColumnNotFound(col.clone())))?;
                for val in list {
                    self.check_type_match(info, val)?;
                }
            }
            Filter::Contains { column, .. }
            | Filter::StartsWith { column, .. }
            | Filter::EqIgnoreCase { column, .. } => {
                let info = self
                    .table
                    .columns
                    .iter()
                    .find(|c| &c.name == column)
                    .ok_or_else(|| {
                        SourceError::Store(StoreError::ColumnNotFound(column.clone()))
                    })?;
                if info.kind != ColumnKind::Text {
                    return Err(SourceError::Store(StoreError::TypeMismatch {
                        column: column.clone(),
                        expected: "Text",
                    }));
                }
            }
        }
        Ok(())
    }

    fn check_type_match(&self, info: &ColumnInfo, val: &Owned) -> Result<(), SourceError> {
        let ok = matches!(
            (info.kind, val),
            (ColumnKind::Int, Owned::Int(_))
                | (ColumnKind::Int, Owned::Float(_))
                | (ColumnKind::Float, Owned::Float(_))
                | (ColumnKind::Bool, Owned::Bool(_))
                | (ColumnKind::Text, Owned::Text(_))
        );
        if !ok {
            let expected = match info.kind {
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
        Ok(())
    }

    /// Kinds of the effective sort keys (the table key is an integer).
    fn sort_kinds(&self, sort_keys: &[SortKey]) -> Vec<ColumnKind> {
        sort_keys
            .iter()
            .map(|k| {
                self.table
                    .columns
                    .iter()
                    .find(|c| c.name == k.column)
                    .map_or(ColumnKind::Int, |c| c.kind)
            })
            .collect()
    }

    /// Runs `sql` and decodes each row into the query's sort-key tuple, after a leading id
    /// column when `with_id` (single-row callers rely on their SQL: unique key or `LIMIT 1`).
    fn query_tuples(
        &self,
        sql: &str,
        params: &NamedParams,
        query: &Query,
        with_id: bool,
    ) -> Result<Vec<(Option<u64>, Tuple)>, SourceError> {
        let kinds = self.sort_kinds(&effective_sort_keys(&query.sort, &self.table.key));
        let mut visitor = TupleVisitor {
            kinds: &kinds,
            with_id,
            out: Vec::new(),
        };
        let mut guard = self
            .conn
            .lock()
            .map_err(|_| SourceError::Backend("connection lock poisoned".to_string()))?;
        self.rt
            .block_on(async { guard.query_each_ref(sql, params, &mut visitor).await })
            .map_err(|e| SourceError::Backend(e.to_string()))?;
        Ok(visitor.out)
    }

    /// Runs a `SELECT COUNT(*)`.
    fn scalar_count(&self, sql: &str, params: &NamedParams) -> Result<usize, SourceError> {
        struct CountVisitor {
            count: usize,
        }
        impl RowVisitor for CountVisitor {
            fn visit<R: RowRef>(&mut self, row: &R) -> Result<ControlFlow<()>, portadb::Error> {
                if let ValueRef::Integer(c) = row.value(0)? {
                    self.count = c as usize;
                }
                Ok(ControlFlow::Break(()))
            }
        }
        let mut visitor = CountVisitor { count: 0 };
        let mut guard = self
            .conn
            .lock()
            .map_err(|_| SourceError::Backend("connection lock poisoned".to_string()))?;
        self.rt
            .block_on(async { guard.query_each_ref(sql, params, &mut visitor).await })
            .map_err(|e| SourceError::Backend(e.to_string()))?;
        Ok(visitor.count)
    }

    /// Rows of the result strictly before the row whose sort-key tuple is `tuple`.
    fn count_before(&self, query: &Query, tuple: &[Option<Owned>]) -> Result<usize, SourceError> {
        let (sql, params) = translate_count_before(&self.table, query, tuple, &SqliteDialect);
        self.scalar_count(&sql, &params)
    }

    /// Remembers `tuple` as the row just before position `pos` of `query`.
    fn remember(&self, query: &Query, pos: usize, tuple: Tuple) -> Result<(), SourceError> {
        let mut bm_guard = self
            .bookmarks
            .lock()
            .map_err(|_| SourceError::Backend("bookmarks mutex poisoned".to_string()))?;
        let query_bms = bm_guard.entry(NormalizedQueryKey::of(query)).or_default();
        if query_bms.len() >= 4096
            && let Some(&first_k) = query_bms.keys().next()
        {
            query_bms.remove(&first_k);
        }
        query_bms.insert(pos, tuple);
        Ok(())
    }

    /// The remembered position nearest at or before `p`, with its tuple.
    fn bookmark_before(
        &self,
        norm_key: &NormalizedQueryKey,
        p: usize,
    ) -> Result<Option<(usize, Tuple)>, SourceError> {
        let bm_guard = self
            .bookmarks
            .lock()
            .map_err(|_| SourceError::Backend("bookmarks mutex poisoned".to_string()))?;
        Ok(bm_guard.get(norm_key).and_then(|query_bms| {
            query_bms
                .range(..=p)
                .next_back()
                .map(|(&pos, tuple)| (pos, tuple.clone()))
        }))
    }

    /// The row count of `query`, asked once per filter.
    fn cached_count(&self, query: &Query) -> Result<usize, SourceError> {
        let filter = query.filter.as_ref().map(Filter::normalized);
        if let Some(&count) = self
            .counts
            .lock()
            .map_err(|_| SourceError::Backend("counts mutex poisoned".to_string()))?
            .get(&filter)
        {
            return Ok(count);
        }
        let count = self.count(query)?;
        self.counts
            .lock()
            .map_err(|_| SourceError::Backend("counts mutex poisoned".to_string()))?
            .insert(filter, count);
        Ok(count)
    }

    /// Runs a window query and decodes it into columns, the window starting at `start`.
    fn run_window(
        &self,
        sql: &str,
        params: &NamedParams,
        len: usize,
        start: usize,
    ) -> Result<Rows, SourceError> {
        let mut decoder = ColumnarDecoder::new(&self.table.columns, len);
        let mut guard = self
            .conn
            .lock()
            .map_err(|_| SourceError::Backend("connection lock poisoned".to_string()))?;
        self.rt
            .block_on(async { guard.query_each_ref(sql, params, &mut decoder).await })
            .map_err(|e| SourceError::Backend(e.to_string()))?;
        drop(guard);
        Ok(decoder.finish(start))
    }

    /// Window `range` (ending at most `count`) read from the end: the same rows in the
    /// reversed order with a short OFFSET, then turned back — `last()` costs a first window.
    fn fetch_from_end(
        &self,
        query: &Query,
        range: std::ops::Range<usize>,
        count: usize,
    ) -> Result<Rows, SourceError> {
        let backwards = Query {
            sort: reversed(&effective_sort_keys(&query.sort, &self.table.key)),
            ..query.clone()
        };
        let len = range.end - range.start;
        let from_end = count - range.end;
        let (sql, params) = translate_fetch_offset(
            &self.table,
            &backwards,
            from_end..from_end + len,
            &SqliteDialect,
        );
        self.tail_fetches.fetch_add(1, Ordering::Relaxed);
        let rows = self.run_window(&sql, &params, len, range.start)?;
        let back: Vec<u32> = (0..rows.len as u32).rev().collect();
        Ok(Rows {
            start: rows.start,
            len: rows.len,
            columns: rows.columns.iter().map(|c| c.gather(&back)).collect(),
            ids: rows.ids.into_iter().rev().collect(),
        })
    }

    /// Stride of this query's checkpoints, if they were built.
    fn known_stride(&self, norm_key: &NormalizedQueryKey) -> Result<Option<usize>, SourceError> {
        let cp_guard = self
            .checkpoints
            .lock()
            .map_err(|_| SourceError::Backend("checkpoints mutex poisoned".to_string()))?;
        Ok(cp_guard.get(norm_key).map(|(stride, _)| *stride))
    }

    /// The checkpoint nearest at or before `p` (building this query's checkpoints the first
    /// time), or `None` when `p` is before the first one or past the last.
    fn checkpoint_before(
        &self,
        query: &Query,
        norm_key: &NormalizedQueryKey,
        p: usize,
    ) -> Result<Option<(usize, Tuple)>, SourceError> {
        let mut cp_guard = self
            .checkpoints
            .lock()
            .map_err(|_| SourceError::Backend("checkpoints mutex poisoned".to_string()))?;
        if !cp_guard.contains_key(norm_key) {
            let stride = checkpoint_stride(self.cached_count(query)?);
            let (sql, params) = translate_checkpoints(&self.table, query, stride, &SqliteDialect);
            let tuples = self
                .query_tuples(&sql, &params, query, false)?
                .into_iter()
                .map(|(_, tuple)| tuple)
                .collect();
            self.checkpoint_builds.fetch_add(1, Ordering::Relaxed);
            cp_guard.insert(norm_key.clone(), (stride, tuples));
        }
        let (stride, tuples) = &cp_guard[norm_key];
        // tuple k-1 is the row at position k*stride - 1, i.e. just before position k*stride
        let k = p / stride;
        Ok((k >= 1)
            .then(|| tuples.get(k - 1))
            .flatten()
            .map(|tuple| (k * stride, tuple.clone())))
    }
}

fn value_ref_to_owned(val: ValueRef<'_>, kind: ColumnKind) -> Option<Owned> {
    match val {
        ValueRef::Null => None,
        ValueRef::Integer(i) => match kind {
            ColumnKind::Bool => Some(Owned::Bool(i != 0)),
            _ => Some(Owned::Int(i)),
        },
        ValueRef::Real(f) => match kind {
            ColumnKind::Int => Some(Owned::Int(f as i64)),
            _ => Some(Owned::Float(f)),
        },
        ValueRef::Boolean(b) => Some(Owned::Bool(b)),
        ValueRef::Text(s) => Some(Owned::Text(s.to_string())),
        ValueRef::Blob(b) => Some(Owned::Text(String::from_utf8_lossy(b).into_owned())),
    }
}

/// One row's values for the effective sort keys (query sort + table key tail).
type Tuple = Vec<Option<Owned>>;

/// Rows between checkpoints: at most ~4096 checkpoints per query, never closer than 1024 rows.
const MIN_STRIDE: usize = 1024;

fn checkpoint_stride(count: usize) -> usize {
    MIN_STRIDE.max(count.div_ceil(4096))
}

/// Decodes rows into sort-key tuples, after a leading integer id column when `with_id`.
struct TupleVisitor<'a> {
    kinds: &'a [ColumnKind],
    with_id: bool,
    out: Vec<(Option<u64>, Tuple)>,
}

impl RowVisitor for TupleVisitor<'_> {
    fn visit<R: RowRef>(&mut self, row: &R) -> Result<ControlFlow<()>, portadb::Error> {
        let lead = usize::from(self.with_id);
        let id = match self.with_id {
            true => match row.value(0)? {
                ValueRef::Integer(v) => Some(v as u64),
                _ => None,
            },
            false => None,
        };
        let mut tuple = Vec::with_capacity(self.kinds.len());
        for (i, &kind) in self.kinds.iter().enumerate() {
            tuple.push(value_ref_to_owned(row.value(lead + i)?, kind));
        }
        self.out.push((id, tuple));
        Ok(ControlFlow::Continue(()))
    }
}

/// Visitor decoding SQLite database rows directly into contiguous columnar buffers.
///
/// Implements zero per-row heap allocations on the fetch path.
struct ColumnarDecoder<'a> {
    schema: &'a [ColumnInfo],
    int_cols: Vec<Vec<i64>>,
    float_cols: Vec<Vec<f64>>,
    bool_words: Vec<Vec<u64>>,
    text_bytes: Vec<Vec<u8>>,
    text_offsets: Vec<Vec<u32>>,
    valid_words: Vec<Vec<u64>>,
    has_nulls: Vec<bool>,
    ids: Vec<u64>,
    row_count: usize,
}

impl<'a> ColumnarDecoder<'a> {
    fn new(schema: &'a [ColumnInfo], capacity: usize) -> Self {
        let num_cols = schema.len();
        let mut int_cols = Vec::with_capacity(num_cols);
        let mut float_cols = Vec::with_capacity(num_cols);
        let mut bool_words = Vec::with_capacity(num_cols);
        let mut text_bytes = Vec::with_capacity(num_cols);
        let mut text_offsets = Vec::with_capacity(num_cols);
        let mut valid_words = Vec::with_capacity(num_cols);
        let mut has_nulls = Vec::with_capacity(num_cols);

        let initial_words = capacity.div_ceil(64).max(1);

        for info in schema {
            match info.kind {
                ColumnKind::Int => {
                    int_cols.push(Vec::with_capacity(capacity));
                    float_cols.push(Vec::new());
                    bool_words.push(Vec::new());
                    text_bytes.push(Vec::new());
                    text_offsets.push(Vec::new());
                }
                ColumnKind::Float => {
                    int_cols.push(Vec::new());
                    float_cols.push(Vec::with_capacity(capacity));
                    bool_words.push(Vec::new());
                    text_bytes.push(Vec::new());
                    text_offsets.push(Vec::new());
                }
                ColumnKind::Bool => {
                    int_cols.push(Vec::new());
                    float_cols.push(Vec::new());
                    bool_words.push(vec![0u64; initial_words]);
                    text_bytes.push(Vec::new());
                    text_offsets.push(Vec::new());
                }
                ColumnKind::Text => {
                    int_cols.push(Vec::new());
                    float_cols.push(Vec::new());
                    bool_words.push(Vec::new());
                    text_bytes.push(Vec::with_capacity(capacity * 8));
                    let mut offsets = Vec::with_capacity(capacity + 1);
                    offsets.push(0);
                    text_offsets.push(offsets);
                }
            }
            valid_words.push(vec![0u64; initial_words]);
            has_nulls.push(false);
        }

        Self {
            schema,
            int_cols,
            float_cols,
            bool_words,
            text_bytes,
            text_offsets,
            valid_words,
            has_nulls,
            ids: Vec::with_capacity(capacity),
            row_count: 0,
        }
    }

    fn finish(mut self, start: usize) -> Rows {
        let count = self.row_count;
        let mut columns = Vec::with_capacity(self.schema.len());

        for (i, info) in self.schema.iter().enumerate() {
            let valid = if self.has_nulls[i] {
                Some(Bitmap::from_words(
                    std::mem::take(&mut self.valid_words[i]),
                    count,
                ))
            } else {
                None
            };

            let col = match info.kind {
                ColumnKind::Int => Column::Int {
                    values: std::mem::take(&mut self.int_cols[i]),
                    valid,
                },
                ColumnKind::Float => Column::Float {
                    values: std::mem::take(&mut self.float_cols[i]),
                    valid,
                },
                ColumnKind::Bool => Column::Bool {
                    values: Bitmap::from_words(std::mem::take(&mut self.bool_words[i]), count),
                    valid,
                },
                ColumnKind::Text => Column::Text {
                    bytes: std::mem::take(&mut self.text_bytes[i]),
                    offsets: std::mem::take(&mut self.text_offsets[i]),
                    valid,
                },
            };
            columns.push(col);
        }

        Rows {
            start,
            len: count,
            columns,
            ids: self.ids,
        }
    }
}

impl RowVisitor for ColumnarDecoder<'_> {
    fn visit<R: RowRef>(&mut self, row: &R) -> Result<ControlFlow<()>, portadb::Error> {
        let current_row = self.row_count;
        let word_idx = current_row / 64;
        let bit_mask = 1u64 << (current_row % 64);

        for (i, info) in self.schema.iter().enumerate() {
            if word_idx >= self.valid_words[i].len() {
                self.valid_words[i].push(0);
            }

            let cell = row.value(i)?;
            if cell.is_null() {
                self.has_nulls[i] = true;
                // Leave valid_words bit at 0
                match info.kind {
                    ColumnKind::Int => self.int_cols[i].push(0),
                    ColumnKind::Float => self.float_cols[i].push(0.0),
                    ColumnKind::Bool => {
                        if word_idx >= self.bool_words[i].len() {
                            self.bool_words[i].push(0);
                        }
                    }
                    ColumnKind::Text => {
                        let cur_len = self.text_bytes[i].len() as u32;
                        self.text_offsets[i].push(cur_len);
                    }
                }
            } else {
                self.valid_words[i][word_idx] |= bit_mask;
                match info.kind {
                    ColumnKind::Int => {
                        let n = match cell {
                            ValueRef::Integer(v) => v,
                            ValueRef::Boolean(b) => i64::from(b),
                            _ => 0,
                        };
                        self.int_cols[i].push(n);
                    }
                    ColumnKind::Float => {
                        let f = match cell {
                            ValueRef::Real(v) => v,
                            ValueRef::Integer(v) => v as f64,
                            _ => 0.0,
                        };
                        self.float_cols[i].push(f);
                    }
                    ColumnKind::Bool => {
                        if word_idx >= self.bool_words[i].len() {
                            self.bool_words[i].push(0);
                        }
                        let b = match cell {
                            ValueRef::Boolean(v) => v,
                            ValueRef::Integer(v) => v != 0,
                            _ => false,
                        };
                        if b {
                            self.bool_words[i][word_idx] |= bit_mask;
                        }
                    }
                    ColumnKind::Text => {
                        let s = match cell {
                            ValueRef::Text(v) => v,
                            _ => "",
                        };
                        self.text_bytes[i].extend_from_slice(s.as_bytes());
                        let cur_len = self.text_bytes[i].len() as u32;
                        self.text_offsets[i].push(cur_len);
                    }
                }
            }
        }

        // Table key is selected as the trailing column
        let key_idx = self.schema.len();
        let key_cell = row.value(key_idx)?;
        let key_id = match key_cell {
            ValueRef::Integer(v) => v as u64,
            _ => 0u64,
        };
        self.ids.push(key_id);

        self.row_count += 1;
        Ok(ControlFlow::Continue(()))
    }
}

impl RowSource for SqlSource {
    fn schema(&self) -> &[ColumnInfo] {
        &self.table.columns
    }

    fn capabilities(&self) -> Capabilities {
        self.capabilities
    }

    fn count(&self, query: &Query) -> Result<usize, SourceError> {
        self.validate_query(query)?;
        let (sql, params) = translate_count(&self.table, query, &SqliteDialect);
        self.scalar_count(&sql, &params)
    }

    fn fetch(&self, query: &Query, range: std::ops::Range<usize>) -> Result<Rows, SourceError> {
        self.validate_query(query)?;

        let dialect = SqliteDialect;
        let norm_key = NormalizedQueryKey::of(query);
        let requested_len = range.end.saturating_sub(range.start);

        if requested_len == 0 {
            // `start` is clamped to the count, as in every source
            let start = if range.start == 0 {
                0
            } else {
                range.start.min(self.count(query)?)
            };
            return Ok(Rows {
                start,
                len: 0,
                columns: self
                    .table
                    .columns
                    .iter()
                    .map(|info| match info.kind {
                        ColumnKind::Int => Column::Int {
                            values: Vec::new(),
                            valid: None,
                        },
                        ColumnKind::Float => Column::Float {
                            values: Vec::new(),
                            valid: None,
                        },
                        ColumnKind::Bool => Column::Bool {
                            values: Bitmap::new(0, false),
                            valid: None,
                        },
                        ColumnKind::Text => Column::Text {
                            bytes: Vec::new(),
                            offsets: vec![0],
                            valid: None,
                        },
                    })
                    .collect(),
                ids: Vec::new(),
            });
        }

        // Where the window starts, cheapest first; an OFFSET never walks more than one stride:
        // an exact bookmark, a bookmark at most one stride before, a short OFFSET from the top,
        // and only then the checkpoints (built once per query by one ordered scan).
        let p = range.start;
        let (sql, params) = if p == 0 {
            translate_fetch_offset(&self.table, query, 0..requested_len, &dialect)
        } else {
            // no COUNT per page: the stride is known once the checkpoints exist, else the minimum
            let stride = self.known_stride(&norm_key)?.unwrap_or(MIN_STRIDE);
            let near = self
                .bookmark_before(&norm_key, p)?
                .filter(|(b, _)| p - b <= stride);
            let start = match near {
                Some(found) => Some(found),
                None => {
                    let count = self.cached_count(query)?;
                    let end = range.end.min(count);
                    if p < end && count - end <= stride {
                        return self.fetch_from_end(query, p..end, count);
                    }
                    if p <= stride {
                        None
                    } else {
                        self.checkpoint_before(query, &norm_key, p)?
                    }
                }
            };
            match start {
                Some((b, tuple)) => {
                    self.keyset_fetches.fetch_add(1, Ordering::Relaxed);
                    translate_fetch_keyset(
                        &self.table,
                        query,
                        &tuple,
                        requested_len,
                        p - b,
                        &dialect,
                    )
                }
                None => {
                    // `p <= stride`, or past the end (no checkpoint that far): a bounded walk
                    self.offset_fetches.fetch_add(1, Ordering::Relaxed);
                    translate_fetch_offset(&self.table, query, range.clone(), &dialect)
                }
            }
        };

        let mut rows = self.run_window(&sql, &params, requested_len, range.start)?;
        if rows.len == 0 && range.start > 0 {
            // past the end: report the clamped start, as in every source;
            // only this rare empty window pays for the extra count
            rows.start = range.start.min(self.count(query)?);
        }

        // Store bookmark from the last row of this window for consecutive scrolling
        if rows.len > 0 {
            let last_idx = rows.len - 1;
            let sort_keys = effective_sort_keys(&query.sort, &self.table.key);
            let mut key_tuple = Vec::with_capacity(sort_keys.len());

            for k in &sort_keys {
                if k.column.eq_ignore_ascii_case(&self.table.key) {
                    let id_val = rows.ids[last_idx];
                    key_tuple.push(Some(Owned::Int(id_val as i64)));
                } else {
                    let col_idx = self
                        .table
                        .columns
                        .iter()
                        .position(|c| c.name == k.column)
                        .expect("sort column exists");
                    let cell_val = rows.columns[col_idx].get(last_idx);
                    let owned = match cell_val {
                        datagrid::column::Value::Null => None,
                        datagrid::column::Value::Int(v) => Some(Owned::Int(v)),
                        datagrid::column::Value::Float(v) => Some(Owned::Float(v)),
                        datagrid::column::Value::Bool(v) => Some(Owned::Bool(v)),
                        datagrid::column::Value::Text(v) => Some(Owned::Text(v.to_string())),
                    };
                    key_tuple.push(owned);
                }
            }

            let next_pos = range.start + rows.len;
            let mut bm_guard = self
                .bookmarks
                .lock()
                .map_err(|_| SourceError::Backend("bookmarks mutex poisoned".to_string()))?;
            let query_bms = bm_guard.entry(norm_key).or_default();
            if query_bms.len() >= 4096
                && let Some(&first_k) = query_bms.keys().next()
            {
                query_bms.remove(&first_k);
            }
            query_bms.insert(next_pos, key_tuple);
        }

        Ok(rows)
    }

    fn groups(
        &self,
        query: &Query,
        parent: &[Option<Owned>],
    ) -> Result<Vec<SourceGroup>, SourceError> {
        self.validate_query(query)?;

        let level = parent.len();
        if level >= query.group_by.len() {
            return Ok(Vec::new());
        }

        let dialect = SqliteDialect;
        let (sql, params) =
            translate_groups(&self.table, query, parent, &dialect).map_err(SourceError::Backend)?;

        let group_level = &query.group_by[level];
        let group_col = self
            .table
            .columns
            .iter()
            .find(|c| c.name == group_level.column)
            .expect("group column exists");
        let group_kind = group_col.kind;

        let num_totals = group_level.totals.len();

        struct GroupRowVisitor {
            group_kind: ColumnKind,
            num_totals: usize,
            groups: Vec<SourceGroup>,
        }

        impl RowVisitor for GroupRowVisitor {
            fn visit<R: RowRef>(&mut self, row: &R) -> Result<ControlFlow<()>, portadb::Error> {
                let key_ref = row.value(0)?;
                let key_val = match key_ref {
                    ValueRef::Null => None,
                    ValueRef::Integer(i) => {
                        if self.group_kind == ColumnKind::Bool {
                            Some(Owned::Bool(i != 0))
                        } else {
                            Some(Owned::Int(i))
                        }
                    }
                    ValueRef::Real(f) => Some(Owned::Float(f)),
                    ValueRef::Boolean(b) => Some(Owned::Bool(b)),
                    ValueRef::Text(s) => Some(Owned::Text(s.to_string())),
                    ValueRef::Blob(b) => Some(Owned::Text(String::from_utf8_lossy(b).into_owned())),
                };

                let count_ref = row.value(1)?;
                let row_count = match count_ref {
                    ValueRef::Integer(cnt) => cnt as usize,
                    _ => 0,
                };

                let mut totals = Vec::with_capacity(self.num_totals);
                for t_idx in 0..self.num_totals {
                    let tot_ref = row.value(2 + t_idx)?;
                    let tot_val = match tot_ref {
                        ValueRef::Null => None,
                        ValueRef::Real(f) => Some(f),
                        ValueRef::Integer(i) => Some(i as f64),
                        _ => None,
                    };
                    totals.push(tot_val);
                }

                self.groups.push(SourceGroup {
                    key: vec![key_val],
                    row_count,
                    totals,
                });

                Ok(ControlFlow::Continue(()))
            }
        }

        let mut visitor = GroupRowVisitor {
            group_kind,
            num_totals,
            groups: Vec::new(),
        };

        self.visit_rows(&sql, &params, &mut visitor)?;

        Ok(visitor.groups)
    }

    fn position_of(&self, query: &Query, id: u64) -> Result<Option<usize>, SourceError> {
        self.validate_query(query)?;
        let dialect = SqliteDialect;
        let (sql, params) = translate_sort_tuple_of(&self.table, query, id, &dialect);
        let Some((_, tuple)) = self.query_tuples(&sql, &params, query, false)?.pop() else {
            return Ok(None);
        };
        let pos = self.count_before(query, &tuple)?;
        self.remember(query, pos + 1, tuple)?;
        Ok(Some(pos))
    }

    fn locate(&self, query: &Query, filter: &Filter) -> Result<Option<(usize, u64)>, SourceError> {
        self.validate_query(query)?;
        self.validate_filter(filter)?;
        let dialect = SqliteDialect;
        let (sql, params) = translate_locate(&self.table, query, filter, &dialect);
        let Some((id, tuple)) = self.query_tuples(&sql, &params, query, true)?.pop() else {
            return Ok(None);
        };
        let id =
            id.ok_or_else(|| SourceError::Backend("table key is not an integer".to_string()))?;
        let pos = self.count_before(query, &tuple)?;
        self.remember(query, pos + 1, tuple)?;
        Ok(Some((pos, id)))
    }

    fn summary(&self, query: &Query) -> Result<Vec<Option<f64>>, SourceError> {
        self.validate_query(query)?;
        if query.summary.is_empty() {
            return Ok(Vec::new());
        }

        let dialect = SqliteDialect;
        let (sql, params) = translate_summary(&self.table, query, &dialect);

        struct SummaryRowVisitor {
            num_totals: usize,
            totals: Option<Vec<Option<f64>>>,
        }

        impl RowVisitor for SummaryRowVisitor {
            fn visit<R: RowRef>(&mut self, row: &R) -> Result<ControlFlow<()>, portadb::Error> {
                let mut totals = Vec::with_capacity(self.num_totals);
                for i in 0..self.num_totals {
                    let val_ref = row.value(i)?;
                    let val = match val_ref {
                        ValueRef::Null => None,
                        ValueRef::Real(f) => Some(f),
                        ValueRef::Integer(i) => Some(i as f64),
                        _ => None,
                    };
                    totals.push(val);
                }
                self.totals = Some(totals);
                Ok(ControlFlow::Break(()))
            }
        }

        let mut visitor = SummaryRowVisitor {
            num_totals: query.summary.len(),
            totals: None,
        };

        self.visit_rows(&sql, &params, &mut visitor)?;

        Ok(visitor
            .totals
            .unwrap_or_else(|| vec![None; query.summary.len()]))
    }

    fn distinct(&self, query: &Query, column: &str, limit: usize) -> Result<Distinct, SourceError> {
        self.validate_query(query)?;
        if !self.table.columns.iter().any(|c| c.name == column) {
            return Err(SourceError::Store(StoreError::ColumnNotFound(
                column.to_string(),
            )));
        }

        let dialect = SqliteDialect;
        let (sql, params) = translate_distinct(&self.table, query, column, limit, &dialect);

        struct DistinctRowVisitor {
            values: Vec<Option<String>>,
        }

        impl RowVisitor for DistinctRowVisitor {
            fn visit<R: RowRef>(&mut self, row: &R) -> Result<ControlFlow<()>, portadb::Error> {
                let val_ref = row.value(0)?;
                let val = match val_ref {
                    ValueRef::Null => None,
                    ValueRef::Text(s) => Some(s.to_string()),
                    ValueRef::Integer(i) => Some(i.to_string()),
                    ValueRef::Real(f) => Some(f.to_string()),
                    ValueRef::Boolean(b) => Some(b.to_string()),
                    _ => None,
                };
                self.values.push(val);
                Ok(ControlFlow::Continue(()))
            }
        }

        let mut visitor = DistinctRowVisitor { values: Vec::new() };

        self.visit_rows(&sql, &params, &mut visitor)?;

        Ok(visitor.values)
    }

    fn can_write(&self) -> bool {
        !self.read_only
    }

    fn key_column(&self) -> Option<&str> {
        match &self.rowid_alias {
            Some(alias) if self.table.key.eq_ignore_ascii_case("rowid") => Some(alias),
            _ => Some(&self.table.key),
        }
    }

    fn update(&self, id: u64, changes: &[Change]) -> Result<(), SourceError> {
        if self.read_only {
            return Err(SourceError::Unsupported("update"));
        }
        if changes.is_empty() {
            return Ok(());
        }
        self.validate_changes(changes)?;
        let (sql, params) = translate_update(&self.table, id, changes, &SqliteDialect);
        let affected = self.execute(&sql, &params)?;
        self.clear_cache();
        match affected {
            1 => Ok(()),
            0 => Err(SourceError::Backend(format!("row {id} no longer exists"))),
            n => Err(SourceError::Backend(format!("{n} rows share the key {id}"))),
        }
    }

    fn insert(&self, values: &[Change]) -> Result<u64, SourceError> {
        if self.read_only {
            return Err(SourceError::Unsupported("insert"));
        }
        self.validate_changes(values)?;
        let (sql, params) = translate_insert(&self.table, values, &SqliteDialect);

        struct KeyVisitor(Option<i64>);
        impl RowVisitor for KeyVisitor {
            fn visit<R: RowRef>(&mut self, row: &R) -> Result<ControlFlow<()>, portadb::Error> {
                if let ValueRef::Integer(key) = row.value(0)? {
                    self.0 = Some(key);
                }
                Ok(ControlFlow::Break(()))
            }
        }

        let mut key = KeyVisitor(None);
        self.visit_rows(&sql, &params, &mut key)?;
        self.clear_cache();
        key.0
            .and_then(|k| u64::try_from(k).ok())
            .ok_or_else(|| SourceError::Backend("the new row has no integer key".to_string()))
    }
}
