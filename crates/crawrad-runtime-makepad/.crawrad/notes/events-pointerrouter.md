---
id: events-pointerrouter
kind: doc
target_file: src/events.rs
line_hint: 86
author: 
created: 
title: "PointerRouter"
tags: []
signature: "pub struct PointerRouter"
---

Tracks pointer state and routes pointer events to controls.

What the router remembers between events: the control under the pointer
and the one the press went to.

# Examples

```rust
use crawrad_runtime_makepad::events::PointerRouter;

let router = PointerRouter::default();
assert_eq!(router, PointerRouter::default());
```
