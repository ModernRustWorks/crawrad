//! SQLite lookup source implementation for DataGrid.

use std::ops::ControlFlow;
use std::sync::Mutex;

use datagrid::LookupSource;
use datagrid::filter::Owned;
use datagrid::source::SourceError;
use portadb::connection::Connection;
use portadb::dialect::{Dialect, SqliteDialect};
use portadb::drivers::RusqliteDriver;
use portadb::options::ConnectOptions;
use portadb::params::NamedParams;
use portadb::row_ref::{RowRef, RowVisitor, ValueRef};
use portadb::value::Value;

use crate::translate::ParamContext;

fn is_plain_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn value_ref_to_owned_any(val: ValueRef<'_>) -> Option<Owned> {
    match val {
        ValueRef::Null => None,
        ValueRef::Integer(i) => Some(Owned::Int(i)),
        ValueRef::Real(f) => Some(Owned::Float(f)),
        ValueRef::Boolean(b) => Some(Owned::Bool(b)),
        ValueRef::Text(s) => Some(Owned::Text(s.to_string())),
        ValueRef::Blob(b) => Some(Owned::Text(String::from_utf8_lossy(b).into_owned())),
    }
}

fn value_ref_to_string(val: ValueRef<'_>) -> String {
    match val {
        ValueRef::Null => String::new(),
        ValueRef::Integer(i) => i.to_string(),
        ValueRef::Real(f) => f.to_string(),
        ValueRef::Boolean(b) => b.to_string(),
        ValueRef::Text(s) => s.to_string(),
        ValueRef::Blob(b) => String::from_utf8_lossy(b).into_owned(),
    }
}

/// SQL-backed lookup source for resolving display labels and incremental search.
///
/// The connection opens on first use: a lookup no cell ever needs never touches the database.
pub struct SqlLookup {
    conn: Mutex<Option<Connection>>,
    options: ConnectOptions,
    rt: tokio::runtime::Runtime,
    from_source: String,
    key_col: String,
    display_cols: Vec<String>,
}

impl SqlLookup {
    /// Opens an SQLite lookup over a dataset: its `sql` when it has one, else its `table`
    /// — the same choice the dataset itself makes at run time.
    pub fn open_sqlite<I, S>(
        url: impl AsRef<str>,
        table: &str,
        sql: &str,
        key: impl Into<String>,
        display: I,
    ) -> Result<Self, SourceError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let from_source = if !sql.trim().is_empty() {
            format!("({sql}) AS \"_src\"")
        } else if is_plain_identifier(table) {
            SqliteDialect.quote_identifier(table)
        } else {
            return Err(SourceError::Backend(format!("invalid table name: {table}")));
        };
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| {
                SourceError::Backend(format!("failed to initialize tokio runtime: {e}"))
            })?;
        Ok(Self {
            conn: Mutex::new(None),
            options: ConnectOptions::new(url.as_ref()),
            rt,
            from_source,
            key_col: key.into(),
            display_cols: display.into_iter().map(Into::into).collect(),
        })
    }

    /// The connection, opened the first time a lookup needs it.
    fn connection(&self) -> Result<std::sync::MutexGuard<'_, Option<Connection>>, SourceError> {
        let mut guard = self
            .conn
            .lock()
            .map_err(|_| SourceError::Backend("connection lock poisoned".to_string()))?;
        if guard.is_none() {
            let conn = self
                .rt
                .block_on(async { Connection::open(RusqliteDriver::new(), &self.options).await })
                .map_err(|e| SourceError::Backend(format!("database connection failed: {e}")))?;
            *guard = Some(conn);
        }
        Ok(guard)
    }

    /// Whether the connection has been opened yet.
    pub fn is_connected(&self) -> bool {
        self.conn.lock().map(|g| g.is_some()).unwrap_or(false)
    }

    /// Helper to execute arbitrary SQL statements (e.g. schema setup in tests).
    pub fn execute(&self, sql: &str, params: &NamedParams) -> Result<u64, SourceError> {
        let mut guard = self.connection()?;
        let conn = guard.as_mut().expect("opened by connection()");
        self.rt
            .block_on(async { conn.execute(sql, params).await })
            .map_err(|e| SourceError::Backend(e.to_string()))
    }

    fn select_columns(&self) -> String {
        let quoted_key = SqliteDialect.quote_identifier(&self.key_col);
        if self.display_cols.is_empty() {
            quoted_key
        } else {
            let quoted_displays: Vec<String> = self
                .display_cols
                .iter()
                .map(|col| SqliteDialect.quote_identifier(col))
                .collect();
            format!("{quoted_key}, {}", quoted_displays.join(", "))
        }
    }
}

struct LookupVisitor {
    num_displays: usize,
    rows: Vec<(Owned, Vec<String>)>,
}

impl RowVisitor for LookupVisitor {
    fn visit<R: RowRef>(&mut self, row: &R) -> Result<ControlFlow<()>, portadb::Error> {
        let key = match value_ref_to_owned_any(row.value(0)?) {
            Some(k) => k,
            None => return Ok(ControlFlow::Continue(())),
        };
        let mut displays = Vec::with_capacity(self.num_displays);
        for i in 0..self.num_displays {
            displays.push(value_ref_to_string(row.value(1 + i)?));
        }
        self.rows.push((key, displays));
        Ok(ControlFlow::Continue(()))
    }
}

impl LookupSource for SqlLookup {
    fn labels(&self, keys: &[Owned]) -> Result<Vec<(Owned, Vec<String>)>, SourceError> {
        if keys.is_empty() {
            return Ok(Vec::new());
        }
        let mut ctx = ParamContext::new();
        let mut placeholders = Vec::with_capacity(keys.len());
        for key in keys {
            placeholders.push(ctx.bind_owned(key));
        }
        let in_clause = placeholders.join(", ");
        let cols = self.select_columns();
        let quoted_key = SqliteDialect.quote_identifier(&self.key_col);
        let sql = format!(
            "SELECT {cols} FROM {from} WHERE {quoted_key} IN ({in_clause})",
            from = self.from_source,
        );
        let params = ctx.finish();
        let mut visitor = LookupVisitor {
            num_displays: self.display_cols.len(),
            rows: Vec::new(),
        };
        let mut guard = self.connection()?;
        let conn = guard.as_mut().expect("opened by connection()");
        self.rt
            .block_on(async { conn.query_each_ref(&sql, &params, &mut visitor).await })
            .map_err(|e| SourceError::Backend(e.to_string()))?;
        Ok(visitor.rows)
    }

    fn search(&self, prefix: &str, limit: usize) -> Result<Vec<(Owned, Vec<String>)>, SourceError> {
        let cols = self.select_columns();
        let d1 = if let Some(first_display) = self.display_cols.first() {
            SqliteDialect.quote_identifier(first_display)
        } else {
            SqliteDialect.quote_identifier(&self.key_col)
        };

        let mut ctx = ParamContext::new();
        let p_limit = ctx.bind(Value::Integer(limit as i64));
        let sql = if prefix.is_empty() {
            format!(
                "SELECT {cols} FROM {from} ORDER BY {d1} LIMIT {p_limit}",
                from = self.from_source,
            )
        } else {
            // What the user typed is text, not a pattern: `%` and `_` match themselves.
            let escaped = prefix
                .replace('\\', "\\\\")
                .replace('%', "\\%")
                .replace('_', "\\_");
            let p_prefix = ctx.bind(Value::Text(escaped));
            format!(
                "SELECT {cols} FROM {from} WHERE {d1} LIKE {p_prefix} || '%' ESCAPE '\\' ORDER BY {d1} LIMIT {p_limit}",
                from = self.from_source,
            )
        };
        let params = ctx.finish();
        let mut visitor = LookupVisitor {
            num_displays: self.display_cols.len(),
            rows: Vec::new(),
        };
        let mut guard = self.connection()?;
        let conn = guard.as_mut().expect("opened by connection()");
        self.rt
            .block_on(async { conn.query_each_ref(&sql, &params, &mut visitor).await })
            .map_err(|e| SourceError::Backend(e.to_string()))?;
        Ok(visitor.rows)
    }
}
