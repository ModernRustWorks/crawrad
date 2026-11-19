---
id: components-comum-makepad-snap-pointer
kind: doc
target_file: src/components/comum.rs
line_hint: 49
author: 
created: 
title: "MAKEPAD_SNAP_POINTER"
tags: []
signature: "pub const MAKEPAD_SNAP_POINTER : [f32 ; 4] = rgb (0x9A , 0x9E , 0xB5)"
---

Where the pointer would have put a control that snapped (#564).

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_SNAP_POINTER;

assert_eq!(MAKEPAD_SNAP_POINTER[3], 1.0);
```
