use std::sync::Arc;

use super::*;
use crate::value::FromValue;

fn test_values() -> Vec<Value> {
    vec![
        Value::Null,
        Value::Integer(0),
        Value::Integer(1),
        Value::Integer(-42),
        Value::Integer(i64::from(i32::MAX)),
        Value::Integer(i64::MAX),
        Value::Integer(i64::MIN),
        Value::Real(0.0),
        Value::Real(2.5),
        Value::Real(-1.25),
        Value::Text(String::new()),
        Value::Text("hello world".to_string()),
        Value::Blob(vec![]),
        Value::Blob(vec![1, 2, 3, 4]),
        Value::Boolean(true),
        Value::Boolean(false),
    ]
}

fn assert_same<T: FromValue + FromValueRef + PartialEq + std::fmt::Debug>(val: &Value) {
    let from_val = T::from_value("test_col", val);
    let from_val_ref = T::from_value_ref("test_col", val.as_value_ref());
    match (from_val, from_val_ref) {
        (Ok(v1), Ok(v2)) => assert_eq!(v1, v2, "mismatch for value {val:?}"),
        (Err(e1), Err(e2)) => assert_eq!(
            format!("{e1:?}"),
            format!("{e2:?}"),
            "error mismatch for value {val:?}"
        ),
        (r1, r2) => {
            panic!("outcome mismatch for {val:?}: from_value={r1:?}, from_value_ref={r2:?}")
        }
    }
}

#[test]
fn test_from_value_ref_parity_with_from_value() {
    let values = test_values();
    for v in &values {
        assert_same::<i64>(v);
        assert_same::<i32>(v);
        assert_same::<f64>(v);
        assert_same::<String>(v);
        assert_same::<bool>(v);
        assert_same::<Vec<u8>>(v);
        assert_same::<Value>(v);
        assert_same::<Option<i64>>(v);
        assert_same::<Option<String>>(v);
        assert_same::<Option<bool>>(v);
    }
}

#[test]
fn test_column_index_helper() {
    let cols = vec![
        "id".to_string(),
        "UserName".to_string(),
        "EMAIL".to_string(),
    ];
    assert_eq!(column_index(&cols, "id").unwrap(), 0);
    assert_eq!(column_index(&cols, "ID").unwrap(), 0);
    assert_eq!(column_index(&cols, "username").unwrap(), 1);
    assert_eq!(column_index(&cols, "USERNAME").unwrap(), 1);
    assert_eq!(column_index(&cols, "email").unwrap(), 2);

    let err = column_index(&cols, "missing").unwrap_err();
    assert!(matches!(err, Error::ColumnNotFound { ref name } if name == "missing"));
}

#[test]
fn test_decode_and_out_of_range() {
    let cols = vec!["a".to_string(), "b".to_string()];
    let vals = vec![Value::Integer(10), Value::Text("hello".to_string())];
    let row = Row::new(cols, vals);
    let row_ref = &row as &dyn RowRef;

    let v1: i64 = decode(row_ref, 0).unwrap();
    assert_eq!(v1, 10);
    let v2: String = decode(row_ref, 1).unwrap();
    assert_eq!(v2, "hello");

    let err = decode::<i64, _>(row_ref, 2).unwrap_err();
    assert!(matches!(err, Error::QueryError(_)));
}

#[test]
fn test_row_ref_for_row() {
    let cols = vec!["a".to_string(), "b".to_string()];
    let vals = vec![Value::Integer(10), Value::Text("abc".to_string())];
    let row = Row::new(cols, vals);

    let row_ref = &row as &dyn RowRef;
    assert_eq!(row_ref.columns(), &["a", "b"]);
    assert_eq!(row_ref.value(0).unwrap(), ValueRef::Integer(10));
    assert_eq!(row_ref.value(1).unwrap(), ValueRef::Text("abc"));

    let err = row_ref.value(2).unwrap_err();
    assert!(matches!(err, Error::QueryError(_)));
}

#[test]
fn test_from_row_tuple_arities_1_to_12() {
    let cols: Vec<String> = (0..12).map(|i| format!("c{i}")).collect();
    let vals: Vec<Value> = vec![
        Value::Integer(1),
        Value::Text("two".to_string()),
        Value::Boolean(true),
        Value::Real(4.0),
        Value::Integer(5),
        Value::Text("six".to_string()),
        Value::Boolean(false),
        Value::Real(8.0),
        Value::Integer(9),
        Value::Text("ten".to_string()),
        Value::Boolean(true),
        Value::Real(12.0),
    ];
    let row = Row::new(cols.clone(), vals);

    // Test arity 1
    type T1 = (i64,);
    T1::plan(&cols).unwrap();
    let res1 = T1::from_row(&(), &row).unwrap();
    assert_eq!(res1, (1,));

    // Test arity 2
    type T2 = (i64, String);
    T2::plan(&cols).unwrap();
    let res2 = T2::from_row(&(), &row).unwrap();
    assert_eq!(res2, (1, "two".to_string()));

    // Test arity 3
    type T3 = (i64, String, bool);
    T3::plan(&cols).unwrap();
    let res3 = T3::from_row(&(), &row).unwrap();
    assert_eq!(res3, (1, "two".to_string(), true));

    // Test arity 12
    type T12 = (
        i64,
        String,
        bool,
        f64,
        i64,
        String,
        bool,
        f64,
        i64,
        String,
        bool,
        f64,
    );
    T12::plan(&cols).unwrap();
    let res12 = T12::from_row(&(), &row).unwrap();
    assert_eq!(
        res12,
        (
            1,
            "two".to_string(),
            true,
            4.0,
            5,
            "six".to_string(),
            false,
            8.0,
            9,
            "ten".to_string(),
            true,
            12.0
        )
    );

    // Tuple wider than result set
    let short_cols = vec!["c0".to_string()];
    let err = <(i64, String)>::plan(&short_cols).unwrap_err();
    assert!(matches!(err, Error::QueryError(_)));
}

#[test]
fn test_from_row_for_row() {
    let cols = vec!["x".to_string(), "y".to_string()];
    let vals = vec![Value::Integer(100), Value::Text("val".to_string())];
    let original = Row::new(cols.clone(), vals);

    let plan = Row::plan(&cols).unwrap();
    let decoded = Row::from_row(&plan, &original).unwrap();
    assert_eq!(original, decoded);
    assert!(Arc::ptr_eq(decoded.shared_columns(), &plan));
}

#[test]
fn test_decode_out_of_range() {
    let cols = vec!["a".to_string()];
    let vals = vec![Value::Integer(42)];
    let row = Row::new(cols, vals);

    let ok: Result<i64, _> = decode(&row, 0);
    assert_eq!(ok.unwrap(), 42);

    let err: Result<i64, _> = decode(&row, 1);
    assert!(matches!(err, Err(Error::QueryError(msg)) if msg.contains("out of range")));
}
