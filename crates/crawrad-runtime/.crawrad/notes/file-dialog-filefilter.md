---
id: file-dialog-filefilter
kind: doc
target_file: src/file_dialog.rs
line_hint: 27
author: 
created: 
title: "FileFilter"
tags: []
signature: "pub struct FileFilter"
---

One filter pair for the file dialog, with a label and extension patterns.

One filter pair for the file dialog, consisting of a descriptive label
and a list of file extension patterns (without the `*.` prefix).

# Examples

```rust
use crawrad_runtime::file_dialog::FileFilter;

let filter = FileFilter {
    name: "Text files".to_string(),
    patterns: vec!["txt".to_string()],
};
assert_eq!(filter.patterns.len(), 1);
```
