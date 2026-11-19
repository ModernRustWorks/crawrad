---
id: targets-makepad-menus-item-click
kind: doc
target_file: src/targets/makepad/menus.rs
line_hint: 253
author: 
created: 
title: "item_click"
tags: []
signature: "pub fn item_click (doc : & Document , emitted : & [& Node] , item : & Node) -> Result < Option < String > , Erro >"
---

Generates click handling code for a menu item.

What a click on `item` does to the lists, `None` when it does nothing: an
item with items opens its list (under it on the bar, beside it in a list)
and closes the others but its own parents; one without closes them all.

# Examples

```rust
use crawrad_codegen::targets::makepad::menus::item_click;
use crawrad_ir::{Document, Node};

let doc = Document::new("form", "Form");
let item = Node::new("item1", "MenuItem");
let code = item_click(&doc, &[], &item).unwrap();
assert!(code.is_none());
```

# Errors

Returns an error if menu component names cannot be converted to identifiers.
