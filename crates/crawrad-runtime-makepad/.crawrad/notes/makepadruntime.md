---
id: makepadruntime
kind: doc
target_file: src/lib.rs
line_hint: 243
author: 
created: 
title: "MakepadRuntime"
tags: []
signature: "pub struct MakepadRuntime"
---

The Makepad runtime managing the control hierarchy and DSL generation.

# Examples

```rust
use crawrad_runtime_makepad::MakepadRuntime;

let rt = MakepadRuntime::default();
assert_eq!(rt.quantidade(), 0);
```
