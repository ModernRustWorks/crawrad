---
id: hostmodel-open-form
kind: doc
target_file: src/lib.rs
line_hint: 265
author: 
created: 
title: "HostModel::open_form"
tags: []
signature: "pub fn open_form (& mut self, doc : Doc, path : Option < PathBuf >) -> usize"
---

Opens a form in the strip and brings it to the front, answering its index.

A form whose file is already open comes forward instead of opening twice: two tabs
of one file would each hold their own edits, and saving would decide between them
silently.

# Examples

```rust
use crawrad_host::HostModel;
use crawrad_doc::Doc;
use crawrad_ir::Document;
use std::path::PathBuf;

let mut host = HostModel::new();
let file = PathBuf::from("my_app/src/form_main.crawform");
let first = host.open_form(Doc::new(Document::new("form_main", "Form")), Some(file.clone()));
let again = host.open_form(Doc::new(Document::new("form_main", "Form")), Some(file));
assert_eq!(first, again, "the same file must not open twice");
assert_eq!(host.forms().len(), 1);
```
