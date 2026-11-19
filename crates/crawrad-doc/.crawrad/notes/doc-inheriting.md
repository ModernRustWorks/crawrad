---
id: doc-inheriting
kind: doc
target_file: src/lib.rs
line_hint: 196
author:
created:
title: "Doc::inheriting"
tags: []
signature: "pub fn inheriting (own : & Document , base : Document) -> Result < Self , DocError >"
---

Opens a derived form for editing, with the base's components on it.

The designer gets the RESOLVED form, and keeps `base` to tell them apart and to take
the own part back out when the form is saved ([`Doc::own_ir`]).

`base` is the whole chain already folded, as
[`crawrad_ir::Document::resolve_chain`] gives it.

# Examples

```rust
use crawrad_doc::Doc;
use crawrad_ir::{Document, Node};

let mut base = Document::new("form_a", "Form");
base.add_child("form_a", Node::new("btn_save", "Button")).unwrap();
let own = Document::new_inherited("form_b", "Form", "form_a");
let doc = Doc::inheriting(&own, base).unwrap();
assert!(doc.is_inherited("btn_save"));
assert_eq!(doc.own_ir().nodes.len(), 1);
```

# Errors

What [`crawrad_ir::Document::resolve_inheritance`] refuses: a form that
inherits from itself or a cycle.
