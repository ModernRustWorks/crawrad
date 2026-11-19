---
id: targets-makepad-menus-none-open
kind: doc
target_file: src/targets/makepad/menus.rs
line_hint: 209
author: 
created: 
title: "none_open"
tags: []
signature: "pub fn none_open (doc : & Document , emitted : & [& Node]) -> Result < Option < String > , Erro >"
---

Condition checking that no menu list is currently open.

The condition "no list is open", for what must not show over a menu (a
control's hint); `None` when the form has no list.

# Examples

```rust
use crawrad_codegen::targets::makepad::menus::none_open;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
let cond = none_open(&doc, &[]).unwrap();
assert!(cond.is_none());
```

# Errors

Returns an error if menu component names cannot be converted to identifiers.
