---
id: grid-gridcolumn-from-schema
kind: doc
target_file: src/grid.rs
line_hint: 27
author: 
created: 
title: "GridColumn::from_schema"
tags: []
signature: "pub fn from_schema (columns : & [datagrid :: ColumnInfo]) -> Vec < GridColumn >"
---

One column per field of a source's schema, in the source's order.

For a grid pointed at a table: the title comes from the field name and
numbers line up on the right, as they do in every column of figures.

# Examples

```rust
use datagrid::{ColumnInfo, ColumnKind, DisplayHint};
use crawrad_runtime::grid::{ColumnAlign, GridColumn};

let schema = vec![
    ColumnInfo { name: "id".into(), kind: ColumnKind::Int, nullable: false, hint: DisplayHint::Plain },
    ColumnInfo { name: "name".into(), kind: ColumnKind::Text, nullable: false, hint: DisplayHint::Plain },
];
let cols = GridColumn::from_schema(&schema);
assert_eq!(cols.len(), 2);
assert_eq!(cols[0].align, ColumnAlign::Right);
assert_eq!(cols[1].align, ColumnAlign::Left);
```
