---
id: hostmodel-bookmarks
kind: doc
target_file: src/lib.rs
line_hint: 220
author: 
created: 
title: "HostModel::bookmarks"
tags: []
signature: "pub fn bookmarks (& self) -> & Bookmarks"
---

The numbered bookmarks of the session.

# Examples

```rust
use std::path::Path;
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(host.bookmarks().of(Path::new("src/main.rs")).is_empty());
```
