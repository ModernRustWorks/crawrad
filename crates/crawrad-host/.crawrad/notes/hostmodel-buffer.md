---
id: hostmodel-buffer
kind: doc
target_file: src/lib.rs
line_hint: 392
author: 
created: 
title: "HostModel::buffer"
tags: []
signature: "pub fn buffer (& self , path : & Path) -> Option < & str >"
---

Returns the in-memory content of `path`, if it has unsaved edits.

# Examples

```rust
use crawrad_host::HostModel;
use std::path::Path;

let mut host = HostModel::default();
host.set_buffer("src/lib.rs".into(), "content".into());
assert_eq!(host.buffer(Path::new("src/lib.rs")), Some("content"));
assert_eq!(host.buffer(Path::new("missing.rs")), None);
```
