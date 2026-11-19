---
id: makepadruntime-rect-da-alca
kind: doc
target_file: src/lib.rs
line_hint: 346
author: 
created: 
title: "MakepadRuntime::rect_da_alca"
tags: []
signature: "pub fn rect_da_alca (& self) -> Option < Rect >"
---

The rectangle that receives the EIGHT handles, or `None` when there is none.

Four rejections, and each one matches a designer path:

1. only the **first** in the selection, because that is the only one where
   `Designer::hit_handle` looks (`crawrad-designer/src/lib.rs:511`);
2. never the **form**, which has three handles and its own path
   (`hit_form_handle`, `:494`);
3. never a node **off-screen** — see [`Self::desenhavel`];
4. never a **non-visual**, which does not resize.

A single function, because what DRAWS the handle and what chooses its CURSOR
must agree. When they were two readings, the arrow appeared over
something that could not be dragged.

# Examples

```rust
use crawrad_runtime_makepad::MakepadRuntime;

let rt = MakepadRuntime::new();
assert_eq!(rt.rect_da_alca(), None);
```
