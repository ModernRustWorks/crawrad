---
id: events-pointerhit
kind: doc
target_file: src/events.rs
line_hint: 72
author: 
created: 
title: "PointerHit"
tags: []
signature: "pub struct PointerHit"
---

Represents one pointer event targeted to a specific control.

One event for one control: its index in the rect list, the kind, and where
the pointer is in the control's own coordinates.

# Examples

```rust
use makepad_widgets::{dvec2, KeyModifiers};
use crawrad_runtime_makepad::events::{Pointer, PointerHit};

let hit = PointerHit {
    control: 0,
    kind: Pointer::Down,
    x: 10.0,
    y: 5.0,
    at: dvec2(10.0, 5.0),
    button: None,
    modifiers: KeyModifiers::default(),
};
assert_eq!(hit.control, 0);
```
