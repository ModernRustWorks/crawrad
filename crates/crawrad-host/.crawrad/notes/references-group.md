---
id: references-group
kind: doc
target_file: src/references.rs
line_hint: 38
author: 
created: 
title: "Group"
tags: []
signature: "pub struct Group"
---

The occurrences in one file, by line.

# Examples

```rust
use crawrad_host::references::Group;
use std::path::PathBuf;

let group = Group {
    file: PathBuf::from("src/main.rs"),
    generated: false,
    read_only: false,
    open: true,
    hits: vec![],
};
assert!(group.open);
```
