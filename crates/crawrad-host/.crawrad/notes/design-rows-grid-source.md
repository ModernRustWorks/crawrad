---
id: design-rows-grid-source
kind: doc
target_file: src/design_rows.rs
line_hint: 35
author: 
created: 
title: "grid_source"
tags: []
signature: "pub fn grid_source(doc: &Document, project_dir: Option<&Path>, grid_id: &str) -> Option<GridSource>"
---

Where the `Grid` `grid_id` reads its design-time rows from.

`None` when it is not bound, not complete, or its design rows are Off.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_host::design_rows::grid_source;

let doc = Document::new("form", "Form");
assert_eq!(grid_source(&doc, None, "grid1"), None);
```
