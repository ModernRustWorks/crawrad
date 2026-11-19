use std::any::Any;
use std::ops::ControlFlow;

use portadb::error::Error;
use portadb::params;
use portadb::row_ref::{FromRow, RowRef};
use portadb::{
    BoxFuture, ConnectOptions, Connection, Database, Driver, DriverConnection, DriverTransaction,
    NamedParams, Row, Value, register_driver,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Person {
    id: i64,
    name: String,
}

struct PersonPlan {
    id_idx: usize,
    name_idx: usize,
}

impl FromRow for Person {
    type Plan = PersonPlan;

    fn plan(columns: &[String]) -> Result<Self::Plan, Error> {
        let id_idx = portadb::row_ref::column_index(columns, "id")?;
        let name_idx = portadb::row_ref::column_index(columns, "name")?;
        Ok(PersonPlan { id_idx, name_idx })
    }

    fn from_row<R: RowRef + ?Sized>(plan: &Self::Plan, row: &R) -> Result<Self, Error> {
        let id: i64 = portadb::row_ref::decode(row, plan.id_idx)?;
        let name: String = portadb::row_ref::decode(row, plan.name_idx)?;
        Ok(Person { id, name })
    }
}

// ---------------------------------------------------------------------------
// Third-party driver written purely against the native traits
// ---------------------------------------------------------------------------

struct NativeMockDriver;

impl Driver for NativeMockDriver {
    type Connection = NativeMockConn;

    fn name(&self) -> &'static str {
        "native-mock"
    }

    fn supports_database(&self, database: Database) -> bool {
        matches!(database, Database::Sqlite)
    }

    async fn connect(&self, _options: &ConnectOptions) -> Result<Self::Connection, Error> {
        Ok(NativeMockConn { counter: 0 })
    }
}

struct NativeMockConn {
    counter: u64,
}

impl DriverConnection for NativeMockConn {
    async fn execute(&mut self, _sql: &str, _params: &NamedParams) -> Result<u64, Error> {
        self.counter += 1;
        Ok(self.counter)
    }

    async fn query(&mut self, _sql: &str, _params: &NamedParams) -> Result<Vec<Row>, Error> {
        Ok(vec![Row::new(
            vec!["result".to_string()],
            vec![Value::Integer(123)],
        )])
    }

    async fn begin_transaction(&mut self) -> Result<Box<dyn DriverTransaction + '_>, Error> {
        Ok(Box::new(NativeMockTx))
    }

    async fn close(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct NativeMockTx;

impl DriverTransaction for NativeMockTx {
    fn execute<'a>(
        &'a mut self,
        _sql: &'a str,
        _params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<u64, Error>> {
        Box::pin(async move { Ok(1) })
    }

    fn query<'a>(
        &'a mut self,
        _sql: &'a str,
        _params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<Vec<Row>, Error>> {
        Box::pin(async move { Ok(vec![]) })
    }

    fn commit<'a>(&'a mut self) -> BoxFuture<'a, Result<(), Error>> {
        Box::pin(async move { Ok(()) })
    }

    fn rollback<'a>(&'a mut self) -> BoxFuture<'a, Result<(), Error>> {
        Box::pin(async move { Ok(()) })
    }
}

#[tokio::test]
async fn test_third_party_driver_typed_and_registry() {
    let driver = NativeMockDriver;
    let options = ConnectOptions::new("sqlite::memory:").with_engine("native-mock");

    // 1. Works through typed dispatch
    let mut typed_conn = Connection::open_typed(&driver, &options)
        .await
        .expect("open typed third-party");
    let affected = typed_conn
        .execute("SELECT 1", &params![])
        .await
        .expect("execute typed");
    assert_eq!(affected, 1);
    let rows = typed_conn
        .query("SELECT 1", &params![])
        .await
        .expect("query typed");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<i64>("result").unwrap(), 123);

    // 2. Works through registry / dyn dispatch
    register_driver(driver);
    let mut dyn_conn = Connection::connect(&options)
        .await
        .expect("connect via registry");
    let affected_dyn = dyn_conn
        .execute("SELECT 1", &params![])
        .await
        .expect("execute dyn");
    assert_eq!(affected_dyn, 1);
    let rows_dyn = dyn_conn
        .query("SELECT 1", &params![])
        .await
        .expect("query dyn");
    assert_eq!(rows_dyn.len(), 1);
    assert_eq!(rows_dyn[0].get::<i64>("result").unwrap(), 123);
}

// ---------------------------------------------------------------------------
// Memory Driver tests: typed vs dyn parity
// ---------------------------------------------------------------------------

#[cfg(feature = "memory")]
#[tokio::test]
async fn test_memory_typed_vs_dyn_parity() {
    use portadb::drivers::MemoryDriver;

    // Run identical scenario on typed Connection and on dyn Connection
    let typed_driver = MemoryDriver::new();
    let typed_opts = ConnectOptions::new("memory://typed_test");
    let mut typed = Connection::open_typed(&typed_driver, &typed_opts)
        .await
        .expect("open typed memory");

    let dyn_driver = MemoryDriver::new();
    let dyn_opts = ConnectOptions::new("memory://dyn_test");
    let mut dyn_conn = Connection::open(dyn_driver, &dyn_opts)
        .await
        .expect("open dyn memory");

    // 1. CREATE TABLE
    let ddl = "CREATE TABLE people (id INTEGER PRIMARY KEY, name TEXT);";
    typed.execute(ddl, &params![]).await.expect("typed create");
    dyn_conn.execute(ddl, &params![]).await.expect("dyn create");

    // 2. INSERT rows
    let insert_sql = "INSERT INTO people (id, name) VALUES (:id, :name);";
    for (id, name) in [(1i64, "Alice"), (2i64, "Bob"), (3i64, "Charlie")] {
        let p = params![("id", id), ("name", name)];
        typed.execute(insert_sql, &p).await.expect("typed insert");
        dyn_conn.execute(insert_sql, &p).await.expect("dyn insert");
    }

    // 3. Query
    let q = "SELECT id, name FROM people ORDER BY id;";
    let typed_rows = typed.query(q, &params![]).await.expect("typed query");
    let dyn_rows = dyn_conn.query(q, &params![]).await.expect("dyn query");
    assert_eq!(typed_rows.len(), 3);
    assert_eq!(typed_rows, dyn_rows);

    // 4. query_each with early Break
    let mut typed_break_count = 0usize;
    let typed_handed = typed
        .query_each(q, &params![], |row| {
            typed_break_count += 1;
            let id: i64 = row.get("id").unwrap();
            if id >= 2 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .await
        .expect("typed query_each");

    let mut dyn_break_count = 0usize;
    let dyn_handed = dyn_conn
        .query_each(q, &params![], |row| {
            dyn_break_count += 1;
            let id: i64 = row.get("id").unwrap();
            if id >= 2 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .await
        .expect("dyn query_each");

    assert_eq!(typed_handed, dyn_handed);
    assert_eq!(typed_break_count, 2);
    assert_eq!(typed_break_count, dyn_break_count);

    // 5. query_as tuple
    let typed_tuples: Vec<(i64, String)> = typed
        .query_as(q, &params![])
        .await
        .expect("typed query_as tuple");
    let dyn_tuples: Vec<(i64, String)> = dyn_conn
        .query_as(q, &params![])
        .await
        .expect("dyn query_as tuple");
    assert_eq!(typed_tuples, dyn_tuples);
    assert_eq!(
        typed_tuples,
        vec![
            (1, "Alice".to_string()),
            (2, "Bob".to_string()),
            (3, "Charlie".to_string())
        ]
    );

    // 6. query_as Person struct
    let typed_people: Vec<Person> = typed
        .query_as(q, &params![])
        .await
        .expect("typed query_as struct");
    let dyn_people: Vec<Person> = dyn_conn
        .query_as(q, &params![])
        .await
        .expect("dyn query_as struct");
    assert_eq!(typed_people, dyn_people);
    assert_eq!(typed_people.len(), 3);

    // 7. Transaction commit
    {
        let mut tx_t = typed.begin().await.expect("typed begin");
        tx_t.execute(insert_sql, &params![("id", 4i64), ("name", "David")])
            .await
            .expect("typed tx insert");
        tx_t.commit().await.expect("typed commit");

        let mut tx_d = dyn_conn.begin().await.expect("dyn begin");
        tx_d.execute(insert_sql, &params![("id", 4i64), ("name", "David")])
            .await
            .expect("dyn tx insert");
        tx_d.commit().await.expect("dyn commit");
    }

    let typed_after_commit: Vec<(i64, String)> = typed
        .query_as(q, &params![])
        .await
        .expect("query after commit");
    let dyn_after_commit: Vec<(i64, String)> = dyn_conn
        .query_as(q, &params![])
        .await
        .expect("query after commit");
    assert_eq!(typed_after_commit, dyn_after_commit);
    assert_eq!(typed_after_commit.len(), 4);

    // 8. Transaction rollback
    {
        let mut tx_t = typed.begin().await.expect("typed begin");
        tx_t.execute(insert_sql, &params![("id", 5i64), ("name", "Eve")])
            .await
            .expect("typed tx insert");
        tx_t.rollback().await.expect("typed rollback");

        let mut tx_d = dyn_conn.begin().await.expect("dyn begin");
        tx_d.execute(insert_sql, &params![("id", 5i64), ("name", "Eve")])
            .await
            .expect("dyn tx insert");
        tx_d.rollback().await.expect("dyn rollback");
    }

    let typed_after_rollback: Vec<(i64, String)> = typed
        .query_as(q, &params![])
        .await
        .expect("query after rollback");
    let dyn_after_rollback: Vec<(i64, String)> = dyn_conn
        .query_as(q, &params![])
        .await
        .expect("query after rollback");
    assert_eq!(typed_after_rollback, dyn_after_rollback);
    assert_eq!(typed_after_rollback.len(), 4); // Still 4, Eve was rolled back
}

// ---------------------------------------------------------------------------
// Rusqlite Driver tests: typed vs dyn parity
// ---------------------------------------------------------------------------

/// A fresh database file per connection: the rusqlite driver shares one
/// in-memory database between every `:memory:` connection, so two of them
/// would see each other's tables.
#[cfg(feature = "rusqlite")]
fn scratch_db(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("portadb_{name}_{}.db", std::process::id()));
    let _ = std::fs::remove_file(&path);
    path
}

#[cfg(feature = "rusqlite")]
#[tokio::test]
async fn test_rusqlite_typed_vs_dyn_parity() {
    use portadb::drivers::RusqliteDriver;

    let typed_driver = RusqliteDriver::new();
    let typed_path = scratch_db("rusqlite_typed");
    let typed_opts = ConnectOptions::new(typed_path.to_string_lossy().into_owned());
    let mut typed = Connection::open_typed(&typed_driver, &typed_opts)
        .await
        .expect("open typed rusqlite");

    let dyn_driver = RusqliteDriver::new();
    let dyn_path = scratch_db("rusqlite_dyn");
    let dyn_opts = ConnectOptions::new(dyn_path.to_string_lossy().into_owned());
    let mut dyn_conn = Connection::open(dyn_driver, &dyn_opts)
        .await
        .expect("open dyn rusqlite");

    // 1. CREATE TABLE
    let ddl = "CREATE TABLE people (id INTEGER PRIMARY KEY, name TEXT);";
    typed.execute(ddl, &params![]).await.expect("typed create");
    dyn_conn.execute(ddl, &params![]).await.expect("dyn create");

    // 2. INSERT rows
    let insert_sql = "INSERT INTO people (id, name) VALUES (:id, :name);";
    for (id, name) in [(1i64, "Alice"), (2i64, "Bob"), (3i64, "Charlie")] {
        let p = params![("id", id), ("name", name)];
        typed.execute(insert_sql, &p).await.expect("typed insert");
        dyn_conn.execute(insert_sql, &p).await.expect("dyn insert");
    }

    // 3. Query
    let q = "SELECT id, name FROM people ORDER BY id;";
    let typed_rows = typed.query(q, &params![]).await.expect("typed query");
    let dyn_rows = dyn_conn.query(q, &params![]).await.expect("dyn query");
    assert_eq!(typed_rows.len(), 3);
    assert_eq!(typed_rows, dyn_rows);

    // 4. query_each with early Break
    let mut typed_break_count = 0usize;
    let typed_handed = typed
        .query_each(q, &params![], |row| {
            typed_break_count += 1;
            let id: i64 = row.get("id").unwrap();
            if id >= 2 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .await
        .expect("typed query_each");

    let mut dyn_break_count = 0usize;
    let dyn_handed = dyn_conn
        .query_each(q, &params![], |row| {
            dyn_break_count += 1;
            let id: i64 = row.get("id").unwrap();
            if id >= 2 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .await
        .expect("dyn query_each");

    assert_eq!(typed_handed, dyn_handed);
    assert_eq!(typed_break_count, 2);
    assert_eq!(typed_break_count, dyn_break_count);

    // 5. query_as tuple
    let typed_tuples: Vec<(i64, String)> = typed
        .query_as(q, &params![])
        .await
        .expect("typed query_as tuple");
    let dyn_tuples: Vec<(i64, String)> = dyn_conn
        .query_as(q, &params![])
        .await
        .expect("dyn query_as tuple");
    assert_eq!(typed_tuples, dyn_tuples);
    assert_eq!(
        typed_tuples,
        vec![
            (1, "Alice".to_string()),
            (2, "Bob".to_string()),
            (3, "Charlie".to_string())
        ]
    );

    // 6. query_as Person struct
    let typed_people: Vec<Person> = typed
        .query_as(q, &params![])
        .await
        .expect("typed query_as struct");
    let dyn_people: Vec<Person> = dyn_conn
        .query_as(q, &params![])
        .await
        .expect("dyn query_as struct");
    assert_eq!(typed_people, dyn_people);
    assert_eq!(typed_people.len(), 3);

    // 7. Transaction commit
    {
        let mut tx_t = typed.begin().await.expect("typed begin");
        tx_t.execute(insert_sql, &params![("id", 4i64), ("name", "David")])
            .await
            .expect("typed tx insert");
        tx_t.commit().await.expect("typed commit");

        let mut tx_d = dyn_conn.begin().await.expect("dyn begin");
        tx_d.execute(insert_sql, &params![("id", 4i64), ("name", "David")])
            .await
            .expect("dyn tx insert");
        tx_d.commit().await.expect("dyn commit");
    }

    let typed_after_commit: Vec<(i64, String)> = typed
        .query_as(q, &params![])
        .await
        .expect("query after commit");
    let dyn_after_commit: Vec<(i64, String)> = dyn_conn
        .query_as(q, &params![])
        .await
        .expect("query after commit");
    assert_eq!(typed_after_commit, dyn_after_commit);
    assert_eq!(typed_after_commit.len(), 4);

    // 8. Transaction rollback
    {
        let mut tx_t = typed.begin().await.expect("typed begin");
        tx_t.execute(insert_sql, &params![("id", 5i64), ("name", "Eve")])
            .await
            .expect("typed tx insert");
        tx_t.rollback().await.expect("typed rollback");

        let mut tx_d = dyn_conn.begin().await.expect("dyn begin");
        tx_d.execute(insert_sql, &params![("id", 5i64), ("name", "Eve")])
            .await
            .expect("dyn tx insert");
        tx_d.rollback().await.expect("dyn rollback");
    }

    let typed_after_rollback: Vec<(i64, String)> = typed
        .query_as(q, &params![])
        .await
        .expect("query after rollback");
    let dyn_after_rollback: Vec<(i64, String)> = dyn_conn
        .query_as(q, &params![])
        .await
        .expect("query after rollback");
    assert_eq!(typed_after_rollback, dyn_after_rollback);
    assert_eq!(typed_after_rollback.len(), 4);
    drop(typed);
    drop(dyn_conn);
    let _ = std::fs::remove_file(&typed_path);
    let _ = std::fs::remove_file(&dyn_path);
}

// ---------------------------------------------------------------------------
// Sqlx-sqlite Driver tests: typed vs dyn parity
// ---------------------------------------------------------------------------

#[cfg(feature = "sqlx-sqlite")]
#[tokio::test]
async fn test_sqlx_sqlite_typed_vs_dyn_parity() {
    use portadb::drivers::SqlxDriver;

    let typed_driver = SqlxDriver::new();
    let typed_opts = ConnectOptions::new("sqlite:file:sqlx_typed_test?mode=memory&cache=shared");
    let mut typed = Connection::open_typed(&typed_driver, &typed_opts)
        .await
        .expect("open typed sqlx");

    let dyn_driver = SqlxDriver::new();
    let dyn_opts = ConnectOptions::new("sqlite:file:sqlx_dyn_test?mode=memory&cache=shared");
    let mut dyn_conn = Connection::open(dyn_driver, &dyn_opts)
        .await
        .expect("open dyn sqlx");

    // 1. CREATE TABLE
    let ddl = "CREATE TABLE people (id INTEGER PRIMARY KEY, name TEXT);";
    typed.execute(ddl, &params![]).await.expect("typed create");
    dyn_conn.execute(ddl, &params![]).await.expect("dyn create");

    // 2. INSERT rows
    let insert_sql = "INSERT INTO people (id, name) VALUES (:id, :name);";
    for (id, name) in [(1i64, "Alice"), (2i64, "Bob"), (3i64, "Charlie")] {
        let p = params![("id", id), ("name", name)];
        typed.execute(insert_sql, &p).await.expect("typed insert");
        dyn_conn.execute(insert_sql, &p).await.expect("dyn insert");
    }

    // 3. Query
    let q = "SELECT id, name FROM people ORDER BY id;";
    let typed_rows = typed.query(q, &params![]).await.expect("typed query");
    let dyn_rows = dyn_conn.query(q, &params![]).await.expect("dyn query");
    assert_eq!(typed_rows.len(), 3);
    assert_eq!(typed_rows, dyn_rows);

    // 4. query_each with early Break
    let mut typed_break_count = 0usize;
    let typed_handed = typed
        .query_each(q, &params![], |row| {
            typed_break_count += 1;
            let id: i64 = row.get("id").unwrap();
            if id >= 2 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .await
        .expect("typed query_each");

    let mut dyn_break_count = 0usize;
    let dyn_handed = dyn_conn
        .query_each(q, &params![], |row| {
            dyn_break_count += 1;
            let id: i64 = row.get("id").unwrap();
            if id >= 2 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .await
        .expect("dyn query_each");

    assert_eq!(typed_handed, dyn_handed);
    assert_eq!(typed_break_count, 2);
    assert_eq!(typed_break_count, dyn_break_count);

    // 5. query_as tuple
    let typed_tuples: Vec<(i64, String)> = typed
        .query_as(q, &params![])
        .await
        .expect("typed query_as tuple");
    let dyn_tuples: Vec<(i64, String)> = dyn_conn
        .query_as(q, &params![])
        .await
        .expect("dyn query_as tuple");
    assert_eq!(typed_tuples, dyn_tuples);
    assert_eq!(
        typed_tuples,
        vec![
            (1, "Alice".to_string()),
            (2, "Bob".to_string()),
            (3, "Charlie".to_string())
        ]
    );

    // 6. query_as Person struct
    let typed_people: Vec<Person> = typed
        .query_as(q, &params![])
        .await
        .expect("typed query_as struct");
    let dyn_people: Vec<Person> = dyn_conn
        .query_as(q, &params![])
        .await
        .expect("dyn query_as struct");
    assert_eq!(typed_people, dyn_people);
    assert_eq!(typed_people.len(), 3);

    // 7. Transaction commit
    {
        let mut tx_t = typed.begin().await.expect("typed begin");
        tx_t.execute(insert_sql, &params![("id", 4i64), ("name", "David")])
            .await
            .expect("typed tx insert");
        tx_t.commit().await.expect("typed commit");

        let mut tx_d = dyn_conn.begin().await.expect("dyn begin");
        tx_d.execute(insert_sql, &params![("id", 4i64), ("name", "David")])
            .await
            .expect("dyn tx insert");
        tx_d.commit().await.expect("dyn commit");
    }

    let typed_after_commit: Vec<(i64, String)> = typed
        .query_as(q, &params![])
        .await
        .expect("query after commit");
    let dyn_after_commit: Vec<(i64, String)> = dyn_conn
        .query_as(q, &params![])
        .await
        .expect("query after commit");
    assert_eq!(typed_after_commit, dyn_after_commit);
    assert_eq!(typed_after_commit.len(), 4);

    // 8. Transaction rollback
    {
        let mut tx_t = typed.begin().await.expect("typed begin");
        tx_t.execute(insert_sql, &params![("id", 5i64), ("name", "Eve")])
            .await
            .expect("typed tx insert");
        tx_t.rollback().await.expect("typed rollback");

        let mut tx_d = dyn_conn.begin().await.expect("dyn begin");
        tx_d.execute(insert_sql, &params![("id", 5i64), ("name", "Eve")])
            .await
            .expect("dyn tx insert");
        tx_d.rollback().await.expect("dyn rollback");
    }

    let typed_after_rollback: Vec<(i64, String)> = typed
        .query_as(q, &params![])
        .await
        .expect("query after rollback");
    let dyn_after_rollback: Vec<(i64, String)> = dyn_conn
        .query_as(q, &params![])
        .await
        .expect("query after rollback");
    assert_eq!(typed_after_rollback, dyn_after_rollback);
    assert_eq!(typed_after_rollback.len(), 4);
}
