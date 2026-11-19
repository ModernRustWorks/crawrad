---
id: snap-hint
kind: doc
target_file: src/lib.rs
line_hint: 160
author: 
created: 
title: "SNAP_HINT"
tags: []
signature: "pub const SNAP_HINT : & str = \"Snap to grid (hold Alt to place freely)\""
---

The Snap button's tooltip: the switch and the way around it (#564).

# Examples

```rust
use crawrad_designer::SNAP_HINT;

assert!(SNAP_HINT.contains("Snap"));
```
