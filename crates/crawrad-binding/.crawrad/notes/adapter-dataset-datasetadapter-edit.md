---
id: adapter-dataset-datasetadapter-edit
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 170
author: 
created: 
title: "DatasetAdapter::edit"
tags: []
signature: "pub fn edit (& self) -> Result < () , String >"
---

Transition to `Edit` mode, capturing a snapshot for potential `cancel()`.

# Examples

```rust
use crawrad_binding::{DataRecord, DatasetAdapter, DatasetState};

let mut row = DataRecord::new();
row.insert("id".to_string(), "1".to_string());
let adapter = DatasetAdapter::new(&["id"], vec![row]);
assert!(adapter.edit().is_ok());
assert_eq!(adapter.state().get(), DatasetState::Edit);
```

# Errors

Returns an error if the underlying store cannot enter edit mode.
