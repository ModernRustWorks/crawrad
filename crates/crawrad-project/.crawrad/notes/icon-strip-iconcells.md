---
id: icon-strip-iconcells
kind: doc
target_file: src/icon_strip.rs
line_hint: 60
author: 
created: 
title: "IconCells"
tags: []
signature: "pub struct IconCells"
---

The cells of a strip: the 52 native icons, then one per third-party icon.

# Examples

```rust
use crawrad_project::icon_strip::IconCells;

let cells = IconCells::default();
assert!(cells.count() > 0.0);
```
