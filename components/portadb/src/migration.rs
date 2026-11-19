use crate::engine::Engine;
use crate::error::Error;
use crate::params;

/// Represents a versioned database schema migration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Migration {
    /// Monotonically increasing version number (e.g. 1, 2, 2026091601).
    pub version: i64,
    /// Descriptive name of the migration.
    pub name: &'static str,
    /// Forward DDL/DML script applied by this migration.
    pub up: &'static str,
    /// Rollback script reverting this migration.
    pub down: &'static str,
}

impl Migration {
    /// Constructs a new migration definition.
    #[must_use]
    pub const fn new(
        version: i64,
        name: &'static str,
        up: &'static str,
        down: &'static str,
    ) -> Self {
        Self {
            version,
            name,
            up,
            down,
        }
    }
}

/// Transactional migration runner managing version history and schema evolution.
#[derive(Default, Clone, Debug)]
pub struct Migrator {
    migrations: Vec<Migration>,
}

impl Migrator {
    /// Constructs an empty migrator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    /// Appends a migration definition to the migration set.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, migration: Migration) -> Self {
        self.migrations.push(migration);
        self.migrations.sort_by_key(|m| m.version);
        self
    }

    /// Appends a migration definition using fluent builder naming.
    #[must_use]
    pub fn with_migration(self, migration: Migration) -> Self {
        self.add(migration)
    }

    /// Applies all pending migrations within an atomic transaction.
    /// Returns the number of newly applied migrations.
    pub async fn run(&self, engine: &Engine) -> Result<usize, Error> {
        let mut conn = engine.acquire().await?;
        let mut tx = conn.begin().await?;

        // 1. Ensure migrations table exists
        tx.execute(
            "CREATE TABLE IF NOT EXISTS _dataengine_migrations (
                version BIGINT PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at TEXT NOT NULL
            );",
            &params![],
        )
        .await?;

        // 2. Discover already applied versions
        let rows = tx
            .query(
                "SELECT version FROM _dataengine_migrations ORDER BY version ASC;",
                &params![],
            )
            .await?;

        let applied_versions: Vec<i64> = rows
            .into_iter()
            .filter_map(|r| r.get::<i64>("version").ok())
            .collect();

        // 3. Apply missing migrations in strict order
        let mut applied_count = 0;
        for m in &self.migrations {
            if !applied_versions.contains(&m.version) {
                // Execute migration UP
                tx.execute(m.up, &params![]).await?;

                // Record applied migration
                let now = "2026-09-16T19:00:00Z";
                tx.execute(
                    "INSERT INTO _dataengine_migrations (version, name, applied_at) VALUES (:version, :name, :applied_at);",
                    &params![
                        ("version", m.version),
                        ("name", m.name),
                        ("applied_at", now)
                    ],
                )
                .await?;

                applied_count += 1;
            }
        }

        tx.commit().await?;
        Ok(applied_count)
    }

    /// Rolls back the latest applied migration.
    /// Returns the rolled back version, or `None` if no migrations were applied.
    pub async fn rollback_last(&self, engine: &Engine) -> Result<Option<i64>, Error> {
        let mut conn = engine.acquire().await?;
        let mut tx = conn.begin().await?;

        // Discover latest applied version
        let rows = tx
            .query(
                "SELECT version FROM _dataengine_migrations ORDER BY version DESC LIMIT 1;",
                &params![],
            )
            .await?;

        if rows.is_empty() {
            tx.rollback().await?;
            return Ok(None);
        }

        let latest_version: i64 = rows[0].get("version")?;
        let migration = self
            .migrations
            .iter()
            .find(|m| m.version == latest_version)
            .ok_or_else(|| {
                Error::ConfigurationError(format!(
                    "migration definition for applied version {latest_version} not found in migrator"
                ))
            })?;

        // Execute migration DOWN
        tx.execute(migration.down, &params![]).await?;

        // Remove from tracking table
        tx.execute(
            "DELETE FROM _dataengine_migrations WHERE version = :version;",
            &params![("version", latest_version)],
        )
        .await?;

        tx.commit().await?;
        Ok(Some(latest_version))
    }
}
