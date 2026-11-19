---
id: source-form-source
kind: doc
target_file: src/source.rs
line_hint: 100
author: 
created: 
title: "form_source"
tags: []
signature: "pub fn form_source (doc : & crawrad_ir :: Document , package : & str) -> Result < String , String >"
---

The `.designer.rs` of the open form, **planned at this instant**.

Pure: [`crawrad_codegen::Gerador::gerar`] decides the files without touching
the disk, and that separation is what makes this callable on every
`Alt+F12` without the risk of writing over somebody's work.

# Errors

Returns an error message if code generation fails or does not produce a designer file.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_project::source::form_source;

let doc = Document::new("form", "Form");
let src = form_source(&doc, "my_app");
assert!(src.is_ok());
```
