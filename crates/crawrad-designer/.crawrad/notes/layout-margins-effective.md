---
id: layout-margins-effective
kind: doc
target_file: src/layout.rs
line_hint: 72
author: 
created: 
title: "Margins::effective"
tags: []
signature: "pub fn effective (n : & Node) -> Edges"
---

Returns the margins that `Align` should honour for a node.

The margins that `Align` should honour: those of the property when
`AlignWithMargins` is enabled, and four zeros when it is not.

# Examples

```rust
use crawrad_designer::layout::Margins;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
assert_eq!(Margins::effective(&node).top, 0.0);
```
