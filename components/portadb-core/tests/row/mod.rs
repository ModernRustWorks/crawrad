use std::sync::Arc;

use super::*;

#[test]
fn test_row_distinguishes_absent_and_null() {
    let cols = vec!["id".into(), "name".into(), "bio".into()];
    let vals = vec![Value::Integer(1), Value::Text("Alice".into()), Value::Null];
    let row = Row::new(cols, vals);

    // Present non-null column
    assert_eq!(row.get::<i64>("id").unwrap(), 1);
    assert_eq!(row.get::<String>("name").unwrap(), "Alice");
    assert_eq!(row.get_opt::<String>("name").unwrap(), Some("Alice".into()));

    // Case-insensitive lookup
    assert_eq!(row.get::<String>("NAME").unwrap(), "Alice");

    // Present NULL column
    assert_eq!(row.get_opt::<String>("bio").unwrap(), None);
    let null_err = row.get::<String>("bio").unwrap_err();
    assert!(matches!(null_err, Error::NullValue { .. }));

    // Absent column: both get and get_opt return ColumnNotFound!
    let missing_err1 = row.get::<String>("missing").unwrap_err();
    assert!(matches!(missing_err1, Error::ColumnNotFound { .. }));
    let missing_err2 = row.get_opt::<String>("missing").unwrap_err();
    assert!(matches!(missing_err2, Error::ColumnNotFound { .. }));

    // Type mismatch
    let mismatch = row.get::<i64>("name").unwrap_err();
    assert!(matches!(mismatch, Error::TypeMismatch { .. }));
}

#[test]
fn test_row_len_and_columns() {
    let cols = vec!["a".into(), "b".into()];
    let vals = vec![Value::Integer(1), Value::Integer(2)];
    let row = Row::new(cols, vals);

    assert_eq!(row.len(), 2);
    assert!(!row.is_empty());
    assert_eq!(row.columns(), &["a", "b"]);
    assert!(row.has_column("a"));
    assert!(row.has_column("B"));
    assert!(!row.has_column("c"));
}

#[test]
fn test_row_with_shared_columns() {
    let cols: Arc<[String]> = vec!["name".to_string()].into();
    let row1 = Row::with_columns(Arc::clone(&cols), vec![Value::Integer(42)]);
    let row2 = Row::with_columns(Arc::clone(&cols), vec![Value::Integer(84)]);

    assert!(Arc::ptr_eq(row1.shared_columns(), row2.shared_columns()));
    assert_eq!(row1.get::<i64>("name").unwrap(), 42);
    assert_eq!(row1.get::<i64>("NAME").unwrap(), 42);
    assert_eq!(row2.get::<i64>("name").unwrap(), 84);
    assert_eq!(row2.get::<i64>("NAME").unwrap(), 84);
}
