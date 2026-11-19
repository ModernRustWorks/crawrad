---
id: adapter-dataset-datasetadapter-with-store
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 92
author: 
created: 
title: "DatasetAdapter::with_store"
tags: []
signature: "pub fn with_store (store : Arc < dyn RecordStore >) -> Self"
---

Construct a new dataset adapter driven by an arbitrary [`RecordStore`] backend.

# Examples

```rust
use std::sync::Arc;
use crawrad_binding::{DatasetAdapter, MemoryStore};

let store = Arc::new(MemoryStore::new(&["id"], Vec::new()));
let adapter = DatasetAdapter::with_store(store);
assert_eq!(adapter.collection().len(), 0);
```
