---
id: document-own-part
kind: doc
target_file: src/lib.rs
line_hint: 282
author:
created:
title: "Document::own_part"
tags: []
signature: "pub fn own_part (& self , base : & Document) -> Document"
---

What of this resolved form belongs in the derived form's own file.

It is the inverse of [`Document::resolve_inheritance`].

The designer edits the RESOLVED form, the one with the base's components on
it; the file keeps only what the derived form adds. That is its own
components whole, and, of an inherited one, only the properties, events and
bindings whose value differs from the base's. An inherited component that
differs in nothing stays out, unless it must be there for the file to load:
an own component points at it, or an own one sits inside it.

A value set back to the base's leaves the file, which is how an event goes
back to the inherited handler.

# Examples

```rust
use crawrad_ir::{Document, Node};

let mut base = Document::new("form_a", "Form");
base.add_child("form_a", Node::new("btn_save", "Button")).unwrap();
let mut derived = Document::new_inherited("form_b", "Form", "form_a");
derived.add_child("form_b", Node::new("btn_new", "Button")).unwrap();
let resolved = derived.resolve_inheritance(&base).unwrap();
let own = resolved.own_part(&base);
assert!(own.node("btn_new").is_some());
assert!(own.node("btn_save").is_none());
assert_eq!(own.resolve_inheritance(&base).unwrap(), resolved);
```
