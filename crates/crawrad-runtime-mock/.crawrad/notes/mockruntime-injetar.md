---
id: mockruntime-injetar
kind: doc
target_file: src/lib.rs
line_hint: 70
author: 
created: 
title: "MockRuntime::injetar"
tags: []
signature: "pub fn injetar (& mut self , e : RuntimeEvent)"
---

Injects an event as if the user had made the gesture.

# Examples

```rust
use crawrad_runtime_mock::MockRuntime;
use crawrad_runtime::{Runtime, RuntimeEvent};

let mut rt = MockRuntime::new();
rt.injetar(RuntimeEvent::Cancel);
assert_eq!(rt.drain_events(), vec![RuntimeEvent::Cancel]);
```
