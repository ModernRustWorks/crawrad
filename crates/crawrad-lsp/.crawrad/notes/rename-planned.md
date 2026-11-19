---
id: rename-planned
kind: doc
target_file: src/rename.rs
line_hint: 190
author: 
created: 
title: "Planned"
tags: []
signature: "pub struct Planned"
---

One file of a rename: how many edits, its text before and after.

# Examples

```rust
use crawrad_lsp::rename::Planned;
use std::path::PathBuf;

let planned = Planned {
    file: PathBuf::from("src/main.rs"),
    count: 1,
    before: "fn old_name() {}\n".to_string(),
    after: "fn new_name() {}\n".to_string(),
};
assert_eq!(planned.count, 1);
```
