use super::*;

#[test]
fn test_value_conversions_and_type_names() {
    assert_eq!(Value::from(42i64), Value::Integer(42));
    assert_eq!(Value::from(10i32), Value::Integer(10));
    assert_eq!(Value::from(5i16), Value::Integer(5));
    assert_eq!(Value::from(1i8), Value::Integer(1));
    assert_eq!(Value::from(100u32), Value::Integer(100));
    assert_eq!(Value::from(50u16), Value::Integer(50));
    assert_eq!(Value::from(25u8), Value::Integer(25));
    assert_eq!(Value::from(2.5f64), Value::Real(2.5));
    assert_eq!(Value::from("hello"), Value::Text("hello".to_string()));
    assert_eq!(Value::from(true), Value::Boolean(true));
    assert_eq!(Value::from(vec![1, 2, 3]), Value::Blob(vec![1, 2, 3]));
    assert_eq!(Value::from(Option::<i64>::None), Value::Null);
    assert_eq!(Value::from(Some(10i64)), Value::Integer(10));

    assert_eq!(Value::Null.type_name(), "NULL");
    assert_eq!(Value::Integer(1).type_name(), "INTEGER");
    assert_eq!(Value::Real(1.0).type_name(), "REAL");
    assert_eq!(Value::Text("a".into()).type_name(), "TEXT");
    assert_eq!(Value::Blob(vec![]).type_name(), "BLOB");
    assert_eq!(Value::Boolean(true).type_name(), "BOOLEAN");
}

#[test]
fn test_from_value_primitives() {
    let col = "test_col";
    assert_eq!(i64::from_value(col, &Value::Integer(42)).unwrap(), 42);
    assert_eq!(i32::from_value(col, &Value::Integer(10)).unwrap(), 10);
    assert_eq!(f64::from_value(col, &Value::Real(2.5)).unwrap(), 2.5);
    assert_eq!(
        String::from_value(col, &Value::Text("abc".into())).unwrap(),
        "abc"
    );
    assert!(bool::from_value(col, &Value::Boolean(true)).unwrap());
    assert_eq!(
        Vec::<u8>::from_value(col, &Value::Blob(vec![4, 5])).unwrap(),
        vec![4, 5]
    );

    // Option<T>
    assert_eq!(Option::<i64>::from_value(col, &Value::Null).unwrap(), None);
    assert_eq!(
        Option::<i64>::from_value(col, &Value::Integer(99)).unwrap(),
        Some(99)
    );

    // Display
    assert_eq!(format!("{}", Value::Integer(123)), "123");
    assert_eq!(format!("{}", Value::Null), "NULL");
}
