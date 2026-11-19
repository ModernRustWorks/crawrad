//! Lazy per-column lookup and range indexes.

use crate::column::Column;
use crate::filter::Owned;
use crate::ordinal;
use crate::sort::{Nulls, Order, SortKey};
use crate::store::{Store, StoreError};
use std::collections::HashMap;

/// Lazy index cache supporting fast point-lookup (`find_eq`) and range queries (`range`).
pub struct Locator {
    pub(crate) sorted_indexes: HashMap<String, Vec<u32>>,
}

impl Default for Locator {
    fn default() -> Self {
        Self::new()
    }
}

impl Locator {
    /// Creates a new, empty locator with no cached indexes.
    pub fn new() -> Self {
        Self {
            sorted_indexes: HashMap::new(),
        }
    }

    fn ensure_sorted_index(&mut self, store: &Store, column: &str) -> Result<(), StoreError> {
        if !self.sorted_indexes.contains_key(column) {
            let col = store
                .column(column)
                .ok_or_else(|| StoreError::ColumnNotFound(column.to_string()))?;
            let num_rows = store.rows();
            let non_null_rows: Vec<u32> = (0..num_rows as u32)
                .filter(|&r| !col.is_null(r as usize))
                .collect();
            let sorted = store.sort(
                &non_null_rows,
                &[SortKey {
                    column: column.to_string(),
                    order: Order::Asc,
                    nulls: Nulls::Last,
                }],
            )?;
            self.sorted_indexes.insert(column.to_string(), sorted);
        }
        Ok(())
    }

    /// Returns the first row ID whose cell equals `value`, querying the sorted permutation
    /// via binary search.
    pub fn find_eq(
        &mut self,
        store: &Store,
        column: &str,
        value: &Owned,
    ) -> Result<Option<u32>, StoreError> {
        let col = store
            .column(column)
            .ok_or_else(|| StoreError::ColumnNotFound(column.to_string()))?;

        validate_type(col, value, column)?;

        self.ensure_sorted_index(store, column)?;

        let sorted = self
            .sorted_indexes
            .get(column)
            .expect("sorted index was just inserted or verified");

        let start =
            sorted.partition_point(|&row| cmp_col_with_owned(col, row as usize, value).is_lt());

        if start < sorted.len() && cmp_col_with_owned(col, sorted[start] as usize, value).is_eq() {
            Ok(Some(sorted[start]))
        } else {
            Ok(None)
        }
    }

    /// Returns all row IDs whose column value falls within `[lo, hi]` (inclusive),
    /// building the sorted permutation on first use and querying via binary search.
    pub fn range(
        &mut self,
        store: &Store,
        column: &str,
        lo: &Owned,
        hi: &Owned,
    ) -> Result<Vec<u32>, StoreError> {
        let col = store
            .column(column)
            .ok_or_else(|| StoreError::ColumnNotFound(column.to_string()))?;

        validate_type(col, lo, column)?;
        validate_type(col, hi, column)?;

        self.ensure_sorted_index(store, column)?;

        let sorted = self
            .sorted_indexes
            .get(column)
            .expect("sorted index was just inserted or verified");

        let start =
            sorted.partition_point(|&row| cmp_col_with_owned(col, row as usize, lo).is_lt());
        let end = sorted.partition_point(|&row| cmp_col_with_owned(col, row as usize, hi).is_le());

        if start >= end {
            Ok(Vec::new())
        } else {
            Ok(sorted[start..end].to_vec())
        }
    }

    /// Clears all cached sorted indexes.
    pub fn clear(&mut self) {
        self.sorted_indexes.clear();
    }
}

fn validate_type(col: &Column, val: &Owned, col_name: &str) -> Result<(), StoreError> {
    let matches = matches!(
        (col, val),
        (Column::Int { .. }, Owned::Int(_))
            | (Column::Float { .. }, Owned::Float(_))
            | (Column::Bool { .. }, Owned::Bool(_))
            | (Column::Text { .. } | Column::Dict { .. }, Owned::Text(_))
    );
    if matches {
        Ok(())
    } else {
        let expected = match col {
            Column::Int { .. } => "Int",
            Column::Float { .. } => "Float",
            Column::Bool { .. } => "Bool",
            Column::Text { .. } | Column::Dict { .. } => "Text",
        };
        Err(StoreError::TypeMismatch {
            column: col_name.to_string(),
            expected,
        })
    }
}

fn cmp_col_with_owned(col: &Column, row: usize, target: &Owned) -> std::cmp::Ordering {
    match (col, target) {
        (Column::Int { values, .. }, Owned::Int(t)) => values[row].cmp(t),
        (Column::Float { values, .. }, Owned::Float(t)) => {
            ordinal::float(values[row]).cmp(&ordinal::float(*t))
        }
        (Column::Bool { values, .. }, Owned::Bool(t)) => values.get(row).cmp(t),
        (Column::Text { bytes, offsets, .. }, Owned::Text(t)) => {
            let start = offsets[row] as usize;
            let end = offsets[row + 1] as usize;
            let s = std::str::from_utf8(&bytes[start..end]).expect("valid utf-8");
            s.cmp(t.as_str())
        }
        (Column::Dict { codes, dict, .. }, Owned::Text(target)) => {
            let s = &dict[codes[row] as usize];
            s.as_str().cmp(target.as_str())
        }
        _ => std::cmp::Ordering::Equal,
    }
}

#[cfg(test)]
#[path = "test/locate/test.rs"]
mod test;
