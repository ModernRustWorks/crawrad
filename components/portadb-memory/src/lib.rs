use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, OnceLock, RwLock};

use portadb_core::capability::Capabilities;
use portadb_core::database::Database;
use portadb_core::driver::{Driver as DriverTrait, DriverConnection, DriverTransaction};
use portadb_core::error::Error;
use portadb_core::options::ConnectOptions;
use portadb_core::params::NamedParams;
use portadb_core::row::Row;
use portadb_core::value::Value;

/// A column, containing what `CREATE TABLE` declared about it.
///
/// # It used to store only the NAME, and schema introspection lied
///
/// Storage was `Vec<String>`, and `PRAGMA TABLE_INFO` answered hardcoded `TEXT`
/// for every column, with `notnull = 0` and `pk = 0`. Anyone querying the
/// schema of a table received a response with the correct SHAPE and fabricated
/// CONTENT — which is worse than not responding at all.
///
/// The declared type was already being read from `CREATE TABLE` and discarded
/// one line later. Now it is preserved.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Column {
    name: String,
    /// The type as written by `CREATE TABLE` — `INTEGER`, `TEXT`, `REAL`,
    /// `BLOB`. Empty when the column originated from an `INSERT` without a
    /// declared table, which is the convenience path of this test double.
    decl: String,
    not_null: bool,
    primary_key: bool,
}

impl Column {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            decl: String::new(),
            not_null: false,
            primary_key: false,
        }
    }
}

/// Column names, which is what a [`Row`] carries.
fn nomes(cols: &[Column]) -> Vec<String> {
    cols.iter().map(|c| c.name.clone()).collect()
}

/// Finds the index of the column with this name.
fn posicao(cols: &[Column], nome: &str) -> Option<usize> {
    cols.iter().position(|c| c.name.eq_ignore_ascii_case(nome))
}

type TableStore = HashMap<String, (Vec<Column>, Vec<Vec<Value>>)>;
type Catalog = RwLock<HashMap<String, Arc<RwLock<TableStore>>>>;

fn global_catalog() -> &'static Catalog {
    static CATALOG: OnceLock<Catalog> = OnceLock::new();
    CATALOG.get_or_init(|| RwLock::new(HashMap::new()))
}

/// In-memory mock driver for zero-dependency unit tests.
#[derive(Debug, Default, Clone, Copy)]
pub struct MemoryDriver;

/// Standard alias for memory driver.
pub type Driver = MemoryDriver;
/// Standard alias for memory connection.
pub type Connection = MemoryConnection;
/// Standard alias for memory transaction.
pub type Transaction<'a> = MemoryTransaction<'a>;

impl MemoryDriver {
    /// Creates a new in-memory database driver.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl DriverTrait for MemoryDriver {
    type Connection = MemoryConnection;

    fn name(&self) -> &'static str {
        "memory"
    }

    fn supports_database(&self, database: Database) -> bool {
        matches!(database, Database::Sqlite)
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities::memory()
    }

    async fn connect(&self, options: &ConnectOptions) -> Result<Self::Connection, Error> {
        let url = if options.url.is_empty() {
            "memory://default".to_string()
        } else {
            options.url.clone()
        };

        let tables = {
            let mut catalog = global_catalog()
                .write()
                .map_err(|_| Error::ConnectionLost("memory catalog lock poisoned".to_string()))?;
            catalog
                .entry(url)
                .or_insert_with(|| Arc::new(RwLock::new(HashMap::new())))
                .clone()
        };

        Ok(MemoryConnection { tables })
    }
}

/// Active in-memory database connection.
pub struct MemoryConnection {
    tables: Arc<RwLock<TableStore>>,
}

impl MemoryConnection {
    /// Creates a new standalone in-memory connection.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tables: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn execute_internal(&self, sql: &str, params: &NamedParams) -> Result<u64, Error> {
        let trimmed = sql.trim();
        let upper = trimmed.to_ascii_uppercase();

        if upper.starts_with("CREATE TABLE") {
            let table_name = extract_table_name(trimmed, "CREATE TABLE")?;
            let mut cols = Vec::new();
            if let Some(start) = trimmed.find('(')
                && let Some(end) = trimmed.rfind(')')
            {
                let col_defs = &trimmed[start + 1..end];
                for def in col_defs.split(',') {
                    let parts: Vec<&str> = def.split_whitespace().collect();
                    if let Some(col_name) = parts.first() {
                        let clean_col = col_name
                            .trim_matches('"')
                            .trim_matches('`')
                            .trim_matches('[')
                            .trim_matches(']');
                        if !clean_col.is_empty()
                            && !clean_col.eq_ignore_ascii_case("PRIMARY")
                            && !clean_col.eq_ignore_ascii_case("FOREIGN")
                            && !clean_col.eq_ignore_ascii_case("CONSTRAINT")
                        {
                            // THE REST OF THE DEFINITION was discarded one line
                            // after being parsed. The declared type and `NOT NULL`
                            // are here, which schema introspection previously fabricated.
                            let resto = parts[1..].join(" ").to_ascii_uppercase();
                            cols.push(Column {
                                name: clean_col.to_string(),
                                decl: parts
                                    .get(1)
                                    .map(|t| t.to_ascii_uppercase())
                                    .unwrap_or_default(),
                                not_null: resto.contains("NOT NULL"),
                                primary_key: resto.contains("PRIMARY KEY"),
                            });
                        }
                    }
                }
            }
            let mut map = self
                .tables
                .write()
                .map_err(|_| Error::ConnectionLost("memory database lock poisoned".to_string()))?;
            map.entry(table_name.to_ascii_lowercase())
                .or_insert_with(|| (cols, Vec::new()));
            Ok(0)
        } else if upper.starts_with("INSERT INTO") {
            let table_name = extract_table_name(trimmed, "INSERT INTO")?;
            let mut map = self
                .tables
                .write()
                .map_err(|_| Error::ConnectionLost("memory database lock poisoned".to_string()))?;
            let (cols, rows) = map
                .get_mut(&table_name.to_ascii_lowercase())
                .ok_or_else(|| {
                    Error::query_error(format!("table '{table_name}' does not exist"))
                })?;

            // Parse VALUES (...) whether literal or parameterized
            let mut count = 0;
            if let Some(val_pos) = upper.find("VALUES") {
                let val_part = &trimmed[val_pos + 6..].trim();
                // THE COLUMN LIST NAMES WHERE EACH VALUE GOES.
                //
                // It was ignored: `INSERT INTO t (name, age) VALUES (..)` put
                // `name` in the first column, whatever it was, and a row shorter
                // than the table — which the next `SELECT` panicked on. Without
                // a list the values are positional, as in SQLite.
                let named = insert_column_list(trimmed, val_pos);
                for tuple in extract_tuples(val_part) {
                    let mut given = Vec::new();
                    for token in tuple {
                        let trimmed_tok = token.trim();
                        if let Some(param_name) = trimmed_tok.strip_prefix(':') {
                            given.push(params.get(param_name).cloned().unwrap_or(Value::Null));
                        } else {
                            given.push(parse_literal_value(trimmed_tok));
                        }
                    }
                    let new_row = match &named {
                        None => given,
                        Some(names) => {
                            let mut full = vec![Value::Null; cols.len()];
                            for (name, value) in names.iter().zip(given) {
                                let index = cols
                                    .iter()
                                    .position(|c| c.name.eq_ignore_ascii_case(name))
                                    .ok_or_else(|| {
                                        Error::query_error(format!(
                                            "table '{table_name}' has no column named '{name}'"
                                        ))
                                    })?;
                                full[index] = value;
                            }
                            full
                        }
                    };
                    let new_row = with_row_id(cols, rows, new_row);
                    rows.push(new_row);
                    count += 1;
                }
                return Ok(count);
            }

            if !params.is_empty() {
                // Single row fallback
                if cols.is_empty() {
                    for (name, _) in params.iter() {
                        cols.push(Column::new(name));
                    }
                }

                let mut new_row = Vec::new();
                for col in cols.iter() {
                    let val = params.get(&col.name).cloned().unwrap_or(Value::Null);
                    new_row.push(val);
                }

                rows.push(new_row);
                return Ok(1);
            }

            Ok(0)
        } else if upper.starts_with("ALTER TABLE") {
            let table_name = extract_table_name(trimmed, "ALTER TABLE")?;
            let mut map = self
                .tables
                .write()
                .map_err(|_| Error::ConnectionLost("memory database lock poisoned".to_string()))?;
            if let Some((cols, rows)) = map.get_mut(&table_name.to_ascii_lowercase())
                && let Some(add_pos) = upper.find("ADD")
            {
                let mut rem = trimmed[add_pos + 3..].trim();
                if rem.to_ascii_uppercase().starts_with("COLUMN") {
                    rem = rem[6..].trim();
                }
                if let Some(col_name) = rem.split_whitespace().next() {
                    let clean_col = col_name
                        .trim_matches('"')
                        .trim_matches('`')
                        .trim_matches('[');
                    if posicao(cols, clean_col).is_none() {
                        cols.push(Column::new(clean_col));
                        for r in rows.iter_mut() {
                            r.push(Value::Null);
                        }
                    }
                }
            }
            Ok(0)
        } else if upper.starts_with("DROP TABLE") {
            let table_name = extract_table_name(trimmed, "DROP TABLE")?;
            let mut map = self
                .tables
                .write()
                .map_err(|_| Error::ConnectionLost("memory database lock poisoned".to_string()))?;
            map.remove(&table_name.to_ascii_lowercase());
            Ok(0)
        } else if upper.starts_with("DELETE FROM") {
            // `DELETE` DID NOT EXIST, and its absence was silent.
            //
            // See the final arm: every unknown statement answered `Ok(0)` —
            // *"zero rows affected"* —, which is indistinguishable from a `DELETE`
            // that matched nothing. `Migrator::rollback_last` would delete the row
            // of the reverted migration and the row remained there; the second call
            // reverted the same migration over again, forever.
            let table_name = extract_table_name(trimmed, "DELETE FROM")?;
            let mut map = self
                .tables
                .write()
                .map_err(|_| Error::ConnectionLost("memory database lock poisoned".to_string()))?;
            let Some((cols, rows)) = map.get_mut(&table_name.to_ascii_lowercase()) else {
                return Ok(0);
            };
            let filters = where_filters(trimmed, cols, params);
            let antes = rows.len();
            rows.retain(|row| !matches(row, &filters));
            Ok((antes - rows.len()) as u64)
        } else if upper.starts_with("UPDATE ") {
            // For the same reason as `DELETE` above: without it, an `UPDATE`
            // answered "0 rows" and the table remained unchanged.
            let table_name = extract_table_name(trimmed, "UPDATE")?;
            let atribuicoes = set_clause(trimmed, params);
            let mut map = self
                .tables
                .write()
                .map_err(|_| Error::ConnectionLost("memory database lock poisoned".to_string()))?;
            let Some((cols, rows)) = map.get_mut(&table_name.to_ascii_lowercase()) else {
                return Ok(0);
            };
            let filters = where_filters(trimmed, cols, params);
            let alvos: Vec<(usize, Value)> = atribuicoes
                .into_iter()
                .filter_map(|(col, val)| posicao(cols, &col).map(|pos| (pos, val)))
                .collect();
            let mut tocadas = 0u64;
            for row in rows.iter_mut() {
                if matches(row, &filters) {
                    for (pos, val) in &alvos {
                        row[*pos] = val.clone();
                    }
                    tocadas += 1;
                }
            }
            Ok(tocadas)
        } else {
            // **ANY STATEMENT THIS TEST DOUBLE DOES NOT UNDERSTAND IS REJECTED.**
            //
            // This previously returned `Ok(0)`, which was the source of two failures
            // in `tests/migration_test.rs` brought along with this crate: an
            // unimplemented `DELETE` answered *"zero rows affected"*, exactly what
            // a correct `DELETE` answers when matching nothing. The distinction between
            // *"did nothing"* and *"nothing to do"* had vanished.
            //
            // An in-memory double does not need to be a full database. It must **not
            // lie**: unsupported statements must fail citing the statement,
            // not slip away silently.
            Err(Error::UnsupportedFeature(format!(
                "the memory driver does not implement this statement: {trimmed}"
            )))
        }
    }

    fn query_internal(&self, sql: &str, params: &NamedParams) -> Result<Vec<Row>, Error> {
        let trimmed = sql.trim();
        let upper = trimmed.to_ascii_uppercase();

        if upper.starts_with("PRAGMA TABLE_INFO") {
            let start = trimmed.find('(').unwrap_or(0);
            let end = trimmed.rfind(')').unwrap_or(trimmed.len());
            let raw_table = if start < end {
                &trimmed[start + 1..end]
            } else {
                ""
            };
            let table = raw_table
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .trim_matches('`');
            let map = self
                .tables
                .read()
                .map_err(|_| Error::ConnectionLost("memory database lock poisoned".to_string()))?;
            if let Some((cols, _)) = map.get(&table.to_ascii_lowercase()) {
                let mut pragma_rows = Vec::new();
                for (cid, col) in cols.iter().enumerate() {
                    pragma_rows.push(Row::new(
                        vec![
                            "cid".to_string(),
                            "name".to_string(),
                            "type".to_string(),
                            "notnull".to_string(),
                            "dflt_value".to_string(),
                            "pk".to_string(),
                        ],
                        vec![
                            Value::Integer(cid as i64),
                            Value::Text(col.name.clone()),
                            // THE DECLARED TYPE, rather than hardcoded `TEXT`.
                            //
                            // This was previously `Value::Text("TEXT".to_string())`
                            // for EVERY column, with `notnull` and `pk` always zero:
                            // introspection answered with the right shape and fabricated
                            // content. An `INTEGER NOT NULL PRIMARY KEY` column came
                            // out as text, nullable, and keyless.
                            //
                            // `TEXT` remains the default for undeclared columns —
                            // such as an `INSERT` into a double without `CREATE TABLE` —,
                            // which accurately reflects that no type was declared.
                            Value::Text(if col.decl.is_empty() {
                                "TEXT".to_string()
                            } else {
                                col.decl.clone()
                            }),
                            Value::Integer(i64::from(col.not_null)),
                            Value::Null,
                            Value::Integer(i64::from(col.primary_key)),
                        ],
                    ));
                }
                return Ok(pragma_rows);
            }
            return Ok(Vec::new());
        }

        if upper.starts_with("SELECT") {
            let table_name = extract_table_name_from_select(trimmed)?;
            let map = self
                .tables
                .read()
                .map_err(|_| Error::ConnectionLost("memory database lock poisoned".to_string()))?;

            if table_name.eq_ignore_ascii_case("sqlite_master") {
                let mut master_rows = Vec::new();
                for tbl in map.keys() {
                    if !tbl.starts_with("sqlite_") && !tbl.starts_with("_portadb_") {
                        master_rows.push(Row::new(
                            vec![
                                "type".to_string(),
                                "name".to_string(),
                                "tbl_name".to_string(),
                            ],
                            vec![
                                Value::Text("table".to_string()),
                                Value::Text(tbl.clone()),
                                Value::Text(tbl.clone()),
                            ],
                        ));
                    }
                }
                return Ok(master_rows);
            }

            // A NONEXISTENT TABLE CAUSES `SELECT` TO FAIL.
            //
            // This was previously `.unwrap_or_else(|| (Vec::new(), Vec::new()))`:
            // querying a nonexistent table returned **zero rows**, identical to an
            // empty table. A typo in table name turned into a plausible empty result.
            //
            // `INSERT INTO` in this driver already failed in this case with this
            // phrase. The same rule applies as the final arm of `execute_internal`:
            // the double may not support everything, but it must not lie.
            let (cols, rows) = map
                .get(&table_name.to_ascii_lowercase())
                .cloned()
                .ok_or_else(|| {
                    Error::query_error(format!("table '{table_name}' does not exist"))
                })?;

            // `WHERE` conditions come from the SAME reader used by `DELETE` and
            // `UPDATE` — see [`where_filters`].
            let filters = where_filters(trimmed, &cols, params);

            let columns: Arc<[String]> = nomes(&cols).into();
            let mut result_rows = Vec::new();
            for row_vals in rows {
                if matches(&row_vals, &filters) {
                    result_rows.push(Row::with_columns(Arc::clone(&columns), row_vals));
                }
            }

            // ORDER BY and LIMIT, in that order — and they were NOT applied.
            //
            // # The bug, discovered on 2026-09-17 when bringing this crate into CrawRAD workspace
            //
            // `ORDER BY` was parsed only to find **where `WHERE` ends** (see `end_pos`
            // above), and after that nothing was sorted; `LIMIT` was ignored completely.
            // Consequently, `ORDER BY version DESC LIMIT 1` returned the FIRST inserted
            // row instead of the latest.
            //
            // Two consequences, both red in the test suite:
            // `Migrator::rollback_last` reverted migration 1 instead of 2
            // (`tests/migration_test.rs`), and the atomic rollback test depended
            // on the same query.
            //
            // An in-memory double does not need to be a full database, but it must
            // **not lie**: queries the driver cannot handle must return what was
            // requested or fail, not silently return an arbitrary row.
            ordenar(&mut result_rows, trimmed);
            limitar(&mut result_rows, trimmed);

            if upper.contains("COUNT(*)") {
                let count = result_rows.len();
                let col_name = if upper.contains("AS TOTAL") {
                    "total".to_string()
                } else if let Some(as_pos) = upper.find("AS") {
                    let rem = &trimmed[as_pos + 2..].trim();
                    let alias = rem
                        .split(|c: char| c.is_whitespace() || c == ';')
                        .next()
                        .unwrap_or("count");
                    alias.trim_matches('"').trim_matches('`').to_string()
                } else {
                    "count".to_string()
                };
                return Ok(vec![Row::new(
                    vec![col_name],
                    vec![Value::Integer(count as i64)],
                )]);
            }

            Ok(result_rows)
        } else {
            Ok(Vec::new())
        }
    }
}

impl Default for MemoryConnection {
    fn default() -> Self {
        Self::new()
    }
}

impl DriverConnection for MemoryConnection {
    async fn execute(&mut self, sql: &str, params: &NamedParams) -> Result<u64, Error> {
        self.execute_internal(sql, params)
    }

    async fn query(&mut self, sql: &str, params: &NamedParams) -> Result<Vec<Row>, Error> {
        self.query_internal(sql, params)
    }

    async fn begin_transaction(&mut self) -> Result<Box<dyn DriverTransaction + '_>, Error> {
        let snapshot = {
            let map = self
                .tables
                .read()
                .map_err(|_| Error::ConnectionLost("memory database lock poisoned".to_string()))?;
            map.clone()
        };
        let tx: Box<dyn DriverTransaction + '_> = Box::new(MemoryTransaction {
            conn: self,
            snapshot,
            is_active: true,
        });
        Ok(tx)
    }

    async fn close(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Transaction handle for in-memory database.
pub struct MemoryTransaction<'a> {
    conn: &'a mut MemoryConnection,
    snapshot: TableStore,
    is_active: bool,
}

impl DriverTransaction for MemoryTransaction<'_> {
    fn execute<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> Pin<Box<dyn Future<Output = Result<u64, Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.conn.execute_internal(sql, params)
        })
    }

    fn query<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Row>, Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.conn.query_internal(sql, params)
        })
    }

    fn commit<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.is_active = false;
            Ok(())
        })
    }

    fn rollback<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.is_active = false;
            let mut map =
                self.conn.tables.write().map_err(|_| {
                    Error::ConnectionLost("memory database lock poisoned".to_string())
                })?;
            *map = self.snapshot.clone();
            Ok(())
        })
    }
}

impl Drop for MemoryTransaction<'_> {
    fn drop(&mut self) {
        if self.is_active {
            self.is_active = false;
            if let Ok(mut map) = self.conn.tables.write() {
                *map = self.snapshot.clone();
            }
        }
    }
}

/// The column list of an `INSERT INTO t (a, b) VALUES ...`, or `None` when the
/// statement has none and its values are positional.
fn insert_column_list(sql: &str, values_at: usize) -> Option<Vec<String>> {
    let head = &sql[..values_at];
    let open = head.find('(')?;
    let close = head.rfind(')')?;
    (open < close).then(|| {
        head[open + 1..close]
            .split(',')
            .map(|c| c.trim().trim_matches('"').trim_matches('`').to_string())
            .filter(|c| !c.is_empty())
            .collect()
    })
}

/// SQLite's rule for an `INTEGER PRIMARY KEY` left `NULL`: it becomes the row id,
/// one past the largest one in the table. Without it an inserted row had no key,
/// and an `UPDATE`/`DELETE` by key could not find it again.
fn with_row_id(cols: &[Column], rows: &[Vec<Value>], mut row: Vec<Value>) -> Vec<Value> {
    let key = cols
        .iter()
        .position(|c| c.primary_key && c.decl.eq_ignore_ascii_case("INTEGER"));
    if let Some(k) = key.filter(|&k| row.get(k).is_some_and(Value::is_null)) {
        let next = rows
            .iter()
            .filter_map(|r| match r.get(k) {
                Some(Value::Integer(i)) => Some(*i),
                _ => None,
            })
            .max()
            .unwrap_or(0)
            + 1;
        row[k] = Value::Integer(next);
    }
    row
}

fn extract_table_name(sql: &str, prefix: &str) -> Result<String, Error> {
    let mut remainder = sql[prefix.len()..].trim();
    let upper = remainder.to_ascii_uppercase();
    if upper.starts_with("IF NOT EXISTS") {
        remainder = remainder[13..].trim();
    }
    let name = remainder
        .split(|c: char| c.is_whitespace() || c == '(' || c == ';')
        .find(|s| !s.is_empty())
        .ok_or_else(|| Error::query_error("failed to parse table name"))?;
    Ok(name.trim_matches('"').trim_matches('`').to_string())
}

fn extract_table_name_from_select(sql: &str) -> Result<String, Error> {
    let upper = sql.to_ascii_uppercase();
    if let Some(pos) = upper.find("FROM") {
        let remainder = &sql[pos + 4..].trim();
        let name = remainder
            .split(|c: char| c.is_whitespace() || c == ';' || c == ',')
            .find(|s| !s.is_empty())
            .ok_or_else(|| Error::query_error("failed to parse table name from query"))?;
        Ok(name.trim_matches('"').trim_matches('`').to_string())
    } else {
        Err(Error::query_error("SELECT statement missing FROM clause"))
    }
}

fn extract_tuples(s: &str) -> Vec<Vec<String>> {
    let mut tuples = Vec::new();
    let mut in_paren = false;
    let mut current_tuple = Vec::new();
    let mut current_token = String::new();
    let mut in_quote = false;

    for c in s.chars() {
        if c == '\'' {
            in_quote = !in_quote;
            current_token.push(c);
        } else if !in_quote && c == '(' {
            in_paren = true;
            current_tuple.clear();
            current_token.clear();
        } else if !in_quote && c == ')' {
            if in_paren {
                let trimmed = current_token.trim();
                if !trimmed.is_empty() {
                    current_tuple.push(trimmed.to_string());
                }
                tuples.push(current_tuple.clone());
                current_tuple.clear();
                current_token.clear();
                in_paren = false;
            }
        } else if !in_quote && in_paren && c == ',' {
            current_tuple.push(current_token.trim().to_string());
            current_token.clear();
        } else if in_paren {
            current_token.push(c);
        }
    }
    tuples
}

/// The equality conditions of a `WHERE`, as `(column index, value)` pairs.
///
/// Extracted from the `SELECT` path so that `DELETE` and `UPDATE` filter rows
/// by the SAME rule. Two copies of this would be two answers to *"which rows
/// does this statement touch?"*, and the day they disagreed a `DELETE` would
/// remove a row a `SELECT` never showed.
///
/// With no `WHERE` at all, the named parameters are matched against columns of
/// the same name — the behaviour the `SELECT` path already had.
fn where_filters(sql: &str, cols: &[Column], params: &NamedParams) -> Vec<(usize, Value)> {
    let upper = sql.to_ascii_uppercase();
    let mut filters: Vec<(usize, Value)> = Vec::new();
    let Some(where_pos) = upper.find("WHERE") else {
        for (param_name, param_val) in params.iter() {
            if let Some(pos) = posicao(cols, param_name) {
                filters.push((pos, param_val.clone()));
            }
        }
        return filters;
    };
    let where_part = &sql[where_pos + "WHERE".len()..];
    let end_pos = where_part
        .to_ascii_uppercase()
        .find("ORDER BY")
        .or_else(|| where_part.to_ascii_uppercase().find("LIMIT"))
        .or_else(|| where_part.find(';'))
        .unwrap_or(where_part.len());
    for cond in where_part[..end_pos].split("AND") {
        let Some((left, right)) = cond.split_once('=') else {
            continue;
        };
        let col = left
            .trim()
            .trim_matches('"')
            .trim_matches('`')
            .trim_matches('[')
            .trim_matches(']');
        let right_token = right.trim().trim_end_matches(';').trim();
        let val = if let Some(param_name) = right_token.strip_prefix(':') {
            params.get(param_name).cloned()
        } else {
            Some(parse_literal_value(right_token))
        };
        if let Some(v) = val
            && let Some(pos) = posicao(cols, col)
        {
            filters.push((pos, v));
        }
    }
    filters
}

/// Does this row satisfy every condition?
fn matches(row: &[Value], filters: &[(usize, Value)]) -> bool {
    filters.iter().all(|(pos, val)| &row[*pos] == val)
}

/// The assignments of an `UPDATE ... SET a = 1, b = :x`, as `(column, value)`.
///
/// It stops at the `WHERE`, which is the only thing that can follow a `SET` —
/// and reading past it would take the condition for another assignment.
fn set_clause(sql: &str, params: &NamedParams) -> Vec<(String, Value)> {
    let upper = sql.to_ascii_uppercase();
    let Some(set_pos) = upper.find(" SET ") else {
        return Vec::new();
    };
    let rest = &sql[set_pos + " SET ".len()..];
    let end = rest
        .to_ascii_uppercase()
        .find("WHERE")
        .or_else(|| rest.find(';'))
        .unwrap_or(rest.len());
    rest[..end]
        .split(',')
        .filter_map(|atrib| {
            let (left, right) = atrib.split_once('=')?;
            let col = left
                .trim()
                .trim_matches('"')
                .trim_matches('`')
                .trim_matches('[')
                .trim_matches(']')
                .to_string();
            let token = right.trim().trim_end_matches(';').trim();
            let val = match token.strip_prefix(':') {
                Some(nome) => params.get(nome).cloned()?,
                None => parse_literal_value(token),
            };
            Some((col, val))
        })
        .collect()
}

/// The `ORDER BY` clause of a statement, as `(column, descending)` pairs.
///
/// `None` when the statement has none. The clause ends at the `LIMIT`, at the
/// `;` or at the end of the text — the same three boundaries the `WHERE` parser
/// above already knows.
fn order_by_clause(sql: &str) -> Option<Vec<(String, bool)>> {
    let upper = sql.to_ascii_uppercase();
    let start = upper.find("ORDER BY")? + "ORDER BY".len();
    let rest = &sql[start..];
    let end = rest
        .to_ascii_uppercase()
        .find("LIMIT")
        .or_else(|| rest.find(';'))
        .unwrap_or(rest.len());
    let terms: Vec<(String, bool)> = rest[..end]
        .split(',')
        .filter_map(|term| {
            let mut words = term.split_whitespace();
            let column = words.next()?;
            let descending = words.next().is_some_and(|d| d.eq_ignore_ascii_case("DESC"));
            Some((
                column
                    .trim_matches('"')
                    .trim_matches('`')
                    .trim_matches('[')
                    .trim_matches(']')
                    .to_string(),
                descending,
            ))
        })
        .collect();
    (!terms.is_empty()).then_some(terms)
}

/// Orders two values the way SQL does, within the same type.
///
/// Values of different types keep the order they were inserted in — an
/// in-memory double is not a database, and inventing an order between a number
/// and a string would be inventing a semantics no real driver agrees on.
fn compare(a: &Value, b: &Value) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
        (Value::Real(x), Value::Real(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
        (Value::Text(x), Value::Text(y)) => x.cmp(y),
        (Value::Boolean(x), Value::Boolean(y)) => x.cmp(y),
        (Value::Blob(x), Value::Blob(y)) => x.cmp(y),
        // NULL comes first, matching the default in SQLite and PostgreSQL with `ASC`.
        (Value::Null, Value::Null) => Ordering::Equal,
        (Value::Null, _) => Ordering::Less,
        (_, Value::Null) => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

/// Applies the `ORDER BY` of the statement to the rows already filtered.
///
/// Stable on purpose: two rows that tie on the ordered column keep the order
/// they were inserted in, which is what makes a `SELECT` over a memory table
/// reproducible between runs.
fn ordenar(rows: &mut [Row], sql: &str) {
    let Some(terms) = order_by_clause(sql) else {
        return;
    };
    rows.sort_by(|a, b| {
        for (column, descending) in &terms {
            let (Some(ia), Some(ib)) = (a.column_index(column), b.column_index(column)) else {
                continue;
            };
            let ord = compare(&a.values()[ia], &b.values()[ib]);
            let ord = if *descending { ord.reverse() } else { ord };
            if ord != std::cmp::Ordering::Equal {
                return ord;
            }
        }
        std::cmp::Ordering::Equal
    });
}

/// Applies the `LIMIT` of the statement, after the ordering.
///
/// After, and never before: `ORDER BY version DESC LIMIT 1` means *the biggest*,
/// and cutting first would answer *the first one inserted* — which is exactly
/// the defect this pair of functions was born to fix.
fn limitar(rows: &mut Vec<Row>, sql: &str) {
    let upper = sql.to_ascii_uppercase();
    let Some(start) = upper.find("LIMIT") else {
        return;
    };
    let rest = &sql[start + "LIMIT".len()..];
    let Some(n) = rest
        .split(|c: char| c.is_whitespace() || c == ';')
        .find(|t| !t.is_empty())
        .and_then(|t| t.parse::<usize>().ok())
    else {
        return;
    };
    rows.truncate(n);
}

fn parse_literal_value(s: &str) -> Value {
    let trimmed = s.trim().trim_end_matches(';').trim();
    if (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        || (trimmed.starts_with('"') && trimmed.ends_with('"'))
    {
        let inner = &trimmed[1..trimmed.len() - 1];
        Value::Text(inner.to_string())
    } else if let Ok(i) = trimmed.parse::<i64>() {
        Value::Integer(i)
    } else if let Ok(f) = trimmed.parse::<f64>() {
        Value::Real(f)
    } else if trimmed.eq_ignore_ascii_case("TRUE") {
        Value::Boolean(true)
    } else if trimmed.eq_ignore_ascii_case("FALSE") {
        Value::Boolean(false)
    } else if trimmed.eq_ignore_ascii_case("NULL") {
        Value::Null
    } else {
        Value::Text(trimmed.to_string())
    }
}
