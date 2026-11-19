---
id: adapter-dataset-datasetadapter-delete
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 203
author: 
created: 
title: "DatasetAdapter::delete"
tags: []
signature: "pub fn delete (& self) -> Result < () , String >"
---

Delete the current record and advance or reposition cursor.

# Examples

```rust
use crawrad_binding::{DataRecord, DatasetAdapter};

let mut row = DataRecord::new();
row.insert("id".to_string(), "1".to_string());
let adapter = DatasetAdapter::new(&["id"], vec![row]);
assert!(adapter.delete().is_ok());
assert_eq!(adapter.collection().len(), 0);
```

# Errors

Returns an error if no record is active or if the underlying store fails to delete.
