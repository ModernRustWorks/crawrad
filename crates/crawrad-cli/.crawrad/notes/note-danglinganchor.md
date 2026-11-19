---
id: note-danglinganchor
kind: doc
target_file: src/note.rs
line_hint: 37
author: 
created: 
title: "DanglingAnchor"
tags: []
signature: "pub struct DanglingAnchor"
---

An anchor in source code pointing to a non-existent note.

# Examples

```rust
use crawrad_cli::note::DanglingAnchor;

let anchor = DanglingAnchor {
    id: "missing_note".to_string(),
    file: "src/lib.rs".to_string(),
    line: 15,
};
assert_eq!(anchor.line, 15);
```
