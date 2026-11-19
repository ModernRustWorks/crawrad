//! The registry the engine resolves drivers from.
//!
//! It lives here and not in `portadb-core` because only this crate knows which
//! providers the application turned on (the cargo features).

use std::sync::{Arc, OnceLock};

use crate::driver::{Driver, DriverRegistry};

/// The shared registry, holding every provider whose cargo feature is on.
pub fn registry() -> &'static DriverRegistry {
    static REGISTRY: OnceLock<DriverRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        let reg = DriverRegistry::new();
        register_defaults(&reg);
        reg
    })
}

/// Registers a driver with the shared [`registry`].
///
/// This enables third-party crates to add drivers without editing any portadb code.
pub fn register_driver(driver: impl Driver) {
    registry().register(Arc::new(driver));
}

fn register_defaults(reg: &DriverRegistry) {
    #[cfg(feature = "memory")]
    reg.register(Arc::new(crate::drivers::MemoryDriver::new()));
    #[cfg(feature = "rusqlite")]
    reg.register(Arc::new(crate::drivers::RusqliteDriver::new()));
    #[cfg(feature = "sqlx-sqlite")]
    reg.register(Arc::new(crate::drivers::SqlxDriver::new()));
    #[cfg(feature = "sqlx-postgres")]
    reg.register(Arc::new(crate::drivers::SqlxPostgresDriver::new()));
    reg.register(Arc::new(crate::drivers::RedisDriver::new()));
}
