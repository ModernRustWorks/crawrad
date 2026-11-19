---
id: explorer-rows-rows-file-file-with-unsaved
kind: doc
target_file: src/explorer_rows/rows/file.rs
line_hint: 80
author: 
created: 
title: "File::with_unsaved"
tags: []
signature: "pub fn with_unsaved (mut self , unsaved : bool) -> Self"
---

Whether the file is modified in an editor buffer and not saved.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_rows::rows::file::File;
use crawrad_project::explorer_model::FileState;
use crawrad_project::explorer_rows::row::ExplorerRow;

let f = File::new(Path::new("src/main.rs"), FileState::OnDisk, None, 0)
    .with_unsaved(true);
assert_eq!(f.text(), "main.rs");
```
