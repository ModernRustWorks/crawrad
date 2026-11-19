use std::sync::{Arc, Mutex};

use super::event::{TableEvent, TableSignal};
use super::schema::TableSchema;
use crate::error::Error;
use crate::executor::Executor;
use crate::params::NamedParams;
use crate::row::Row;
use crate::value::FromValue;

type Listener = Arc<dyn Fn(TableSignal) + Send + Sync + 'static>;

struct TableStateInner {
    schema: TableSchema,
    rows: Vec<Row>,
    selected_index: Option<usize>,
    version: u64,
    listeners: Vec<Listener>,
}

/// An observable, reactive tabular data view linking database query results
/// directly to UI controls (e.g. data tables, grids, and detail forms).
///
/// Designed with pure Rust idioms:
/// - **Zero Shared Mutable Buffers**: Rows are immutable value snapshots.
/// - **Pull-friendly**: Exposes a monotonic [`TableView::version`] counter for immediate-mode
///   rendering loops (such as Makepad or egui).
/// - **Push-friendly**: Supports reactive event subscription via [`TableView::subscribe`].
/// - **Shared Cursor**: Controls linked to the same `TableView` share selection position automatically.
#[derive(Clone)]
pub struct TableView {
    state: Arc<Mutex<TableStateInner>>,
}

/// Backward-compatible alias for [`TableView`].
pub type DataSource = TableView;

/// Backward-compatible alias for [`TableView`].
pub type TableState = TableView;

/// Backward-compatible alias for [`TableView`].
pub type DataCursor = TableView;

impl TableView {
    /// Constructs a new observable `TableView` with schema and initial rows.
    ///
    /// If rows are non-empty, the selection is automatically set to row 0.
    #[must_use]
    pub fn new(schema: TableSchema, rows: Vec<Row>) -> Self {
        let selected_index = if rows.is_empty() { None } else { Some(0) };
        Self {
            state: Arc::new(Mutex::new(TableStateInner {
                schema,
                rows,
                selected_index,
                version: 1,
                listeners: Vec::new(),
            })),
        }
    }

    /// Fetches rows from any database [`Executor`] and initializes a new `TableView`.
    pub async fn fetch(
        exec: &mut impl Executor,
        sql: &str,
        params: &NamedParams,
        schema: TableSchema,
    ) -> Result<Self, Error> {
        let rows = exec.query(sql, params).await?;
        Ok(Self::new(schema, rows))
    }

    /// Returns a copy of the schema descriptor.
    #[must_use]
    pub fn schema(&self) -> TableSchema {
        self.state.lock().expect("lock table state").schema.clone()
    }

    /// Returns the total number of rows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.state.lock().expect("lock table state").rows.len()
    }

    /// Returns the total number of rows (alias).
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.len()
    }

    /// Returns true if there are no rows.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the 0-based index of the currently selected row, or `None` if empty.
    #[must_use]
    pub fn selected_index(&self) -> Option<usize> {
        self.state.lock().expect("lock table state").selected_index
    }

    /// Returns the 0-based index of the currently selected row (alias).
    #[must_use]
    pub fn position(&self) -> Option<usize> {
        self.selected_index()
    }

    /// Returns the monotonic version counter, incremented whenever rows or selection change.
    ///
    /// Ideal for immediate-mode GUI frameworks that check `if view.version() != last_version`.
    #[must_use]
    pub fn version(&self) -> u64 {
        self.state.lock().expect("lock table state").version
    }

    /// Subscribes a listener to change events (push-based notification).
    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(TableSignal) + Send + Sync + 'static,
    {
        let mut guard = self.state.lock().expect("lock table state");
        guard.listeners.push(Arc::new(callback));
    }

    /// Subscribes a listener (alias for backward compatibility).
    pub fn add_listener<F>(&self, callback: F)
    where
        F: Fn(TableSignal) + Send + Sync + 'static,
    {
        self.subscribe(callback);
    }

    /// Selects a row by index.
    ///
    /// Returns `true` if selection changed successfully, or `false` if index was out of bounds.
    pub fn select(&self, index: usize) -> bool {
        let (signal, listeners) = {
            let mut guard = self.state.lock().expect("lock table state");
            if index >= guard.rows.len() {
                return false;
            }
            if guard.selected_index == Some(index) {
                return true;
            }

            let prev = guard.selected_index;
            guard.selected_index = Some(index);
            guard.version += 1;

            let signal = TableSignal {
                version: guard.version,
                kind: TableEvent::SelectionChanged {
                    previous: prev,
                    current: Some(index),
                },
            };
            (signal, guard.listeners.clone())
        };

        for listener in listeners {
            listener(signal);
        }

        true
    }

    /// Moves selection to a target index (alias for backward compatibility).
    pub fn move_to(&self, index: usize) -> bool {
        self.select(index)
    }

    /// Advances selection to the next row.
    pub fn next(&self) -> bool {
        match self.selected_index() {
            Some(idx) => self.select(idx + 1),
            None => false,
        }
    }

    /// Moves selection to the preceding row.
    pub fn prev(&self) -> bool {
        match self.selected_index() {
            Some(0) | None => false,
            Some(idx) => self.select(idx - 1),
        }
    }

    /// Moves selection to the first row.
    pub fn first(&self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.select(0)
        }
    }

    /// Moves selection to the last row.
    pub fn last(&self) -> bool {
        let total = self.len();
        if total == 0 {
            false
        } else {
            self.select(total - 1)
        }
    }

    /// Returns a cloned snapshot of the currently selected row.
    #[must_use]
    pub fn selected_row(&self) -> Option<Row> {
        let guard = self.state.lock().expect("lock table state");
        let idx = guard.selected_index?;
        guard.rows.get(idx).cloned()
    }

    /// Returns a cloned snapshot of the currently selected row (alias).
    #[must_use]
    pub fn current_row(&self) -> Option<Row> {
        self.selected_row()
    }

    /// Returns a cloned snapshot of the row at `index`, or `None` if out of bounds.
    #[must_use]
    pub fn row(&self, index: usize) -> Option<Row> {
        let guard = self.state.lock().expect("lock table state");
        guard.rows.get(index).cloned()
    }

    /// Retrieves a typed value from a column in the currently selected row.
    pub fn get_value<T: FromValue>(&self, column_name: &str) -> Result<T, Error> {
        let row = self
            .selected_row()
            .ok_or_else(|| Error::query_error("table view has no selected row or is empty"))?;
        row.get::<T>(column_name)
    }

    /// Retrieves an optional typed value from a column in the currently selected row.
    pub fn get_value_opt<T: FromValue>(&self, column_name: &str) -> Result<Option<T>, Error> {
        let row = self
            .selected_row()
            .ok_or_else(|| Error::query_error("table view has no selected row or is empty"))?;
        row.get_opt::<T>(column_name)
    }

    /// Retrieves a typed value (alias for backward compatibility).
    pub fn get_field<T: FromValue>(&self, field_name: &str) -> Result<T, Error> {
        self.get_value(field_name)
    }

    /// Retrieves an optional typed value (alias for backward compatibility).
    pub fn get_field_opt<T: FromValue>(&self, field_name: &str) -> Result<Option<T>, Error> {
        self.get_value_opt(field_name)
    }

    /// Replaces the entire row set, increments the version counter, and notifies listeners with [`TableEvent::RowsChanged`].
    ///
    /// Cursor positioning: retains current selected index if within bounds of the new rows,
    /// otherwise moves to the last available row, or `None` if the new rows are empty.
    pub fn replace_rows(&self, rows: Vec<Row>) {
        let (signal, listeners) = {
            let mut guard = self.state.lock().expect("lock table state");
            guard.rows = rows;
            guard.selected_index = if guard.rows.is_empty() {
                None
            } else {
                match guard.selected_index {
                    Some(idx) if idx < guard.rows.len() => Some(idx),
                    _ => Some(guard.rows.len() - 1),
                }
            };
            guard.version += 1;
            let signal = TableSignal {
                version: guard.version,
                kind: TableEvent::RowsChanged,
            };
            (signal, guard.listeners.clone())
        };

        for listener in listeners {
            listener(signal);
        }
    }

    /// Replaces an existing row at `index`, incrementing the version counter and emitting [`TableEvent::RowChanged`].
    ///
    /// Returns `true` if the row was updated, or `false` if `index` was out of bounds.
    pub fn set_row(&self, index: usize, row: Row) -> bool {
        let (signal, listeners) = {
            let mut guard = self.state.lock().expect("lock table state");
            if index >= guard.rows.len() {
                return false;
            }
            guard.rows[index] = row;
            guard.version += 1;
            let signal = TableSignal {
                version: guard.version,
                kind: TableEvent::RowChanged { index },
            };
            (signal, guard.listeners.clone())
        };

        for listener in listeners {
            listener(signal);
        }

        true
    }

    /// Inserts a new row at `index` (up to `len`), advancing the cursor to it and emitting [`TableEvent::RowInserted`].
    ///
    /// Returns `true` if inserted successfully, or `false` if `index > len`.
    pub fn insert_row(&self, index: usize, row: Row) -> bool {
        let (signal, listeners) = {
            let mut guard = self.state.lock().expect("lock table state");
            if index > guard.rows.len() {
                return false;
            }
            guard.rows.insert(index, row);
            guard.selected_index = Some(index);
            guard.version += 1;
            let signal = TableSignal {
                version: guard.version,
                kind: TableEvent::RowInserted { index },
            };
            (signal, guard.listeners.clone())
        };

        for listener in listeners {
            listener(signal);
        }

        true
    }

    /// Removes the row at `index`, incrementing the version counter and emitting [`TableEvent::RowRemoved`].
    ///
    /// Cursor positioning: retains current selected index if still within bounds of the reduced rows,
    /// otherwise moves to the new last row, or `None` if the table became empty.
    /// Returns `true` if removed, or `false` if `index` was out of bounds.
    pub fn remove_row(&self, index: usize) -> bool {
        let (signal, listeners) = {
            let mut guard = self.state.lock().expect("lock table state");
            if index >= guard.rows.len() {
                return false;
            }
            guard.rows.remove(index);
            guard.selected_index = if guard.rows.is_empty() {
                None
            } else {
                match guard.selected_index {
                    Some(idx) if idx < guard.rows.len() => Some(idx),
                    Some(_) => Some(guard.rows.len() - 1),
                    None => None,
                }
            };
            guard.version += 1;
            let signal = TableSignal {
                version: guard.version,
                kind: TableEvent::RowRemoved { index },
            };
            (signal, guard.listeners.clone())
        };

        for listener in listeners {
            listener(signal);
        }

        true
    }
}
