---
id: file-dialog-with-default-ext
kind: doc
target_file: src/file_dialog.rs
line_hint: 93
author: 
created: 
title: "with_default_ext"
tags: []
signature: "pub fn with_default_ext (path : PathBuf , default_ext : & str) -> PathBuf"
---

Pure helper that appends `default_ext` to `path` if `path` lacks an extension.

# Examples

```rust
use std::path::PathBuf;
use crawrad_runtime::file_dialog::with_default_ext;

let path = with_default_ext(PathBuf::from("document"), "txt");
assert_eq!(path, PathBuf::from("document.txt"));
```
