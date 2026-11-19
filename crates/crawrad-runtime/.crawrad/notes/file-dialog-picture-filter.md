---
id: file-dialog-picture-filter
kind: doc
target_file: src/file_dialog.rs
line_hint: 171
author: 
created: 
title: "PICTURE_FILTER"
tags: []
signature: "pub const PICTURE_FILTER : & str = \"Images|*.png;*.jpg;*.jpeg;*.bmp;*.svg;*.ico\""
---

File dialog filter for supported picture formats.

The file dialog's filter for a picture: the formats `crawrad_ir::ImageFormat`
knows. GIF is not among them, so it is not offered — a file the document
cannot hold would be refused after the user picked it.

# Examples

```rust
use crawrad_runtime::file_dialog::PICTURE_FILTER;

assert!(PICTURE_FILTER.contains("*.png"));
```
