---
id: sources
kind: doc
target_file: src/lib.rs
line_hint: 102
author: 
created: 
title: "sources"
tags: []
signature: "pub fn sources (root : & Path) -> Vec < PathBuf >"
---

Collects sorted paths of all Rust source files under root.

Every Rust source under `root`, sorted; `target`, hidden folders and the
IDE's own `.crawrad` left out.

# Examples

```rust
use std::path::Path;
use crawrad_ext_todo::sources;

let list = sources(Path::new("non_existent_folder_xyz"));
assert!(list.is_empty());
```
