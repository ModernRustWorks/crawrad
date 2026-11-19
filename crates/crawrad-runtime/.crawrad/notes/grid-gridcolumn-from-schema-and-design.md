---
id: grid-gridcolumn-from-schema-and-design
kind: doc
target_file: src/grid.rs
line_hint: 156
author: 
created: 
title: "GridColumn::from_schema_and_design"
tags: []
signature: "pub fn from_schema_and_design (columns : & [datagrid :: ColumnInfo] , specs : & [ColumnSpec] ,) -> Vec < GridColumn >"
---

[`GridColumn::from_design`] over a source's schema.

This is what the generated app calls.

# Examples

```rust
use datagrid::{ColumnInfo, ColumnKind, DisplayHint};
use crawrad_runtime::grid::{ColumnAlign, GridColumn};

let schema = vec![ColumnInfo { name: "id".into(), kind: ColumnKind::Int, nullable: false, hint: DisplayHint::Plain }];
let cols = GridColumn::from_schema_and_design(&schema, &[]);
assert_eq!(cols[0].align, ColumnAlign::Right);
```
