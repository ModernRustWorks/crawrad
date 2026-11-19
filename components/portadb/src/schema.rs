use crate::database::Database;
use crate::engine::Engine;
use crate::error::Error;
use crate::params;
use crate::view::{ColumnDef, DataType, TableSchema};

/// Introspects database schema to discover available tables and columns dynamically.
pub struct SchemaInspector;

impl SchemaInspector {
    /// Discovers all user table names in the target database.
    pub async fn inspect_tables(engine: &Engine) -> Result<Vec<String>, Error> {
        match engine.database() {
            Database::Sqlite => {
                let rows = engine
                    .query(
                        "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE '_portadb_%' ORDER BY name",
                        &params![],
                    )
                    .await?;
                let mut tables = Vec::new();
                for r in rows {
                    if let Ok(name) = r.get::<String>("name") {
                        tables.push(name);
                    }
                }
                Ok(tables)
            }
            Database::Postgres => {
                let rows = engine
                    .query(
                        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_name NOT LIKE '_portadb_%' ORDER BY table_name",
                        &params![],
                    )
                    .await?;
                let mut tables = Vec::new();
                for r in rows {
                    if let Ok(name) = r.get::<String>("table_name") {
                        tables.push(name);
                    }
                }
                Ok(tables)
            }
            Database::Redis => {
                // In Redis, abstract keys as a single 'keys' or 'kv' collection
                Ok(vec!["kv".to_string()])
            }
            _ => {
                // Fallback generic inspection
                let rows = engine
                    .query(
                        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'",
                        &params![],
                    )
                    .await?;
                let mut tables = Vec::new();
                for r in rows {
                    if let Ok(name) = r.get::<String>("table_name") {
                        tables.push(name);
                    }
                }
                Ok(tables)
            }
        }
    }

    /// Discovers column definitions and types for a specific table, returning a [`TableSchema`].
    pub async fn inspect_table_schema(engine: &Engine, table: &str) -> Result<TableSchema, Error> {
        match engine.database() {
            Database::Sqlite => {
                let sql = format!("PRAGMA table_info(\"{table}\")");
                let rows = engine.query(&sql, &params![]).await?;
                let mut schema = TableSchema::new();
                for r in rows {
                    let name: String = r.get("name")?;
                    let type_str: String = r.get::<String>("type").unwrap_or_default();
                    let notnull: i64 = r.get::<i64>("notnull").unwrap_or(0);
                    let nullable = notnull == 0;
                    let pk: i64 = r.get::<i64>("pk").unwrap_or(0);

                    let data_type = map_sqlite_type(&type_str);
                    let mut col = ColumnDef::new(name, data_type, nullable);
                    if pk > 0 {
                        col = col.primary_key();
                    }
                    schema = schema.with_column(col);
                }
                Ok(schema)
            }
            Database::Postgres => {
                // Primary keys for non-SQLite databases leave `primary_key` as false (no guessing).
                let sql = "SELECT column_name, data_type, is_nullable FROM information_schema.columns WHERE table_name = :table ORDER BY ordinal_position";
                let rows = engine.query(sql, &params![("table", table)]).await?;
                let mut schema = TableSchema::new();
                for r in rows {
                    let name: String = r.get("column_name")?;
                    let type_str: String = r.get::<String>("data_type").unwrap_or_default();
                    let is_null_str: String = r.get::<String>("is_nullable").unwrap_or_default();
                    let nullable = is_null_str.eq_ignore_ascii_case("YES");

                    let data_type = map_postgres_type(&type_str);
                    schema = schema.with_column(ColumnDef::new(name, data_type, nullable));
                }
                Ok(schema)
            }
            Database::Redis => {
                // Key-value default schema (no primary key guessing).
                let schema = TableSchema::new()
                    .with_column(ColumnDef::new("key", DataType::Text, false))
                    .with_column(ColumnDef::new("value", DataType::Text, true));
                Ok(schema)
            }
            _ => {
                // Fallback generic inspection leaves `primary_key` as false (no guessing).
                Ok(TableSchema::new())
            }
        }
    }
}

fn map_sqlite_type(t: &str) -> DataType {
    let s = t.to_ascii_uppercase();
    if s.contains("INT") {
        DataType::Integer
    } else if s.contains("CHAR") || s.contains("TEXT") || s.contains("CLOB") {
        DataType::Text
    } else if s.contains("REAL") || s.contains("FLOA") || s.contains("DOUB") {
        DataType::Real
    } else if s.contains("BLOB") {
        DataType::Blob
    } else if s.contains("BOOL") {
        DataType::Boolean
    } else {
        DataType::Text
    }
}

fn map_postgres_type(t: &str) -> DataType {
    let s = t.to_ascii_uppercase();
    if s.contains("INT") || s.contains("SERIAL") {
        DataType::Integer
    } else if s.contains("CHAR") || s.contains("TEXT") {
        DataType::Text
    } else if s.contains("DOUBLE") || s.contains("NUMERIC") || s.contains("REAL") {
        DataType::Real
    } else if s.contains("BYTEA") {
        DataType::Blob
    } else if s.contains("BOOL") {
        DataType::Boolean
    } else {
        DataType::Text
    }
}
