---
id: adapter-dataset-datasetadapter-last-error
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 145
author: 
created: 
title: "DatasetAdapter::last_error"
tags: []
signature: "pub fn last_error (& self) -> Observable < Option < String > >"
---

Access the last encountered operation or validation error.

# Examples

```rust
use crawrad_binding::DatasetAdapter;

let adapter = DatasetAdapter::new(&["id"], Vec::new());
assert_eq!(adapter.last_error().get(), None);
```
