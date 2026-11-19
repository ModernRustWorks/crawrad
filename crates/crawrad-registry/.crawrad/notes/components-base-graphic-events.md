---
id: components-base-graphic-events
kind: doc
target_file: src/components/base.rs
line_hint: 148
author: 
created: 
title: "graphic_events"
tags: []
signature: "pub fn graphic_events () -> Vec < EventDesc >"
---

The events classic RAD gives a control drawn on its parent: the mouse.

# Examples

```rust
use crawrad_registry::graphic_events;

let events = graphic_events();
assert!(!events.is_empty());
```
