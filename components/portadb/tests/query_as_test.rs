use std::ops::ControlFlow;

use portadb::row_ref::column_index;
use portadb::{
    ConnectOptions, Connection, Database, Error, FromRow, NamedParams, RowRef, Value, decode,
};

#[derive(Debug, PartialEq, Eq)]
struct Person {
    id: i64,
    name: String,
    bio: Option<String>,
}

struct PersonPlan {
    id_idx: usize,
    name_idx: usize,
    bio_idx: usize,
}

impl FromRow for Person {
    type Plan = PersonPlan;

    fn plan(columns: &[String]) -> Result<Self::Plan, Error> {
        let id_idx = column_index(columns, "id")?;
        let name_idx = column_index(columns, "name")?;
        let bio_idx = column_index(columns, "bio")?;
        Ok(PersonPlan {
            id_idx,
            name_idx,
            bio_idx,
        })
    }

    fn from_row<R: RowRef + ?Sized>(plan: &Self::Plan, row: &R) -> Result<Self, Error> {
        let id = decode(row, plan.id_idx)?;
        let name = decode(row, plan.name_idx)?;
        let bio = decode(row, plan.bio_idx)?;
        Ok(Self { id, name, bio })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct StrictPerson {
    id: i64,
    bio: String,
}

struct StrictPersonPlan {
    id_idx: usize,
    bio_idx: usize,
}

impl FromRow for StrictPerson {
    type Plan = StrictPersonPlan;

    fn plan(columns: &[String]) -> Result<Self::Plan, Error> {
        let id_idx = column_index(columns, "id")?;
        let bio_idx = column_index(columns, "bio")?;
        Ok(StrictPersonPlan { id_idx, bio_idx })
    }

    fn from_row<R: RowRef + ?Sized>(plan: &Self::Plan, row: &R) -> Result<Self, Error> {
        let id = decode(row, plan.id_idx)?;
        let bio = decode(row, plan.bio_idx)?;
        Ok(Self { id, bio })
    }
}

/// Needs a column no test table has.
#[derive(Debug)]
struct WithEmail;

impl FromRow for WithEmail {
    type Plan = usize;

    fn plan(columns: &[String]) -> Result<Self::Plan, Error> {
        column_index(columns, "email")
    }

    fn from_row<R: RowRef + ?Sized>(_plan: &Self::Plan, _row: &R) -> Result<Self, Error> {
        Ok(WithEmail)
    }
}

async fn verify_engine_query_as(mut conn: Connection) {
    conn.execute(
        "CREATE TABLE users (id INTEGER, name TEXT, bio TEXT);",
        &NamedParams::new(),
    )
    .await
    .expect("create table");

    let mut p1 = NamedParams::new();
    p1.insert("id", Value::Integer(1));
    p1.insert("name", Value::Text("Alice".to_string()));
    p1.insert("bio", Value::Text("developer".to_string()));
    conn.execute(
        "INSERT INTO users (id, name, bio) VALUES (:id, :name, :bio);",
        &p1,
    )
    .await
    .expect("insert alice");

    let mut p2 = NamedParams::new();
    p2.insert("id", Value::Integer(2));
    p2.insert("name", Value::Text("Bob".to_string()));
    p2.insert("bio", Value::Null);
    conn.execute(
        "INSERT INTO users (id, name, bio) VALUES (:id, :name, :bio);",
        &p2,
    )
    .await
    .expect("insert bob");

    // 1. query_as matches query + row.get
    let raw_rows = conn
        .query(
            "SELECT id, name, bio FROM users ORDER BY id;",
            &NamedParams::new(),
        )
        .await
        .expect("query");
    let mut expected_people = Vec::new();
    for r in &raw_rows {
        expected_people.push(Person {
            id: r.get("id").expect("id"),
            name: r.get("name").expect("name"),
            bio: r.get_opt("bio").expect("bio"),
        });
    }

    let people: Vec<Person> = conn
        .query_as(
            "SELECT id, name, bio FROM users ORDER BY id;",
            &NamedParams::new(),
        )
        .await
        .expect("query_as people");
    assert_eq!(people, expected_people);
    assert_eq!(people.len(), 2);
    assert_eq!(people[0].bio, Some("developer".to_string()));
    assert_eq!(people[1].bio, None);

    // Case-insensitive column resolution in hand-written FromRow
    let people_upper: Vec<Person> = conn
        .query_as(
            "SELECT ID, NAME, BIO FROM users ORDER BY ID;",
            &NamedParams::new(),
        )
        .await
        .expect("query_as upper columns");
    assert_eq!(people_upper, expected_people);

    // Tuple decoding matches query + row.get
    let tuples: Vec<(i64, String)> = conn
        .query_as(
            "SELECT id, name FROM users ORDER BY id;",
            &NamedParams::new(),
        )
        .await
        .expect("query_as tuple");
    assert_eq!(
        tuples,
        vec![(1, "Alice".to_string()), (2, "Bob".to_string())]
    );

    // 2. Missing column -> ColumnNotFound
    // (asks for a column the table does not have: the memory driver ignores
    // the SELECT list, so leaving one out of the query would not be missing there)
    let err_missing = conn
        .query_as::<WithEmail>("SELECT id, name FROM users;", &NamedParams::new())
        .await
        .unwrap_err();
    assert!(matches!(err_missing, Error::ColumnNotFound { ref name } if name == "email"));

    // 3. NULL into non-Option -> NullValue
    let err_null = conn
        .query_as::<StrictPerson>(
            "SELECT id, bio FROM users ORDER BY id;",
            &NamedParams::new(),
        )
        .await
        .unwrap_err();
    assert!(matches!(err_null, Error::NullValue { ref column } if column == "bio"));

    // 4. Tuple wider than result -> Error::QueryError
    let err_tuple = conn
        .query_as::<(i64, String, String, String)>(
            "SELECT id, name FROM users;",
            &NamedParams::new(),
        )
        .await
        .unwrap_err();
    assert!(matches!(err_tuple, Error::QueryError(_)));

    // 5. Zero rows -> empty without plan failure
    let empty_vec: Vec<Person> = conn
        .query_as(
            "SELECT id, name, bio FROM users WHERE id = 9999;",
            &NamedParams::new(),
        )
        .await
        .expect("empty query_as");
    assert!(empty_vec.is_empty());

    let mut empty_callback_count = 0;
    let empty_count = conn
        .query_each_as::<Person>(
            "SELECT id, name, bio FROM users WHERE id = 9999;",
            &NamedParams::new(),
            |_| {
                empty_callback_count += 1;
                ControlFlow::Continue(())
            },
        )
        .await
        .expect("empty query_each_as");
    assert_eq!(empty_count, 0);
    assert_eq!(empty_callback_count, 0);

    // 6. Early Break stops
    let mut callback_count = 0;
    let mut collected = Vec::new();
    let handed = conn
        .query_each_as::<Person>(
            "SELECT id, name, bio FROM users ORDER BY id;",
            &NamedParams::new(),
            |p| {
                callback_count += 1;
                collected.push(p);
                ControlFlow::Break(())
            },
        )
        .await
        .expect("query_each_as early break");
    assert_eq!(handed, 1);
    assert_eq!(callback_count, 1);
    assert_eq!(collected.len(), 1);
    assert_eq!(collected[0].id, 1);

    // 7. An Err from decoding stops and is returned
    let mut decoded_before_err = 0;
    let err_decoding = conn
        .query_each_as::<StrictPerson>(
            "SELECT id, bio FROM users ORDER BY id;",
            &NamedParams::new(),
            |_| {
                decoded_before_err += 1;
                ControlFlow::Continue(())
            },
        )
        .await
        .unwrap_err();
    assert_eq!(decoded_before_err, 1);
    assert!(matches!(err_decoding, Error::NullValue { ref column } if column == "bio"));
}

#[cfg(feature = "memory")]
#[tokio::test]
async fn test_query_as_memory() {
    let options = ConnectOptions::new("memory://test_query_as_mem")
        .with_database(Database::Sqlite)
        .with_engine("memory");
    let conn = Connection::connect(&options)
        .await
        .expect("connect memory driver");
    verify_engine_query_as(conn).await;
}

#[cfg(feature = "rusqlite")]
#[tokio::test]
async fn test_query_as_rusqlite() {
    let conn = Connection::open(
        portadb::RusqliteDriver::new(),
        &ConnectOptions::new(":memory:"),
    )
    .await
    .expect("connect rusqlite");
    verify_engine_query_as(conn).await;
}

#[cfg(feature = "sqlx-sqlite")]
#[tokio::test]
async fn test_query_as_sqlx_sqlite() {
    let conn = Connection::open(
        portadb::SqlxDriver::new(),
        &ConnectOptions::new("sqlite::memory:"),
    )
    .await
    .expect("connect sqlx-sqlite");
    verify_engine_query_as(conn).await;
}

#[cfg(feature = "sqlx-postgres")]
#[test]
fn test_sqlx_postgres_compile_check() {
    // Compile-time check ensuring DriverConnection and query_as / query_each_as work with sqlx-postgres types
    fn assert_driver_conn<T: portadb::DriverConnection>() {}
    assert_driver_conn::<portadb::drivers::SqlxPostgresConnection>();

    async fn _compile_query_as(mut conn: Connection) {
        let _ = conn
            .query_as::<Person>("SELECT id, name, bio FROM users", &NamedParams::new())
            .await;
        let _ = conn
            .query_each_as::<Person>(
                "SELECT id, name, bio FROM users",
                &NamedParams::new(),
                |_| ControlFlow::Continue(()),
            )
            .await;
    }
}
