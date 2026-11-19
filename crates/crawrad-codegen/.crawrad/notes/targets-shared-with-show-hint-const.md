---
id: targets-shared-with-show-hint-const
kind: doc
target_file: src/targets/shared.rs
line_hint: 424
author: 
created: 
title: "with_show_hint_const"
tags: []
signature: "pub fn with_show_hint_const (doc : & crawrad_ir :: Document , code : String) -> String"
---

Appends `CRAWRAD_FORM_SHOW_HINT` constant if referenced in generated code.

The form's `ShowHint` as a constant (#139): off, no control on it shows
its hint. Decided on the whole generated file, not on the catalogue: a class
that declares `Hint` does not always draw a tooltip (an `Action`), and an unread
constant is a warning in the user's project.

# Examples

```rust
use crawrad_codegen::targets::shared::with_show_hint_const;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
let out = with_show_hint_const(&doc, "fn test() {}".to_string());
assert_eq!(out, "fn test() {}");
```
