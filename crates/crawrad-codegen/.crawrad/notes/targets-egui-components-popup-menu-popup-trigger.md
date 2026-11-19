---
id: targets-egui-components-popup-menu-popup-trigger
kind: doc
target_file: src/targets/egui/components/popup_menu.rs
line_hint: 59
author: 
created: 
title: "popup_trigger"
tags: []
signature: "pub fn popup_trigger (doc : & Document , control : & str) -> Result < String , Erro >"
---

Generates context menu trigger code for a control.

**A control's context menu** (#456), one generic piece after any control
that has a `PopupMenu`: a secondary click in its box opens the menu its
`popup_menu` names — read at run time, so your code can swap it — at the
pointer, after that menu's `OnPopup`. Nothing when the form has no menu
with items.

# Examples

```rust
use crawrad_codegen::targets::egui::components::popup_menu::popup_trigger;
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
let trigger = popup_trigger(&doc, "btn").unwrap();
assert!(trigger.is_empty());
```

# Errors

Returns an error if an identifier name is invalid.
