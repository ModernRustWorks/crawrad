---
id: explorer-rows-problems-key
kind: doc
target_file: src/explorer_rows/problems.rs
line_hint: 23
author: 
created: 
title: "key"
tags: []
signature: "pub fn key (path : & Path) -> String"
---

Normalizes a path into a problem map key.

A file as the map keys it: `/` separators, and folded on Windows, where a
path's case does not tell two files apart — the language client keys its
diagnostics the same way.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_rows::problems::key;

let k = key(Path::new("src/main.rs"));
assert!(k.contains("main.rs"));
```
