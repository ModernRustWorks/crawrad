---
id: components-comum-makepad-snap
kind: doc
target_file: src/components/comum.rs
line_hint: 52
author: 
created: 
title: "MAKEPAD_SNAP"
tags: []
signature: "pub const MAKEPAD_SNAP : [f32 ; 4] = rgb (0xDE , 0x5A , 0x2B)"
---

The accent color where a snapped control lands.

Where a snapped control lands, and the ruler marks of its edges: the
IDE's own accent, the one the Snap button lights up in (#564).

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_SNAP;

assert_eq!(MAKEPAD_SNAP[3], 1.0);
```
