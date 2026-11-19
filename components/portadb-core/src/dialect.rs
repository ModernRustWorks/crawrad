use crate::database::Database;
use crate::error::Error;
use crate::params::NamedParams;
use crate::value::Value;

/// Prepared query statement containing rewritten SQL and ordered positional parameter values.
#[derive(Debug, Clone, PartialEq)]
pub struct PreparedQuery {
    /// Rewritten SQL statement ready for driver execution.
    pub sql: String,
    /// Parameter values ordered exactly to match placeholders in `sql`.
    pub params: Vec<Value>,
}

/// SQL Dialect contract representing target RDBMS syntax variations.
///
/// Handles database-specific parameter placeholder generation (e.g. `?` vs `$1` vs `@p1`),
/// identifier quoting, and SQL query preparation.
pub trait Dialect: Send + Sync {
    /// Canonical dialect name (e.g. "sqlite", "postgres", "mysql", "sqlserver").
    fn name(&self) -> &'static str;

    /// Corresponding database family.
    fn database(&self) -> Database;

    /// Quotes an SQL identifier according to this dialect's rules.
    fn quote_identifier(&self, ident: &str) -> String;

    /// Prepares a query by rewriting `:named` parameters to the target dialect's native placeholders.
    fn prepare_query(&self, sql: &str, params: &NamedParams) -> Result<PreparedQuery, Error>;
}

/// SQLite dialect implementation.
///
/// Uses `?` for positional parameters and double quotes for identifiers.
#[derive(Debug, Default, Clone, Copy)]
pub struct SqliteDialect;

impl Dialect for SqliteDialect {
    fn name(&self) -> &'static str {
        "sqlite"
    }

    fn database(&self) -> Database {
        Database::Sqlite
    }

    fn quote_identifier(&self, ident: &str) -> String {
        format!("\"{ident}\"")
    }

    fn prepare_query(&self, sql: &str, params: &NamedParams) -> Result<PreparedQuery, Error> {
        rewrite_named_query(sql, params, |_idx| "?".to_string())
    }
}

/// PostgreSQL dialect implementation.
///
/// Uses `$1`, `$2`, ... for positional parameters and double quotes for identifiers.
#[derive(Debug, Default, Clone, Copy)]
pub struct PostgresDialect;

impl Dialect for PostgresDialect {
    fn name(&self) -> &'static str {
        "postgres"
    }

    fn database(&self) -> Database {
        Database::Postgres
    }

    fn quote_identifier(&self, ident: &str) -> String {
        format!("\"{ident}\"")
    }

    fn prepare_query(&self, sql: &str, params: &NamedParams) -> Result<PreparedQuery, Error> {
        rewrite_named_query(sql, params, |idx| format!("${idx}"))
    }
}

/// MySQL dialect implementation.
///
/// Uses `?` for positional parameters and backticks for identifiers.
#[derive(Debug, Default, Clone, Copy)]
pub struct MySqlDialect;

impl Dialect for MySqlDialect {
    fn name(&self) -> &'static str {
        "mysql"
    }

    fn database(&self) -> Database {
        Database::MySql
    }

    fn quote_identifier(&self, ident: &str) -> String {
        format!("`{ident}`")
    }

    fn prepare_query(&self, sql: &str, params: &NamedParams) -> Result<PreparedQuery, Error> {
        rewrite_named_query(sql, params, |_idx| "?".to_string())
    }
}

/// Microsoft SQL Server dialect implementation.
///
/// Uses `@p1`, `@p2`, ... for positional parameters and square brackets for identifiers.
#[derive(Debug, Default, Clone, Copy)]
pub struct SqlServerDialect;

impl Dialect for SqlServerDialect {
    fn name(&self) -> &'static str {
        "sqlserver"
    }

    fn database(&self) -> Database {
        Database::SqlServer
    }

    fn quote_identifier(&self, ident: &str) -> String {
        format!("[{ident}]")
    }

    fn prepare_query(&self, sql: &str, params: &NamedParams) -> Result<PreparedQuery, Error> {
        rewrite_named_query(sql, params, |idx| format!("@p{idx}"))
    }
}

/// Redis command dialect implementation.
#[derive(Debug, Default, Clone, Copy)]
pub struct RedisDialect;

impl Dialect for RedisDialect {
    fn name(&self) -> &'static str {
        "redis"
    }

    fn database(&self) -> Database {
        Database::Redis
    }

    fn quote_identifier(&self, ident: &str) -> String {
        ident.to_string()
    }

    fn prepare_query(&self, sql: &str, params: &NamedParams) -> Result<PreparedQuery, Error> {
        rewrite_named_query(sql, params, |_idx| "?".to_string())
    }
}

/// Returns the default dialect for the given database system.
#[must_use]
pub fn dialect_for(database: Database) -> &'static (dyn Dialect + Send + Sync) {
    match database {
        Database::Sqlite => &SqliteDialect,
        Database::Postgres => &PostgresDialect,
        Database::MySql => &MySqlDialect,
        Database::SqlServer => &SqlServerDialect,
        Database::Redis => &RedisDialect,
    }
}

/// Generic SQL rewriter that parses `:identifier` parameters outside string literals and comments,
/// converting each parameter according to `format_placeholder(1-based index)`.
fn rewrite_named_query<F>(
    sql: &str,
    params: &NamedParams,
    format_placeholder: F,
) -> Result<PreparedQuery, Error>
where
    F: Fn(usize) -> String,
{
    let mut rewritten = String::with_capacity(sql.len());
    let mut ordered_params = Vec::new();

    let bytes = sql.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    let mut param_index = 0;

    while i < len {
        let b = bytes[i];

        // String literal: '...'
        if b == b'\'' {
            rewritten.push('\'');
            i += 1;
            while i < len {
                let cur = bytes[i];
                rewritten.push(cur as char);
                if cur == b'\'' {
                    if i + 1 < len && bytes[i + 1] == b'\'' {
                        rewritten.push('\'');
                        i += 2;
                    } else {
                        i += 1;
                        break;
                    }
                } else {
                    i += 1;
                }
            }
            continue;
        }

        // Line comment: -- ...
        if b == b'-' && i + 1 < len && bytes[i + 1] == b'-' {
            while i < len && bytes[i] != b'\n' {
                rewritten.push(bytes[i] as char);
                i += 1;
            }
            continue;
        }

        // Block comment: /* ... */
        if b == b'/' && i + 1 < len && bytes[i + 1] == b'*' {
            rewritten.push_str("/*");
            i += 2;
            while i + 1 < len && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                rewritten.push(bytes[i] as char);
                i += 1;
            }
            if i + 1 < len {
                rewritten.push_str("*/");
                i += 2;
            }
            continue;
        }

        // Named parameter: :identifier (ignore :: casts)
        if b == b':' {
            if i + 1 < len && bytes[i + 1] == b':' {
                rewritten.push_str("::");
                i += 2;
                continue;
            }

            if i + 1 < len && (bytes[i + 1].is_ascii_alphabetic() || bytes[i + 1] == b'_') {
                i += 1;
                let start = i;
                while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                let param_name = &sql[start..i];
                let val = params.get(param_name).ok_or_else(|| {
                    Error::query_error(format!("missing named parameter ':{param_name}'"))
                })?;

                param_index += 1;
                let placeholder = format_placeholder(param_index);
                rewritten.push_str(&placeholder);
                ordered_params.push(val.clone());
                continue;
            }
        }

        rewritten.push(b as char);
        i += 1;
    }

    Ok(PreparedQuery {
        sql: rewritten,
        params: ordered_params,
    })
}
