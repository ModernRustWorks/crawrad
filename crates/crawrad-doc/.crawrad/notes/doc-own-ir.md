---
id: doc-own-ir
kind: doc
target_file: src/lib.rs
line_hint: 216
author:
created:
title: "Doc::own_ir"
tags: []
signature: "pub fn own_ir (& self) -> std :: borrow :: Cow < '_ , Document >"
---

What goes in this form's file and to the generator.

The whole form when it inherits from nothing, and only its own part when it
does ([`crawrad_ir::Document::own_part`]).

# Examples

```rust
use crawrad_doc::Doc;
use crawrad_ir::Document;

let doc = Doc::new(Document::new("form_a", "Form"));
assert_eq!(doc.own_ir().root, "form_a");
```
