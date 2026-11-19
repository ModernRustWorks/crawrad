---
id: file-dialog-open-answer
kind: doc
target_file: src/file_dialog.rs
line_hint: 151
author: 
created: 
title: "open_answer"
tags: []
signature: "pub fn open_answer (answer : & str , req : & FileDialogRequest) -> Option < Vec < PathBuf > >"
---

What [`pick_open`] returns for a scripted `answer`.

# Examples

```rust
use std::path::PathBuf;
use crawrad_runtime::file_dialog::{open_answer, FileDialogRequest};

let req = FileDialogRequest { multi: true, ..Default::default() };
let paths = open_answer("file1.txt|file2.txt", &req);
assert_eq!(paths, Some(vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")]));
```
