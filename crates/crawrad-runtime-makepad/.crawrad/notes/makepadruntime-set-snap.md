---
id: makepadruntime-set-snap
kind: doc
target_file: src/lib.rs
line_hint: 311
author: 
created: 
title: "MakepadRuntime::set_snap"
tags: []
signature: "pub fn set_snap (& mut self , on : bool)"
---

Snap on or off, which the grid shows (#564).

# Examples

```rust
use crawrad_runtime_makepad::MakepadRuntime;

let mut rt = MakepadRuntime::new();
rt.set_snap(false);
```
