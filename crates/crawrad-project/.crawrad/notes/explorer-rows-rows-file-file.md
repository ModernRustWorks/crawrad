---
id: explorer-rows-rows-file-file
kind: doc
target_file: src/explorer_rows/rows/file.rs
line_hint: 39
author: 
created: 
title: "File"
tags: []
signature: "pub struct File"
---

One file of the plan, with the owner badge.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_rows::rows::file::File;
use crawrad_project::explorer_model::FileState;
use crawrad_project::explorer_rows::row::ExplorerRow;

let f = File::new(Path::new("src/main.rs"), FileState::OnDisk, None, 0);
assert_eq!(f.text(), "main.rs");
```
