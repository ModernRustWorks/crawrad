---
id: props-props-grid-lines
kind: doc
target_file: src/props.rs
line_hint: 1824
author: 
created: 
title: "Props::grid_lines"
tags: []
signature: "pub fn grid_lines (default : bool) -> PropDesc"
---

Grid lines display in Grid (`GridLines`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::grid_lines(true);
assert_eq!(p.key, "grid_lines");
```
