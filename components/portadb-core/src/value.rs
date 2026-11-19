use crate::error::Error;
use std::fmt;

/// An engine-agnostic SQL scalar value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// SQL NULL value.
    Null,
    /// 64-bit signed integer.
    Integer(i64),
    /// 64-bit floating point number.
    Real(f64),
    /// Text / UTF-8 string.
    Text(String),
    /// Binary blob.
    Blob(Vec<u8>),
    /// Boolean value.
    Boolean(bool),
}

impl Value {
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

    /// Returns a borrowed [`ValueRef`](crate::row_ref::ValueRef) view of this value.
    #[must_use]
    pub fn as_value_ref(&self) -> crate::row_ref::ValueRef<'_> {
        match self {
            Self::Null => crate::row_ref::ValueRef::Null,
            Self::Integer(v) => crate::row_ref::ValueRef::Integer(*v),
            Self::Real(v) => crate::row_ref::ValueRef::Real(*v),
            Self::Text(v) => crate::row_ref::ValueRef::Text(v.as_str()),
            Self::Blob(v) => crate::row_ref::ValueRef::Blob(v.as_slice()),
            Self::Boolean(v) => crate::row_ref::ValueRef::Boolean(*v),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "NULL"),
            Self::Integer(v) => write!(f, "{v}"),
            Self::Real(v) => write!(f, "{v}"),
            Self::Text(v) => write!(f, "{v}"),
            Self::Blob(v) => write!(f, "<{} bytes>", v.len()),
            Self::Boolean(v) => write!(f, "{v}"),
        }
    }
}

impl From<i64> for Value {
    fn from(val: i64) -> Self {
        Self::Integer(val)
    }
}

impl From<i32> for Value {
    fn from(val: i32) -> Self {
        Self::Integer(i64::from(val))
    }
}

impl From<i16> for Value {
    fn from(val: i16) -> Self {
        Self::Integer(i64::from(val))
    }
}

impl From<i8> for Value {
    fn from(val: i8) -> Self {
        Self::Integer(i64::from(val))
    }
}

impl From<u32> for Value {
    fn from(val: u32) -> Self {
        Self::Integer(i64::from(val))
    }
}

impl From<u16> for Value {
    fn from(val: u16) -> Self {
        Self::Integer(i64::from(val))
    }
}

impl From<u8> for Value {
    fn from(val: u8) -> Self {
        Self::Integer(i64::from(val))
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Self::Real(val)
    }
}

impl From<f32> for Value {
    fn from(val: f32) -> Self {
        Self::Real(f64::from(val))
    }
}

impl From<String> for Value {
    fn from(val: String) -> Self {
        Self::Text(val)
    }
}

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        Self::Text(val.to_string())
    }
}

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Self::Boolean(val)
    }
}

impl From<Vec<u8>> for Value {
    fn from(val: Vec<u8>) -> Self {
        Self::Blob(val)
    }
}

impl From<&[u8]> for Value {
    fn from(val: &[u8]) -> Self {
        Self::Blob(val.to_vec())
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(val: Option<T>) -> Self {
        match val {
            Some(v) => v.into(),
            None => Self::Null,
        }
    }
}

/// Trait for converting from a database [`Value`] into a concrete Rust type.
pub trait FromValue: Sized {
    /// Converts a reference to a database [`Value`] into `Self`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TypeMismatch`] if the value cannot be converted,
    /// or [`Error::NullValue`] if the value is NULL and the target type does not support NULL.
    fn from_value(column_name: &str, value: &Value) -> Result<Self, Error>;
}

impl FromValue for Value {
    fn from_value(_column_name: &str, value: &Value) -> Result<Self, Error> {
        Ok(value.clone())
    }
}

impl FromValue for i64 {
    fn from_value(column_name: &str, value: &Value) -> Result<Self, Error> {
        match value {
            Value::Integer(v) => Ok(*v),
            Value::Boolean(b) => Ok(if *b { 1 } else { 0 }),
            Value::Null => Err(Error::NullValue {
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

impl FromValue for i32 {
    fn from_value(column_name: &str, value: &Value) -> Result<Self, Error> {
        let n = i64::from_value(column_name, value)?;
        i32::try_from(n).map_err(|_| Error::TypeMismatch {
            column: column_name.to_string(),
            expected: "i32 (within range)",
            found: format!("i64 value {n}"),
        })
    }
}

impl FromValue for f64 {
    fn from_value(column_name: &str, value: &Value) -> Result<Self, Error> {
        match value {
            Value::Real(v) => Ok(*v),
            Value::Integer(v) => Ok(*v as f64),
            Value::Null => Err(Error::NullValue {
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

impl FromValue for String {
    fn from_value(column_name: &str, value: &Value) -> Result<Self, Error> {
        match value {
            Value::Text(v) => Ok(v.clone()),
            Value::Null => Err(Error::NullValue {
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

impl FromValue for bool {
    fn from_value(column_name: &str, value: &Value) -> Result<Self, Error> {
        match value {
            Value::Boolean(b) => Ok(*b),
            Value::Integer(n) => Ok(*n != 0),
            Value::Null => Err(Error::NullValue {
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

impl FromValue for Vec<u8> {
    fn from_value(column_name: &str, value: &Value) -> Result<Self, Error> {
        match value {
            Value::Blob(v) => Ok(v.clone()),
            Value::Text(s) => Ok(s.as_bytes().to_vec()),
            Value::Null => Err(Error::NullValue {
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

impl<T: FromValue> FromValue for Option<T> {
    fn from_value(column_name: &str, value: &Value) -> Result<Self, Error> {
        match value {
            Value::Null => Ok(None),
            other => T::from_value(column_name, other).map(Some),
        }
    }
}

#[cfg(test)]
#[path = "../tests/value/mod.rs"]
mod test;
