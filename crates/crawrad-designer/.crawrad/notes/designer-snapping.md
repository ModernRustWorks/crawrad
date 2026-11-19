---
id: designer-snapping
kind: doc
target_file: src/lib.rs
line_hint: 292
author: 
created: 
title: "Designer::snapping"
tags: []
signature: "pub fn snapping (& self) -> bool"
---

Whether a gesture lands on the grid right now.

# Examples

```rust
use crawrad_designer::Designer;

let d = Designer::default();
assert!(d.snapping());
```
