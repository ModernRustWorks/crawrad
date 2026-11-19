use std::any::Any;

use portadb::{
    BoxFuture, ConnectOptions, Database, Driver, DriverConnection, DriverTransaction, Error,
    NamedParams, Row,
};

struct MockDriver;

impl Driver for MockDriver {
    type Connection = MockConn;

    fn name(&self) -> &'static str {
        "mock"
    }

    fn supports_database(&self, database: Database) -> bool {
        matches!(database, Database::Sqlite)
    }

    async fn connect(&self, _options: &ConnectOptions) -> Result<Self::Connection, Error> {
        Ok(MockConn)
    }
}

struct MockConn;

impl DriverConnection for MockConn {
    async fn execute(&mut self, _sql: &str, _params: &NamedParams) -> Result<u64, Error> {
        Ok(42)
    }

    async fn query(&mut self, _sql: &str, _params: &NamedParams) -> Result<Vec<Row>, Error> {
        Ok(vec![])
    }

    async fn begin_transaction(&mut self) -> Result<Box<dyn DriverTransaction + '_>, Error> {
        let tx: Box<dyn DriverTransaction + '_> = Box::new(MockTx);
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

struct MockTx;

impl DriverTransaction for MockTx {
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
async fn test_driver_contract() {
    let driver = MockDriver;
    assert_eq!(driver.name(), "mock");
    assert!(driver.supports_database(Database::Sqlite));
    assert!(!driver.supports_database(Database::Postgres));

    let options = ConnectOptions::new(":memory:");
    let mut conn = driver.connect(&options).await.unwrap();

    let count = conn.execute("SELECT 1", &NamedParams::new()).await.unwrap();
    assert_eq!(count, 42);

    let rows = conn.query("SELECT 1", &NamedParams::new()).await.unwrap();
    assert!(rows.is_empty());

    let mut tx = conn.begin_transaction().await.unwrap();
    assert_eq!(
        tx.execute("UPDATE t SET x = 1", &NamedParams::new())
            .await
            .unwrap(),
        1
    );
    tx.commit().await.unwrap();
    drop(tx);

    conn.close().await.unwrap();
}
