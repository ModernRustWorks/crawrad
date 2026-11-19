# Connection

A decoupled, driver-agnostic database connection engine abstraction for Rust.

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/crates/portadb)
[![Documentation](https://docs.rs/portadb/badge.svg)](https://docs.rs/portadb)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/example/portadb)

---

## Overview

When building applications that communicate with databases in Rust, developers are typically forced to pick an access crate early on: `sqlx`, `rusqlite`, `diesel`, `tokio-postgres`, or NoSQL drivers.

That single decision usually infiltrates the entire application: driver-specific structs populate domain signatures, the driver's execution model dictates whether business services are sync or async, and database vendor vocabulary leaks into business rules. Changing database drivers later requires rewriting repository and service layers.

**`portadb` eliminates this coupling.** Applications program against **unified, pure Rust abstractions: [`Engine`], [`Connection`], and [`Executor`]**. Which engine executes the queries is purely a matter of configuration and driver selection—never business logic rewrite.

Importantly, `portadb` treats the **Engine** (the client access library) and the **Database** (the RDBMS or data store) as two orthogonal dimensions:

| Axis | Meaning | Examples |
|---|---|---|
| **Engine** (the library) | The client library executing queries and networking | `sqlx` · `rusqlite` · `memory` · `redis` · custom SPI |
| **Database** (the data store) | The target storage engine or server | SQLite · PostgreSQL · MySQL · SQL Server · Redis |

---

## Architectural Design

```text
+------------------------------------------------------------------------+
|                      Application Business Logic                        |
|   Accepts: &Engine, &mut Connection, or &mut impl Executor             |
+------------------------------------------------------------------------+
                                     |
                                     v
+------------------------------------------------------------------------+
|                       Unified Engine Façade                            |
|        Engine  <--->  Connection  <--->  Transaction                   |
|        [Executor]     [Executor]        [Executor]                     |
+------------------------------------------------------------------------+
                |                                    |
                v                                    v
+--------------------------------+   +-----------------------------------+
|      SQL Dialect Layer         |   |       Open Driver SPI             |
|  - SqliteDialect    (?)        |   |   DriverRegistry (thread-safe)    |
|  - PostgresDialect  ($1, $2)   |   |   Driver <--> DriverConnection    |
|  - MySqlDialect     (?)        |   +-----------------------------------+
|  - SqlServerDialect (@p1, @p2) |                     |
|  - RedisDialect                |                     |
+--------------------------------+                     v
                               +-----------------------------------------+
                               |  Built-in & Third-Party Drivers         |
                               |  - RusqliteDriver   (SQLite sync)       |
                               |  - SqlxDriver       (SQLite async)      |
                               |  - SqlxPostgres     (PostgreSQL async)  |
                               |  - MemoryDriver     (In-Memory store)   |
                               |  - RedisDriver      (Key-Value store)   |
                               |  - Custom Drivers   (Extensible SPI)    |
                               +-----------------------------------------+
```

### The Core Architectural Pillars

1. **Unified Façade ([`Engine`])**:
   A thread-safe (`Send + Sync + Clone`), long-lived database handle coordinating connection lifecycle, configuration, and dialect resolution. Provides direct execution methods (`engine.execute(...)`, `engine.query(...)`) as well as dedicated connection checkouts (`engine.acquire()`).

2. **Unified Connection & Transaction ([`Connection`], [`Transaction`])**:
   Pure, idiomatic Rust connection and transaction scopes. [`Connection::begin`] starts an atomic transaction with **guaranteed RAII automatic rollback on `Drop`** if neither `commit()` nor `rollback()` is explicitly called.

3. **Unified Execution Contract ([`Executor`])**:
   Inspired by modern Rust database best practices (`sqlx`, `sea-orm`), both [`Connection`] and [`Transaction`] implement [`Executor`]. Services and repositories can take `&mut impl Executor` to run queries seamlessly inside or outside a transactional context.

4. **Multi-Database Dialect Translation ([`Dialect`])**:
   Translates named parameter placeholders (`:param`) to the target engine's native parameter format at execution time:
   - **SQLite**: `?` positional parameters, double quotes (`"identifier"`)
   - **PostgreSQL**: `$1`, `$2` numbered parameters, double quotes (`"identifier"`), supports `::type` casts
   - **MySQL**: `?` positional parameters, backticks (`` `identifier` ``)
   - **SQL Server**: `@p1`, `@p2` parameters, brackets (`[identifier]`)
   - **Redis**: whitespace command formatting

5. **Open Driver SPI & Dynamic Registry ([`DriverRegistry`])**:
   Implements the Open-Closed Principle (OCP). Adding a third-party driver requires implementing [`Driver`] and registering it via [`register_driver`]. **Zero modifications to core `portadb` files.**

6. **Reactive UI Tabular View Layer ([`TableView`])**:
   Observable tabular state designed for desktop and reactive GUI frameworks (such as Makepad, egui, Slint, or Iced):
   - **Pull-friendly**: Generational monotonic version counter (`view.version()`) for immediate-mode render loops.
   - **Push-friendly**: Reactive event subscription (`view.subscribe(...)`) for event-driven UIs.
   - **Zero Shared Mutable Buffers**: Rows are immutable value snapshots.

7. **Graceful Feature Abstraction for Specialized Stores (Redis)**:
   When non-relational engines (such as Redis) are targeted, native commands (`SET`, `GET`, `DEL`, `EXISTS`, `KEYS`, `PING`, transactions via `MULTI`/`EXEC`/`DISCARD`) are executed seamlessly, while unsupported relational operations (e.g. `JOIN`) are cleanly abstracted and report [`Error::UnsupportedFeature`].

---

## Quickstart

Add `portadb` to your `Cargo.toml`:

```toml
[dependencies]
portadb = { version = "0.1.0", features = ["rusqlite", "sqlx-sqlite", "sqlx-postgres", "memory"] }
```

### Writing Engine-Agnostic Business Code

```rust
use portadb::{Connection, Error, Executor, params};

/// Business service accepting any database executor (Connection or Transaction).
async fn create_and_fetch_user(exec: &mut impl Executor) -> Result<(), Error> {
    // 1. Insert with named parameters (SQL injection safe)
    exec.execute(
        "INSERT INTO users (id, name, email) VALUES (:id, :name, :email)",
        &params![
            ("id", 1i64),
            ("name", "Alice"),
            ("email", "alice@example.com"),
        ],
    ).await?;

    // 2. Query with typed column access
    let rows = exec.query(
        "SELECT id, name, email FROM users WHERE id = :id",
        &params![("id", 1i64)],
    ).await?;

    assert_eq!(rows.len(), 1);
    let name: String = rows[0].get("name")?;
    let email: Option<String> = rows[0].get_opt("email")?;
    println!("Found user: {name} <{email:?}>");

    Ok(())
}
```

### Switching Engines (Configuration Only!)

```rust
use portadb::{ConnectOptions, Engine};

#[tokio::main]
async fn main() -> Result<(), portadb::Error> {
    // Option A: SQLite via rusqlite
    let opts = ConnectOptions::new(":memory:").with_engine("rusqlite");
    let engine = Engine::connect(&opts).await?;
    let mut conn = engine.acquire().await?;
    create_and_fetch_user(&mut conn).await?;

    // Option B: SQLite via async sqlx — ONLY the configuration changes!
    let opts = ConnectOptions::new("sqlite::memory:").with_engine("sqlx");
    let engine = Engine::connect(&opts).await?;
    let mut conn = engine.acquire().await?;
    create_and_fetch_user(&mut conn).await?;

    // Option C: PostgreSQL via async sqlx
    // let opts = ConnectOptions::new("postgres://user:pass@localhost/db");
    // let engine = Engine::connect(&opts).await?;

    // Option D: In-Memory Mock Store (zero dependencies, instant unit tests)
    let opts = ConnectOptions::new("memory://testdb");
    let engine = Engine::connect(&opts).await?;

    // Option E: Redis Key-Value Store
    let opts = ConnectOptions::new("redis://127.0.0.1:6379");
    let engine = Engine::connect(&opts).await?;

    Ok(())
}
```

### Connecting to Redis

```rust
use portadb::{ConnectOptions, Engine, params};

#[tokio::main]
async fn main() -> Result<(), portadb::Error> {
    let opts = ConnectOptions::new("redis://127.0.0.1:6379");
    let engine = Engine::connect(&opts).await?;
    let mut conn = engine.acquire().await?;

    // SET key value
    conn.execute("SET :key :val", &params![("key", "session:user_1"), ("val", "token_abc")]).await?;

    // GET key
    let rows = conn.query("GET :key", &params![("key", "session:user_1")]).await?;
    assert_eq!(rows[0].get::<String>("value")?, "token_abc");

    // Transactions map to Redis MULTI / EXEC / DISCARD
    let mut tx = conn.begin().await?;
    tx.execute("SET :key :val", &params![("key", "temp"), ("val", "discard_me")]).await?;
    tx.rollback().await?; // DISCARD

    Ok(())
}
```

---

## Escape Hatch: Accessing the Raw Driver

When specialized driver-specific features are needed (e.g. SQLite backup APIs, raw pragmas, or low-level connection pooling), use [`Connection::downcast_raw`]:

```rust
use portadb::Connection;
use portadb::drivers::RusqliteConnection;

fn access_low_level_driver(conn: &Connection) {
    if let Ok(rusqlite_conn) = conn.downcast_raw::<RusqliteConnection>() {
        let guard = rusqlite_conn.raw_lock().expect("mutex lock");
        if let Some(raw) = guard.as_ref() {
            println!("SQLite autocommit mode: {}", raw.is_autocommit());
        }
    }
}
```

---

## Cargo Features

| Feature | Description | Default |
|---|---|---|
| `memory` | Built-in in-memory catalog database engine (zero external dependencies) | **Yes** |
| `rusqlite` | Synchronous SQLite driver via `rusqlite` | No |
| `sqlx-sqlite` | Native asynchronous SQLite driver via `sqlx` | No |
| `sqlx-postgres` | Native asynchronous PostgreSQL driver via `sqlx` | No |

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Crates

| Crate | What it is |
|---|---|
| `portadb-core` | Types and contracts: values, rows, parameters, dialects, the driver SPI. Depends on nothing of CrawRAD. |
| `portadb-memory` | Provider: in-memory database, for forms that run without a server. |
| `portadb-rusqlite` | Provider: SQLite through `rusqlite`. |
| `portadb-sqlite` | Provider: SQLite through `sqlx`. |
| `portadb-postgres` | Provider: PostgreSQL through `sqlx`. |
| `portadb-redis` | Provider: Redis. |
| `portadb` | The engine applications use (`Engine`, `Connection`, pool, migrations, data views). Turns providers on by cargo feature and re-exports them. |
| `portadb-crawrad` | The CrawRAD glue: palette components (`Connection`, `DataSet`, engines). |
