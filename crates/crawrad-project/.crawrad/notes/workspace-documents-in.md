---
id: workspace-documents-in
kind: doc
target_file: src/workspace.rs
line_hint: 170
author: 
created: 
title: "documents_in"
tags: []
signature: "pub fn documents_in (root : & Path) -> Vec < PathBuf >"
---

The documents of an open root, as **absolute** paths and without duplicates.

Absolute because it is the rule of ADR 0017 and because it survives a
different `current_dir`. The order is that of members — active first —, and
alphabetical within each member: the list appears on screen, and a list that
changes order on every launch is a list where one cannot learn where to click.

Only `<member>/src/`, without descending.

# Examples

```rust
use std::path::Path;
use crawrad_project::workspace::documents_in;

let docs = documents_in(Path::new("non_existent_root_xyz"));
assert!(docs.is_empty());
```
