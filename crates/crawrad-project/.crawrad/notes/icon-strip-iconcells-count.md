---
id: icon-strip-iconcells-count
kind: doc
target_file: src/icon_strip.rs
line_hint: 86
author: 
created: 
title: "IconCells::count"
tags: []
signature: "pub fn count (& self) -> f64"
---

Total number of icons available in this strip (native + extra).

# Examples

```rust
use crawrad_project::icon_strip::IconCells;

let cells = IconCells::default();
assert!(cells.count() > 0.0);
```
