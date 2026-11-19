use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::params::NamedParams;
use crate::row::Row;

#[derive(Clone)]
struct CacheEntry {
    rows: Arc<Vec<Row>>,
    expires_at: Instant,
}

/// Contract for query result cache providers (L1 Memory, L2 Redis, etc.).
pub trait CacheProvider: Send + Sync + 'static {
    /// Retrieves cached query rows if present and not expired.
    fn get(&self, key: &str) -> Option<Arc<Vec<Row>>>;

    /// Stores query rows with a specific Time-To-Live (TTL).
    fn set(&self, key: &str, rows: Arc<Vec<Row>>, ttl: Duration);

    /// Invalidates a specific cached key.
    fn invalidate(&self, key: &str);

    /// Clears all entries in this cache provider.
    fn clear(&self);
}

/// In-memory thread-safe L1 query result cache with automatic expiration check on read.
#[derive(Default, Clone)]
pub struct MemoryCache {
    store: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl MemoryCache {
    /// Creates a new in-memory cache provider.
    #[must_use]
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl CacheProvider for MemoryCache {
    fn get(&self, key: &str) -> Option<Arc<Vec<Row>>> {
        let store = self.store.read().ok()?;
        let entry = store.get(key)?;
        if Instant::now() < entry.expires_at {
            Some(Arc::clone(&entry.rows))
        } else {
            None
        }
    }

    fn set(&self, key: &str, rows: Arc<Vec<Row>>, ttl: Duration) {
        if let Ok(mut store) = self.store.write() {
            let expires_at = Instant::now() + ttl;
            store.insert(key.to_string(), CacheEntry { rows, expires_at });
        }
    }

    fn invalidate(&self, key: &str) {
        if let Ok(mut store) = self.store.write() {
            store.remove(key);
        }
    }

    fn clear(&self) {
        if let Ok(mut store) = self.store.write() {
            store.clear();
        }
    }
}

/// Computes a deterministic cache key from an SQL statement and its parameters.
#[must_use]
pub fn cache_key(sql: &str, params: &NamedParams) -> String {
    let mut key = sql.trim().to_string();
    if !params.is_empty() {
        key.push('|');
        // Sort keys for determinism
        let mut entries: Vec<_> = params.iter().collect();
        entries.sort_by_key(|(k, _)| *k);
        for (k, v) in entries {
            key.push_str(k);
            key.push('=');
            key.push_str(&v.to_string());
            key.push(';');
        }
    }
    key
}
