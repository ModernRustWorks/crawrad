//! # `crawrad-binding` — Unified Data Binding Engine for CrawRAD
//!
//! Implements the foundational data binding architecture established in [ADR 0041]
//! and [ADR 0036].
//!
//! ## Core Architecture
//!
//! 1. **Single Visual Family**: Visual UI controls (`Edit`, `CheckBox`, `ComboBox`, `Grid`)
//!    are completely decoupled from physical database access. There are no duplicate `DB*` controls.
//! 2. **Four Canonical Modes**:
//!    - `OneWay`: Source -> Target (Read-only UI)
//!    - `TwoWay`: Source <-> Target (Bidirectional UI sync)
//!    - `OneWayToSource`: Target -> Source (Form dispatch)
//!    - `OneTime`: Source -> Target (One-time load)
//! 3. **Scalar vs Collection**:
//!    - `ScalarBinding`: atomic properties (`Edit.text` <-> `customer.name`)
//!    - `CollectionBinding`: tabular datasets with cursor navigation, selection, and
//!      virtual GPU window slicing (`offset..offset + limit`).
//! 4. **Adapters**:
//!    - `ModelAdapter`: bidirectional property projection for business structs and REST APIs.
//!    - `DatasetAdapter`: relational datasets with classic `Browse/Edit/Insert` lifecycle.
//!    - `PortaDbBridge`: seamless connection to the `portadb` database framework.
//!

#![forbid(unsafe_code)]

pub mod adapter;
pub mod collection;
pub mod converter;
pub mod mode;
pub mod observable;
pub mod scalar;

#[cfg(feature = "portadb")]
pub use adapter::PortaDbBridge;
pub use adapter::{
    DataRecord, DatasetAdapter, DatasetState, MemoryStore, ModelAdapter, RecordStore,
};
pub use collection::{CollectionBinding, CollectionChange};
pub use converter::{ClosureConverter, DisplayFromStrConverter, IdentityConverter, ValueConverter};
pub use mode::BindingMode;
pub use observable::{Observable, SubscriptionId};
pub use scalar::ScalarBinding;
