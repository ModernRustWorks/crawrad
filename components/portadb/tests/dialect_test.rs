use portadb::{
    Dialect, MySqlDialect, PostgresDialect, SqlServerDialect, SqliteDialect, Value, params,
};

#[test]
fn test_sqlite_dialect_placeholders_and_quoting() {
    let dialect = SqliteDialect;
    assert_eq!(dialect.name(), "sqlite");
    assert_eq!(dialect.quote_identifier("users"), "\"users\"");

    let params = params![("id", 1i64), ("name", "Alice")];
    let prepared = dialect
        .prepare_query(
            "SELECT * FROM users WHERE id = :id AND name = :name",
            &params,
        )
        .unwrap();

    assert_eq!(
        prepared.sql,
        "SELECT * FROM users WHERE id = ? AND name = ?"
    );
    assert_eq!(
        prepared.params,
        vec![Value::Integer(1), Value::Text("Alice".into())]
    );
}

#[test]
fn test_postgres_dialect_numbered_placeholders_and_casting() {
    let dialect = PostgresDialect;
    assert_eq!(dialect.name(), "postgres");
    assert_eq!(dialect.quote_identifier("orders"), "\"orders\"");

    let params = params![("user_id", 42i64), ("status", "active")];
    // Note: ::text cast should NOT be treated as a named parameter
    let prepared = dialect
        .prepare_query(
            "SELECT id, status::text FROM orders WHERE user_id = :user_id AND status = :status",
            &params,
        )
        .unwrap();

    assert_eq!(
        prepared.sql,
        "SELECT id, status::text FROM orders WHERE user_id = $1 AND status = $2"
    );
    assert_eq!(
        prepared.params,
        vec![Value::Integer(42), Value::Text("active".into())]
    );
}

#[test]
fn test_mysql_dialect_placeholders_and_backtick_quoting() {
    let dialect = MySqlDialect;
    assert_eq!(dialect.name(), "mysql");
    assert_eq!(dialect.quote_identifier("schema"), "`schema`");

    let params = params![("limit", 10i64)];
    let prepared = dialect
        .prepare_query("SELECT * FROM items LIMIT :limit", &params)
        .unwrap();

    assert_eq!(prepared.sql, "SELECT * FROM items LIMIT ?");
    assert_eq!(prepared.params, vec![Value::Integer(10)]);
}

#[test]
fn test_sqlserver_dialect_named_placeholders_and_bracket_quoting() {
    let dialect = SqlServerDialect;
    assert_eq!(dialect.name(), "sqlserver");
    assert_eq!(dialect.quote_identifier("table"), "[table]");

    let params = params![("id", 99i64), ("category", "Books")];
    let prepared = dialect
        .prepare_query(
            "SELECT TOP 1 * FROM inventory WHERE id = :id AND category = :category",
            &params,
        )
        .unwrap();

    assert_eq!(
        prepared.sql,
        "SELECT TOP 1 * FROM inventory WHERE id = @p1 AND category = @p2"
    );
    assert_eq!(
        prepared.params,
        vec![Value::Integer(99), Value::Text("Books".into())]
    );
}
