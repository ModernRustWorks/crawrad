---
id: targets-makepad-menus-list-id
kind: doc
target_file: src/targets/makepad/menus.rs
line_hint: 44
author: 
created: 
title: "list_id"
tags: []
signature: "pub fn list_id (owner : & Node) -> Result < String , Erro >"
---

The widget of `owner`'s list.

# Examples

```rust
use crawrad_codegen::targets::makepad::menus::list_id;
use crawrad_ir::Node;

let owner = Node::new("menu", "MainMenu");
let id = list_id(&owner).unwrap();
assert_eq!(id, "menu_list");
```

# Errors

Returns an error if the owner name is not a valid Rust identifier.
