---
id: class-in-form
kind: doc
target_file: src/lib.rs
line_hint: 121
author: 
created: 
title: "class_in_form"
tags: []
signature: "pub fn class_in_form < 'a > (form : & 'a Document , field : & str) -> Option < & 'a str >"
---

Finds the component class of a named node in a form document.

The class of the component the form field `field` names, in the live
form: the node whose `Name` is `field`.

# Examples

```rust
use crawrad_insight::class_in_form;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert!(class_in_form(&doc, "missing").is_none());
```
