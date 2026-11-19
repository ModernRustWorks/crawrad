---
id: templates-template-document
kind: doc
target_file: src/templates.rs
line_hint: 149
author: 
created: 
title: "Template::document"
tags: []
signature: "pub fn document (& self , registry : & Registry) -> Option < Document >"
---

The template document, instantiated by the catalogue.

**This function, and only this function.** The welcome screen preview paints
what it returns, and `New Project…` saves what it returns — the exact same
document, not two similar ones.

Returns `None` when a piece is not in the catalogue. That is a programming
defect, caught by `every_template_instantiates_from_catalogue` before the IDE starts.

# Examples

```rust
use crawrad_project::templates::Template;
use crawrad_registry::Registry;

let t = Template::default_template();
let reg = Registry::builtin();
let doc = t.document(&reg);
assert!(doc.is_some());
```
