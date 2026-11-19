---
id: makepadruntime-set-snap-ghost
kind: doc
target_file: src/lib.rs
line_hint: 316
author: 
created: 
title: "MakepadRuntime::set_snap_ghost"
tags: []
signature: "pub fn set_snap_ghost (& mut self , ghost : Option < (Rect , Rect) >)"
---

The landing and the pointer's place of a move that snaps; `None` clears.

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_runtime_makepad::MakepadRuntime;

let mut rt = MakepadRuntime::new();
rt.set_snap_ghost(Some((Rect::new(0.0, 0.0, 10.0, 10.0), Rect::new(5.0, 5.0, 10.0, 10.0))));
```
