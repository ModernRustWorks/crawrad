use std::sync::Arc;

use crate::error::Error;
use crate::value::{FromValue, Value};

/// A single row returned by a query execution.
///
/// Columns are indexed both by position and by case-insensitive name.
/// Provides typed accessors that strictly distinguish between a column
/// being absent from the query result set versus a column being present with a SQL NULL value.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    columns: Arc<[String]>,
    values: Vec<Value>,
}

impl Row {
    /// Constructs a new `Row` with the given column names and values.
    ///
    /// # Panics
    ///
    /// Panics if the number of column names does not match the number of values.
    #[must_use]
    pub fn new(columns: Vec<String>, values: Vec<Value>) -> Self {
        Self::with_columns(columns.into(), values)
    }

    /// Constructs a new `Row` with shared column names and values.
    ///
    /// # Panics
    ///
    /// Panics if the number of column names does not match the number of values.
    #[must_use]
    pub fn with_columns(columns: Arc<[String]>, values: Vec<Value>) -> Self {
        assert_eq!(
            columns.len(),
            values.len(),
            "columns length must match values length"
        );
        Self { columns, values }
    }

    /// Returns the column names in order.
    #[must_use]
    pub fn columns(&self) -> &[String] {
        &self.columns
    }

    /// Returns a reference to the shared column names `Arc`.
    #[must_use]
    pub fn shared_columns(&self) -> &Arc<[String]> {
        &self.columns
    }

    /// Returns the raw values in order.
    #[must_use]
    pub fn values(&self) -> &[Value] {
        &self.values
    }

    /// Returns the number of columns in this row.
    #[must_use]
    pub fn len(&self) -> usize {
        self.columns.len()
    }

    /// Returns true if this row has no columns.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }

    /// Finds the index of a column by name (case-insensitive).
    #[must_use]
    pub fn column_index(&self, column_name: &str) -> Option<usize> {
        self.columns
            .iter()
            .position(|c| c.eq_ignore_ascii_case(column_name))
    }

    /// Returns true if the column exists in the row.
    #[must_use]
    pub fn has_column(&self, column_name: &str) -> bool {
        self.column_index(column_name).is_some()
    }

    /// Retrieves a raw reference to the [`Value`] for a column.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ColumnNotFound`] if no column matches `column_name`.
    pub fn get_value(&self, column_name: &str) -> Result<&Value, Error> {
        match self.column_index(column_name) {
            Some(idx) => Ok(&self.values[idx]),
            None => Err(Error::ColumnNotFound {
                name: column_name.to_string(),
            }),
        }
    }

    /// Checks if a column's value is SQL NULL.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ColumnNotFound`] if the column does not exist in the row.
    pub fn is_null(&self, column_name: &str) -> Result<bool, Error> {
        let val = self.get_value(column_name)?;
        Ok(val.is_null())
    }

    /// Retrieves a non-null typed value from the specified column.
    ///
    /// # Errors
    ///
    /// - Returns [`Error::ColumnNotFound`] if the column is not in the row.
    /// - Returns [`Error::NullValue`] if the column is present but holds SQL NULL.
    /// - Returns [`Error::TypeMismatch`] if the value cannot be converted to `T`.
    pub fn get<T: FromValue>(&self, column_name: &str) -> Result<T, Error> {
        let val = self.get_value(column_name)?;
        T::from_value(column_name, val)
    }

    /// Retrieves an optional typed value from the specified column.
    ///
    /// # Errors
    ///
    /// - Returns [`Error::ColumnNotFound`] if the column is not in the row.
    /// - Returns `Ok(None)` if the column is present and holds SQL NULL.
    /// - Returns `Ok(Some(v))` if the column is present and holds a convertible value.
    /// - Returns [`Error::TypeMismatch`] if the value cannot be converted to `T`.
    pub fn get_opt<T: FromValue>(&self, column_name: &str) -> Result<Option<T>, Error> {
        let val = self.get_value(column_name)?;
        if val.is_null() {
            Ok(None)
        } else {
            T::from_value(column_name, val).map(Some)
        }
    }
}

#[cfg(test)]
#[path = "../tests/row/mod.rs"]
mod test;
