---
id: adapter-dataset-datasetstate-is-open
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 31
author: 
created: 
title: "DatasetState::is_open"
tags: []
signature: "pub const fn is_open (& self) -> bool"
---

Returns true if the dataset allows reading field data.

# Examples

```rust
use crawrad_binding::DatasetState;

assert!(DatasetState::Browse.is_open());
assert!(!DatasetState::Inactive.is_open());
```
