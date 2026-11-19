---
id: adapter-dataset-datasetadapter-cancel
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 198
author: 
created: 
title: "DatasetAdapter::cancel"
tags: []
signature: "pub fn cancel (& self) -> Result < () , String >"
---

Cancel current edits or insert, rolling back to the previous snapshot.

# Examples

```rust
use crawrad_binding::{DatasetAdapter, DatasetState};

let adapter = DatasetAdapter::new(&["id"], Vec::new());
adapter.insert().unwrap();
assert!(adapter.cancel().is_ok());
// Back to the state before the insert: no rows, inactive.
assert_eq!(adapter.state().get(), DatasetState::Inactive);
```

# Errors

Returns an error if the underlying store cannot cancel the current operation.
