---
id: runtimeevent
kind: doc
target_file: src/lib.rs
line_hint: 120
author: 
created: 
title: "RuntimeEvent"
tags: []
signature: "pub enum RuntimeEvent"
---

User input event on the design surface in design units.

What the user did on the design surface, already converted to
**design units**. The runtime translates; the designer decides.

# Examples

```rust
use crawrad_runtime::RuntimeEvent;

let event = RuntimeEvent::PointerDown { x: 10.0, y: 20.0, ctrl: false };
match event {
    RuntimeEvent::PointerDown { x, y, .. } => assert_eq!(x, 10.0),
    _ => unreachable!(),
}
```
