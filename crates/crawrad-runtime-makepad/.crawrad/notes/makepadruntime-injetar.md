---
id: makepadruntime-injetar
kind: doc
target_file: src/lib.rs
line_hint: 336
author: 
created: 
title: "MakepadRuntime::injetar"
tags: []
signature: "pub fn injetar (& mut self , e : RuntimeEvent)"
---

Injects an input event into the runtime event queue.

# Examples

```rust
use crawrad_runtime::RuntimeEvent;
use crawrad_runtime_makepad::MakepadRuntime;

let mut rt = MakepadRuntime::new();
rt.injetar(RuntimeEvent::PointerUp { x: 0.0, y: 0.0 });
```
