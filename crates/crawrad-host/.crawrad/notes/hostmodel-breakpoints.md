---
id: hostmodel-breakpoints
kind: doc
target_file: src/lib.rs
line_hint: 211
author: 
created: 
title: "HostModel::breakpoints"
tags: []
signature: "pub fn breakpoints (& self) -> & Breakpoints"
---

The breakpoints of the session (issue #276).

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert_eq!(host.breakpoints().count(), 0);
```
