---
id: live-design
kind: doc
target_file: src/lib.rs
line_hint: 111
author: 
created: 
title: "live_design"
tags: []
signature: "pub fn live_design (cx : & mut makepad_widgets :: Cx)"
---

Registers this crate's DSL in the `Cx`.

This is what the shell calls in `live_register`, after
`makepad_widgets::live_design`. The DSL here declares **shaders**, and nothing
more — the form is drawn by code (ADR 0027).

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::live_design;

fn register(cx: &mut Cx) {
    live_design(cx);
}
```
