---
id: targets-makepad-menus-popup-open
kind: doc
target_file: src/targets/makepad/menus.rs
line_hint: 290
author: 
created: 
title: "popup_open"
tags: []
signature: "pub fn popup_open (doc : & Document , emitted : & [& Node] , field : & str) -> Result < Option < String > , Erro >"
---

Generates context menu opening code on secondary clicks.

**A control's context menu** (#456): on its secondary press, the
`PopupMenu` its `popup_menu` names — read at run time — opens at the
pointer, after that menu's `OnPopup`. `None` when the form has no
`PopupMenu` with items.

# Examples

```rust
use crawrad_codegen::targets::makepad::menus::popup_open;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
let code = popup_open(&doc, &[], "btn").unwrap();
assert!(code.is_none());
```

# Errors

Returns an error if popup menu names cannot be converted to identifiers.
