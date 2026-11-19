---
id: components-comum-makepad-grid-faint
kind: doc
target_file: src/components/comum.rs
line_hint: 46
author: 
created: 
title: "MAKEPAD_GRID_FAINT"
tags: []
signature: "pub const MAKEPAD_GRID_FAINT : [f32 ; 4] = rgb (0x52 , 0x55 , 0x6B)"
---

The grid colour when snap is disabled.

The grid with Snap OFF (#564): halfway to the paper, still there with
`Display grid` on, so Snap ON and OFF tell apart at rest.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_GRID_FAINT;

assert_eq!(MAKEPAD_GRID_FAINT[3], 1.0);
```
