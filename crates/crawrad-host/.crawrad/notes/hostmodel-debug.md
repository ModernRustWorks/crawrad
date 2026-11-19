---
id: hostmodel-debug
kind: doc
target_file: src/lib.rs
line_hint: 297
author: 
created: 
title: "HostModel::debug"
tags: []
signature: "pub fn debug (& self) -> & Debug"
---

The debug session (issue #276).

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(!host.debug().is_active());
```
