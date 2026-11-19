---
id: hostmodel-set-buffer
kind: doc
target_file: src/lib.rs
line_hint: 417
author: 
created: 
title: "HostModel::set_buffer"
tags: []
signature: "pub fn set_buffer (& mut self , path : PathBuf , content : String)"
---

Inserts or replaces the in-memory edit for `path`.

# Examples

```rust
use crawrad_host::HostModel;
use std::path::Path;

let mut host = HostModel::default();
host.set_buffer("a.rs".into(), "hello".into());
assert_eq!(host.buffer(Path::new("a.rs")), Some("hello"));
```
