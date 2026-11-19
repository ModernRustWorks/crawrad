---
id: doc-note-crate-dir-of
kind: doc
target_file: src/doc_note.rs
line_hint: 928
author: 
created: 
title: "crate_dir_of"
tags: []
signature: "pub fn crate_dir_of (file : & Path) -> Option < PathBuf >"
---

Returns the crate directory that owns the given file.

The crate that owns `file`: the nearest ancestor directory holding a Cargo.toml with a package table.

# Examples

```rust
use std::path::Path;
use crawrad_project::doc_note::crate_dir_of;

let dir = crate_dir_of(Path::new("Cargo.toml"));
assert!(dir.is_some());
```
