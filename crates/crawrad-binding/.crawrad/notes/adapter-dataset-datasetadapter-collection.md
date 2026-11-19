---
id: adapter-dataset-datasetadapter-collection
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 140
author: 
created: 
title: "DatasetAdapter::collection"
tags: []
signature: "pub fn collection (& self) -> & CollectionBinding < DataRecord >"
---

Access the collection binding (for Grids and Lists).

# Examples

```rust
use crawrad_binding::DatasetAdapter;

let adapter = DatasetAdapter::new(&["id"], Vec::new());
assert_eq!(adapter.collection().len(), 0);
```
