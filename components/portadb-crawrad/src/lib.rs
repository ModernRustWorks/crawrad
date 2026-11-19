//! **Connection components** — native providers and the abstraction over them.
//!
//! # The design follows a classic RAD IDE, as stated by the product owner
//!
//! > *"The Engine has several connections, the native ones — MySQL, PostgreSQL, SQLite.
//! > Then there is our abstraction, which requires dropping one of those native components
//! > on the canvas, dropping our abstraction, and connecting the two. Our
//! > abstraction will always need a property to receive a native component.
//! > Therefore, the native component must exist."*
//!
//! It is the connection plus the *driver link* of a classic RAD IDE: two components
//! on the canvas, where the second ensures the driver exists in the binary.
//!
//! **In Rust, the driver link is a Cargo feature.** Without `portadb/rusqlite`
//! the driver module is not even compiled — so dropping `SqliteProvider` onto the
//! form literally enables the driver via the `requires` clause it declares.
//! Having one component per provider is not redundant with the abstraction:
//! the abstraction says *"I want to connect"*, and the provider says *"and THIS
//! is the driver that must exist in the binary"*.
//!
//! # The abstraction does not enumerate providers, making it truly abstract
//!
//! Each provider declares `provides = ["DbProvider"]`; [`Connection`] declares that
//! its `provider` property accepts anything offering `DbProvider`. When provider
//! **number six** is created, it declares that same capability and immediately appears
//! in the Object Inspector list — without anyone modifying this file.
//!
//! The alternative was hardcoding all provider names into the abstraction, which
//! would require editing it for every new provider. See ADR 0040.
//!
//! # What this file does NOT do, stated explicitly
//!
//! It holds the **state** of the component: what the designer authored, available
//! to the application. It does not connect to any database. The actual engine is
//! the `portadb` crate in `components/portadb/`, which is driver-agnostic and
//! **unaware of the IDE's existence**.
//!
//! A `connect()` returning `Ok(())` without connecting would be worse than not existing.
//!
//! # This package's manifest is GENERATED
//!
//! Modify a `struct` here and run `CRAWRAD_BLESS=1 cargo test`, or
//! `crawrad manifest components/portadb-crawrad`. Hand-editing `manifest.json` is
//! what the macro exists to prevent.

use crawrad_component_derive::Component;

/// **The capability that every native provider offers.**
///
/// It is the name requested by [`Connection`]'s `provider` property, and the only
/// contract between the abstraction and providers. It is kept in a constant so
/// neither side can write it differently.
pub const DB_PROVIDER: &str = "DbProvider";

/// **The capability that a database connection offers.**
///
/// It is the single capability name that the `connection` property of a [`Dataset`]
/// asks for, and the single contract between the connection and datasets. It is
/// kept in a constant so that neither side can misspell it.
pub const CONNECTION_CAPABILITY: &str = "DbConnection";

/// **The capability that a database dataset offers.**
///
/// It is the single capability name that dataset-aware actions ask for,
/// and the contract between datasets and dataset actions. It is kept in
/// a constant so that neither side can misspell it.
pub const DATASET_CAPABILITY: &str = "DbDataset";

/// **The capability of everything a Grid can read its rows from.**
///
/// A Grid's `data_source` asks for it: a [`Connection`] (with the Grid's own
/// `table_name`) and a [`Dataset`] (its `sql`, over its connection) both offer
/// it. The reference asks for a capability, never for a class (ADR 0040).
pub const ROW_SOURCE_CAPABILITY: &str = "DbRowSource";

/// The in-memory provider — allows a form to run without a server.
#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
#[crawrad(
    display = "Memory Provider",
    palette = "Data",
    prefix = "mem",
    size = (28.0, 28.0),
    visual = false,
    provides = ["DbProvider"]
)]
#[requires(crate = "portadb", version = "0.1", features = ["memory"])]
#[icon(
    move_to(10.0, 1.0),
    line_to(10.0, 8.0),
    line_to(3.5, 9.5),
    close_path(),
    fill(),
    move_to(6.0, 15.0),
    line_to(6.0, 8.0),
    line_to(12.5, 6.5),
    close_path(),
    fill()
)]
pub struct MemoryProvider {
    /// Rows survive a reconnection.
    #[prop(display = "Persistent", category = "behavior", default = false)]
    pub persistent: bool,
}

/// SQLite via `rusqlite` — embedded driver, serverless.
#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
#[crawrad(
    display = "SQLite Provider (rusqlite)",
    palette = "Data",
    prefix = "sqlite",
    size = (28.0, 28.0),
    visual = false,
    provides = ["DbProvider"]
)]
#[requires(crate = "portadb", version = "0.1", features = ["rusqlite"])]
#[icon(
    move_to(10.0, 1.0),
    line_to(10.0, 8.0),
    line_to(3.5, 9.5),
    close_path(),
    fill(),
    move_to(6.0, 15.0),
    line_to(6.0, 8.0),
    line_to(12.5, 6.5),
    close_path(),
    fill()
)]
pub struct SqliteProvider {}

/// SQLite via `sqlx` — the same database, the alternative client.
///
/// Both exist on purpose: the **database** and the **provider** are independent
/// axes, and switching clients without switching databases is a choice the user
/// must be able to make.
#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
#[crawrad(
    display = "SQLite Provider (sqlx)",
    palette = "Data",
    prefix = "sxlt",
    size = (28.0, 28.0),
    visual = false,
    provides = ["DbProvider"]
)]
#[requires(crate = "portadb", version = "0.1", features = ["sqlx-sqlite"])]
#[icon(
    move_to(10.0, 1.0),
    line_to(10.0, 8.0),
    line_to(3.5, 9.5),
    close_path(),
    fill(),
    move_to(6.0, 15.0),
    line_to(6.0, 8.0),
    line_to(12.5, 6.5),
    close_path(),
    fill()
)]
pub struct SqliteSqlxProvider {}

/// PostgreSQL via `sqlx`.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[crawrad(
    display = "PostgreSQL Provider (sqlx)",
    palette = "Data",
    prefix = "pg",
    size = (28.0, 28.0),
    visual = false,
    provides = ["DbProvider"]
)]
#[requires(crate = "portadb", version = "0.1", features = ["sqlx-postgres"])]
#[icon(
    move_to(10.0, 1.0),
    line_to(10.0, 8.0),
    line_to(3.5, 9.5),
    close_path(),
    fill(),
    move_to(6.0, 15.0),
    line_to(6.0, 8.0),
    line_to(12.5, 6.5),
    close_path(),
    fill()
)]
pub struct PostgresProvider {
    /// Where the server is located.
    #[prop(display = "Host", category = "behavior", default = "localhost")]
    pub host: String,
    /// The server port.
    #[prop(
        display = "Port",
        category = "behavior",
        default = 5432,
        min = 1,
        max = 65535
    )]
    pub port: i64,
}

/// Redis.
///
/// **Without `features`**, and not by omission: `redis` is the only one not
/// gated in today's `portadb` crate. The manifest reflects the truth about the
/// crate, rather than what would be symmetrical.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[crawrad(
    display = "Redis Provider",
    palette = "Data",
    prefix = "rds",
    size = (28.0, 28.0),
    visual = false,
    provides = ["DbProvider"]
)]
#[requires(crate = "portadb", version = "0.1")]
#[icon(
    move_to(10.0, 1.0),
    line_to(10.0, 8.0),
    line_to(3.5, 9.5),
    close_path(),
    fill(),
    move_to(6.0, 15.0),
    line_to(6.0, 8.0),
    line_to(12.5, 6.5),
    close_path(),
    fill()
)]
pub struct RedisProvider {
    /// Where the server is located.
    #[prop(display = "Host", category = "behavior", default = "localhost")]
    pub host: String,
}

impl Default for PostgresProvider {
    fn default() -> Self {
        Self {
            host: String::from("localhost"),
            port: 5432,
        }
    }
}

impl Default for RedisProvider {
    fn default() -> Self {
        Self {
            host: String::from("localhost"),
        }
    }
}

/// **The connection declared by the form** — connection of a classic RAD form.
///
/// It does not connect and does not know which provider exists: it **points** to one,
/// and whichever offers the `DbProvider` capability responds.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[crawrad(
    palette = "Data",
    prefix = "conn",
    size = (28.0, 28.0),
    visual = false,
    display = "Connection",
    provides = ["DbConnection", "DbRowSource"]
)]
#[event(name = "on_connect")]
#[event(name = "on_error", signature = "fn(&mut self, message: &str)")]
#[icon(
    r#box(3.0, 1.5, 10.0, 3.0, 1.5),
    stroke(),
    move_to(3.0, 3.0),
    line_to(3.0, 13.0),
    line_to(5.0, 14.5),
    line_to(11.0, 14.5),
    line_to(13.0, 13.0),
    line_to(13.0, 3.0),
    stroke(),
    move_to(3.0, 8.0),
    line_to(5.0, 9.5),
    line_to(11.0, 9.5),
    line_to(13.0, 8.0),
    stroke()
)]
pub struct Connection {
    /// **The native provider managed by this connection.**
    ///
    /// Empty indicates an unassigned connection — the state in which it is born,
    /// because the provider is a component someone will drop on the canvas.
    #[prop(
        display = "Provider",
        category = "behavior",
        kind = "reference",
        ref_class = "DbProvider",
        default = ""
    )]
    pub provider: String,

    /// Database location, in the format understood by the chosen provider.
    #[prop(display = "Url", category = "behavior", default = "")]
    pub url: String,

    /// The pool ceiling.
    #[prop(
        display = "Max connections",
        category = "behavior",
        default = 10,
        min = 1,
        max = 1000
    )]
    pub max_connections: i64,

    /// How long to wait for a connection in milliseconds — the same unit and
    /// reasoning as `Timer`'s `interval`.
    #[prop(
        display = "Connect timeout",
        category = "behavior",
        default = 30000,
        min = 100,
        max = 600000
    )]
    pub connect_timeout: i64,

    /// **The designer requests the connection, it does not establish it.**
    ///
    /// In a classic RAD IDE, enabling connection at design time connects for
    /// real, which is how query components show data in designer. Here it is,
    /// for now, what the form stored — see module doc.
    #[prop(display = "Connected", category = "behavior", default = false)]
    pub connected: bool,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            provider: String::new(),
            url: String::new(),
            max_connections: 10,
            connect_timeout: 30000,
            connected: false,
        }
    }
}

/// **The dataset that ordinary UI components bind to.**
///
/// It is the layer between a database connection and ordinary components
/// (ADR 0041: there are no special DB controls — standard controls like an Edit
/// or a Grid bind directly to a dataset).
///
/// `table` and `sql` are mutually exclusive data sources, matching
/// `portadb::DatasetSource`: specifying a `table` creates an editable,
/// CRUD-capable dataset, whereas providing an `sql` query creates a read-only
/// dataset.
///
/// Like every component in this crate, this struct represents the designer
/// **state**, not the runtime engine: it stores what the user configured in the
/// designer without executing database operations.
#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
#[crawrad(
    palette = "Data",
    prefix = "dst",
    size = (28.0, 28.0),
    visual = false,
    display = "Dataset",
    provides = ["DbDataset", "DbRowSource"]
)]
#[requires(crate = "portadb", version = "0.1")]
#[event(name = "on_after_post")]
#[event(name = "on_error", signature = "fn(&mut self, message: &str)")]
#[icon(
    r#box(2.0, 2.5, 12.0, 11.0, 1.5),
    stroke(),
    move_to(2.0, 6.0),
    line_to(14.0, 6.0),
    stroke(),
    move_to(6.5, 6.0),
    line_to(6.5, 13.5),
    stroke()
)]
pub struct Dataset {
    /// **The connection component that this dataset points to.**
    ///
    /// Empty is a dataset that has not yet chosen its connection. It binds to
    /// any component offering the `DbConnection` capability.
    #[prop(
        display = "Connection",
        category = "behavior",
        kind = "reference",
        ref_class = "DbConnection",
        default = ""
    )]
    pub connection: String,

    /// The database table this dataset edits with full CRUD capabilities.
    ///
    /// When set, the dataset operates in editable mode over the specified table.
    /// This is an alternative to [`sql`](Self::sql).
    #[prop(display = "Table", category = "behavior", default = "")]
    pub table: String,

    /// A custom SQL query that provides a read-only dataset.
    ///
    /// When set instead of [`table`](Self::table), the dataset executes this query
    /// in read-only mode.
    #[prop(display = "SQL", category = "behavior", multiline, default = "")]
    pub sql: String,

    /// **The designer asks for an active dataset, it does not open it.**
    ///
    /// Following the same reasoning as [`Connection::connected`], toggling this
    /// flag in the designer records the desired state without opening live
    /// database cursors at design time.
    #[prop(display = "Active", category = "behavior", default = false)]
    pub active: bool,
}

// The manifest for this package is derived from the structs above. The order
// matches the palette: native providers first, followed by the abstraction pointing to one of them.
crawrad_component::manifest!(
    MemoryProvider,
    SqliteProvider,
    SqliteSqlxProvider,
    PostgresProvider,
    RedisProvider,
    Connection,
    Dataset,
);

#[cfg(test)]
mod tests {
    use super::*;

    /// **Every provider offers the capability, and the abstraction requests it.**
    ///
    /// It is the entire contract between both sides, and it is a single word. If a
    /// new provider writes `provides = ["DBProvider"]` with mismatched casing, it
    /// disappears from the Object Inspector list until someone notices — so the
    /// word is kept in a constant, and this test enforces it.
    #[test]
    fn todo_engine_oferece_a_capacidade_que_a_abstracao_pede() {
        let esperado = format!("\"provides\": [\"{DB_PROVIDER}\"]");
        for bloco in ENGINES {
            assert!(
                bloco.contains(&esperado),
                "um provider deixou de oferecer `{DB_PROVIDER}`:\n{bloco}"
            );
        }
        assert!(
            Connection::CRAWRAD_COMPONENT_JSON
                .contains(&format!("\"ref_class\": \"{DB_PROVIDER}\"")),
            "a abstração deixou de pedir a capacidade:\n{}",
            Connection::CRAWRAD_COMPONENT_JSON
        );
    }

    /// **The connection offers the capability that the dataset asks for.**
    ///
    /// It is the single contract between the connection abstraction and the
    /// dataset. Storing it in a constant ensures neither side can misspell it.
    #[test]
    fn the_connection_offers_what_the_dataset_asks_for() {
        let expected = format!("\"{CONNECTION_CAPABILITY}\"");
        assert!(
            Connection::CRAWRAD_COMPONENT_JSON.contains(&expected),
            "Connection left out provides `{CONNECTION_CAPABILITY}`:\n{}",
            Connection::CRAWRAD_COMPONENT_JSON
        );
        assert!(
            Dataset::CRAWRAD_COMPONENT_JSON
                .contains(&format!("\"ref_class\": \"{CONNECTION_CAPABILITY}\"")),
            "Dataset left out ref_class `{CONNECTION_CAPABILITY}`:\n{}",
            Dataset::CRAWRAD_COMPONENT_JSON
        );
    }

    /// **Both a Connection and a Dataset offer what a Grid's data source asks for.**
    #[test]
    fn a_connection_and_a_dataset_are_row_sources() {
        for json in [
            Connection::CRAWRAD_COMPONENT_JSON,
            Dataset::CRAWRAD_COMPONENT_JSON,
        ] {
            assert!(
                json.contains(&format!("\"{ROW_SOURCE_CAPABILITY}\"")),
                "left out `{ROW_SOURCE_CAPABILITY}`:
{json}"
            );
        }
    }

    /// The five native providers, as each is written in the manifest.
    const ENGINES: &[&str] = &[
        MemoryProvider::CRAWRAD_COMPONENT_JSON,
        SqliteProvider::CRAWRAD_COMPONENT_JSON,
        SqliteSqlxProvider::CRAWRAD_COMPONENT_JSON,
        PostgresProvider::CRAWRAD_COMPONENT_JSON,
        RedisProvider::CRAWRAD_COMPONENT_JSON,
    ];

    /// **The abstraction starts without a provider.**
    ///
    /// A default pointing to a class would make the connection appear linked to a
    /// component that no one placed on the canvas.
    #[test]
    fn a_abstracao_nasce_sem_provider() {
        assert!(Connection::default().provider.is_empty());
    }

    /// **A dataset starts closed and unbound.**
    ///
    /// Like the connection starting without an provider, a dataset starts with no
    /// connection, no table or query selected, and in the inactive state.
    #[test]
    fn a_dataset_is_born_closed_and_unbound() {
        let dataset = Dataset::default();
        assert!(dataset.connection.is_empty());
        assert!(dataset.table.is_empty());
        assert!(dataset.sql.is_empty());
        assert!(!dataset.active);
    }

    /// **The dataset requires the underlying engine crate.**
    ///
    /// The generated application needs the `portadb` crate to compile
    /// and operate on dataset state at runtime.
    #[test]
    fn the_dataset_requires_the_engine_crate() {
        assert!(
            Dataset::CRAWRAD_COMPONENT_JSON.contains("\"crate\": \"portadb\""),
            "Dataset JSON is missing crate requirement `portadb`:\n{}",
            Dataset::CRAWRAD_COMPONENT_JSON
        );
    }

    /// **The dataset defines its own icon for the component palette.**
    ///
    /// This uses the third-party icon feature so the IDE palette can render
    /// a distinctive visual glyph on the 16x16 grid without built-in assets.
    #[test]
    fn the_dataset_draws_its_own_icon() {
        assert!(
            Dataset::CRAWRAD_COMPONENT_JSON.contains("\"icon\""),
            "Dataset JSON is missing icon definition:\n{}",
            Dataset::CRAWRAD_COMPONENT_JSON
        );
    }

    /// The dataset offers the `DbDataset` capability so standard dataset
    /// actions (First, Prior, Next, Last, Post, Cancel, etc.) can bind to it.
    #[test]
    fn the_dataset_offers_what_dataset_actions_ask_for() {
        assert!(
            Dataset::CRAWRAD_COMPONENT_JSON.contains("\"DbDataset\""),
            "Dataset JSON is missing capability `DbDataset`:\n{}",
            Dataset::CRAWRAD_COMPONENT_JSON
        );
    }

    /// **Each provider declares the feature that the `portadb` crate actually has.**
    ///
    /// `requires` is the driver link: it adds the feature to the generated
    /// project's `Cargo.toml`, and without it the driver module will not compile.
    #[test]
    fn cada_engine_declara_a_feature_do_driver() {
        for (bloco, feature) in [
            (MemoryProvider::CRAWRAD_COMPONENT_JSON, "memory"),
            (SqliteProvider::CRAWRAD_COMPONENT_JSON, "rusqlite"),
            (SqliteSqlxProvider::CRAWRAD_COMPONENT_JSON, "sqlx-sqlite"),
            (PostgresProvider::CRAWRAD_COMPONENT_JSON, "sqlx-postgres"),
        ] {
            assert!(
                bloco.contains(&format!("\"{feature}\"")),
                "a feature `{feature}` sumiu do provider:\n{bloco}"
            );
            assert!(
                bloco.contains("\"crate\": \"portadb\""),
                "o requisito deixou de apontar para o motor:\n{bloco}"
            );
        }
    }
}
