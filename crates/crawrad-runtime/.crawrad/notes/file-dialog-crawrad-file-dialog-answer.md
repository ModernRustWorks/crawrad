---
id: file-dialog-crawrad-file-dialog-answer
kind: doc
target_file: src/file_dialog.rs
line_hint: 24
author: 
created: 
title: "CRAWRAD_FILE_DIALOG_ANSWER"
tags: []
signature: "pub const CRAWRAD_FILE_DIALOG_ANSWER : & str = \"CRAWRAD_FILE_DIALOG_ANSWER\""
---

The environment variable name used to mock file dialog answers during tests.

# Examples

```rust
use crawrad_runtime::file_dialog::CRAWRAD_FILE_DIALOG_ANSWER;

assert_eq!(CRAWRAD_FILE_DIALOG_ANSWER, "CRAWRAD_FILE_DIALOG_ANSWER");
```
