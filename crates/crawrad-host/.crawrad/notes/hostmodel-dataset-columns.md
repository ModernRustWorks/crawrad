---
id: hostmodel-dataset-columns
kind: doc
target_file: src/lib.rs
line_hint: 176
author: 
created: 
title: "HostModel::dataset_columns"
tags: []
signature: "pub fn dataset_columns (& self , project_dir : Option < & Path > , id : & str) -> Option < Result < Vec < String > , String > >"
---

The columns of `dataset_id` when the probe has answered, or why it could not.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(host.dataset_columns(None, "ds").is_none());
```
