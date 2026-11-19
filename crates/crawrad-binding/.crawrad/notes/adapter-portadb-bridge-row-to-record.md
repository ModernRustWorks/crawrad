---
id: adapter-portadb-bridge-row-to-record
kind: doc
target_file: src/adapter/portadb_bridge.rs
line_hint: 109
author: 
created: 
title: "row_to_record"
tags: []
signature: "pub fn row_to_record (schema : & TableSchema , row : & Row) -> DataRecord"
---

Converts a database [`Row`] into an in-memory string record.

Converts a database [`Row`] into an in-memory string record mapped by column name.

# Examples

```rust
use portadb::{ColumnDef, DataType, Row, TableSchema, Value};
use crawrad_binding::adapter::portadb_bridge::row_to_record;

let schema = TableSchema::new().with_column(ColumnDef::new("name", DataType::Text, false));
let row = Row::new(vec!["name".to_string()], vec![Value::Text("Test".to_string())]);
let record = row_to_record(&schema, &row);
assert_eq!(record.get("name").map(|s| s.as_str()), Some("Test"));
```
