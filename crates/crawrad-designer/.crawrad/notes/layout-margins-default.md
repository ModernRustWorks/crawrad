---
id: layout-margins-default
kind: doc
target_file: src/layout.rs
line_hint: 61
author: 
created: 
title: "Margins::DEFAULT"
tags: []
signature: "pub const DEFAULT : Edges = Edges :: uniform (3.0)"
---

The default of the classic component library: 3 on each side.

# Examples

```rust
use crawrad_designer::layout::Margins;

assert_eq!(Margins::DEFAULT.left, 3.0);
```
