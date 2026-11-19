---
id: quick-open-filter-files
kind: doc
target_file: src/quick_open.rs
line_hint: 23
author: 
created: 
title: "filter_files"
tags: []
signature: "pub fn filter_files (files : & [PathBuf] , filter : & str) -> Vec < PathBuf >"
---

The paths whose name or path contains `filter`, ignoring case.

# Examples

```rust
use crawrad_host::quick_open::filter_files;
use std::path::PathBuf;

let files = vec![PathBuf::from("src/main.rs"), PathBuf::from("src/lib.rs")];
let matches = filter_files(&files, "main");
assert_eq!(matches, vec![PathBuf::from("src/main.rs")]);
```
