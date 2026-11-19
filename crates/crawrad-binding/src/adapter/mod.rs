//! Source adapters linking business models and relational datasets to the CrawRAD binding engine.

pub mod dataset;
pub mod model;
pub mod store;

#[cfg(feature = "portadb")]
pub mod portadb_bridge;

pub use dataset::{DataRecord, DatasetAdapter, DatasetState};
pub use model::ModelAdapter;
pub use store::{MemoryStore, RecordStore};

#[cfg(feature = "portadb")]
pub use portadb_bridge::PortaDbBridge;
