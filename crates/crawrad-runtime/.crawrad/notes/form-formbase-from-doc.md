---
id: form-formbase-from-doc
kind: doc
target_file: src/form.rs
line_hint: 45
author: 
created: 
title: "FormBase::from_doc"
tags: []
signature: "pub fn from_doc (doc : & Document) -> Self"
---

Extracts form properties from an IR document root node.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_runtime::FormBase;

let doc = Document::new("main_form", "Form");
let form = FormBase::from_doc(&doc);
assert_eq!(form.title, "main_form");
```
