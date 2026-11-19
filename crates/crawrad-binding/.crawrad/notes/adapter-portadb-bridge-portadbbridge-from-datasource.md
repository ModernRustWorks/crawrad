---
id: adapter-portadb-bridge-portadbbridge-from-datasource
kind: doc
target_file: src/adapter/portadb_bridge.rs
line_hint: 23
author: 
created: 
title: "PortaDbBridge::from_datasource"
tags: []
signature: "pub fn from_datasource (source : & DataSource) -> DatasetAdapter"
---

Create a [`DatasetAdapter`] directly from a populated [`DataSource`].

This provides a read-only mirror of a [`TableView`][portadb::TableView]; user edits made in
visual controls are not persisted back to the database. For full CRUD read/write synchronization,
prefer [`PortaDbBridge::from_dataset`].

# Examples

```rust
use portadb::{ColumnDef, DataType, DataSource, Row, TableSchema, Value};
use crawrad_binding::PortaDbBridge;

let schema = TableSchema::new().with_column(ColumnDef::new("title", DataType::Text, false));
let row = Row::new(vec!["title".to_string()], vec![Value::Text("CrawRAD".to_string())]);
let ds = DataSource::new(schema, vec![row]);
let adapter = PortaDbBridge::from_datasource(&ds);
assert_eq!(adapter.collection().len(), 1);
```
