//! # Observable Tabular View Layer
//!
//! Provides reactive tabular data views ([`TableView`]), datasets ([`Dataset`]),
//! and design-time schema declarations ([`TableSchema`])
//! linking database query results directly to UI components (data grids, lists, detail forms).

pub mod dataset;
pub mod event;
pub mod schema;
pub mod table;

pub use dataset::{Dataset, DatasetEvent, DatasetSource, DatasetState};
pub use event::{ChangeKind, ChangeSignal, TableEvent, TableSignal};
pub use schema::{ColumnDef, DataType, FieldDef, FieldType, SchemaDescriptor, TableSchema};
pub use table::{DataCursor, DataSource, TableState, TableView};
