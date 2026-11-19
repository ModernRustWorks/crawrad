---
id: hostmodel-set-active-file
kind: doc
target_file: src/lib.rs
line_hint: 336
author: 
created: 
title: "HostModel::set_active_file"
tags: []
signature: "pub fn set_active_file (& mut self , path : Option < PathBuf >)"
---

Sets the active editor file path.

# Examples

```rust
use std::path::PathBuf;
use crawrad_host::HostModel;

let mut host = HostModel::new();
host.set_active_file(Some(PathBuf::from("src/main.rs")));
assert!(host.active_file().is_some());
```
