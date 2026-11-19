---
id: note-skippeddoc
kind: doc
target_file: src/note.rs
line_hint: 216
author: 
created: 
title: "SkippedDoc"
tags: []
signature: "pub struct SkippedDoc"
---

An item whose `///` stays, and why.

# Examples

```rust
use crawrad_cli::note::SkippedDoc;

let doc = SkippedDoc {
    file: "src/lib.rs".to_string(),
    line: 1,
    item: "my_crate".to_string(),
    reason: "crate root doc".to_string(),
};
assert_eq!(doc.item, "my_crate");
```
