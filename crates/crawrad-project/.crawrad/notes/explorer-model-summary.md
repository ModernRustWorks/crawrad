---
id: explorer-model-summary
kind: doc
target_file: src/explorer_model.rs
line_hint: 277
author: 
created: 
title: "summary"
tags: []
signature: "pub fn summary (lines : & [Line]) -> String"
---

`4 files · 2 not generated yet`, and only what is non-zero.

Disk state lives here, and not in a second badge per row: the column already carries
the owner, which is the information the panel exists to provide, and two badges
competing for the same row cause both to lose.

# Examples

```rust
use crawrad_project::explorer_model::{summary, FileState, Line};
use std::path::PathBuf;

let line = Line {
    path: PathBuf::from("src/lib.rs"),
    owner: None,
    state: FileState::Pending,
};
assert_eq!(summary(&[line]), "1 file  ·  1 not generated yet");
```
