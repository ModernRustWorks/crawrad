---
id: makepadruntime-new
kind: doc
target_file: src/lib.rs
line_hint: 286
author: 
created: 
title: "MakepadRuntime::new"
tags: []
signature: "pub fn new () -> Self"
---

Creates a new native Makepad runtime instance.

# Examples

```rust
use crawrad_runtime_makepad::MakepadRuntime;

let rt = MakepadRuntime::new();
assert_eq!(rt.quantidade(), 0);
```
