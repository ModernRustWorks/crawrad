---
id: hostmodel-openform
kind: doc
target_file: src/lib.rs
line_hint: 142
author: 
created: 
title: "OpenForm"
tags: []
signature: "pub struct OpenForm"
---

A form open in the strip: its document, and the file it came from.

The path is what tells two tabs apart and what keeps a file from opening twice.
It is `None` for a form that has never been saved, which is why two unsaved forms
can sit side by side without either claiming to be the other.

# Examples

```rust
use crawrad_host::OpenForm;
use crawrad_doc::Doc;
use crawrad_ir::Document;

let open = OpenForm {
    doc: Doc::new(Document::new("form_main", "Form")),
    path: None,
    saved_revision: 0,
    previous_rect: None,
};
assert!(open.path.is_none(), "a form that was never saved has no file");
```
