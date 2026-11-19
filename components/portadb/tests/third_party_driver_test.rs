use std::any::Any;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use portadb::{
    BoxFuture, ConnectOptions, Database, Driver, DriverConnection, DriverTransaction, Engine,
    Error, NamedParams, Row, Value, params, register_driver,
};

/// A custom third-party database driver implemented completely outside the `portadb` crate.
///
/// Proves Rule 6 of the challenge:
/// "Acrescentar o terceiro engine tem de custar CRIAR UM ARQUIVO, não editar um match central."
pub struct CustomMockDriver;

impl Driver for CustomMockDriver {
    type Connection = CustomMockConnection;

    fn name(&self) -> &'static str {
        "custom-mock"
    }

    fn supports_database(&self, database: Database) -> bool {
        matches!(database, Database::Sqlite | Database::Postgres)
    }

    async fn connect(&self, _options: &ConnectOptions) -> Result<Self::Connection, Error> {
        Ok(CustomMockConnection::default())
    }
}

#[derive(Default)]
pub struct CustomMockConnection {
    execute_count: Arc<AtomicU64>,
}

impl DriverConnection for CustomMockConnection {
    async fn execute(&mut self, _sql: &str, _params: &NamedParams) -> Result<u64, Error> {
        let prev = self.execute_count.fetch_add(1, Ordering::SeqCst);
        Ok(prev + 1)
    }

    async fn query(&mut self, _sql: &str, params: &NamedParams) -> Result<Vec<Row>, Error> {
        let name_val = params
            .get("name")
            .cloned()
            .unwrap_or(Value::Text("Mocked User".into()));
        let row = Row::new(
            vec!["id".to_string(), "name".to_string()],
            vec![Value::Integer(42), name_val],
        );
        Ok(vec![row])
    }

    async fn begin_transaction(&mut self) -> Result<Box<dyn DriverTransaction + '_>, Error> {
        let tx: Box<dyn DriverTransaction + '_> = Box::new(CustomMockTransaction);
        Ok(tx)
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

pub struct CustomMockTransaction;

impl DriverTransaction for CustomMockTransaction {
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
        Box::pin(async move { Ok(Vec::new()) })
    }

    fn commit<'a>(&'a mut self) -> BoxFuture<'a, Result<(), Error>> {
        Box::pin(async move { Ok(()) })
    }

    fn rollback<'a>(&'a mut self) -> BoxFuture<'a, Result<(), Error>> {
        Box::pin(async move { Ok(()) })
    }
}

#[tokio::test]
async fn test_third_party_driver_registration_without_core_modification() {
    // 1. Register third-party driver into the open SPI registry
    register_driver(CustomMockDriver);

    // 2. Connect specifying the new third-party engine in configuration
    let options = ConnectOptions::new("sqlite://app.db").with_engine("custom-mock");
    let engine = Engine::connect(&options)
        .await
        .expect("connect using third-party driver");

    assert_eq!(engine.driver_name(), "custom-mock");

    // 3. Execute queries through the standard abstraction
    let affected = engine
        .execute(
            "INSERT INTO users (name) VALUES (:name)",
            &params![("name", "Bob")],
        )
        .await
        .expect("execute");
    assert_eq!(affected, 1);

    // 4. Query through standard abstraction
    let rows = engine
        .query(
            "SELECT id, name FROM users WHERE name = :name",
            &params![("name", "Bob")],
        )
        .await
        .expect("query");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<i64>("id").unwrap(), 42);
    assert_eq!(rows[0].get::<String>("name").unwrap(), "Bob");
}
