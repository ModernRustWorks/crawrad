---
id: targets-shared-bound-grid-target
kind: doc
target_file: src/targets/shared.rs
line_hint: 141
author: 
created: 
title: "bound_grid_target"
tags: []
signature: "pub fn bound_grid_target (doc : & crawrad_ir :: Document , node : & crawrad_ir :: Node , prefix : & str , grid_field : & str ,) -> (& 'static str , String)"
---

How a bound grid opens: a Dataset's sql, or the grid's own table_name.

# Examples

```rust
use crawrad_codegen::targets::shared::bound_grid_target;
use crawrad_ir::{Document, Node};

let doc = Document::new("form", "Form");
let grid = Node::new("grid1", "Grid");
let (opener, arg) = bound_grid_target(&doc, &grid, "self.", "grid1");
assert_eq!(opener, "open_sqlite_table");
assert_eq!(arg, "&self.grid1.table_name");
```
