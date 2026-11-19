---
id: makepadruntime-ctl
kind: doc
target_file: src/lib.rs
line_hint: 331
author: 
created: 
title: "MakepadRuntime::ctl"
tags: []
signature: "pub fn ctl (& self , id : & str) -> Option < & MakepadCtl >"
---

Retrieves a reference to a control by its Node ID.

# Examples

```rust
use crawrad_runtime_makepad::MakepadRuntime;

let rt = MakepadRuntime::new();
assert!(rt.ctl("btn1").is_none());
```
