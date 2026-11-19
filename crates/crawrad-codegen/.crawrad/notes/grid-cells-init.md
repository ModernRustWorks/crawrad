---
id: grid-cells-init
kind: doc
target_file: src/lib.rs
line_hint: 876
author: 
created: 
title: "grid_cells_init"
tags: []
signature: "pub fn grid_cells_init (node : & crawrad_ir :: Node) -> String"
---

Initialiser expression for the `cells` field of a `Grid` (#442).

# Examples

```rust
use crawrad_codegen::grid_cells_init;
use crawrad_ir::Node;

let node = Node::new("grid1", "Grid");
assert_eq!(grid_cells_init(&node), "Default::default()");
```

The design-time `cell_text` parsed at startup; a bound grid never reads it.
