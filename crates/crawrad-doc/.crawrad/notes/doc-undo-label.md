---
id: doc-undo-label
kind: doc
target_file: src/lib.rs
line_hint: 341
author: 
created: 
title: "Doc::undo_label"
tags: []
signature: "pub fn undo_label (& self) -> Option < & str >"
---

Label of what will be undone — for the IDE to write "Undo: move btn_ok".

# Examples

```rust
use crawrad_doc::Doc;
use crawrad_ir::Document;

let doc = Doc::new(Document::new("form", "Form"));
assert_eq!(doc.undo_label(), None);
```
