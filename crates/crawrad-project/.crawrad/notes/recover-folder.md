---
id: recover-folder
kind: doc
target_file: src/recover.rs
line_hint: 18
author: 
created: 
title: "folder"
tags: []
signature: "pub fn folder (root : & Path) -> PathBuf"
---

Where a project keeps its recovery copies.

# Examples

```rust
use std::path::Path;
use crawrad_project::recover::folder;

assert_eq!(folder(Path::new("project")), Path::new("project/.crawrad/recover"));
```
