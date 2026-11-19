---
id: file-dialog-save-answer
kind: doc
target_file: src/file_dialog.rs
line_hint: 166
author: 
created: 
title: "save_answer"
tags: []
signature: "pub fn save_answer (answer : & str , req : & FileDialogRequest) -> Option < PathBuf >"
---

What [`pick_save`] returns for a scripted `answer`.

# Examples

```rust
use std::path::PathBuf;
use crawrad_runtime::file_dialog::{save_answer, FileDialogRequest};

let req = FileDialogRequest { default_ext: "txt".to_string(), ..Default::default() };
let path = save_answer("document", &req);
assert_eq!(path, Some(PathBuf::from("document.txt")));
```
