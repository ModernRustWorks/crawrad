//! SQLite-backed settings and recent folders store for the CrawRAD IDE.

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crawrad_note::note;
use portadb::params;

static LAST_TIMESTAMP: AtomicI64 = AtomicI64::new(0);

fn monotonic_timestamp() -> i64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros() as i64)
        .unwrap_or(0);
    loop {
        let prev = LAST_TIMESTAMP.load(Ordering::Relaxed);
        let candidate = if now > prev { now } else { prev + 1 };
        if LAST_TIMESTAMP
            .compare_exchange_weak(prev, candidate, Ordering::SeqCst, Ordering::Relaxed)
            .is_ok()
        {
            return candidate;
        }
    }
}

/// Errors produced by settings store operations.
#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    /// An underlying database error occurred.
    #[error("database error: {0}")]
    Database(#[from] portadb::Error),
    /// An I/O error occurred during directory or file setup.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// An asynchronous runtime initialization failure.
    #[error("runtime error: {0}")]
    Runtime(String),
}

/// Synchronous store managing IDE preferences and recent folders in SQLite.
pub struct SettingsStore {
    conn: portadb::Connection,
    rt: tokio::runtime::Runtime,
}

impl SettingsStore {
    /// Opens or creates a settings store database at the given path.
    #[note("settings-open")]
    pub fn open(file: &Path) -> Result<Self, SettingsError> {
        let is_memory = file == Path::new(":memory:");
        if !is_memory
            && let Some(parent) = file.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)?;
        }

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| SettingsError::Runtime(e.to_string()))?;

        let url = if is_memory {
            "sqlite::memory:".to_string()
        } else {
            format!("sqlite:{}", file.to_string_lossy())
        };

        let options = portadb::ConnectOptions::new(url)
            .with_database(portadb::Database::Sqlite)
            .with_engine("rusqlite");

        let mut conn = rt.block_on(portadb::Connection::connect(&options))?;

        rt.block_on(async {
            conn.execute("PRAGMA journal_mode = WAL;", &params![]).await?;
            conn.execute("PRAGMA synchronous = NORMAL;", &params![]).await?;

            conn.execute(
                "CREATE TABLE IF NOT EXISTS meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
                &params![],
            )
            .await?;
            conn.execute(
                "INSERT OR IGNORE INTO meta (key, value) VALUES ('schema_version', '1');",
                &params![],
            )
            .await?;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL, updated_at INTEGER NOT NULL);",
                &params![],
            )
            .await?;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS recent_folders (path TEXT PRIMARY KEY, opened_at INTEGER NOT NULL);",
                &params![],
            )
            .await?;
            Ok::<(), portadb::Error>(())
        })?;

        Ok(Self { conn, rt })
    }

    /// Retrieves a single preference value by its key.
    #[note("settings-get")]
    pub fn get(&mut self, key: &str) -> Option<String> {
        let conn = &mut self.conn;
        let rows = self
            .rt
            .block_on(async {
                conn.query(
                    "SELECT value FROM settings WHERE key = :key LIMIT 1;",
                    &params![("key", key)],
                )
                .await
            })
            .ok()?;
        rows.first().and_then(|r| r.get::<String>("value").ok())
    }

    /// Stores a single key-value preference within a transaction.
    #[note("settings-set")]
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), SettingsError> {
        self.set_many(&[(key, value)])
    }

    /// Stores multiple key-value pairs within a single atomic transaction.
    #[note("settings-set-many")]
    pub fn set_many(&mut self, pairs: &[(&str, &str)]) -> Result<(), SettingsError> {
        let conn = &mut self.conn;
        self.rt.block_on(async {
            let mut tx = conn.begin().await?;
            for (key, value) in pairs {
                let now = monotonic_timestamp();
                tx.execute(
                    "INSERT INTO settings (key, value, updated_at) VALUES (:key, :value, :updated_at) \
                     ON CONFLICT(key) DO UPDATE SET value = :value, updated_at = :updated_at;",
                    &params![
                        ("key", *key),
                        ("value", *value),
                        ("updated_at", now),
                    ],
                )
                .await?;
            }
            tx.commit().await?;
            Ok::<(), SettingsError>(())
        })
    }

    /// Returns all key-value preferences ordered alphabetically by key.
    #[note("settings-all")]
    pub fn all(&mut self) -> Vec<(String, String)> {
        let conn = &mut self.conn;
        let rows = self
            .rt
            .block_on(async {
                conn.query(
                    "SELECT key, value FROM settings ORDER BY key ASC;",
                    &params![],
                )
                .await
            })
            .unwrap_or_default();
        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            if let (Ok(k), Ok(v)) = (row.get::<String>("key"), row.get::<String>("value")) {
                out.push((k, v));
            }
        }
        out
    }

    /// Returns recent folder paths ordered by most recently opened first.
    #[note("settings-recent-folders")]
    pub fn recent_folders(&mut self, limit: usize) -> Vec<PathBuf> {
        let conn = &mut self.conn;
        let rows = self
            .rt
            .block_on(async {
                conn.query(
                    "SELECT path FROM recent_folders ORDER BY opened_at DESC LIMIT :limit;",
                    &params![("limit", limit as i64)],
                )
                .await
            })
            .unwrap_or_default();
        let mut out: Vec<PathBuf> = Vec::with_capacity(rows.len());
        for row in rows {
            if let Ok(p) = row.get::<String>("path") {
                let p = PathBuf::from(p);
                // Older lists hold one folder under two spellings (`/` and `\`).
                if !out.iter().any(|o| folder_key(o) == folder_key(&p)) {
                    out.push(p);
                }
            }
        }
        out
    }

    /// Records a folder path, bumping its timestamp to the top of the recents list.
    #[note("settings-push-recent-folder")]
    pub fn push_recent_folder(&mut self, folder: &Path) -> Result<(), SettingsError> {
        let path_str = folder_key(folder);
        let now = monotonic_timestamp();
        let conn = &mut self.conn;
        self.rt.block_on(async {
            let mut tx = conn.begin().await?;
            tx.execute(SAME_FOLDER_DELETE, &params![("path", path_str.as_str())])
                .await?;
            tx.execute(
                "INSERT INTO recent_folders (path, opened_at) VALUES (:path, :opened_at) \
                 ON CONFLICT(path) DO UPDATE SET opened_at = :opened_at;",
                &params![("path", path_str.as_str()), ("opened_at", now),],
            )
            .await?;
            tx.commit().await?;
            Ok::<(), SettingsError>(())
        })
    }

    /// Removes a folder path from the recent folders list.
    #[note("settings-forget-recent-folder")]
    pub fn forget_recent_folder(&mut self, folder: &Path) -> Result<(), SettingsError> {
        let path_str = folder_key(folder);
        let conn = &mut self.conn;
        self.rt.block_on(async {
            let mut tx = conn.begin().await?;
            tx.execute(SAME_FOLDER_DELETE, &params![("path", path_str.as_str())])
                .await?;
            tx.commit().await?;
            Ok::<(), SettingsError>(())
        })
    }
}

/// One spelling per folder: on Windows `C:/a/b` and `C:\a\b\` are the same
/// recent, so `/` becomes `\` and a trailing separator goes.
pub fn folder_key(folder: &Path) -> String {
    let text = folder.to_string_lossy();
    let text = if cfg!(windows) {
        text.replace('/', "\\")
    } else {
        text.into_owned()
    };
    let trimmed = text.trim_end_matches(['/', '\\']);
    if trimmed.is_empty() || trimmed.ends_with(':') {
        text
    } else {
        trimmed.to_string()
    }
}

/// Removes every row naming the folder `:path` (already a [`folder_key`]), whatever
/// separators it was stored with.
const SAME_FOLDER_DELETE: &str = if cfg!(windows) {
    "DELETE FROM recent_folders WHERE REPLACE(path, '/', '\\') = :path;"
} else {
    "DELETE FROM recent_folders WHERE path = :path;"
};

#[cfg(test)]
#[path = "test/store/test.rs"]
mod tests;
