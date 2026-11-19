---
id: adapter-dataset-datasetadapter-new
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 86
author: 
created: 
title: "DatasetAdapter::new"
tags: []
signature: "pub fn new (schema_fields : & [& str] , records : Vec < DataRecord >) -> Self"
---

Constructs a new dataset adapter using an in-memory store.

Construct a new dataset adapter populated with initial records using an in-memory store.

# Examples

```rust
use crawrad_binding::DatasetAdapter;

let adapter = DatasetAdapter::new(&["name"], Vec::new());
assert_eq!(adapter.collection().len(), 0);
```
