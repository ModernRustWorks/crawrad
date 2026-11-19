---
id: hostmodel-open-file-paths
kind: doc
target_file: src/lib.rs
line_hint: 357
author: 
created: 
title: "HostModel::open_file_paths"
tags: []
signature: "pub fn open_file_paths (& self) -> Vec < PathBuf >"
---

Returns the paths of all files that have unsaved in-memory edits.

# Examples

```rust
use crawrad_host::HostModel;
use std::path::PathBuf;

let mut host = HostModel::default();
host.set_buffer(PathBuf::from("src/main.rs"), "fn main() {}".into());
assert_eq!(host.open_file_paths(), vec![PathBuf::from("src/main.rs")]);
```
