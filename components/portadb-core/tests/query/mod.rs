use super::*;
use crate::params;

#[test]
fn test_prepare_query_basic() {
    let p = params![("id", 42i64), ("name", "Alice")];
    let prep = prepare_query("SELECT * FROM users WHERE id = :id AND name = :name", &p).unwrap();
    assert_eq!(prep.sql, "SELECT * FROM users WHERE id = ? AND name = ?");
    assert_eq!(
        prep.params,
        vec![Value::Integer(42), Value::Text("Alice".into())]
    );
}

#[test]
fn test_prepare_query_repeated_param() {
    let p = params![("val", 10i64)];
    let prep = prepare_query("SELECT * FROM t WHERE a = :val OR b = :val", &p).unwrap();
    assert_eq!(prep.sql, "SELECT * FROM t WHERE a = ? OR b = ?");
    assert_eq!(prep.params, vec![Value::Integer(10), Value::Integer(10)]);
}

#[test]
fn test_prepare_query_ignores_string_literals_and_escapes() {
    let p = params![("id", 1i64)];
    let sql = "SELECT 'hello :not_param and ''it'':param'' inside' FROM t WHERE id = :id";
    let prep = prepare_query(sql, &p).unwrap();
    assert_eq!(
        prep.sql,
        "SELECT 'hello :not_param and ''it'':param'' inside' FROM t WHERE id = ?"
    );
    assert_eq!(prep.params, vec![Value::Integer(1)]);
}

#[test]
fn test_prepare_query_ignores_comments_and_double_colon() {
    let p = params![("id", 100i64)];
    let sql = "-- comment with :ignored_param\n/* block with :ignored_too */ SELECT id::text FROM t WHERE id = :id";
    let prep = prepare_query(sql, &p).unwrap();
    assert_eq!(
        prep.sql,
        "-- comment with :ignored_param\n/* block with :ignored_too */ SELECT id::text FROM t WHERE id = ?"
    );
    assert_eq!(prep.params, vec![Value::Integer(100)]);
}

#[test]
fn test_prepare_query_missing_param_returns_error() {
    let p = params![("id", 1i64)];
    let sql = "SELECT * FROM t WHERE id = :id AND missing = :not_provided";
    let err = prepare_query(sql, &p).unwrap_err();
    assert!(matches!(err, Error::QueryError(_)));
}
