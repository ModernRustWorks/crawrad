---
id: note-missingdoc
kind: doc
target_file: src/note.rs
line_hint: 16
author: 
created: 
title: "MissingDoc"
tags: []
signature: "pub struct MissingDoc"
---

A public item missing documentation or a note.

# Examples

```rust
use std::path::PathBuf;
use crawrad_cli::note::MissingDoc;

let doc = MissingDoc {
    file: PathBuf::from("src/lib.rs"),
    line: 12,
    kind: "fn",
    name: "run".to_string(),
};
assert_eq!(doc.line, 12);
```
