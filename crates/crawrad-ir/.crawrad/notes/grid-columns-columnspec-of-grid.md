---
id: grid-columns-columnspec-of-grid
kind: doc
target_file: src/grid_columns.rs
line_hint: 41
author: 
created: 
title: "ColumnSpec::of_grid"
tags: []
signature: "pub fn of_grid (doc : & Document , grid : & Node) -> Vec < ColumnSpec >"
---

The `GridColumn` children of a grid node, in order (#936).

What the Designer canvas draws and what the generators write into the app.

# Examples

```rust
use crawrad_ir::{Document, Node, Value};
use crawrad_ir::grid_columns::{ColumnSpec, GRID_COLUMN_CLASS};

let mut doc = Document::new("form1", "Form");
let mut grid = Node::new("grid1", "Grid");
grid.children = vec!["col1".into()];
let mut col = Node::new("col1", GRID_COLUMN_CLASS);
col.props.insert("field".into(), Value::Str("city".into()));
col.props.insert("alignment".into(), Value::Enum("taCenter".into()));
doc.nodes.insert(col.id.clone(), col);
let specs = ColumnSpec::of_grid(&doc, &grid);
assert_eq!(specs[0].field, "city");
assert_eq!(specs[0].align, "taCenter");
```
