---
id: targets-makepad-menus-list
kind: doc
target_file: src/targets/makepad/menus.rs
line_hint: 24
author: 
created: 
title: "List"
tags: []
signature: "pub struct List < 'a >"
---

Representation of a drop-down menu list in Makepad.

One drop-down list: whose items it shows, and the lists it opens inside
(a sub-menu stays open with its parents).

# Examples

```rust
use crawrad_codegen::targets::makepad::menus::List;
use crawrad_ir::Node;

let owner = Node::new("menu", "MainMenu");
let list = List {
    id: "menu_list".to_string(),
    owner: &owner,
    items: Vec::new(),
    ancestors: Vec::new(),
};
assert_eq!(list.id, "menu_list");
```
