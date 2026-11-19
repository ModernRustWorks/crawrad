---
id: hostmodel-new
kind: doc
target_file: src/lib.rs
line_hint: 136
author: 
created: 
title: "HostModel::new"
tags: []
signature: "pub fn new () -> Self"
---

Creates an empty `HostModel` with no active document.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(!host.has_doc());
```
