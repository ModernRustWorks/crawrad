---
id: makepadruntime-quantidade
kind: doc
target_file: src/lib.rs
line_hint: 326
author: 
created: 
title: "MakepadRuntime::quantidade"
tags: []
signature: "pub fn quantidade (& self) -> usize"
---

Number of active visual controls in the runtime.

# Examples

```rust
use crawrad_runtime_makepad::MakepadRuntime;

let rt = MakepadRuntime::new();
assert_eq!(rt.quantidade(), 0);
```
