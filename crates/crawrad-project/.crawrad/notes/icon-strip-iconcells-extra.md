---
id: icon-strip-iconcells-extra
kind: doc
target_file: src/icon_strip.rs
line_hint: 71
author: 
created: 
title: "IconCells::extra"
tags: []
signature: "pub fn extra (& self) -> & [(String , Vec < IconStep >)]"
---

The third-party icons, in the order of their cells.

# Examples

```rust
use crawrad_project::icon_strip::IconCells;

let cells = IconCells::new(Vec::new());
assert!(cells.extra().is_empty());
```
