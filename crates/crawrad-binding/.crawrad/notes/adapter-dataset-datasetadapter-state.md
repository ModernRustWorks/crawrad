---
id: adapter-dataset-datasetadapter-state
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 135
author: 
created: 
title: "DatasetAdapter::state"
tags: []
signature: "pub fn state (& self) -> & Observable < DatasetState >"
---

Access the state observable.

# Examples

```rust
use crawrad_binding::{DatasetAdapter, DatasetState};

// No rows: the dataset starts inactive.
let adapter = DatasetAdapter::new(&["id"], Vec::new());
assert_eq!(adapter.state().get(), DatasetState::Inactive);
```
