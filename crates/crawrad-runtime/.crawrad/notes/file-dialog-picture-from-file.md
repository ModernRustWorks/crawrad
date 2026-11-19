---
id: file-dialog-picture-from-file
kind: doc
target_file: src/file_dialog.rs
line_hint: 188
author: 
created: 
title: "picture_from_file"
tags: []
signature: "pub fn picture_from_file (path : & Path) -> Result < crawrad_ir :: Picture , String >"
---

Loads a picture from a file path into memory.

The picture a file holds: its bytes go into the document, as the classic
RAD IDE embeds them in the form, and its NAME is kept as the source the
row shows. The format is read from the bytes, not trusted from the
extension, and a file that is not an image the IR knows is refused.

# Examples

```rust
use std::path::Path;
use crawrad_runtime::file_dialog::picture_from_file;

let res = picture_from_file(Path::new("missing-image.png"));
assert!(res.is_err());
```

# Errors

Returns an error if the file cannot be read or is not a supported image format.
