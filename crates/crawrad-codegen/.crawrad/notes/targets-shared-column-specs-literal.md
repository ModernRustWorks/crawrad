---
id: targets-shared-column-specs-literal
kind: doc
target_file: src/targets/shared.rs
line_hint: 152
author: 
created: 
title: "column_specs_literal"
tags: []
signature: "pub fn column_specs_literal (doc : & crawrad_ir :: Document , grid : & crawrad_ir :: Node) -> String"
---

The grid's `GridColumn` children as a Rust expression (#936).

Its type is `&[crawrad_runtime::grid::ColumnSpec]`; `&[]` when it has none.

# Examples

```rust
use crawrad_codegen::targets::shared::column_specs_literal;
use crawrad_ir::{Document, Node};

let doc = Document::new("form1", "Form");
assert_eq!(column_specs_literal(&doc, &Node::new("grid1", "Grid")), "&[]");
```
