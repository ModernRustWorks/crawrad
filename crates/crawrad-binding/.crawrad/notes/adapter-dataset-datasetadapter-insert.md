---
id: adapter-dataset-datasetadapter-insert
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 175
author: 
created: 
title: "DatasetAdapter::insert"
tags: []
signature: "pub fn insert (& self) -> Result < () , String >"
---

Transition to `Insert` mode, initializing fields with empty/default values.

# Examples

```rust
use crawrad_binding::{DatasetAdapter, DatasetState};

let adapter = DatasetAdapter::new(&["id"], Vec::new());
assert!(adapter.insert().is_ok());
assert_eq!(adapter.state().get(), DatasetState::Insert);
```

# Errors

Returns an error if the underlying store cannot enter insert mode.
