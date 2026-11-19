//! In-memory columnar table store.

use crate::column::{Column, Value};

/// Errors encountered in table store operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    LengthMismatch {
        expected: usize,
        got: usize,
    },
    DuplicateColumn(String),
    ColumnNotFound(String),
    TypeMismatch {
        column: String,
        expected: &'static str,
    },
    NotGroupable(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::LengthMismatch { expected, got } => {
                write!(
                    f,
                    "column length mismatch: expected {}, got {}",
                    expected, got
                )
            }
            StoreError::DuplicateColumn(name) => write!(f, "duplicate column: {}", name),
            StoreError::ColumnNotFound(name) => write!(f, "column not found: {}", name),
            StoreError::TypeMismatch { column, expected } => {
                write!(
                    f,
                    "type mismatch on column '{}': expected {}",
                    column, expected
                )
            }
            StoreError::NotGroupable(name) => {
                write!(f, "column '{}' cannot be grouped in this store", name)
            }
        }
    }
}

impl std::error::Error for StoreError {}

/// Columnar in-memory store holding named columns of equal length.
#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    names: Vec<String>,
    pub(crate) columns: Vec<Column>,
    num_rows: usize,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    /// Creates an empty table store.
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            columns: Vec::new(),
            num_rows: 0,
        }
    }

    /// Appends a named column to the store.
    ///
    /// Returns an error if the column name already exists or if its length
    /// does not match the existing table row count.
    pub fn add(&mut self, name: &str, column: Column) -> Result<(), StoreError> {
        if self.column_index(name).is_some() {
            return Err(StoreError::DuplicateColumn(name.to_string()));
        }
        if self.columns.is_empty() {
            self.num_rows = column.len();
        } else if column.len() != self.num_rows {
            return Err(StoreError::LengthMismatch {
                expected: self.num_rows,
                got: column.len(),
            });
        }
        self.names.push(name.to_string());
        self.columns.push(column);
        Ok(())
    }

    /// Returns the number of rows in the store.
    #[inline]
    pub fn rows(&self) -> usize {
        self.num_rows
    }

    /// Returns the number of columns in the store.
    #[inline]
    pub fn columns_count(&self) -> usize {
        self.columns.len()
    }

    /// Looks up a column reference by name.
    pub fn column(&self, name: &str) -> Option<&Column> {
        self.column_index(name).map(|idx| &self.columns[idx])
    }

    /// Looks up the index of a column by name.
    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.names.iter().position(|n| n == name)
    }

    /// Returns the column name at index `idx`.
    pub fn column_name(&self, idx: usize) -> Option<&str> {
        self.names.get(idx).map(|s| s.as_str())
    }

    /// Returns the slice of column names in insertion order.
    #[inline]
    pub fn column_names(&self) -> &[String] {
        &self.names
    }

    /// Returns the slice of columns in insertion order.
    #[inline]
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    /// Returns the cell value at row `row` and column index `col`.
    #[inline]
    pub fn value(&self, row: usize, col: usize) -> Value<'_> {
        assert!(
            col < self.columns.len(),
            "column index out of bounds: {} >= {}",
            col,
            self.columns.len()
        );
        self.columns[col].get(row)
    }
}

#[cfg(test)]
#[path = "test/store/test.rs"]
mod test;
