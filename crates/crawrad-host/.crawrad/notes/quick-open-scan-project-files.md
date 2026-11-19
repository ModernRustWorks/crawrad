---
id: quick-open-scan-project-files
kind: doc
target_file: src/quick_open.rs
line_hint: 43
author: 
created: 
title: "scan_project_files"
tags: []
signature: "pub fn scan_project_files (root : & Path) -> Vec < PathBuf >"
---

Scans and sorts project source files, skipping ignored folders.

The project's files Quick Open lists, sorted: `rs`, `crawform`, `toml`,
`md`, `json`, `txt`, skipping `target`, `.git`, `.crawrad`, `node_modules`.

# Examples

```rust
use crawrad_host::quick_open::scan_project_files;
use std::path::Path;

let files = scan_project_files(Path::new("nonexistent_dir"));
assert!(files.is_empty());
```
