---
id: adapter-dataset-datasetstate-as-str
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 43
author: 
created: 
title: "DatasetState::as_str"
tags: []
signature: "pub const fn as_str (& self) -> & 'static str"
---

Returns the canonical display name.

# Examples

```rust
use crawrad_binding::DatasetState;

assert_eq!(DatasetState::Browse.as_str(), "Browse");
```
