//! Borrowed cell views and typed row decoding (`FromRow`).
//!
//! Provides the fast-path row decoding abstraction where engine cells are decoded directly
//! into target types without intermediate `Row` or `Value` allocations.

use std::ops::ControlFlow;
use std::sync::Arc;

use crate::error::Error;
use crate::row::Row;
use crate::value::Value;

/// A borrowed engine-agnostic SQL scalar value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueRef<'a> {
    /// SQL NULL value.
    Null,
    /// 64-bit signed integer.
    Integer(i64),
    /// 64-bit floating point number.
    Real(f64),
    /// Text / UTF-8 string slice.
    Text(&'a str),
    /// Binary blob byte slice.
    Blob(&'a [u8]),
    /// Boolean value.
    Boolean(bool),
}

impl<'a> ValueRef<'a> {
    /// Returns true if the value is SQL NULL.
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns a human-readable name of the value's type.
    #[must_use]
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "NULL",
            Self::Integer(_) => "INTEGER",
            Self::Real(_) => "REAL",
            Self::Text(_) => "TEXT",
            Self::Blob(_) => "BLOB",
            Self::Boolean(_) => "BOOLEAN",
        }
    }

    /// Converts this borrowed reference into an owned [`Value`].
    #[must_use]
    pub fn to_value(&self) -> Value {
        match *self {
            Self::Null => Value::Null,
            Self::Integer(i) => Value::Integer(i),
            Self::Real(r) => Value::Real(r),
            Self::Text(s) => Value::Text(s.to_string()),
            Self::Blob(b) => Value::Blob(b.to_vec()),
            Self::Boolean(b) => Value::Boolean(b),
        }
    }
}

impl<'a> From<ValueRef<'a>> for Value {
    fn from(val: ValueRef<'a>) -> Self {
        val.to_value()
    }
}

/// The callback a driver hands each borrowed row to; `Break` stops early and
/// an `Err` stops and is returned.
pub type OnRowRef<'a> = dyn FnMut(&dyn RowRef) -> Result<ControlFlow<()>, Error> + Send + 'a;

/// Visitor called once per row, by static dispatch.
pub trait RowVisitor {
    /// Visits a single borrowed row.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if decoding or processing fails.
    fn visit<R: RowRef>(&mut self, row: &R) -> Result<ControlFlow<()>, Error>;
}

/// Object-safe trait representing a borrowed view over a row.
pub trait RowRef {
    /// Returns the column names in order.
    fn columns(&self) -> &[String];

    /// Returns the borrowed value at the specified zero-based column index.
    ///
    /// # Errors
    ///
    /// Returns [`Error::QueryError`] if `index` is out of range.
    fn value(&self, index: usize) -> Result<ValueRef<'_>, Error>;
}

impl<T: RowRef + ?Sized> RowRef for &T {
    fn columns(&self) -> &[String] {
        (**self).columns()
    }

    fn value(&self, index: usize) -> Result<ValueRef<'_>, Error> {
        (**self).value(index)
    }
}

impl RowRef for Row {
    fn columns(&self) -> &[String] {
        self.columns()
    }

    fn value(&self, index: usize) -> Result<ValueRef<'_>, Error> {
        let vals = self.values();
        if index >= vals.len() {
            return Err(Error::QueryError(format!(
                "column index {index} out of range (row has {} columns)",
                vals.len()
            )));
        }
        Ok(vals[index].as_value_ref())
    }
}

/// Trait for converting from a borrowed database [`ValueRef`] into a concrete Rust type.
pub trait FromValueRef: Sized {
    /// Converts a borrowed [`ValueRef`] into `Self`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TypeMismatch`] if the value cannot be converted,
    /// or [`Error::NullValue`] if the value is NULL and the target type does not support NULL.
    fn from_value_ref(column_name: &str, value: ValueRef<'_>) -> Result<Self, Error>;
}

impl FromValueRef for Value {
    fn from_value_ref(_column_name: &str, value: ValueRef<'_>) -> Result<Self, Error> {
        Ok(value.to_value())
    }
}

impl FromValueRef for i64 {
    fn from_value_ref(column_name: &str, value: ValueRef<'_>) -> Result<Self, Error> {
        match value {
            ValueRef::Integer(v) => Ok(v),
            ValueRef::Boolean(b) => Ok(if b { 1 } else { 0 }),
            ValueRef::Null => Err(Error::NullValue {
                column: column_name.to_string(),
            }),
            other => Err(Error::TypeMismatch {
                column: column_name.to_string(),
                expected: "i64",
                found: other.type_name().to_string(),
            }),
        }
    }
}

impl FromValueRef for i32 {
    fn from_value_ref(column_name: &str, value: ValueRef<'_>) -> Result<Self, Error> {
        let n = i64::from_value_ref(column_name, value)?;
        i32::try_from(n).map_err(|_| Error::TypeMismatch {
            column: column_name.to_string(),
            expected: "i32 (within range)",
            found: format!("i64 value {n}"),
        })
    }
}

impl FromValueRef for f64 {
    fn from_value_ref(column_name: &str, value: ValueRef<'_>) -> Result<Self, Error> {
        match value {
            ValueRef::Real(v) => Ok(v),
            ValueRef::Integer(v) => Ok(v as f64),
            ValueRef::Null => Err(Error::NullValue {
                column: column_name.to_string(),
            }),
            other => Err(Error::TypeMismatch {
                column: column_name.to_string(),
                expected: "f64",
                found: other.type_name().to_string(),
            }),
        }
    }
}

impl FromValueRef for String {
    fn from_value_ref(column_name: &str, value: ValueRef<'_>) -> Result<Self, Error> {
        match value {
            ValueRef::Text(v) => Ok(v.to_string()),
            ValueRef::Null => Err(Error::NullValue {
                column: column_name.to_string(),
            }),
            other => Err(Error::TypeMismatch {
                column: column_name.to_string(),
                expected: "String",
                found: other.type_name().to_string(),
            }),
        }
    }
}

impl FromValueRef for bool {
    fn from_value_ref(column_name: &str, value: ValueRef<'_>) -> Result<Self, Error> {
        match value {
            ValueRef::Boolean(b) => Ok(b),
            ValueRef::Integer(n) => Ok(n != 0),
            ValueRef::Null => Err(Error::NullValue {
                column: column_name.to_string(),
            }),
            other => Err(Error::TypeMismatch {
                column: column_name.to_string(),
                expected: "bool",
                found: other.type_name().to_string(),
            }),
        }
    }
}

impl FromValueRef for Vec<u8> {
    fn from_value_ref(column_name: &str, value: ValueRef<'_>) -> Result<Self, Error> {
        match value {
            ValueRef::Blob(v) => Ok(v.to_vec()),
            ValueRef::Text(s) => Ok(s.as_bytes().to_vec()),
            ValueRef::Null => Err(Error::NullValue {
                column: column_name.to_string(),
            }),
            other => Err(Error::TypeMismatch {
                column: column_name.to_string(),
                expected: "Vec<u8>",
                found: other.type_name().to_string(),
            }),
        }
    }
}

impl<T: FromValueRef> FromValueRef for Option<T> {
    fn from_value_ref(column_name: &str, value: ValueRef<'_>) -> Result<Self, Error> {
        match value {
            ValueRef::Null => Ok(None),
            other => T::from_value_ref(column_name, other).map(Some),
        }
    }
}

/// Finds the index of a column by name (case-insensitive).
///
/// # Errors
///
/// Returns [`Error::ColumnNotFound`] if `name` is not in `columns`.
pub fn column_index(columns: &[String], name: &str) -> Result<usize, Error> {
    columns
        .iter()
        .position(|c| c.eq_ignore_ascii_case(name))
        .ok_or_else(|| Error::ColumnNotFound {
            name: name.to_string(),
        })
}

/// Decodes the value at `index` into type `T` using the row's column name.
///
/// # Errors
///
/// Returns [`Error::QueryError`] if `index` is out of range,
/// [`Error::NullValue`] if the value is NULL and `T` is non-optional,
/// or [`Error::TypeMismatch`] if the value cannot be converted into `T`.
pub fn decode<T: FromValueRef, R: RowRef + ?Sized>(row: &R, index: usize) -> Result<T, Error> {
    let cols = row.columns();
    if index >= cols.len() {
        return Err(Error::QueryError(format!(
            "column index {index} out of range (row has {} columns)",
            cols.len()
        )));
    }
    let val_ref = row.value(index)?;
    T::from_value_ref(&cols[index], val_ref)
}

/// Trait for decoding a row into a concrete typed struct or tuple.
pub trait FromRow: Sized {
    /// Resolved once per result (e.g. column indices).
    type Plan: Send;

    /// Plans the decoding for the given query result columns.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if required columns are missing or the column layout is invalid.
    fn plan(columns: &[String]) -> Result<Self::Plan, Error>;

    /// Decodes a row according to the prepared plan.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if a cell cannot be converted or is unexpectedly null.
    fn from_row<R: RowRef + ?Sized>(plan: &Self::Plan, row: &R) -> Result<Self, Error>;
}

impl FromRow for Row {
    type Plan = Arc<[String]>;

    fn plan(columns: &[String]) -> Result<Self::Plan, Error> {
        Ok(Arc::from(columns))
    }

    fn from_row<R: RowRef + ?Sized>(plan: &Self::Plan, row: &R) -> Result<Self, Error> {
        let mut values = Vec::with_capacity(plan.len());
        for i in 0..plan.len() {
            let val_ref = row.value(i)?;
            values.push(val_ref.to_value());
        }
        Ok(Row::with_columns(Arc::clone(plan), values))
    }
}

macro_rules! impl_from_row_tuple {
    ($($arity:literal => ($($idx:tt : $T:ident),+);)+) => {
        $(
            impl<$($T: FromValueRef),+> FromRow for ($($T,)+) {
                type Plan = ();

                fn plan(columns: &[String]) -> Result<Self::Plan, Error> {
                    if columns.len() < $arity {
                        return Err(Error::QueryError(format!(
                            "result set has {} columns, but tuple requires {}",
                            columns.len(),
                            $arity
                        )));
                    }
                    Ok(())
                }

                fn from_row<R: RowRef + ?Sized>(_plan: &Self::Plan, row: &R) -> Result<Self, Error> {
                    Ok((
                        $(
                            decode::<$T, _>(row, $idx)?,
                        )+
                    ))
                }
            }
        )+
    };
}

impl_from_row_tuple! {
    1 => (0: T0);
    2 => (0: T0, 1: T1);
    3 => (0: T0, 1: T1, 2: T2);
    4 => (0: T0, 1: T1, 2: T2, 3: T3);
    5 => (0: T0, 1: T1, 2: T2, 3: T3, 4: T4);
    6 => (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5);
    7 => (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6);
    8 => (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7);
    9 => (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7, 8: T8);
    10 => (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7, 8: T8, 9: T9);
    11 => (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7, 8: T8, 9: T9, 10: T10);
    12 => (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7, 8: T8, 9: T9, 10: T10, 11: T11);
}

#[cfg(test)]
#[path = "../tests/row_ref/mod.rs"]
mod test;
