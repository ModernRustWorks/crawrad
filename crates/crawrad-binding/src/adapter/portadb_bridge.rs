//! Direct bridge connecting `portadb::DataSource` (TableView) and `portadb::Dataset`
//! to the CrawRAD binding engine.

use std::collections::BTreeMap;
use std::sync::Arc;

use portadb::view::event::TableEvent;
use portadb::{DataSource, DataType, Dataset, Row, TableSchema, Value};

use crate::adapter::dataset::{DataRecord, DatasetAdapter, DatasetState};
use crate::adapter::store::RecordStore;
use crawrad_note::note;

#[note("adapter-portadb-bridge-portadbbridge")]
pub struct PortaDbBridge;

impl PortaDbBridge {
    #[note("adapter-portadb-bridge-portadbbridge-from-dataset")]
    pub fn from_dataset(dataset: &Dataset) -> DatasetAdapter {
        DatasetAdapter::with_store(Arc::new(dataset.clone()))
    }

    #[note("adapter-portadb-bridge-portadbbridge-from-datasource")]
    pub fn from_datasource(source: &DataSource) -> DatasetAdapter {
        let schema = source.schema();
        let column_names: Vec<String> = schema.columns().iter().map(|c| c.name.clone()).collect();
        let schema_strs: Vec<&str> = column_names.iter().map(|s| s.as_str()).collect();

        // Convert current rows to DataRecords
        let mut records = Vec::new();
        let count = source.row_count();
        let initial_pos = source.position();

        for i in 0..count {
            source.select(i);
            if let Some(row) = source.current_row() {
                records.push(row_to_record(&schema, &row));
            }
        }
        // Restore initial position
        if let Some(pos) = initial_pos {
            source.select(pos);
        }

        let adapter = DatasetAdapter::new(&schema_strs, records);

        // Sync cursor position from DataSource -> DatasetAdapter
        let adapter_clone = adapter.clone();
        source.subscribe(move |signal| match signal.kind {
            TableEvent::SelectionChanged { current, .. } => {
                adapter_clone.collection().set_cursor(current);
            }
            TableEvent::RowsChanged => {
                // Table rows reloaded
            }
            TableEvent::RowChanged { .. }
            | TableEvent::RowInserted { .. }
            | TableEvent::RowRemoved { .. } => {
                // Row-level mutations are tracked by the table view
            }
        });

        // Sync cursor position from DatasetAdapter -> DataSource
        let source_clone = source.clone();
        adapter.collection().on_cursor_changed(move |new_cursor| {
            if let Some(idx) = new_cursor
                && source_clone.position() != Some(idx)
            {
                source_clone.select(idx);
            }
        });

        adapter
    }

    #[note("adapter-portadb-bridge-portadbbridge-open-table")]
    pub fn open_table(url: &str, table: &str) -> Result<DatasetAdapter, String> {
        if url.trim().is_empty() {
            return Err("URL cannot be empty".to_string());
        }
        if table.trim().is_empty() {
            return Err("table name cannot be empty".to_string());
        }
        // Opening a SQLite file that is not there CREATES it, empty: say so instead.
        let trimmed = url.trim();
        if let Some(path) = trimmed
            .strip_prefix("sqlite://")
            .or_else(|| trimmed.strip_prefix("sqlite:"))
            .filter(|p| !p.is_empty() && *p != ":memory:" && !p.starts_with("file:"))
            .filter(|p| !std::path::Path::new(p).is_file())
        {
            let full = std::path::absolute(path).unwrap_or_else(|_| path.into());
            return Err(format!("database file not found: {}", full.display()));
        }
        let options = portadb::ConnectOptions::new(url);
        block_on(async {
            let engine = portadb::Engine::connect(&options).await?;
            let ds =
                portadb::Dataset::open(&engine, portadb::DatasetSource::Table(table.to_string()))
                    .await?;
            Ok(Self::from_dataset(&ds))
        })
    }
}

#[note("adapter-portadb-bridge-value-to-text")]
pub fn value_to_text(val: &Value) -> String {
    match val {
        Value::Null => String::new(),
        Value::Integer(i) => i.to_string(),
        Value::Real(r) => r.to_string(),
        Value::Text(t) => t.clone(),
        Value::Boolean(b) => b.to_string(),
        Value::Blob(_) => "<blob>".to_string(),
    }
}

#[note("adapter-portadb-bridge-row-to-record")]
pub fn row_to_record(schema: &TableSchema, row: &Row) -> DataRecord {
    let mut record = BTreeMap::new();
    for col in schema.columns() {
        let name = &col.name;
        let val_str = if let Ok(val) = row.get_value(name) {
            value_to_text(val)
        } else {
            String::new()
        };
        record.insert(name.clone(), val_str);
    }
    record
}

impl RecordStore for Dataset {
    fn columns(&self) -> Vec<String> {
        self.schema()
            .columns()
            .iter()
            .map(|c| c.name.clone())
            .collect()
    }

    fn state(&self) -> DatasetState {
        match Dataset::state(self) {
            // An open dataset with zero rows remains in Browse state: it is open and can accept insertions.
            portadb::DatasetState::Browse => DatasetState::Browse,
            portadb::DatasetState::Edit => DatasetState::Edit,
            portadb::DatasetState::Insert => DatasetState::Insert,
        }
    }

    fn len(&self) -> usize {
        Dataset::len(self)
    }

    fn position(&self) -> Option<usize> {
        Dataset::position(self)
    }

    fn move_to(&self, index: usize) -> Result<(), String> {
        Dataset::move_to(self, index)
            .map_err(|e| e.to_string())
            .and_then(|ok| {
                if ok {
                    Ok(())
                } else {
                    Err(format!("Index {index} out of bounds"))
                }
            })
    }

    fn record(&self, index: usize) -> Option<DataRecord> {
        let schema = self.schema();
        self.view().row(index).map(|r| row_to_record(&schema, &r))
    }

    fn field_text(&self, name: &str) -> Option<String> {
        self.field(name).ok().map(|val| value_to_text(&val))
    }

    fn edit(&self) -> Result<(), String> {
        Dataset::edit(self).map_err(|e| e.to_string())
    }

    fn insert(&self) -> Result<(), String> {
        Dataset::insert(self).map_err(|e| e.to_string())
    }

    fn set_field_text(&self, name: &str, text: &str) -> Result<(), String> {
        let schema = self.schema();
        let col = schema
            .find_column(name)
            .ok_or_else(|| format!("Column '{name}' not found in dataset schema"))?;

        let value = if text.is_empty() {
            Value::Null
        } else {
            match col.data_type {
                DataType::Integer => {
                    // SQLite has no boolean type: a CheckBox bound to an INTEGER column
                    // writes `true`/`false`, which the column stores as 1/0 (as SQLite's own
                    // TRUE/FALSE keywords do).
                    let parsed = match text.to_ascii_lowercase().as_str() {
                        "true" => 1,
                        "false" => 0,
                        _ => text.parse::<i64>().map_err(|_| {
                            format!("Cannot parse '{text}' as Integer for column '{name}'")
                        })?,
                    };
                    Value::Integer(parsed)
                }
                DataType::Real => {
                    let parsed = text.parse::<f64>().map_err(|_| {
                        format!("Cannot parse '{text}' as Real for column '{name}'")
                    })?;
                    Value::Real(parsed)
                }
                DataType::Boolean => {
                    let lower = text.to_ascii_lowercase();
                    match lower.as_str() {
                        "true" | "1" => Value::Boolean(true),
                        "false" | "0" => Value::Boolean(false),
                        _ => {
                            return Err(format!(
                                "Cannot parse '{text}' as Boolean for column '{name}'"
                            ));
                        }
                    }
                }
                DataType::Text => Value::Text(text.to_string()),
                DataType::Blob => {
                    return Err(format!(
                        "Setting Blob fields via text is not supported for column '{name}'"
                    ));
                }
            }
        };

        self.set_field(name, value).map_err(|e| e.to_string())
    }

    fn post(&self) -> Result<(), String> {
        block_on(Dataset::post(self))
    }

    fn cancel(&self) -> Result<(), String> {
        Dataset::cancel(self);
        Ok(())
    }

    fn delete(&self) -> Result<(), String> {
        block_on(Dataset::delete(self))
    }

    fn refresh(&self) -> Result<(), String> {
        block_on(Dataset::refresh(self))
    }

    fn on_change(&self, callback: Box<dyn Fn() + Send + Sync>) {
        let cb = Arc::new(callback);
        let cb_ds = Arc::clone(&cb);
        self.subscribe(move |_event| {
            cb_ds();
        });
        let cb_view = Arc::clone(&cb);
        self.view().subscribe(move |_signal| {
            cb_view();
        });
    }
}

#[note("adapter-portadb-bridge-block-on")]
fn block_on<T, F: std::future::Future<Output = Result<T, portadb::Error>>>(
    future: F,
) -> Result<T, String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("Failed to create tokio runtime: {e}"))?
        .block_on(future)
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use portadb::{ColumnDef, DataType, TableSchema};

    #[test]
    fn bridge_from_datasource() {
        let schema = TableSchema::new()
            .with_column(ColumnDef::new("id", DataType::Integer, false))
            .with_column(ColumnDef::new("title", DataType::Text, false));

        let cols = vec!["id".to_string(), "title".to_string()];
        let row1 = Row::new(
            cols.clone(),
            vec![Value::Integer(1), Value::Text("CrawRAD Book".to_string())],
        );
        let row2 = Row::new(
            cols,
            vec![
                Value::Integer(2),
                Value::Text("Modern RAD Guide".to_string()),
            ],
        );

        let datasource = DataSource::new(schema, vec![row1, row2]);
        let adapter = PortaDbBridge::from_datasource(&datasource);

        assert_eq!(adapter.collection().len(), 2);
        assert_eq!(adapter.field("title").unwrap().get(), "CrawRAD Book");

        // Moving datasource moves adapter
        datasource.select(1);
        assert_eq!(adapter.collection().cursor(), Some(1));
        assert_eq!(adapter.field("title").unwrap().get(), "Modern RAD Guide");
    }

    #[cfg(feature = "rusqlite")]
    #[test]
    fn refresh_brings_row_inserted_by_another_connection() {
        let temp_dir = std::env::temp_dir();
        let db_file = temp_dir.join(format!(
            "crawrad_refresh_test_{}_{}.db",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let db_url = format!("sqlite://{}", db_file.display());
        let options = portadb::ConnectOptions::new(&db_url);

        // Connection 1 sets up schema and initial row:
        block_on(async {
            let engine = portadb::Engine::connect(&options).await?;
            engine
                .execute(
                    "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",
                    &portadb::NamedParams::new(),
                )
                .await?;
            let mut p = portadb::NamedParams::new();
            p.insert("id", 1i64);
            p.insert("name", "Alice");
            engine
                .execute("INSERT INTO users (id, name) VALUES (:id, :name);", &p)
                .await?;
            Ok(())
        })
        .unwrap();

        // Dataset is opened via adapter:
        let adapter = PortaDbBridge::open_table(&db_url, "users").unwrap();
        assert_eq!(adapter.collection().len(), 1);
        assert_eq!(adapter.field("name").unwrap().get(), "Alice");

        // Another connection inserts a row:
        block_on(async {
            let engine2 = portadb::Engine::connect(&options).await?;
            let mut p = portadb::NamedParams::new();
            p.insert("id", 2i64);
            p.insert("name", "Bob");
            engine2
                .execute("INSERT INTO users (id, name) VALUES (:id, :name);", &p)
                .await?;
            Ok(())
        })
        .unwrap();

        // Before refresh, adapter still sees 1 row:
        assert_eq!(adapter.collection().len(), 1);

        // After refresh, the new row appears:
        adapter.refresh().unwrap();
        assert_eq!(adapter.collection().len(), 2);

        let _ = std::fs::remove_file(db_file);
    }

    /// A Grid picks a row (the cursor), an Edit bound to the dataset writes its
    /// field, Post: the change is in the .db file, read back by a new connection.
    #[cfg(feature = "rusqlite")]
    #[test]
    fn edit_through_the_cursor_then_post_persists_to_the_file() {
        let db_file = std::env::temp_dir().join(format!(
            "crawrad_post_test_{}_{}.db",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let db_url = format!("sqlite://{}", db_file.display());
        let options = portadb::ConnectOptions::new(&db_url);
        block_on(async {
            let engine = portadb::Engine::connect(&options).await?;
            for sql in [
                "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",
                "INSERT INTO users (id, name) VALUES (1, 'Alice');",
                "INSERT INTO users (id, name) VALUES (2, 'Bob');",
            ] {
                engine.execute(sql, &portadb::NamedParams::new()).await?;
            }
            Ok(())
        })
        .unwrap();

        let adapter = PortaDbBridge::open_table(&db_url, "users").unwrap();
        assert_eq!(adapter.collection().len(), 2);
        // what the generated grid does when the user picks the second row
        adapter.collection().set_cursor(Some(1));
        assert_eq!(adapter.field_or_create("name").get(), "Bob");
        // what the generated Edit write-back does: the field, auto-edit
        adapter.field_or_create("name").set("Carol".to_string());
        assert_eq!(adapter.state().get(), DatasetState::Edit);
        adapter.post().unwrap();
        assert_eq!(adapter.state().get(), DatasetState::Browse);
        drop(adapter);

        let reread = PortaDbBridge::open_table(&db_url, "users").unwrap();
        reread.collection().set_cursor(Some(1));
        assert_eq!(reread.field_or_create("name").get(), "Carol");
        reread.collection().set_cursor(Some(0));
        assert_eq!(reread.field_or_create("name").get(), "Alice");
        drop(reread);
        let _ = std::fs::remove_file(db_file);
    }
}
