---
id: adapter-dataset-datasetadapter-refresh
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 208
author: 
created: 
title: "DatasetAdapter::refresh"
tags: []
signature: "pub fn refresh (& self) -> Result < () , String >"
---

Reload records from the underlying store and update the adapter.

# Examples

```rust
use crawrad_binding::DatasetAdapter;

let adapter = DatasetAdapter::new(&["id"], Vec::new());
assert!(adapter.refresh().is_ok());
```

# Errors

Returns an error if reloading from the underlying store fails.
