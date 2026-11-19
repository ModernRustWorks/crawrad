---
id: value-edges
kind: doc
target_file: src/value.rs
line_hint: 865
author: 
created: 
title: "Edges"
tags: []
signature: "pub struct Edges"
---

Four sides — used for margin, padding, and numerical anchors.

# Examples

```rust
use crawrad_ir::Edges;

let e = Edges::uniform(8.0);
assert_eq!(e.left, 8.0);
assert_eq!(e.top, 8.0);
```
