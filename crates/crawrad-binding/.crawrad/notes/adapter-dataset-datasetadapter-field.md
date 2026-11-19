---
id: adapter-dataset-datasetadapter-field
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 150
author: 
created: 
title: "DatasetAdapter::field"
tags: []
signature: "pub fn field (& self , name : & str) -> Option < Observable < String > >"
---

Retrieve an observable cell for the specified field.

# Examples

```rust
use crawrad_binding::DatasetAdapter;

let adapter = DatasetAdapter::new(&["name"], Vec::new());
assert!(adapter.field("name").is_some());
```

# Panics

Panics if the internal fields mutex is poisoned.
