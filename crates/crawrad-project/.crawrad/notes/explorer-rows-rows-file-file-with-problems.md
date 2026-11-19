---
id: explorer-rows-rows-file-file-with-problems
kind: doc
target_file: src/explorer_rows/rows/file.rs
line_hint: 73
author: 
created: 
title: "File::with_problems"
tags: []
signature: "pub fn with_problems (mut self , problems : Problems) -> Self"
---

The file's errors and warnings: they colour it and count before its badge.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_rows::rows::file::File;
use crawrad_project::explorer_rows::problems::Problems;
use crawrad_project::explorer_model::FileState;
use crawrad_project::explorer_rows::row::ExplorerRow;

let f = File::new(Path::new("src/main.rs"), FileState::OnDisk, None, 0)
    .with_problems(Problems { errors: 1, warnings: 0 });
assert!(f.badge().contains("1"));
```
