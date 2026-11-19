---
id: line-marks-file-key
kind: doc
target_file: src/line_marks.rs
line_hint: 23
author: 
created: 
title: "file_key"
tags: []
signature: "pub fn file_key (file : & str) -> String"
---

Canonical key for a file path used in line mark storage.

A file as the marks are keyed: `/` separators, and folded on Windows,
where a path's case does not tell two files apart.

# Examples

```rust
use crawrad_extension_host::line_marks::file_key;

let key = file_key(r"src\lib.rs");
assert!(key.contains("src/lib.rs"));
```
