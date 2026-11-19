use portadb::{
    ConnectOptions, Engine, MySqlDialect, PostgresDialect, Query, SqlServerDialect, SqliteDialect,
    params,
};

#[tokio::test]
async fn test_fluent_query_builder_and_execution() {
    let options = ConnectOptions::new("sqlite:file:builder_test1?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS builder_customers (id INT, name TEXT, city TEXT);",
            &params![],
        )
        .await
        .expect("create");

    engine
        .execute(
            "INSERT INTO builder_customers VALUES (1, 'Alice', 'Curitiba'), (2, 'Bob', 'São Paulo');",
            &params![],
        )
        .await
        .expect("insert");

    // 1. Build fluent query
    let query = Query::select(&["id", "name"])
        .from("builder_customers")
        .where_eq("city", "Curitiba")
        .order_by_asc("name")
        .limit(10);

    // 2. Dialect translation verification
    let (sqlite_sql, _) = query.build(&SqliteDialect);
    assert_eq!(
        sqlite_sql,
        "SELECT \"id\", \"name\" FROM \"builder_customers\" WHERE \"city\" = :p_1 ORDER BY \"name\" ASC LIMIT 10"
    );

    let (mysql_sql, _) = query.build(&MySqlDialect);
    assert_eq!(
        mysql_sql,
        "SELECT `id`, `name` FROM `builder_customers` WHERE `city` = :p_1 ORDER BY `name` ASC LIMIT 10"
    );

    let (sqlserver_sql, _) = query.build(&SqlServerDialect);
    assert_eq!(
        sqlserver_sql,
        "SELECT [id], [name] FROM [builder_customers] WHERE [city] = :p_1 ORDER BY [name] ASC LIMIT 10"
    );

    // 3. Execute directly through engine
    let rows = engine.execute_query(&query).await.expect("execute query");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<String>("name").unwrap(), "Alice");
}

#[tokio::test]
async fn test_builder_star_and_offsets() {
    let query = Query::select(&[])
        .from("products")
        .where_eq("active", true)
        .order_by_desc("price")
        .limit(20)
        .offset(40);

    let (sql, params) = query.build(&PostgresDialect);
    assert_eq!(
        sql,
        "SELECT * FROM \"products\" WHERE \"active\" = :p_1 ORDER BY \"price\" DESC LIMIT 20 OFFSET 40"
    );
    assert_eq!(params.len(), 1);
    assert!(params.get("p_1").is_some());
}

#[tokio::test]
async fn test_builder_multiple_where_clauses_execution() {
    let options = ConnectOptions::new("sqlite:file:builder_test2?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS builder_multi (id INT, dept TEXT, role TEXT, salary INT);",
            &params![],
        )
        .await
        .expect("create");

    engine
        .execute(
            "INSERT INTO builder_multi VALUES \
             (1, 'Eng', 'Lead', 12000), \
             (2, 'Eng', 'Junior', 4000), \
             (3, 'Sales', 'Lead', 11000);",
            &params![],
        )
        .await
        .expect("insert");

    let query = Query::select(&["id", "salary"])
        .from("builder_multi")
        .where_eq("dept", "Eng")
        .where_eq("role", "Lead");

    let (sql, p) = query.build(engine.dialect());
    assert!(sql.contains("WHERE \"dept\" = :p_1 AND \"role\" = :p_2"));
    assert_eq!(p.len(), 2);

    let rows = engine.execute_query(&query).await.expect("execute");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<i64>("id").unwrap(), 1);
    assert_eq!(rows[0].get::<i64>("salary").unwrap(), 12000);
}
