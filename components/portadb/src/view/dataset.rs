//! Relational dataset abstraction providing record editing lifecycle (Browse, Edit, Insert).
//!
//! A [`Dataset`] links a database table or query to UI components and visual data bindings,
//! providing transactional buffered modifications, typed validation, and cursor navigation.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::schema::{ColumnDef, DataType, TableSchema};
use super::table::TableView;
use crate::capability::Feature;
use crate::engine::Engine;
use crate::error::Error;
use crate::params::NamedParams;
use crate::row::Row;
use crate::value::Value;

/// Operational lifecycle state of a [`Dataset`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatasetState {
    /// Read-only navigation mode positioned on a record.
    Browse,
    /// An existing record is currently being edited in an in-memory buffer.
    Edit,
    /// A new record is currently being constructed in an in-memory buffer.
    Insert,
}

/// The origin of records for a [`Dataset`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatasetSource {
    /// Relational database table name supporting full CRUD operations.
    Table(String),
    /// Raw SQL query producing a read-only dataset.
    Sql(String),
}

/// Notification events emitted when dataset state or fields change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatasetEvent {
    /// Operational state transitioned from `previous` to `current`.
    StateChanged {
        /// Previous lifecycle state.
        previous: DatasetState,
        /// Current lifecycle state.
        current: DatasetState,
    },
    /// A buffered field value was updated.
    FieldChanged {
        /// Name of the modified column.
        name: String,
    },
}

type DatasetListener = Arc<dyn Fn(DatasetEvent) + Send + Sync + 'static>;

struct DatasetInner {
    engine: Engine,
    source: DatasetSource,
    state: DatasetState,
    view: TableView,
    edit_buffer: Option<HashMap<String, Value>>,
    original_row: Option<Row>,
    original_index: Option<usize>,
    listeners: Vec<DatasetListener>,
}

/// An observable, reactive dataset providing structured editing lifecycle and cursor sharing.
///
/// Wraps an underlying [`TableView`] and [`Engine`] to coordinate classic RAD dataset operations:
/// - Navigation (`first`, `next`, `prior`, `last`, `move_to`) validated against editing state.
/// - In-memory buffered editing ([`Dataset::edit`], [`Dataset::insert`], [`Dataset::set_field`], [`Dataset::cancel`]).
/// - SQL-backed persistence ([`Dataset::post`], [`Dataset::delete`], [`Dataset::refresh`]).
#[derive(Clone)]
pub struct Dataset {
    inner: Arc<Mutex<DatasetInner>>,
}

/// `SELECT *` of a table, ordered by its primary key when it has one — the one
/// query `open`, `refresh` and an `INSERT` without `RETURNING` re-read with.
fn select_table_sql(engine: &Engine, table: &str, schema: &TableSchema) -> String {
    let quoted_table = engine.dialect().quote_identifier(table);
    let pks = schema
        .primary_key()
        .iter()
        .map(|c| engine.dialect().quote_identifier(&c.name))
        .collect::<Vec<_>>()
        .join(", ");
    if pks.is_empty() {
        format!("SELECT * FROM {quoted_table}")
    } else {
        format!("SELECT * FROM {quoted_table} ORDER BY {pks}")
    }
}

impl Dataset {
    /// Opens a dataset from an [`Engine`] using either a table name or raw SQL query.
    ///
    /// When opened from a [`DatasetSource::Table`], the table schema is inspected and rows
    /// are ordered by primary key columns if present.
    /// When opened from a [`DatasetSource::Sql`], schema is inferred from query result types,
    /// and the dataset is marked read-only.
    pub async fn open(engine: &Engine, source: DatasetSource) -> Result<Self, Error> {
        let (schema, rows) = match &source {
            DatasetSource::Table(table_name) => {
                let schema = engine.inspect_schema(table_name).await?;
                let sql = select_table_sql(engine, table_name, &schema);
                let rows = engine.query(&sql, &NamedParams::new()).await?;
                (schema, rows)
            }
            DatasetSource::Sql(sql) => {
                let rows = engine.query(sql, &NamedParams::new()).await?;
                let mut schema = TableSchema::new();
                if let Some(first_row) = rows.first() {
                    for (col_idx, name) in first_row.columns().iter().enumerate() {
                        let mut inferred_type = DataType::Text;
                        for r in &rows {
                            if let Some(val) = r.values().get(col_idx) {
                                match val {
                                    Value::Integer(_) => {
                                        inferred_type = DataType::Integer;
                                        break;
                                    }
                                    Value::Real(_) => {
                                        inferred_type = DataType::Real;
                                        break;
                                    }
                                    Value::Text(_) => {
                                        inferred_type = DataType::Text;
                                        break;
                                    }
                                    Value::Blob(_) => {
                                        inferred_type = DataType::Blob;
                                        break;
                                    }
                                    Value::Boolean(_) => {
                                        inferred_type = DataType::Boolean;
                                        break;
                                    }
                                    Value::Null => {}
                                }
                            }
                        }
                        schema = schema.with_column(ColumnDef::new(name, inferred_type, true));
                    }
                }
                (schema, rows)
            }
        };

        let view = TableView::new(schema, rows);
        let inner = DatasetInner {
            engine: engine.clone(),
            source,
            state: DatasetState::Browse,
            view,
            edit_buffer: None,
            original_row: None,
            original_index: None,
            listeners: Vec::new(),
        };

        Ok(Self {
            inner: Arc::new(Mutex::new(inner)),
        })
    }

    /// Returns a cloned reference to the shared underlying [`TableView`].
    ///
    /// The view shares the exact same rows, version counter, and selection cursor.
    #[must_use]
    pub fn view(&self) -> TableView {
        self.inner.lock().expect("lock dataset").view.clone()
    }

    /// Returns the current lifecycle state of this dataset.
    #[must_use]
    pub fn state(&self) -> DatasetState {
        self.inner.lock().expect("lock dataset").state
    }

    /// Returns a copy of the active table schema.
    #[must_use]
    pub fn schema(&self) -> TableSchema {
        self.view().schema()
    }

    /// Returns the total number of rows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.view().len()
    }

    /// Returns true if the dataset has no rows.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.view().is_empty()
    }

    /// Returns the 0-based index of the currently selected row, or `None` if empty.
    #[must_use]
    pub fn position(&self) -> Option<usize> {
        self.view().position()
    }

    /// Returns a cloned snapshot of the row at `index`, or `None` if out of bounds.
    #[must_use]
    pub fn row(&self, index: usize) -> Option<Row> {
        self.view().row(index)
    }

    fn check_can_navigate(&self) -> Result<(), Error> {
        let guard = self.inner.lock().expect("lock dataset");
        if guard.state != DatasetState::Browse {
            return Err(Error::query_error(
                "cannot navigate while dataset is in edit or insert state; post or cancel first",
            ));
        }
        Ok(())
    }

    /// Advances the cursor to the next row.
    ///
    /// Refused with an error if the dataset is currently in `Edit` or `Insert` state.
    pub fn next(&self) -> Result<bool, Error> {
        self.check_can_navigate()?;
        Ok(self.view().next())
    }

    /// Moves the cursor to the preceding row.
    ///
    /// Refused with an error if the dataset is currently in `Edit` or `Insert` state.
    pub fn prior(&self) -> Result<bool, Error> {
        self.check_can_navigate()?;
        Ok(self.view().prev())
    }

    /// Moves the cursor to the first row.
    ///
    /// Refused with an error if the dataset is currently in `Edit` or `Insert` state.
    pub fn first(&self) -> Result<bool, Error> {
        self.check_can_navigate()?;
        Ok(self.view().first())
    }

    /// Moves the cursor to the last row.
    ///
    /// Refused with an error if the dataset is currently in `Edit` or `Insert` state.
    pub fn last(&self) -> Result<bool, Error> {
        self.check_can_navigate()?;
        Ok(self.view().last())
    }

    /// Moves the cursor to a specific row index.
    ///
    /// Refused with an error if the dataset is currently in `Edit` or `Insert` state.
    pub fn move_to(&self, index: usize) -> Result<bool, Error> {
        self.check_can_navigate()?;
        Ok(self.view().move_to(index))
    }

    /// Retrieves the current value of a column.
    ///
    /// Returns the value from the active edit buffer when in `Edit` or `Insert` state,
    /// or from the current row when in `Browse` state.
    /// Returns [`Error::ColumnNotFound`] if the column does not exist in the schema.
    pub fn field(&self, name: &str) -> Result<Value, Error> {
        let guard = self.inner.lock().expect("lock dataset");
        let schema = guard.view.schema();
        let col = schema
            .find_column(name)
            .ok_or_else(|| Error::ColumnNotFound {
                name: name.to_string(),
            })?;

        match guard.state {
            DatasetState::Edit | DatasetState::Insert => {
                if let Some(val) = guard.edit_buffer.as_ref().and_then(|b| b.get(&col.name)) {
                    return Ok(val.clone());
                }
                Err(Error::ColumnNotFound {
                    name: name.to_string(),
                })
            }
            DatasetState::Browse => {
                let row = guard
                    .view
                    .current_row()
                    .ok_or_else(|| Error::query_error("dataset has no selected row or is empty"))?;
                row.get_value(&col.name).cloned()
            }
        }
    }

    /// Enters `Edit` state, copying the current record into the mutable edit buffer.
    ///
    /// Refused for read-only SQL datasets, when not in `Browse` state, or when there is no current row.
    pub fn edit(&self) -> Result<(), Error> {
        let (event, listeners) = {
            let mut guard = self.inner.lock().expect("lock dataset");
            match &guard.source {
                DatasetSource::Sql(_) => {
                    return Err(Error::query_error("cannot edit read-only SQL dataset"));
                }
                DatasetSource::Table(_) => {}
            }
            if guard.state != DatasetState::Browse {
                return Err(Error::query_error("cannot edit unless in Browse state"));
            }
            let idx = guard
                .view
                .selected_index()
                .ok_or_else(|| Error::query_error("cannot edit: dataset has no current row"))?;
            let row = guard
                .view
                .selected_row()
                .ok_or_else(|| Error::query_error("cannot edit: dataset has no current row"))?;
            let schema = guard.view.schema();

            let mut buf = HashMap::new();
            for col in schema.columns() {
                let val = row.get_value(&col.name)?.clone();
                buf.insert(col.name.clone(), val);
            }

            guard.edit_buffer = Some(buf);
            guard.original_row = Some(row);
            guard.original_index = Some(idx);
            guard.state = DatasetState::Edit;

            let event = DatasetEvent::StateChanged {
                previous: DatasetState::Browse,
                current: DatasetState::Edit,
            };
            (event, guard.listeners.clone())
        };

        for listener in listeners {
            listener(event.clone());
        }

        Ok(())
    }

    /// Enters `Insert` state, initializing all columns in the edit buffer to [`Value::Null`].
    ///
    /// Refused for read-only SQL datasets or when not in `Browse` state.
    pub fn insert(&self) -> Result<(), Error> {
        let (event, listeners) = {
            let mut guard = self.inner.lock().expect("lock dataset");
            match &guard.source {
                DatasetSource::Sql(_) => {
                    return Err(Error::query_error(
                        "cannot insert into read-only SQL dataset",
                    ));
                }
                DatasetSource::Table(_) => {}
            }
            if guard.state != DatasetState::Browse {
                return Err(Error::query_error("cannot insert unless in Browse state"));
            }

            let schema = guard.view.schema();
            let mut buf = HashMap::new();
            for col in schema.columns() {
                buf.insert(col.name.clone(), Value::Null);
            }

            guard.edit_buffer = Some(buf);
            guard.original_row = None;
            guard.original_index = None;
            guard.state = DatasetState::Insert;

            let event = DatasetEvent::StateChanged {
                previous: DatasetState::Browse,
                current: DatasetState::Insert,
            };
            (event, guard.listeners.clone())
        };

        for listener in listeners {
            listener(event.clone());
        }

        Ok(())
    }

    /// Sets a field in the active edit buffer with strict schema type verification.
    ///
    /// Only permitted while in `Edit` or `Insert` state.
    /// Type compatibility rules:
    /// - [`DataType::Integer`]: accepts integer values.
    /// - [`DataType::Real`]: accepts float values and automatically widens integer values.
    /// - [`DataType::Text`]: accepts text values.
    /// - [`DataType::Blob`]: accepts byte blobs.
    /// - [`DataType::Boolean`]: accepts boolean flags.
    /// - [`Value::Null`]: permitted if the column definition has `nullable: true`, returning [`Error::NullValue`] otherwise.
    pub fn set_field(&self, name: &str, value: impl Into<Value>) -> Result<(), Error> {
        let raw_val = value.into();
        let (event, listeners) = {
            let mut guard = self.inner.lock().expect("lock dataset");
            if guard.state != DatasetState::Edit && guard.state != DatasetState::Insert {
                return Err(Error::query_error(
                    "cannot set field when dataset is not in Edit or Insert state",
                ));
            }

            let schema = guard.view.schema();
            let col = schema
                .find_column(name)
                .ok_or_else(|| Error::ColumnNotFound {
                    name: name.to_string(),
                })?
                .clone();

            let val = if raw_val.is_null() {
                if !col.nullable {
                    return Err(Error::NullValue {
                        column: col.name.clone(),
                    });
                }
                Value::Null
            } else {
                match col.data_type {
                    DataType::Integer => match raw_val {
                        Value::Integer(_) => raw_val,
                        _ => {
                            return Err(Error::TypeMismatch {
                                column: col.name.clone(),
                                expected: "Integer",
                                found: raw_val.type_name().to_string(),
                            });
                        }
                    },
                    DataType::Real => match raw_val {
                        Value::Real(_) => raw_val,
                        Value::Integer(i) => Value::Real(i as f64),
                        _ => {
                            return Err(Error::TypeMismatch {
                                column: col.name.clone(),
                                expected: "Real",
                                found: raw_val.type_name().to_string(),
                            });
                        }
                    },
                    DataType::Text => match raw_val {
                        Value::Text(_) => raw_val,
                        _ => {
                            return Err(Error::TypeMismatch {
                                column: col.name.clone(),
                                expected: "Text",
                                found: raw_val.type_name().to_string(),
                            });
                        }
                    },
                    DataType::Blob => match raw_val {
                        Value::Blob(_) => raw_val,
                        _ => {
                            return Err(Error::TypeMismatch {
                                column: col.name.clone(),
                                expected: "Blob",
                                found: raw_val.type_name().to_string(),
                            });
                        }
                    },
                    DataType::Boolean => match raw_val {
                        Value::Boolean(_) => raw_val,
                        _ => {
                            return Err(Error::TypeMismatch {
                                column: col.name.clone(),
                                expected: "Boolean",
                                found: raw_val.type_name().to_string(),
                            });
                        }
                    },
                }
            };

            if let Some(ref mut buf) = guard.edit_buffer {
                buf.insert(col.name.clone(), val);
            }

            let event = DatasetEvent::FieldChanged {
                name: col.name.clone(),
            };
            (event, guard.listeners.clone())
        };

        for listener in listeners {
            listener(event.clone());
        }

        Ok(())
    }

    /// Back to `Browse` with the buffer dropped, telling the listeners — the one
    /// exit of `Edit`/`Insert`, so `cancel` and every `post` leave the same way.
    fn back_to_browse(&self) {
        let (previous, listeners) = {
            let mut guard = self.inner.lock().expect("lock dataset");
            if guard.state == DatasetState::Browse {
                return;
            }
            let previous = guard.state;
            guard.state = DatasetState::Browse;
            guard.edit_buffer = None;
            guard.original_row = None;
            guard.original_index = None;
            (previous, guard.listeners.clone())
        };
        let event = DatasetEvent::StateChanged {
            previous,
            current: DatasetState::Browse,
        };
        for listener in listeners {
            listener(event.clone());
        }
    }

    /// Cancels buffered modifications, discarding changes and returning state to `Browse`.
    pub fn cancel(&self) {
        self.back_to_browse();
    }

    /// Persists buffered modifications to the database and synchronizes the underlying [`TableView`].
    ///
    /// - In `Edit` state: executes an `UPDATE` statement targeting modified columns only,
    ///   identifying the row by its original primary key values.
    /// - In `Insert` state: executes an `INSERT` statement containing non-null buffered columns.
    ///   Uses `RETURNING *` when supported by the engine to retrieve server-assigned values.
    ///
    /// Transitions state back to `Browse` upon success.
    pub async fn post(&self) -> Result<(), Error> {
        let (table_name, schema, state, buffer, original_row, original_index) = {
            let guard = self.inner.lock().expect("lock dataset");
            match guard.state {
                DatasetState::Browse => {
                    return Err(Error::query_error(
                        "cannot post: dataset is in Browse state",
                    ));
                }
                DatasetState::Edit => {
                    let table = match &guard.source {
                        DatasetSource::Table(t) => t.clone(),
                        DatasetSource::Sql(_) => {
                            return Err(Error::ConfigurationError(
                                "cannot post changes to read-only SQL dataset".to_string(),
                            ));
                        }
                    };
                    (
                        table,
                        guard.view.schema(),
                        DatasetState::Edit,
                        guard.edit_buffer.clone().unwrap_or_default(),
                        guard.original_row.clone(),
                        guard.original_index,
                    )
                }
                DatasetState::Insert => {
                    let table = match &guard.source {
                        DatasetSource::Table(t) => t.clone(),
                        DatasetSource::Sql(_) => {
                            return Err(Error::ConfigurationError(
                                "cannot post changes to read-only SQL dataset".to_string(),
                            ));
                        }
                    };
                    (
                        table,
                        guard.view.schema(),
                        DatasetState::Insert,
                        guard.edit_buffer.clone().unwrap_or_default(),
                        None,
                        None,
                    )
                }
            }
        };

        let pk_cols = schema.primary_key();
        if pk_cols.is_empty() {
            return Err(Error::ConfigurationError(format!(
                "table '{table_name}' has no primary key defined in schema; cannot post mutations"
            )));
        }

        let engine = {
            let guard = self.inner.lock().expect("lock dataset");
            guard.engine.clone()
        };
        let quoted_table = engine.dialect().quote_identifier(&table_name);

        match state {
            DatasetState::Edit => {
                let orig_row = original_row.ok_or_else(|| {
                    Error::query_error("missing original row for post in Edit state")
                })?;
                let orig_idx = original_index.ok_or_else(|| {
                    Error::query_error("missing original index for post in Edit state")
                })?;

                let mut changed_cols = Vec::new();
                for col in schema.columns() {
                    let buf_val = buffer.get(&col.name);
                    let orig_val = orig_row.get_value(&col.name).ok();
                    if buf_val != orig_val {
                        changed_cols.push(col.name.clone());
                    }
                }

                if changed_cols.is_empty() {
                    self.back_to_browse();
                    return Ok(());
                }

                let set_parts: Vec<String> = changed_cols
                    .iter()
                    .enumerate()
                    .map(|(i, col)| {
                        let quoted_col = engine.dialect().quote_identifier(col);
                        format!("{quoted_col} = :c_{i}")
                    })
                    .collect();
                let set_clause = set_parts.join(", ");

                let where_parts: Vec<String> = pk_cols
                    .iter()
                    .enumerate()
                    .map(|(i, pk)| {
                        let quoted_pk = engine.dialect().quote_identifier(&pk.name);
                        format!("{quoted_pk} = :k_{i}")
                    })
                    .collect();
                let where_clause = where_parts.join(" AND ");

                let sql = format!("UPDATE {quoted_table} SET {set_clause} WHERE {where_clause}");

                let mut params = NamedParams::new();
                for (i, col) in changed_cols.iter().enumerate() {
                    let val = buffer.get(col).cloned().unwrap_or(Value::Null);
                    params.insert(format!("c_{i}"), val);
                }
                for (i, pk) in pk_cols.iter().enumerate() {
                    let val = orig_row.get_value(&pk.name)?.clone();
                    params.insert(format!("k_{i}"), val);
                }

                let affected = engine.execute(&sql, &params).await?;
                if affected == 0 {
                    return Err(Error::QueryError(
                        "row was not found or was modified/deleted by another transaction"
                            .to_string(),
                    ));
                }

                let col_names: Vec<String> =
                    schema.columns().iter().map(|c| c.name.clone()).collect();
                let values: Vec<Value> = schema
                    .columns()
                    .iter()
                    .map(|c| buffer.get(&c.name).cloned().unwrap_or(Value::Null))
                    .collect();
                let updated_row = Row::new(col_names, values);

                self.view().set_row(orig_idx, updated_row);
                self.back_to_browse();
                Ok(())
            }
            DatasetState::Insert => {
                let mut insert_cols = Vec::new();
                let mut insert_values = Vec::new();
                for col in schema.columns() {
                    if let Some(val) = buffer.get(&col.name).filter(|v| !v.is_null()) {
                        insert_cols.push(col.name.clone());
                        insert_values.push(val.clone());
                    }
                }

                let supports_returning = engine.supports(Feature::Returning);

                let mut params = NamedParams::new();
                for (i, val) in insert_values.iter().enumerate() {
                    params.insert(format!("v_{i}"), val.clone());
                }

                if supports_returning {
                    let sql = if insert_cols.is_empty() {
                        format!("INSERT INTO {quoted_table} DEFAULT VALUES RETURNING *")
                    } else {
                        let cols_clause = insert_cols
                            .iter()
                            .map(|c| engine.dialect().quote_identifier(c))
                            .collect::<Vec<_>>()
                            .join(", ");
                        let vals_clause = (0..insert_cols.len())
                            .map(|i| format!(":v_{i}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!(
                            "INSERT INTO {quoted_table} ({cols_clause}) VALUES ({vals_clause}) RETURNING *"
                        )
                    };
                    let returning_rows = engine.query(&sql, &params).await?;
                    let new_row = returning_rows.into_iter().next().ok_or_else(|| {
                        Error::QueryError("insert failed to return row".to_string())
                    })?;

                    let insert_idx = self.view().len();
                    self.view().insert_row(insert_idx, new_row);
                } else {
                    // Without RETURNING support, database-generated primary keys cannot be retrieved directly;
                    // re-querying all table rows synchronizes the view with server-generated identity columns.
                    let sql = if insert_cols.is_empty() {
                        format!("INSERT INTO {quoted_table} DEFAULT VALUES")
                    } else {
                        let cols_clause = insert_cols
                            .iter()
                            .map(|c| engine.dialect().quote_identifier(c))
                            .collect::<Vec<_>>()
                            .join(", ");
                        let vals_clause = (0..insert_cols.len())
                            .map(|i| format!(":v_{i}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("INSERT INTO {quoted_table} ({cols_clause}) VALUES ({vals_clause})")
                    };
                    engine.execute(&sql, &params).await?;

                    let select_sql = select_table_sql(&engine, &table_name, &schema);
                    let rows = engine.query(&select_sql, &NamedParams::new()).await?;
                    self.view().replace_rows(rows);
                    self.view().last();
                }

                self.back_to_browse();
                Ok(())
            }
            DatasetState::Browse => unreachable!(),
        }
    }

    /// Deletes the currently selected row from the database and removes it from the [`TableView`].
    ///
    /// Cursor positioning: retains current selected index if still within bounds of the reduced rows,
    /// otherwise moves to the new last row, or `None` if the table became empty.
    /// Refused for read-only datasets, when not in `Browse` state, or when no primary key is declared.
    pub async fn delete(&self) -> Result<(), Error> {
        let (table_name, schema, pos, current_row) = {
            let guard = self.inner.lock().expect("lock dataset");
            if guard.state != DatasetState::Browse {
                return Err(Error::query_error(
                    "cannot delete while dataset is in edit or insert state",
                ));
            }
            let table = match &guard.source {
                DatasetSource::Table(t) => t.clone(),
                DatasetSource::Sql(_) => {
                    return Err(Error::ConfigurationError(
                        "cannot delete from read-only SQL dataset".to_string(),
                    ));
                }
            };
            let pos = guard
                .view
                .selected_index()
                .ok_or_else(|| Error::query_error("cannot delete: no row selected"))?;
            let row = guard
                .view
                .selected_row()
                .ok_or_else(|| Error::query_error("cannot delete: no row selected"))?;
            (table, guard.view.schema(), pos, row)
        };

        let pk_cols = schema.primary_key();
        if pk_cols.is_empty() {
            return Err(Error::ConfigurationError(format!(
                "table '{table_name}' has no primary key defined in schema; cannot delete without a primary key"
            )));
        }

        let engine = {
            let guard = self.inner.lock().expect("lock dataset");
            guard.engine.clone()
        };
        let quoted_table = engine.dialect().quote_identifier(&table_name);
        let where_parts: Vec<String> = pk_cols
            .iter()
            .enumerate()
            .map(|(i, pk)| {
                let quoted_pk = engine.dialect().quote_identifier(&pk.name);
                format!("{quoted_pk} = :k_{i}")
            })
            .collect();
        let where_clause = where_parts.join(" AND ");
        let sql = format!("DELETE FROM {quoted_table} WHERE {where_clause}");

        let mut params = NamedParams::new();
        for (i, pk) in pk_cols.iter().enumerate() {
            let val = current_row.get_value(&pk.name)?;
            params.insert(format!("k_{i}"), val.clone());
        }

        let affected = engine.execute(&sql, &params).await?;
        if affected == 0 {
            return Err(Error::QueryError(
                "row was not found or was deleted by another transaction".to_string(),
            ));
        }

        self.view().remove_row(pos);
        Ok(())
    }

    /// Re-queries the dataset source and replaces the rows in the underlying [`TableView`].
    ///
    /// Refused while the dataset is in `Edit` or `Insert` state.
    pub async fn refresh(&self) -> Result<(), Error> {
        let (source, engine) = {
            let guard = self.inner.lock().expect("lock dataset");
            if guard.state != DatasetState::Browse {
                return Err(Error::query_error(
                    "cannot refresh while dataset is in edit or insert state",
                ));
            }
            (guard.source.clone(), guard.engine.clone())
        };

        let rows = match &source {
            DatasetSource::Table(t) => {
                let schema = engine.inspect_schema(t).await?;
                let sql = select_table_sql(&engine, t, &schema);
                engine.query(&sql, &NamedParams::new()).await?
            }
            DatasetSource::Sql(sql) => engine.query(sql, &NamedParams::new()).await?,
        };

        self.view().replace_rows(rows);
        Ok(())
    }

    /// Subscribes a listener to dataset lifecycle and field mutation events.
    pub fn subscribe<F: Fn(DatasetEvent) + Send + Sync + 'static>(&self, callback: F) {
        let mut guard = self.inner.lock().expect("lock dataset");
        guard.listeners.push(Arc::new(callback));
    }
}
