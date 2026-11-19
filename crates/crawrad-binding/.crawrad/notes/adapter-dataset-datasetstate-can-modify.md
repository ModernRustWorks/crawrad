---
id: adapter-dataset-datasetstate-can-modify
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 37
author: 
created: 
title: "DatasetState::can_modify"
tags: []
signature: "pub const fn can_modify (& self) -> bool"
---

Returns true if field mutations are currently permitted.

# Examples

```rust
use crawrad_binding::DatasetState;

assert!(DatasetState::Edit.can_modify());
assert!(!DatasetState::Browse.can_modify());
```
