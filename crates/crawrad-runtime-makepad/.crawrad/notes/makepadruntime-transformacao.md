---
id: makepadruntime-transformacao
kind: doc
target_file: src/lib.rs
line_hint: 375
author: 
created: 
title: "MakepadRuntime::transformacao"
tags: []
signature: "pub fn transformacao (& self) -> Transformacao"
---

Returns the coordinate transformation for the current viewport and scale.

# Examples

```rust
use crawrad_runtime_makepad::MakepadRuntime;

let rt = MakepadRuntime::new();
assert_eq!(rt.transformacao().escala, 1.0);
```
