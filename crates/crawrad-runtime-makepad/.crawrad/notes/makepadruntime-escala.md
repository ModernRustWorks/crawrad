---
id: makepadruntime-escala
kind: doc
target_file: src/lib.rs
line_hint: 370
author: 
created: 
title: "MakepadRuntime::escala"
tags: []
signature: "pub fn escala (& self) -> f32"
---

The effective canvas scale: the device factor times the IDE's zoom.

Exists so that the canvas sets up the [`Transformacao`] with the origin that only
it knows (the frame's turtle) without duplicating this multiplication —
ADR 0019, a single transformation.

# Examples

```rust
use crawrad_runtime_makepad::MakepadRuntime;

let rt = MakepadRuntime::new();
assert_eq!(rt.escala(), 1.0);
```
