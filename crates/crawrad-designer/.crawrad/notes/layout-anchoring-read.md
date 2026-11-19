---
id: layout-anchoring-read
kind: doc
target_file: src/layout.rs
line_hint: 320
author: 
created: 
title: "Anchoring::read"
tags: []
signature: "fn read (v : Option < & Value >) -> Self"
---

The anchors that the document stores, or the default of the classic IDE.

# Examples

```rust
use crawrad_designer::layout::Anchoring;
use crawrad_component::Anchors;

let a = Anchors::read(None);
assert!(a.top);
```
