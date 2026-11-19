---
id: explorer-model-line
kind: doc
target_file: src/explorer_model.rs
line_hint: 57
author: 
created: 
title: "Line"
tags: []
signature: "pub struct Line"
---

One row of the panel: a file, its owner, and what the disk says.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::explorer_model::{FileState, Line};

let line = Line {
    path: PathBuf::from("src/main.rs"),
    owner: None,
    state: FileState::OnDisk,
};
assert_eq!(line.state, FileState::OnDisk);
```
