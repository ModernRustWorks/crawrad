---
id: adapter-portadb-bridge-portadbbridge
kind: doc
target_file: src/adapter/portadb_bridge.rs
line_hint: 14
author: 
created: 
title: "PortaDbBridge"
tags: []
signature: "pub struct PortaDbBridge"
---

Bridges a database query view or reactive dataset to a visual adapter.

Bridge utility connecting a database query view ([`DataSource`]) or reactive [`Dataset`]
to a visual [`DatasetAdapter`].

# Examples

```rust
use portadb::{ColumnDef, DataType, DataSource, Row, TableSchema, Value};
use crawrad_binding::PortaDbBridge;

let schema = TableSchema::new().with_column(ColumnDef::new("id", DataType::Integer, false));
let row = Row::new(vec!["id".to_string()], vec![Value::Integer(1)]);
let ds = DataSource::new(schema, vec![row]);
let adapter = PortaDbBridge::from_datasource(&ds);
assert_eq!(adapter.collection().len(), 1);
```
