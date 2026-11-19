---
id: components-base-windowed-events
kind: doc
target_file: src/components/base.rs
line_hint: 161
author: 
created: 
title: "windowed_events"
tags: []
signature: "pub fn windowed_events () -> Vec < EventDesc >"
---

The events classic RAD gives a control that can take the focus.

The events classic RAD gives a control that can take the focus: the
mouse, the focus and the keyboard.

# Examples

```rust
use crawrad_registry::windowed_events;

let events = windowed_events();
assert!(events.len() > 7);
```
