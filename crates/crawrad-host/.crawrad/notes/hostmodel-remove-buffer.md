---
id: hostmodel-remove-buffer
kind: doc
target_file: src/lib.rs
line_hint: 422
author: 
created: 
title: "HostModel::remove_buffer"
tags: []
signature: "pub fn remove_buffer (& mut self , path : & Path) -> Option < String >"
---

Removes the in-memory edit for `path`, returning the content if present.

# Examples

```rust
use crawrad_host::HostModel;
use std::path::Path;

let mut host = HostModel::default();
host.set_buffer("a.rs".into(), "hello".into());
assert_eq!(host.remove_buffer(Path::new("a.rs")), Some("hello".into()));
assert_eq!(host.buffer(Path::new("a.rs")), None);
```
