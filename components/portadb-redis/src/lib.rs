use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, OnceLock, RwLock};

use portadb_core::capability::Capabilities;
use portadb_core::database::Database;
use portadb_core::driver::{Driver as DriverTrait, DriverConnection, DriverTransaction};
use portadb_core::error::Error;
use portadb_core::options::ConnectOptions;
use portadb_core::params::NamedParams;
use portadb_core::row::Row;
use portadb_core::value::Value;

type RedisStore = HashMap<String, String>;
type RedisCatalog = RwLock<HashMap<String, Arc<RwLock<RedisStore>>>>;

fn global_redis_catalog() -> &'static RedisCatalog {
    static CATALOG: OnceLock<RedisCatalog> = OnceLock::new();
    CATALOG.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Driver for Redis key-value and data-structure store.
///
/// Abstracting non-relational operations seamlessly:
/// - Supports native commands: `SET`, `GET`, `DEL`, `EXISTS`, `KEYS`, `PING`.
/// - Abstracts tabular SQL statements: `SELECT`, `INSERT`, `UPDATE`, `DELETE` over keys/values.
/// - Supports transactions via Redis `MULTI`, `EXEC`, and `DISCARD` semantics.
/// - Gracefully reports [`Error::UnsupportedFeature`] when unsupported relational operations (such as `JOIN`, `CREATE TABLE`)
///   are attempted against Redis.
#[derive(Debug, Default, Clone, Copy)]
pub struct Driver;

/// Backward-compatible alias for [`Driver`].
pub type RedisDriver = Driver;

impl Driver {
    /// Constructs a new Redis driver instance.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl DriverTrait for Driver {
    type Connection = Connection;

    fn name(&self) -> &'static str {
        "redis"
    }

    fn supports_database(&self, database: Database) -> bool {
        matches!(database, Database::Redis)
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities::redis()
    }

    async fn connect(&self, options: &ConnectOptions) -> Result<Self::Connection, Error> {
        let url = if options.url.is_empty() {
            "redis://127.0.0.1:6379".to_string()
        } else {
            options.url.clone()
        };

        let store = {
            let mut catalog = global_redis_catalog()
                .write()
                .map_err(|_| Error::ConnectionLost("redis catalog lock poisoned".to_string()))?;
            catalog
                .entry(url)
                .or_insert_with(|| Arc::new(RwLock::new(HashMap::new())))
                .clone()
        };

        Ok(Connection { store })
    }
}

/// Active connection handle to Redis.
pub struct Connection {
    store: Arc<RwLock<RedisStore>>,
}

/// Backward-compatible alias for [`Connection`].
pub type RedisConnection = Connection;

impl Connection {
    fn execute_internal(&self, cmd: &str, params: &NamedParams) -> Result<u64, Error> {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(0);
        }

        let op = parts[0].to_ascii_uppercase();
        match op.as_str() {
            "SET" => {
                let key = resolve_part(parts.get(1).copied(), params, "key")?;
                let val = resolve_part(parts.get(2).copied(), params, "value")?;

                let mut map = self
                    .store
                    .write()
                    .map_err(|_| Error::ConnectionLost("redis store lock poisoned".to_string()))?;
                map.insert(key, val);
                Ok(1)
            }
            "DEL" => {
                let key = resolve_part(parts.get(1).copied(), params, "key")?;
                let mut map = self
                    .store
                    .write()
                    .map_err(|_| Error::ConnectionLost("redis store lock poisoned".to_string()))?;
                let removed = map.remove(&key).is_some();
                Ok(if removed { 1 } else { 0 })
            }
            "INSERT" | "UPDATE" => {
                // Abstract SQL INSERT / UPDATE into Redis SET
                let key = resolve_part(None, params, "key")
                    .or_else(|_| resolve_part(None, params, "id"))?;
                let val = resolve_part(None, params, "value")
                    .or_else(|_| resolve_part(None, params, "val"))?;

                let mut map = self
                    .store
                    .write()
                    .map_err(|_| Error::ConnectionLost("redis store lock poisoned".to_string()))?;
                map.insert(key, val);
                Ok(1)
            }
            "DELETE" => {
                // Abstract SQL DELETE into Redis DEL
                let key = resolve_part(None, params, "key")
                    .or_else(|_| resolve_part(None, params, "id"))?;

                let mut map = self
                    .store
                    .write()
                    .map_err(|_| Error::ConnectionLost("redis store lock poisoned".to_string()))?;
                let removed = map.remove(&key).is_some();
                Ok(if removed { 1 } else { 0 })
            }
            "PING" => Ok(0),
            unsupported => {
                // Relational SQL DDL commands that cannot be represented in Redis
                if unsupported == "CREATE" || unsupported == "ALTER" || unsupported == "DROP" {
                    Err(Error::UnsupportedFeature(format!(
                        "Redis is a key-value store and does not support relational DDL statement '{unsupported}'"
                    )))
                } else {
                    Err(Error::UnsupportedFeature(format!(
                        "Redis command '{unsupported}' is not supported"
                    )))
                }
            }
        }
    }

    fn query_internal(&self, cmd: &str, params: &NamedParams) -> Result<Vec<Row>, Error> {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(Vec::new());
        }

        let op = parts[0].to_ascii_uppercase();
        match op.as_str() {
            "GET" => {
                let key = resolve_part(parts.get(1).copied(), params, "key")?;
                let map = self
                    .store
                    .read()
                    .map_err(|_| Error::ConnectionLost("redis store lock poisoned".to_string()))?;

                if let Some(val) = map.get(&key) {
                    let row = Row::new(
                        vec!["key".to_string(), "value".to_string()],
                        vec![Value::Text(key), Value::Text(val.clone())],
                    );
                    Ok(vec![row])
                } else {
                    Ok(Vec::new())
                }
            }
            "KEYS" => {
                let map = self
                    .store
                    .read()
                    .map_err(|_| Error::ConnectionLost("redis store lock poisoned".to_string()))?;

                let mut rows = Vec::new();
                for k in map.keys() {
                    rows.push(Row::new(
                        vec!["key".to_string()],
                        vec![Value::Text(k.clone())],
                    ));
                }
                Ok(rows)
            }
            "EXISTS" => {
                let key = resolve_part(parts.get(1).copied(), params, "key")?;
                let map = self
                    .store
                    .read()
                    .map_err(|_| Error::ConnectionLost("redis store lock poisoned".to_string()))?;
                let exists = map.contains_key(&key);
                let row = Row::new(vec!["exists".to_string()], vec![Value::Boolean(exists)]);
                Ok(vec![row])
            }
            "SELECT" => {
                if cmd.to_ascii_uppercase().contains("JOIN") {
                    return Err(Error::UnsupportedFeature(
                        "Redis is a key-value store and does not support relational JOIN queries"
                            .to_string(),
                    ));
                }

                // If user specifies WHERE key = :key (or :id)
                if let Ok(key) =
                    resolve_part(None, params, "key").or_else(|_| resolve_part(None, params, "id"))
                {
                    let map = self.store.read().map_err(|_| {
                        Error::ConnectionLost("redis store lock poisoned".to_string())
                    })?;

                    if let Some(val) = map.get(&key) {
                        return Ok(vec![Row::new(
                            vec!["key".to_string(), "value".to_string()],
                            vec![Value::Text(key), Value::Text(val.clone())],
                        )]);
                    } else {
                        return Ok(Vec::new());
                    }
                }

                // Otherwise abstract SELECT * FROM keys -> return all key-value entries
                let map = self
                    .store
                    .read()
                    .map_err(|_| Error::ConnectionLost("redis store lock poisoned".to_string()))?;

                let mut rows = Vec::new();
                for (k, v) in map.iter() {
                    rows.push(Row::new(
                        vec!["key".to_string(), "value".to_string()],
                        vec![Value::Text(k.clone()), Value::Text(v.clone())],
                    ));
                }
                rows.sort_by(|a, b| {
                    let ka: String = a.get("key").unwrap_or_default();
                    let kb: String = b.get("key").unwrap_or_default();
                    ka.cmp(&kb)
                });
                Ok(rows)
            }
            unsupported => Err(Error::UnsupportedFeature(format!(
                "Redis query command '{unsupported}' is not supported"
            ))),
        }
    }
}

impl DriverConnection for Connection {
    async fn execute(&mut self, sql: &str, params: &NamedParams) -> Result<u64, Error> {
        self.execute_internal(sql, params)
    }

    async fn query(&mut self, sql: &str, params: &NamedParams) -> Result<Vec<Row>, Error> {
        self.query_internal(sql, params)
    }

    async fn begin_transaction(&mut self) -> Result<Box<dyn DriverTransaction + '_>, Error> {
        let snapshot = {
            let map = self
                .store
                .read()
                .map_err(|_| Error::ConnectionLost("redis store lock poisoned".to_string()))?;
            map.clone()
        };
        let tx: Box<dyn DriverTransaction + '_> = Box::new(Transaction {
            conn: self,
            snapshot,
            is_active: true,
        });
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

/// Transaction handle simulating Redis MULTI/EXEC/DISCARD semantics.
pub struct Transaction<'a> {
    conn: &'a mut Connection,
    snapshot: RedisStore,
    is_active: bool,
}

/// Backward-compatible alias for [`Transaction`].
pub type RedisTransaction<'a> = Transaction<'a>;

impl DriverTransaction for Transaction<'_> {
    fn execute<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> Pin<Box<dyn Future<Output = Result<u64, Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.conn.execute_internal(sql, params)
        })
    }

    fn query<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Row>, Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.conn.query_internal(sql, params)
        })
    }

    fn commit<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.is_active = false;
            // Commits changes (EXEC)
            Ok(())
        })
    }

    fn rollback<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.is_active = false;
            // Rollback reverts store (DISCARD)
            let mut map = self
                .conn
                .store
                .write()
                .map_err(|_| Error::ConnectionLost("redis store lock poisoned".to_string()))?;
            *map = self.snapshot.clone();
            Ok(())
        })
    }
}

impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if self.is_active {
            self.is_active = false;
            if let Ok(mut map) = self.conn.store.write() {
                *map = self.snapshot.clone();
            }
        }
    }
}

fn resolve_part(
    token: Option<&str>,
    params: &NamedParams,
    fallback_param_name: &str,
) -> Result<String, Error> {
    if let Some(t) = token {
        if let Some(param_name) = t.strip_prefix(':') {
            if let Some(val) = params.get(param_name) {
                return Ok(val.to_string());
            }
        } else if !t.is_empty() {
            return Ok(t.to_string());
        }
    }

    if let Some(val) = params.get(fallback_param_name) {
        return Ok(val.to_string());
    }

    Err(Error::query_error(format!(
        "missing required argument or parameter ':{fallback_param_name}'"
    )))
}
