---
id: field-class
kind: doc
target_file: src/lib.rs
line_hint: 91
author: 
created: 
title: "field_class"
tags: []
signature: "pub fn field_class (designer_source : & str , field : & str) -> Option < String >"
---

Extracts component class for a form field from designer source.

The class of the form field `field`, read from the form structs of a
`.designer.rs` (a struct that carries `pub base: FormBase`). `None` when
no form declares it — never a guess from the name.

# Examples

```rust
use crawrad_insight::field_class;

let src = "pub struct MyForm {\n    pub base: FormBase,\n    pub btn: Button,\n}";
assert_eq!(field_class(src, "btn"), Some("Button".into()));
```
