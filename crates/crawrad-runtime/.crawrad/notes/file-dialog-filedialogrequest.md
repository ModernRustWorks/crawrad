---
id: file-dialog-filedialogrequest
kind: doc
target_file: src/file_dialog.rs
line_hint: 76
author: 
created: 
title: "FileDialogRequest"
tags: []
signature: "pub struct FileDialogRequest"
---

Configuration request used to invoke native open or save dialogs.

# Examples

```rust
use crawrad_runtime::file_dialog::FileDialogRequest;

let req = FileDialogRequest {
    title: "Open File".to_string(),
    multi: false,
    ..Default::default()
};
assert!(!req.multi);
```
