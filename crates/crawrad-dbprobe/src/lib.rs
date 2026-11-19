//! Database schema probe for discovering table columns out-of-process.

#![forbid(unsafe_code)]

use crawrad_note::note;

#[note("sample")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sample {
    /// The columns, in the table's order.
    pub columns: Vec<String>,
    /// The declared data types for each column ("INTEGER", "REAL", etc.).
    pub kinds: Vec<String>,
    /// The first rows, each cell already turned into the text the grid shows (None for NULL).
    pub rows: Vec<Vec<Option<String>>>,
    /// How many rows the whole table has.
    pub total: u64,
}

#[note("max-sample-rows")]
pub const MAX_SAMPLE_ROWS: usize = 50;

#[note("columns")]
pub fn columns(url: &str, table: &str) -> Result<Vec<String>, String> {
    check_target(url, table)?;
    let options = portadb::ConnectOptions::new(url);
    block_on(async {
        let engine = portadb::Engine::connect(&options)
            .await
            .map_err(|e| e.to_string())?;
        let schema = portadb::SchemaInspector::inspect_table_schema(&engine, table)
            .await
            .map_err(|e| e.to_string())?;
        if schema.columns().is_empty() {
            return Err(format!("no table named '{table}'"));
        }
        let cols = schema.columns().iter().map(|c| c.name.clone()).collect();
        Ok(cols)
    })
}

#[note("sample-rows")]
pub fn sample(url: &str, table: &str, limit: usize) -> Result<Sample, String> {
    check_target(url, table)?;
    let limit = limit.clamp(1, MAX_SAMPLE_ROWS);
    let options = portadb::ConnectOptions::new(url);
    block_on(async {
        let engine = portadb::Engine::connect(&options)
            .await
            .map_err(|e| e.to_string())?;
        let schema = portadb::SchemaInspector::inspect_table_schema(&engine, table)
            .await
            .map_err(|e| e.to_string())?;
        if schema.columns().is_empty() {
            return Err(format!("no table named '{table}'"));
        }
        let columns: Vec<String> = schema.columns().iter().map(|c| c.name.clone()).collect();
        let kinds: Vec<String> = schema
            .columns()
            .iter()
            .map(|c| kind_name(c.data_type).to_string())
            .collect();
        let quoted = engine.dialect().quote_identifier(table);
        let query_rows = engine
            .query(
                &format!("SELECT * FROM {quoted} LIMIT {limit}"),
                &portadb::params![],
            )
            .await
            .map_err(|e| e.to_string())?;
        let rows = query_rows
            .iter()
            .map(|row| row.values().iter().map(cell_text).collect())
            .collect();
        let count_rows = engine
            .query(
                &format!("SELECT COUNT(*) AS n FROM {quoted}"),
                &portadb::params![],
            )
            .await
            .map_err(|e| e.to_string())?;
        let total = count_rows
            .first()
            .and_then(|row| row.values().first())
            .and_then(|val| match val {
                portadb::Value::Integer(n) => u64::try_from(*n).ok(),
                _ => None,
            })
            .unwrap_or(0);
        Ok(Sample {
            columns,
            kinds,
            rows,
            total,
        })
    })
}

/// The first rows of a query (a Dataset's `sql`, Grid on Dataset).
///
/// The query is wrapped as a subquery, so the probe only ever reads: the
/// limit and the count are put around it, not spliced into it.
#[note("sample-sql")]
pub fn sample_sql(url: &str, sql: &str, limit: usize) -> Result<Sample, String> {
    let sql = sql.trim().trim_end_matches(';').trim();
    check_target(url, sql).map_err(|e| e.replace("table name", "SQL"))?;
    let limit = limit.clamp(1, MAX_SAMPLE_ROWS);
    let options = portadb::ConnectOptions::new(url);
    block_on(async {
        let engine = portadb::Engine::connect(&options)
            .await
            .map_err(|e| e.to_string())?;
        let query_rows = engine
            .query(
                &format!("SELECT * FROM ({sql}) LIMIT {limit}"),
                &portadb::params![],
            )
            .await
            .map_err(|e| e.to_string())?;
        let columns: Vec<String> = query_rows
            .first()
            .map(|row| row.columns().to_vec())
            .unwrap_or_default();
        // A query has no declared types: the first row's values say them.
        let kinds: Vec<String> = query_rows
            .first()
            .map(|row| {
                row.values()
                    .iter()
                    .map(|v| value_kind(v).to_string())
                    .collect()
            })
            .unwrap_or_default();
        let rows = query_rows
            .iter()
            .map(|row| row.values().iter().map(cell_text).collect())
            .collect();
        let count_rows = engine
            .query(
                &format!("SELECT COUNT(*) AS n FROM ({sql})"),
                &portadb::params![],
            )
            .await
            .map_err(|e| e.to_string())?;
        let total = count_rows
            .first()
            .and_then(|row| row.values().first())
            .and_then(|val| match val {
                portadb::Value::Integer(n) => u64::try_from(*n).ok(),
                _ => None,
            })
            .unwrap_or(0);
        Ok(Sample {
            columns,
            kinds,
            rows,
            total,
        })
    })
}

fn value_kind(v: &portadb::Value) -> &'static str {
    match v {
        portadb::Value::Integer(_) => "INTEGER",
        portadb::Value::Real(_) => "REAL",
        portadb::Value::Blob(_) => "BLOB",
        portadb::Value::Boolean(_) => "BOOLEAN",
        portadb::Value::Text(_) | portadb::Value::Null => "TEXT",
    }
}

#[note("kind-name")]
pub fn kind_name(kind: portadb::DataType) -> &'static str {
    match kind {
        portadb::DataType::Integer => "INTEGER",
        portadb::DataType::Real => "REAL",
        portadb::DataType::Text => "TEXT",
        portadb::DataType::Blob => "BLOB",
        portadb::DataType::Boolean => "BOOLEAN",
    }
}

#[note("answer")]
pub fn answer(args: &[String]) -> (i32, String) {
    let mut url: Option<String> = None;
    let mut table: Option<String> = None;
    let mut sql: Option<String> = None;
    let mut rows: Option<usize> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--url" {
            if i + 1 < args.len() {
                url = Some(args[i + 1].clone());
                i += 2;
                continue;
            }
        } else if let Some(val) = args[i].strip_prefix("--url=") {
            url = Some(val.to_string());
            i += 1;
            continue;
        } else if args[i] == "--table" {
            if i + 1 < args.len() {
                table = Some(args[i + 1].clone());
                i += 2;
                continue;
            }
        } else if let Some(val) = args[i].strip_prefix("--table=") {
            table = Some(val.to_string());
            i += 1;
            continue;
        } else if args[i] == "--sql" {
            if i + 1 < args.len() {
                sql = Some(args[i + 1].clone());
                i += 2;
                continue;
            }
        } else if args[i] == "--rows" {
            if i + 1 < args.len() {
                match args[i + 1].parse::<usize>() {
                    Ok(n) => {
                        rows = Some(n);
                        i += 2;
                        continue;
                    }
                    Err(_) => {
                        let err_json =
                            serde_json::json!({ "error": "--rows needs a number" }).to_string();
                        return (1, err_json);
                    }
                }
            } else {
                let err_json = serde_json::json!({ "error": "--rows needs a number" }).to_string();
                return (1, err_json);
            }
        } else if let Some(val) = args[i].strip_prefix("--rows=") {
            match val.parse::<usize>() {
                Ok(n) => {
                    rows = Some(n);
                    i += 1;
                    continue;
                }
                Err(_) => {
                    let err_json =
                        serde_json::json!({ "error": "--rows needs a number" }).to_string();
                    return (1, err_json);
                }
            }
        }
        i += 1;
    }

    let Some(url) = url else {
        let err_json = serde_json::json!({ "error": "missing --url parameter" }).to_string();
        return (1, err_json);
    };
    if let Some(sql) = sql {
        let answer = sample_sql(&url, &sql, rows.unwrap_or(MAX_SAMPLE_ROWS));
        return match answer {
            Ok(s) => (
                0,
                serde_json::json!({
                    "columns": s.columns,
                    "kinds": s.kinds,
                    "rows": s.rows,
                    "total": s.total,
                })
                .to_string(),
            ),
            Err(err) => (1, serde_json::json!({ "error": err }).to_string()),
        };
    }
    let Some(table) = table else {
        let err_json = serde_json::json!({ "error": "missing --table parameter" }).to_string();
        return (1, err_json);
    };

    if let Some(limit) = rows {
        match sample(&url, &table, limit) {
            Ok(s) => {
                let json = serde_json::json!({
                    "columns": s.columns,
                    "kinds": s.kinds,
                    "rows": s.rows,
                    "total": s.total,
                })
                .to_string();
                (0, json)
            }
            Err(err) => {
                let json = serde_json::json!({ "error": err }).to_string();
                (1, json)
            }
        }
    } else {
        match columns(&url, &table) {
            Ok(cols) => {
                let json = serde_json::json!({ "columns": cols }).to_string();
                (0, json)
            }
            Err(err) => {
                let json = serde_json::json!({ "error": err }).to_string();
                (1, json)
            }
        }
    }
}

fn check_target(url: &str, table: &str) -> Result<(), String> {
    if url.trim().is_empty() {
        return Err("URL cannot be empty".to_string());
    }
    if table.trim().is_empty() {
        return Err("table name cannot be empty".to_string());
    }
    // Opening a SQLite file that is not there CREATES it: a probe run at design
    // time must never leave an empty database in the user's project.
    if let Some(path) = sqlite_file(url).filter(|p| !std::path::Path::new(p).exists()) {
        return Err(format!("database file not found: {path}"));
    }
    Ok(())
}

fn cell_text(v: &portadb::Value) -> Option<String> {
    match v {
        portadb::Value::Null => None,
        portadb::Value::Integer(i) => Some(i.to_string()),
        portadb::Value::Real(r) => Some(format!("{r}")),
        portadb::Value::Boolean(b) => Some(b.to_string()),
        portadb::Value::Blob(b) => Some(format!("({} bytes)", b.len())),
        portadb::Value::Text(s) => {
            if s.chars().count() > 200 {
                let mut out: String = s.chars().take(200).collect();
                out.push('…');
                Some(out)
            } else {
                Some(s.clone())
            }
        }
    }
}

#[note("sqlite-file")]
fn sqlite_file(url: &str) -> Option<&str> {
    let url = url.trim();
    let path = url
        .strip_prefix("sqlite://")
        .or_else(|| url.strip_prefix("sqlite:"))?;
    (!path.is_empty() && path != ":memory:" && !path.starts_with("file:")).then_some(path)
}

fn block_on<T, F: std::future::Future<Output = Result<T, String>>>(future: F) -> Result<T, String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("Failed to create tokio runtime: {e}"))?
        .block_on(future)
}

#[cfg(test)]
#[path = "test/columns/test.rs"]
mod tests;
