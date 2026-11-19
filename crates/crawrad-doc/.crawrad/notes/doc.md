---
id: doc
kind: doc
target_file: src/lib.rs
line_hint: 170
author: 
created: 
title: "Doc"
tags: []
signature: "pub struct Doc"
---

The document with history, selection, and auditing.

# Examples

```rust
use crawrad_doc::Doc;
use crawrad_ir::Document;

let doc = Doc::new(Document::new("form", "Form"));
assert_eq!(doc.revision(), 0);
```
