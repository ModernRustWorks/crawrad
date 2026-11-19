---
id: design-rows-designrows-ensure
kind: doc
target_file: src/design_rows.rs
line_hint: 112
author: 
created: 
title: "DesignRows::ensure"
tags: []
signature: "pub fn ensure(&mut self, doc: &Document, revision: u64, project_dir: Option<&Path>)"
---

Starts a read for every bound grid whose rows are not known yet.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_host::design_rows::DesignRows;

let mut rows = DesignRows::default();
rows.ensure(&Document::new("form", "Form"), 1, None);
```
