---
id: hostmodel-strip-front
kind: doc
target_file: src/lib.rs
line_hint: 340
author: 
created: 
title: "HostModel::strip_front"
tags: []
signature: "pub fn strip_front (& self, active_file : Option < usize >) -> Option < usize >"
---

Says which strip row is lit.

A form is lit only while the centre is showing one. With the editor up, the lit
tab is the loose file's, offset past the forms - so the strip never lights two
tabs at once, whichever view the person is in.

# Examples

```rust
use crawrad_host::{HostModel, ViewCenter};
use crawrad_doc::Doc;
use crawrad_ir::Document;

let mut host = HostModel::new();
host.open_form(Doc::new(Document::new("form_main", "Form")), None);

// Showing a form: the form's own row is lit, and the active file is ignored.
host.set_center(ViewCenter::Design);
assert_eq!(host.strip_front(Some(0)), Some(0));

// Showing code: the lit row is the file's, counted past the one form.
host.set_center(ViewCenter::Code);
assert_eq!(host.strip_front(Some(0)), Some(1));
```
