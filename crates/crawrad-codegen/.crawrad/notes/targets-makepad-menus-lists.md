---
id: targets-makepad-menus-lists
kind: doc
target_file: src/targets/makepad/menus.rs
line_hint: 74
author: 
created: 
title: "lists"
tags: []
signature: "pub fn lists < 'a > (doc : & 'a Document , emitted : & [& 'a Node]) -> Result < Vec < List < 'a > > , Erro >"
---

Returns all menu drop-down lists in the form.

Every list of the form: each bar entry's, each sub-menu's, each
`PopupMenu`'s.

# Examples

```rust
use crawrad_codegen::targets::makepad::menus::lists;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
let res = lists(&doc, &[]).unwrap();
assert!(res.is_empty());
```

# Errors

Returns an error if any menu node name is not a valid Rust identifier.
