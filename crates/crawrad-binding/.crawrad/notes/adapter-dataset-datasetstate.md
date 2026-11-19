---
id: adapter-dataset-datasetstate
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 16
author: 
created: 
title: "DatasetState"
tags: []
signature: "pub enum DatasetState"
---

The operational state of a dataset.

# Examples

```rust
use crawrad_binding::DatasetState;

let state = DatasetState::Browse;
assert!(state.is_open());
```
