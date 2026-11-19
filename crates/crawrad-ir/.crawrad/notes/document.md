---
id: document
kind: doc
target_file: src/lib.rs
line_hint: 161
author: 
created: 
title: "Document"
tags: []
signature: "pub struct Document"
---

A form: `form_main.crawform`.

# Examples

```rust
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
assert_eq!(doc.root, "MainForm");
```
