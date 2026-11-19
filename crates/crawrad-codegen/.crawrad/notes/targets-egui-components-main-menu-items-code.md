---
id: targets-egui-components-main-menu-items-code
kind: doc
target_file: src/targets/egui/components/main_menu.rs
line_hint: 111
author: 
created: 
title: "items_code"
tags: []
signature: "pub fn items_code (doc : & Document , owner : & Node , depth : usize) -> Result < String , Erro >"
---

Emits egui code showing menu items of an owner node.

**The code that shows `owner`'s items in a menu `ui`** (#455), `depth`
levels of four spaces in: each item a button with its shortcut at the
right and a check mark when checked, a `-` caption a separator, and an
item with items of its own a sub-menu. A click runs the linked Action's
`OnExecute` and the item's `OnClick`, as a `Button` does.

# Examples

```rust
use crawrad_codegen::targets::egui::components::main_menu::items_code;
use crawrad_ir::{Document, Node};

let mut doc = Document::new("MainForm", "Form");
let menu = Node::new("menu", "MainMenu");
doc.nodes.insert("menu".to_string(), menu);
let owner = doc.node("menu").unwrap();
let code = items_code(&doc, owner, 2).unwrap();
assert!(code.is_empty());
```

# Errors

Returns an error if an identifier name is invalid.
