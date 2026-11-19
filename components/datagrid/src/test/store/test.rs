use super::*;

#[test]
fn test_store_empty() {
    let store = Store::new();
    assert_eq!(store.rows(), 0);
    assert_eq!(store.columns_count(), 0);
    assert_eq!(store.column("none"), None);
}

#[test]
fn test_store_add_columns() {
    let mut store = Store::new();
    store
        .add("id", Column::ints(vec![Some(1), Some(2), Some(3)]))
        .expect("add id");
    assert_eq!(store.rows(), 3);
    assert_eq!(store.columns_count(), 1);
    assert_eq!(store.column_index("id"), Some(0));

    store
        .add(
            "name",
            Column::texts(vec![
                Some("alice".into()),
                Some("bob".into()),
                Some("charlie".into()),
            ]),
        )
        .expect("add name");
    assert_eq!(store.columns_count(), 2);
    assert_eq!(store.column_index("name"), Some(1));

    assert_eq!(store.value(0, 0), Value::Int(1));
    assert_eq!(store.value(0, 1), Value::Text("alice"));
    assert_eq!(store.value(1, 0), Value::Int(2));
    assert_eq!(store.value(1, 1), Value::Text("bob"));
}

#[test]
fn test_store_duplicate_column_error() {
    let mut store = Store::new();
    store
        .add("col1", Column::ints(vec![Some(10)]))
        .expect("first add");
    let err = store
        .add("col1", Column::ints(vec![Some(20)]))
        .expect_err("must fail");
    assert_eq!(err, StoreError::DuplicateColumn("col1".to_string()));
}

#[test]
fn test_store_length_mismatch_error() {
    let mut store = Store::new();
    store
        .add("col1", Column::ints(vec![Some(1), Some(2)]))
        .expect("first add");
    let err = store
        .add("col2", Column::ints(vec![Some(1)]))
        .expect_err("must fail");
    assert_eq!(
        err,
        StoreError::LengthMismatch {
            expected: 2,
            got: 1
        }
    );
}
