//! Pure SQL translation for datagrid queries against portadb SQL dialects.
//!
//! # Semantics and In-Memory Engine Equivalence
//!
//! Every translation here is designed to mirror the semantics of `datagrid`'s in-memory engine:
//!
//! 1. **SQL Injection Safety & Literals**:
//!    - Literals are never interpolated into SQL text. Every literal operand is bound to a
//!      named parameter (`:p0`, `:p1`, etc.).
//!    - All table, column, and identifier names are escaped via [`Dialect::quote_identifier`].
//!
//! 2. **Three-Valued Logic & Bitmap Complements (`Filter::Not`)**:
//!    - Predicates (`Eq`, `Lt`, `Le`, `Gt`, `Ge`, `Between`, `In`) evaluate to false on NULL cells in
//!      both SQL and the in-memory engine.
//!    - However, in `datagrid::Store::filter`, `Filter::Not` applies a bitwise complement (`Bitmap::not`)
//!      over the entire column bitmap. Therefore, `Not(Eq(col, 5))` evaluates to 1 (matching) for rows
//!      where `col IS NULL`.
//!    - In standard SQL, `NOT (col = 5)` evaluates to `UNKNOWN` (falsy) when `col` is NULL.
//!    - To match the engine's bitwise negation exactly, `Not(x)` is translated as `NOT COALESCE((x), 0)`.
//!      When `(x)` is `NULL`, `COALESCE` converts it to `0` (false), which `NOT` flips to `1` (true).
//!
//! 3. **Substring and Prefix Matching (`Contains`, `StartsWith`)**:
//!    - Case-sensitive: translated using `instr(col, :p) > 0` and `substr(col, 1, length(:p)) = :p`.
//!      This eliminates wildcards (`%`, `_`) and escaping edge cases inherent to `LIKE`.
//!    - Case-insensitive: translated using `lower(...)` on both operand and column.
//!    - *Collation and Case Folding Limit*: SQLite's built-in `lower()` function only folds ASCII
//!      characters (`A-Z` to `a-z`), whereas `datagrid`'s in-memory engine uses Rust's full Unicode
//!      `to_lowercase()`. This limitation applies to case-insensitive queries on non-ASCII characters.
//!
//! 4. **Text Comparison**:
//!    - The in-memory engine compares string slices byte-wise via `as_bytes().cmp(...)`.
//!    - SQLite's default collation (`BINARY`) compares text byte-by-byte using standard `memcmp`,
//!      yielding identical total ordering for text sorting.
//!
//! 5. **Booleans**:
//!    - In SQLite, booleans are represented as integer literals `0` (false) and `1` (true).
//!
//! 6. **Ordering (`ORDER BY`) and Stable Tie-Breaking**:
//!    - Null placement is written explicitly using portable boolean expressions:
//!      `(col IS NULL) DESC` for `Nulls::First` (since `1 > 0`), and
//!      `(col IS NULL) ASC` for `Nulls::Last` (since `0 < 1`).
//!    - The table's unique `key` column (e.g. `rowid` in SQLite) is always appended with `ASC` order
//!      as a deterministic tie-breaker at the end of `ORDER BY`. Since rows inserted into SQLite in
//!      store order have monotonically increasing row ids, ties are resolved in exact insertion/store
//!      order, mirroring the engine's stable sorting.
//!
//! 7. **Totals and Aggregation**:
//!    - `Aggregate::Count` is translated as `COUNT(*)` to count every row including nulls, matching
//!      `update_accumulators` in `datagrid::group`.
//!    - `Aggregate::Sum`, `Avg`, `Min`, `Max` aggregate non-null values and evaluate to `NULL` when
//!      no non-null values exist, matching `finalize_total`.

use datagrid::ColumnInfo;
use datagrid::filter::{Filter, Owned};
use datagrid::group::Aggregate;
use datagrid::query::Query;
use datagrid::sort::{Nulls, Order, SortKey};
use datagrid::source::Change;
use portadb::dialect::Dialect;
use portadb::params::NamedParams;
use portadb::value::Value;

/// Schema metadata for a SQL table queried through [`crate::SqlSource`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Table {
    /// Table name as recognized by the database.
    pub name: String,
    /// Column definitions in ordinal grid order.
    pub columns: Vec<ColumnInfo>,
    /// Unique column identifier used for stable tie-breaking and keyset pagination.
    /// Defaults to `"rowid"` for SQLite.
    pub key: String,
}

impl Table {
    /// Constructs a `Table` with an explicit name, column list, and unique key column.
    pub fn new(name: impl Into<String>, columns: Vec<ColumnInfo>, key: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            columns,
            key: key.into(),
        }
    }

    /// Constructs a `Table` defaulting the unique key column to SQLite's `"rowid"`.
    /// Whether `column` may hold NULL; the key and unknown names count as NOT NULL.
    pub fn is_nullable(&self, column: &str) -> bool {
        self.columns
            .iter()
            .find(|c| c.name == column)
            .is_some_and(|c| c.nullable)
    }

    pub fn sqlite(name: impl Into<String>, columns: Vec<ColumnInfo>) -> Self {
        Self::new(name, columns, "rowid")
    }
}

/// Accumulator context binding named parameters (:p0, :p1, ...) during query translation.
#[derive(Default)]
pub struct ParamContext {
    params: NamedParams,
    counter: usize,
}

impl ParamContext {
    /// Creates a new empty parameter context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Binds an engine value, returning the `:p{index}` placeholder string.
    pub fn bind(&mut self, val: impl Into<Value>) -> String {
        let name = format!("p{}", self.counter);
        self.counter += 1;
        self.params.insert(&name, val.into());
        format!(":{name}")
    }

    /// Binds an in-memory owned cell value, returning the `:p{index}` placeholder string.
    pub fn bind_owned(&mut self, owned: &Owned) -> String {
        let val = match owned {
            Owned::Int(i) => Value::Integer(*i),
            Owned::Float(f) => Value::Real(*f),
            Owned::Bool(b) => Value::Integer(if *b { 1 } else { 0 }),
            Owned::Text(s) => Value::Text(s.clone()),
        };
        self.bind(val)
    }

    /// Finalizes the context and returns the populated [`NamedParams`].
    pub fn finish(self) -> NamedParams {
        self.params
    }
}

/// Translates a `datagrid::Filter` tree into a SQL WHERE predicate expression.
///
/// Returns `None` if the filter is empty.
pub fn translate_filter(
    filter: &Filter,
    dialect: &dyn Dialect,
    ctx: &mut ParamContext,
) -> Option<String> {
    match filter {
        Filter::And(children) => {
            if children.is_empty() {
                return Some("1 = 1".to_string());
            }
            let mut parts = Vec::with_capacity(children.len());
            for child in children {
                if let Some(part) = translate_filter(child, dialect, ctx) {
                    parts.push(part);
                }
            }
            if parts.is_empty() {
                None
            } else if parts.len() == 1 {
                Some(parts.into_iter().next().expect("parts not empty"))
            } else {
                Some(format!("({})", parts.join(" AND ")))
            }
        }
        Filter::Or(children) => {
            if children.is_empty() {
                return Some("1 = 0".to_string());
            }
            let mut parts = Vec::with_capacity(children.len());
            for child in children {
                if let Some(part) = translate_filter(child, dialect, ctx) {
                    parts.push(part);
                }
            }
            if parts.is_empty() {
                None
            } else if parts.len() == 1 {
                Some(parts.into_iter().next().expect("parts not empty"))
            } else {
                Some(format!("({})", parts.join(" OR ")))
            }
        }
        Filter::Not(child) => {
            // In the in-memory engine, Filter::Not inverts the entire validity/match bitmap.
            // When an expression evaluates to NULL, SQLite treats it as unknown/falsy in WHERE,
            // but the bitmap complement in the engine produces a 1 (matching NULL rows).
            // NOT COALESCE((x), 0) turns NULL into 0, which NOT flips to 1, exactly matching the engine.
            let inner =
                translate_filter(child, dialect, ctx).unwrap_or_else(|| "1 = 1".to_string());
            Some(format!("NOT COALESCE(({inner}), 0)"))
        }
        Filter::IsNull(col) => {
            let quoted = dialect.quote_identifier(col);
            Some(format!("{quoted} IS NULL"))
        }
        Filter::Eq(col, val) => {
            let quoted = dialect.quote_identifier(col);
            let p = ctx.bind_owned(val);
            Some(format!("{quoted} = {p}"))
        }
        Filter::Lt(col, val) => {
            let quoted = dialect.quote_identifier(col);
            let p = ctx.bind_owned(val);
            Some(format!("{quoted} < {p}"))
        }
        Filter::Le(col, val) => {
            let quoted = dialect.quote_identifier(col);
            let p = ctx.bind_owned(val);
            Some(format!("{quoted} <= {p}"))
        }
        Filter::Gt(col, val) => {
            let quoted = dialect.quote_identifier(col);
            let p = ctx.bind_owned(val);
            Some(format!("{quoted} > {p}"))
        }
        Filter::Ge(col, val) => {
            let quoted = dialect.quote_identifier(col);
            let p = ctx.bind_owned(val);
            Some(format!("{quoted} >= {p}"))
        }
        Filter::Between(col, lo, hi) => {
            let quoted = dialect.quote_identifier(col);
            let p_lo = ctx.bind_owned(lo);
            let p_hi = ctx.bind_owned(hi);
            Some(format!("{quoted} BETWEEN {p_lo} AND {p_hi}"))
        }
        Filter::In(col, list) => {
            if list.is_empty() {
                return Some("1 = 0".to_string());
            }
            let quoted = dialect.quote_identifier(col);
            let placeholders: Vec<String> = list.iter().map(|item| ctx.bind_owned(item)).collect();
            Some(format!("{quoted} IN ({})", placeholders.join(", ")))
        }
        Filter::Contains {
            column,
            text,
            ignore_case,
        } => {
            let quoted = dialect.quote_identifier(column);
            let p = ctx.bind(Value::Text(text.clone()));
            if *ignore_case {
                // SQLite lower() folds ASCII only; in-memory engine uses full Unicode to_lowercase.
                Some(format!("instr(lower({quoted}), lower({p})) > 0"))
            } else {
                Some(format!("instr({quoted}, {p}) > 0"))
            }
        }
        Filter::StartsWith {
            column,
            text,
            ignore_case,
        } => {
            let quoted = dialect.quote_identifier(column);
            let p = ctx.bind(Value::Text(text.clone()));
            if *ignore_case {
                // SQLite lower() folds ASCII only; in-memory engine uses full Unicode to_lowercase.
                Some(format!(
                    "substr(lower({quoted}), 1, length({p})) = lower({p})"
                ))
            } else {
                Some(format!("substr({quoted}, 1, length({p})) = {p}"))
            }
        }
        Filter::EqIgnoreCase { column, text } => {
            let quoted = dialect.quote_identifier(column);
            let p = ctx.bind(Value::Text(text.clone()));
            Some(format!("lower({quoted}) = lower({p})"))
        }
    }
}

/// Builds the normalized ORDER BY keys, appending the unique table key as a tie-breaker.
pub fn effective_sort_keys(sort: &[SortKey], key_col: &str) -> Vec<SortKey> {
    let mut keys = Vec::with_capacity(sort.len() + 1);
    for k in sort {
        if !keys
            .iter()
            .any(|existing: &SortKey| existing.column == k.column)
        {
            keys.push(k.clone());
        }
    }
    if !keys.iter().any(|k| k.column.eq_ignore_ascii_case(key_col)) {
        keys.push(SortKey {
            column: key_col.to_string(),
            order: Order::Asc,
            nulls: Nulls::Last,
        });
    }
    keys
}

/// Translates sort keys into an SQL ORDER BY clause with explicit portable nulls placement.
///
/// The `(col IS NULL)` term is written only when it changes the order: never for a
/// NOT NULL column (or the table key), and never when SQLite's own placement
/// already matches (it sorts NULL as the smallest value: first in ASC, last in
/// DESC). An expression term stops SQLite from walking an index on `col`, so
/// leaving it out keeps the first window an index walk instead of a full sort.
pub fn translate_order_by(table: &Table, sort_keys: &[SortKey], dialect: &dyn Dialect) -> String {
    let parts: Vec<String> = sort_keys
        .iter()
        .map(|k| {
            let quoted = dialect.quote_identifier(&k.column);
            let dir = match k.order {
                Order::Asc => "ASC",
                Order::Desc => "DESC",
            };
            let natural = matches!(
                (k.order, k.nulls),
                (Order::Asc, Nulls::First) | (Order::Desc, Nulls::Last)
            );
            if natural || !table.is_nullable(&k.column) {
                return format!("{quoted} {dir}");
            }
            let null_term = match k.nulls {
                // Nulls First: col IS NULL evaluates to 1 for NULL, 0 for not null. In DESC order, 1 comes first.
                Nulls::First => format!("({quoted} IS NULL) DESC"),
                // Nulls Last: In ASC order, 0 (not null) comes before 1 (null).
                Nulls::Last => format!("({quoted} IS NULL) ASC"),
            };
            format!("{null_term}, {quoted} {dir}")
        })
        .collect();

    parts.join(", ")
}

/// Translates a COUNT query matching `query.filter`.
pub fn translate_count(
    table: &Table,
    query: &Query,
    dialect: &dyn Dialect,
) -> (String, NamedParams) {
    let mut ctx = ParamContext::new();
    let quoted_table = dialect.quote_identifier(&table.name);

    let where_sql = query
        .filter
        .as_ref()
        .and_then(|f| translate_filter(f, dialect, &mut ctx));

    let sql = match where_sql {
        Some(w) => format!("SELECT COUNT(*) FROM {quoted_table} WHERE {w}"),
        None => format!("SELECT COUNT(*) FROM {quoted_table}"),
    };

    (sql, ctx.finish())
}

/// Translates an offset-based FETCH query (`LIMIT ... OFFSET ...`).
pub fn translate_fetch_offset(
    table: &Table,
    query: &Query,
    range: std::ops::Range<usize>,
    dialect: &dyn Dialect,
) -> (String, NamedParams) {
    let mut ctx = ParamContext::new();
    let quoted_table = dialect.quote_identifier(&table.name);

    let mut select_cols: Vec<String> = table
        .columns
        .iter()
        .map(|c| dialect.quote_identifier(&c.name))
        .collect();
    select_cols.push(dialect.quote_identifier(&table.key));
    let cols_sql = select_cols.join(", ");

    let where_sql = query
        .filter
        .as_ref()
        .and_then(|f| translate_filter(f, dialect, &mut ctx));

    let sort_keys = effective_sort_keys(&query.sort, &table.key);
    let order_by_sql = translate_order_by(table, &sort_keys, dialect);

    let limit_cnt = range.end.saturating_sub(range.start);
    let p_limit = ctx.bind(Value::Integer(limit_cnt as i64));

    let mut sql = format!("SELECT {cols_sql} FROM {quoted_table}");
    if let Some(w) = where_sql {
        sql.push_str(" WHERE ");
        sql.push_str(&w);
    }
    sql.push_str(" ORDER BY ");
    sql.push_str(&order_by_sql);
    if range.start == 0 {
        sql.push_str(&format!(" LIMIT {p_limit}"));
    } else {
        let p_offset = ctx.bind(Value::Integer(range.start as i64));
        sql.push_str(&format!(" LIMIT {p_limit} OFFSET {p_offset}"));
    }

    (sql, ctx.finish())
}

/// Reverses sort keys by flipping both order (Asc <-> Desc) and nulls placement (First <-> Last).
pub fn reversed(keys: &[SortKey]) -> Vec<SortKey> {
    keys.iter()
        .map(|k| SortKey {
            column: k.column.clone(),
            order: match k.order {
                Order::Asc => Order::Desc,
                Order::Desc => Order::Asc,
            },
            nulls: match k.nulls {
                Nulls::First => Nulls::Last,
                Nulls::Last => Nulls::First,
            },
        })
        .collect()
}

/// Translates a keyset inequality predicate specifying rows strictly after `tuple` in `sort_keys` order.
pub fn keyset_after_sql(
    table: &Table,
    sort_keys: &[SortKey],
    tuple: &[Option<Owned>],
    ctx: &mut ParamContext,
    dialect: &dyn Dialect,
) -> String {
    assert_eq!(
        sort_keys.len(),
        tuple.len(),
        "tuple values count must match sort keys"
    );

    let mut or_branches = Vec::new();

    for i in 0..sort_keys.len() {
        let mut and_terms = Vec::with_capacity(i + 1);

        // Preceding keys must be strictly equal
        for j in 0..i {
            let col = dialect.quote_identifier(&sort_keys[j].column);
            match &tuple[j] {
                Some(val) => {
                    let p = ctx.bind_owned(val);
                    and_terms.push(format!("{col} = {p}"));
                }
                None => {
                    and_terms.push(format!("{col} IS NULL"));
                }
            }
        }

        // Current key must be strictly greater in sort order
        let cur_col = dialect.quote_identifier(&sort_keys[i].column);
        let cur_key = &sort_keys[i];
        let branch_condition = match (cur_key.order, cur_key.nulls) {
            (Order::Asc, Nulls::Last) => match &tuple[i] {
                Some(val) => {
                    let p = ctx.bind_owned(val);
                    Some(format!("({cur_col} > {p} OR {cur_col} IS NULL)"))
                }
                None => None, // Nulls are last; nothing comes strictly after NULL
            },
            (Order::Asc, Nulls::First) => match &tuple[i] {
                Some(val) => {
                    let p = ctx.bind_owned(val);
                    Some(format!("{cur_col} > {p}"))
                }
                None => Some(format!("{cur_col} IS NOT NULL")),
            },
            (Order::Desc, Nulls::Last) => match &tuple[i] {
                Some(val) => {
                    let p = ctx.bind_owned(val);
                    Some(format!("({cur_col} < {p} OR {cur_col} IS NULL)"))
                }
                None => None,
            },
            (Order::Desc, Nulls::First) => match &tuple[i] {
                Some(val) => {
                    let p = ctx.bind_owned(val);
                    Some(format!("{cur_col} < {p}"))
                }
                None => Some(format!("{cur_col} IS NOT NULL")),
            },
        };

        if let Some(cond) = branch_condition {
            and_terms.push(cond);
            if and_terms.len() == 1 {
                or_branches.push(and_terms.into_iter().next().expect("not empty"));
            } else {
                or_branches.push(format!("({})", and_terms.join(" AND ")));
            }
        }
    }

    let mut keyset_sql = if or_branches.is_empty() {
        "1 = 0".to_string()
    } else if or_branches.len() == 1 {
        or_branches.into_iter().next().expect("not empty")
    } else {
        format!("({})", or_branches.join(" OR "))
    };

    // Every branch above implies `c0 >= v0` (ASC) / `c0 <= v0` (DESC) whenever no
    // NULL of `c0` can come after the bookmark. Saying it as a plain leading range
    // is redundant for the result but lets SQLite seek the index on `c0` instead
    // of walking it from the start (1M rows, row 900 000: ~60 ms -> a seek).
    let first = &sort_keys[0];
    if let Some(v0) = &tuple[0] {
        let no_null_after = !table.is_nullable(&first.column) || first.nulls == Nulls::First;
        if no_null_after && sort_keys.len() > 1 {
            let c0 = dialect.quote_identifier(&first.column);
            let p = ctx.bind_owned(v0);
            let op = match first.order {
                Order::Asc => ">=",
                Order::Desc => "<=",
            };
            keyset_sql = format!("{c0} {op} {p} AND {keyset_sql}");
        }
    }

    keyset_sql
}

/// Translates a keyset-based FETCH query using a previous row's bookmark tuple and a skip offset.
pub fn translate_fetch_keyset(
    table: &Table,
    query: &Query,
    bookmark_values: &[Option<Owned>],
    limit: usize,
    skip: usize,
    dialect: &dyn Dialect,
) -> (String, NamedParams) {
    let mut ctx = ParamContext::new();
    let quoted_table = dialect.quote_identifier(&table.name);

    let mut select_cols: Vec<String> = table
        .columns
        .iter()
        .map(|c| dialect.quote_identifier(&c.name))
        .collect();
    select_cols.push(dialect.quote_identifier(&table.key));
    let cols_sql = select_cols.join(", ");

    let sort_keys = effective_sort_keys(&query.sort, &table.key);
    let keyset_sql = keyset_after_sql(table, &sort_keys, bookmark_values, &mut ctx, dialect);

    let user_where = query
        .filter
        .as_ref()
        .and_then(|f| translate_filter(f, dialect, &mut ctx));

    let final_where = match user_where {
        Some(uw) => format!("({uw}) AND ({keyset_sql})"),
        None => keyset_sql,
    };

    let order_by_sql = translate_order_by(table, &sort_keys, dialect);
    let p_limit = ctx.bind(Value::Integer(limit as i64));
    let p_skip = ctx.bind(Value::Integer(skip as i64));

    let sql = format!(
        "SELECT {cols_sql} FROM {quoted_table} WHERE {final_where} ORDER BY {order_by_sql} LIMIT {p_limit} OFFSET {p_skip}"
    );

    (sql, ctx.finish())
}

/// Translates a query selecting the effective sort key tuple for a specific row id.
pub fn translate_sort_tuple_of(
    table: &Table,
    query: &Query,
    id: u64,
    dialect: &dyn Dialect,
) -> (String, NamedParams) {
    let mut ctx = ParamContext::new();
    let quoted_table = dialect.quote_identifier(&table.name);
    let quoted_key = dialect.quote_identifier(&table.key);

    let sort_keys = effective_sort_keys(&query.sort, &table.key);
    let cols_sql = sort_keys
        .iter()
        .map(|k| dialect.quote_identifier(&k.column))
        .collect::<Vec<_>>()
        .join(", ");

    let p_id = ctx.bind(Value::Integer(id as i64));
    let id_clause = format!("{quoted_key} = {p_id}");

    let user_where = query
        .filter
        .as_ref()
        .and_then(|f| translate_filter(f, dialect, &mut ctx));

    let where_sql = match user_where {
        Some(w) => format!("{id_clause} AND ({w})"),
        None => id_clause,
    };

    let sql = format!("SELECT {cols_sql} FROM {quoted_table} WHERE {where_sql}");
    (sql, ctx.finish())
}

/// Translates a count query for rows strictly preceding `tuple` in `query` sort order.
pub fn translate_count_before(
    table: &Table,
    query: &Query,
    tuple: &[Option<Owned>],
    dialect: &dyn Dialect,
) -> (String, NamedParams) {
    let mut ctx = ParamContext::new();
    let quoted_table = dialect.quote_identifier(&table.name);

    let sort_keys = effective_sort_keys(&query.sort, &table.key);
    let rev_keys = reversed(&sort_keys);
    let before_sql = keyset_after_sql(table, &rev_keys, tuple, &mut ctx, dialect);

    let user_where = query
        .filter
        .as_ref()
        .and_then(|f| translate_filter(f, dialect, &mut ctx));

    let where_sql = match user_where {
        Some(w) => format!("({w}) AND ({before_sql})"),
        None => format!("({before_sql})"),
    };

    let sql = format!("SELECT COUNT(*) FROM {quoted_table} WHERE {where_sql}");
    (sql, ctx.finish())
}

/// Translates a query to locate the first row matching `extra` within the filtered + sorted `query`.
pub fn translate_locate(
    table: &Table,
    query: &Query,
    extra: &Filter,
    dialect: &dyn Dialect,
) -> (String, NamedParams) {
    let mut ctx = ParamContext::new();
    let quoted_table = dialect.quote_identifier(&table.name);
    let quoted_key = dialect.quote_identifier(&table.key);

    let sort_keys = effective_sort_keys(&query.sort, &table.key);
    let mut cols = Vec::with_capacity(1 + sort_keys.len());
    cols.push(quoted_key);
    for k in &sort_keys {
        cols.push(dialect.quote_identifier(&k.column));
    }
    let cols_sql = cols.join(", ");

    let extra_sql =
        translate_filter(extra, dialect, &mut ctx).unwrap_or_else(|| "1 = 1".to_string());
    let user_where = query
        .filter
        .as_ref()
        .and_then(|f| translate_filter(f, dialect, &mut ctx));

    let where_sql = match user_where {
        Some(w) => format!("({w}) AND ({extra_sql})"),
        None => format!("({extra_sql})"),
    };

    let order_by_sql = translate_order_by(table, &sort_keys, dialect);

    let sql = format!(
        "SELECT {cols_sql} FROM {quoted_table} WHERE {where_sql} ORDER BY {order_by_sql} LIMIT 1"
    );
    (sql, ctx.finish())
}

/// Translates a query generating equidistant checkpoint bookmarks across the ordered result.
pub fn translate_checkpoints(
    table: &Table,
    query: &Query,
    stride: usize,
    dialect: &dyn Dialect,
) -> (String, NamedParams) {
    let mut ctx = ParamContext::new();
    let quoted_table = dialect.quote_identifier(&table.name);

    let sort_keys = effective_sort_keys(&query.sort, &table.key);
    let cols_sql = sort_keys
        .iter()
        .map(|k| dialect.quote_identifier(&k.column))
        .collect::<Vec<_>>()
        .join(", ");

    let order_by_sql = translate_order_by(table, &sort_keys, dialect);

    let user_where = query
        .filter
        .as_ref()
        .and_then(|f| translate_filter(f, dialect, &mut ctx));

    let inner_where = match user_where {
        Some(w) => format!(" WHERE {w}"),
        None => String::new(),
    };

    let p_stride = ctx.bind(Value::Integer(stride as i64));

    let sql = format!(
        "SELECT {cols_sql} FROM (SELECT {cols_sql}, ROW_NUMBER() OVER (ORDER BY {order_by_sql}) AS crawrad_rn FROM {quoted_table}{inner_where}) WHERE crawrad_rn % {p_stride} = 0 ORDER BY crawrad_rn"
    );
    (sql, ctx.finish())
}

/// Translates a GROUP BY query for a specific hierarchy level and parent path.
pub fn translate_groups(
    table: &Table,
    query: &Query,
    parent: &[Option<Owned>],
    dialect: &dyn Dialect,
) -> Result<(String, NamedParams), String> {
    let level = parent.len();
    if level >= query.group_by.len() {
        return Err("group level exceeds query grouping depth".to_string());
    }

    let mut ctx = ParamContext::new();
    let quoted_table = dialect.quote_identifier(&table.name);

    let group_level = &query.group_by[level];
    let quoted_group_col = dialect.quote_identifier(&group_level.column);

    let mut select_terms = Vec::with_capacity(2 + group_level.totals.len());
    select_terms.push(quoted_group_col.clone());
    select_terms.push("COUNT(*)".to_string());

    for tot in &group_level.totals {
        select_terms.push(translate_aggregate(tot, dialect));
    }
    let select_sql = select_terms.join(", ");

    // Filter by equality to parent path
    let mut where_parts = Vec::new();
    for (i, p_val) in parent.iter().enumerate() {
        let parent_col_name = &query.group_by[i].column;
        let quoted_parent_col = dialect.quote_identifier(parent_col_name);
        match p_val {
            Some(v) => {
                let p = ctx.bind_owned(v);
                where_parts.push(format!("{quoted_parent_col} = {p}"));
            }
            None => {
                where_parts.push(format!("{quoted_parent_col} IS NULL"));
            }
        }
    }

    // Append query.filter
    if let Some(user_f) = &query.filter
        && let Some(user_w) = translate_filter(user_f, dialect, &mut ctx)
    {
        where_parts.push(user_w);
    }

    let mut sql = format!("SELECT {select_sql} FROM {quoted_table}");
    if !where_parts.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&where_parts.join(" AND "));
    }
    sql.push_str(&format!(
        " GROUP BY {quoted_group_col} ORDER BY ({quoted_group_col} IS NULL) DESC, {quoted_group_col} ASC"
    ));

    Ok((sql, ctx.finish()))
}

/// Formats an aggregate expression for a total aggregation.
pub fn translate_aggregate(tot: &datagrid::group::Total, dialect: &dyn Dialect) -> String {
    let quoted_tot_col = dialect.quote_identifier(&tot.column);
    match tot.aggregate {
        Aggregate::Count => "COUNT(*)".to_string(),
        Aggregate::Sum => format!("SUM({quoted_tot_col})"),
        Aggregate::Avg => format!("AVG({quoted_tot_col})"),
        Aggregate::Min => format!("MIN({quoted_tot_col})"),
        Aggregate::Max => format!("MAX({quoted_tot_col})"),
    }
}

/// Translates a summary grand total query selecting all aggregates in `query.summary`.
pub fn translate_summary(
    table: &Table,
    query: &Query,
    dialect: &dyn Dialect,
) -> (String, NamedParams) {
    let mut ctx = ParamContext::new();
    let quoted_table = dialect.quote_identifier(&table.name);

    let select_sql = query
        .summary
        .iter()
        .map(|tot| translate_aggregate(tot, dialect))
        .collect::<Vec<_>>()
        .join(", ");

    let user_where = query
        .filter
        .as_ref()
        .and_then(|f| translate_filter(f, dialect, &mut ctx));

    let where_sql = match user_where {
        Some(w) => format!(" WHERE {w}"),
        None => String::new(),
    };

    let sql = format!("SELECT {select_sql} FROM {quoted_table}{where_sql}");
    (sql, ctx.finish())
}

/// Translates a DISTINCT query selecting unique values for a column up to `limit`.
pub fn translate_distinct(
    table: &Table,
    query: &Query,
    column: &str,
    limit: usize,
    dialect: &dyn Dialect,
) -> (String, NamedParams) {
    let mut ctx = ParamContext::new();
    let quoted_table = dialect.quote_identifier(&table.name);
    let quoted_col = dialect.quote_identifier(column);

    let user_where = query
        .filter
        .as_ref()
        .and_then(|f| translate_filter(f, dialect, &mut ctx));

    let where_sql = match user_where {
        Some(w) => format!(" WHERE {w}"),
        None => String::new(),
    };

    let p_limit = ctx.bind(Value::Integer(limit as i64));
    let sql = format!(
        "SELECT DISTINCT {quoted_col} FROM {quoted_table}{where_sql} ORDER BY {quoted_col} LIMIT {p_limit}"
    );
    (sql, ctx.finish())
}

/// Binds one value to write; `None` binds NULL.
fn bind_change(ctx: &mut ParamContext, value: &Option<Owned>) -> String {
    match value {
        Some(owned) => ctx.bind_owned(owned),
        None => ctx.bind(Value::Null),
    }
}

/// Translates `UPDATE table SET c = :p, … WHERE key = :p` for one row (#936 P2).
pub fn translate_update(
    table: &Table,
    id: u64,
    changes: &[Change],
    dialect: &dyn Dialect,
) -> (String, NamedParams) {
    let mut ctx = ParamContext::new();
    let sets: Vec<String> = changes
        .iter()
        .map(|(column, value)| {
            let p = bind_change(&mut ctx, value);
            format!("{} = {p}", dialect.quote_identifier(column))
        })
        .collect();
    let p_id = ctx.bind(Value::Integer(id as i64));
    let sql = format!(
        "UPDATE {} SET {} WHERE {} = {p_id}",
        dialect.quote_identifier(&table.name),
        sets.join(", "),
        dialect.quote_identifier(&table.key),
    );
    (sql, ctx.finish())
}

/// Translates `INSERT INTO table (c, …) VALUES (:p, …) RETURNING key` (#936 P2); no values
/// inserts a row of database defaults.
pub fn translate_insert(
    table: &Table,
    values: &[Change],
    dialect: &dyn Dialect,
) -> (String, NamedParams) {
    let mut ctx = ParamContext::new();
    let quoted_table = dialect.quote_identifier(&table.name);
    let quoted_key = dialect.quote_identifier(&table.key);
    if values.is_empty() {
        return (
            format!("INSERT INTO {quoted_table} DEFAULT VALUES RETURNING {quoted_key}"),
            ctx.finish(),
        );
    }
    let mut columns = Vec::with_capacity(values.len());
    let mut params = Vec::with_capacity(values.len());
    for (column, value) in values {
        columns.push(dialect.quote_identifier(column));
        params.push(bind_change(&mut ctx, value));
    }
    let sql = format!(
        "INSERT INTO {quoted_table} ({}) VALUES ({}) RETURNING {quoted_key}",
        columns.join(", "),
        params.join(", "),
    );
    (sql, ctx.finish())
}

#[cfg(test)]
#[path = "test/translate/test.rs"]
mod test;
