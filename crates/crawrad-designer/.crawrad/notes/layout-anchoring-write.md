---
id: layout-anchoring-write
kind: doc
target_file: src/layout.rs
line_hint: 323
author: 
created: 
title: "Anchoring::write"
tags: []
signature: "fn write (self) -> Value"
---

The shape that `.crawform` records — a `Set` of four flags.

# Examples

```rust
use crawrad_designer::layout::Anchoring;
use crawrad_component::Anchors;

let val = Anchors::default().write();
assert!(matches!(val, crawrad_ir::Value::Map(_)));
```
