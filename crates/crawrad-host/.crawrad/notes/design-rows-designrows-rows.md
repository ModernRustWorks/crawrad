---
id: design-rows-designrows-rows
kind: doc
target_file: src/design_rows.rs
line_hint: 167
author: 
created: 
title: "DesignRows::rows"
tags: []
signature: "pub fn rows(&self, doc: &Document, project_dir: Option<&Path>, grid_id: &str) -> Option<GridRows>"
---

What the `Grid` `grid_id` shows, or `None` when it shows its own cells.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_host::design_rows::DesignRows;

let rows = DesignRows::default();
assert_eq!(rows.rows(&Document::new("form", "Form"), None, "grid1"), None);
```
