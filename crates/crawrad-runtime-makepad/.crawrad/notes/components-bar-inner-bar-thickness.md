---
id: components-bar-inner-bar-thickness
kind: doc
target_file: src/components/bar.rs
line_hint: 49
author: 
created: 
title: "INNER_BAR_THICKNESS"
tags: []
signature: "pub const INNER_BAR_THICKNESS : f32 = 14.0"
---

The thickness of the bar a container draws inside itself, in design units.

The thickness of the bar a CONTAINER draws inside itself, in design units.
It is Windows's `GetSystemMetrics(SM_CXVSCROLL)`, rounded.

# Examples

```rust
use crawrad_runtime_makepad::components::bar::INNER_BAR_THICKNESS;

assert_eq!(INNER_BAR_THICKNESS, 14.0);
```
