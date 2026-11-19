---
id: design-rows-grid-binding
kind: doc
target_file: src/design_rows.rs
line_hint: 139
author: 
created: 
title: "grid_binding"
tags: []
signature: "pub fn grid_binding < 'a > (doc : & 'a crate :: Document , grid : & crate :: Node) -> Option < GridBinding < 'a > >"
---

Follows a Grid's data_source to its connection, through a dataset if any.

# Examples

```rust
use crawrad_ir::{Document, Node};
use crawrad_ir::design_rows::grid_binding;

let doc = Document::new("form_main", "Form");
let grid = Node::new("grid1", "Grid");
assert!(grid_binding(&doc, &grid).is_none());
```
