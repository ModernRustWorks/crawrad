---
id: document-nome-de
kind: doc
target_file: src/document.rs
line_hint: 187
author: 
created: 
title: "nome_de"
tags: []
signature: "pub fn nome_de (doc : & Document , id : & str) -> String"
---

Gets the node name or returns the id itself as fallback.

The node name, with the id as fallback in case it disappeared from the
document between one read and another.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_host::document::nome_de;

let doc = Document::new("form", "Form");
assert_eq!(nome_de(&doc, "missing"), "missing");
```
