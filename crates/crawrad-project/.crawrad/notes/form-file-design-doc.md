---
id: form-file-design-doc
kind: doc
target_file: src/form_file.rs
line_hint: 50
author:
created:
title: "design_doc"
tags: []
signature: "pub fn design_doc (path : & Path , ir : Document) -> crawrad_doc :: Doc"
---

The document the designer edits for the form loaded from `path`.

A form that inherits gets its bases from beside it, the same files the
generator reads, and the designer shows their components
([`crawrad_doc::Doc::inheriting`]). A base that is not there, or a chain that
does not resolve, opens the form alone: what it adds is still editable.

# Examples

```rust
use crawrad_ir::Document;
use std::path::Path;

let doc = crawrad_project::form_file::design_doc(
    Path::new("no/such/dir/form_a.crawform"),
    Document::new("form_a", "Form"),
);
assert!(doc.base().is_none());
```
