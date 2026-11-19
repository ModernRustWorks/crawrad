---
id: document-inherits
kind: doc
target_file: src/lib.rs
line_hint: 198
author: 
created: 
title: "Document::inherits"
tags: []
signature: "pub fn inherits (& self) -> Option < & str >"
---

Returns the name of the ancestor form from which this form inherits, if any.

# Examples

```rust
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
assert_eq!(doc.inherits(), None);
```
