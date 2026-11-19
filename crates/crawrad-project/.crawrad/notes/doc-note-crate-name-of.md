---
id: doc-note-crate-name-of
kind: doc
target_file: src/doc_note.rs
line_hint: 961
author: 
created: 
title: "crate_name_of"
tags: []
signature: "pub fn crate_name_of (crate_dir : & Path) -> Option < String >"
---

Reads `[package] name` from a crate directory's `Cargo.toml`.

# Examples

```rust
use std::path::Path;
use crawrad_project::doc_note::crate_name_of;

let name = crate_name_of(Path::new("."));
assert!(name.is_some());
```
