---
id: adapter-dataset-datasetadapter
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 63
author: 
created: 
title: "DatasetAdapter"
tags: []
signature: "pub struct DatasetAdapter"
---

Adapter connecting a multi-row dataset to visual UI controls.

Adapter connecting a multi-row dataset to UI controls (e.g. Grids and Forms).

It decides nothing: the records, the cursor and the edit state live in the
[`RecordStore`], and the cells, the collection and the state observable are a
mirror re-read on every change the store announces.

# Examples

```rust
use crawrad_binding::DatasetAdapter;

let adapter = DatasetAdapter::new(&["id", "name"], Vec::new());
assert_eq!(adapter.collection().len(), 0);
```
