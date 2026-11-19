---
id: layout-margins-from-value
kind: doc
target_file: src/layout.rs
line_hint: 64
author: 
created: 
title: "Margins::from_value"
tags: []
signature: "pub fn from_value (v : Option < & Value >) -> Edges"
---

Margins written in the property, **without** looking at the switch.

# Examples

```rust
use crawrad_designer::layout::Margins;

let edges = Margins::from_value(None);
assert_eq!(edges, Margins::DEFAULT);
```
