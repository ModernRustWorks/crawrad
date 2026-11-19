---
id: desktop-desktop-file-in
kind: doc
target_file: src/desktop.rs
line_hint: 68
author: 
created: 
title: "Desktop::file_in"
tags: []
signature: "pub fn file_in (root : & Path) -> PathBuf"
---

Where a project keeps its desktop.

# Examples

```rust
use std::path::Path;
use crawrad_project::desktop::Desktop;

let p = Desktop::file_in(Path::new("project"));
assert!(p.ends_with("desktop.txt"));
```
