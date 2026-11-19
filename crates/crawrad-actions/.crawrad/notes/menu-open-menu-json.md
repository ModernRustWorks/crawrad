---
id: menu-open-menu-json
kind: doc
target_file: src/menu.rs
line_hint: 300
author: 
created: 
title: "open_menu_json"
tags: []
signature: "pub fn open_menu_json (menu : Option < String > , highlight : Option < usize > , rows : & [Row] , submenu : Option < (& str , & [Row]) > , rects : serde_json :: Value ,) -> serde_json :: Value"
---

Serializes open menu state to JSON for `view.get_open_menu`.

What `view.get_open_menu` answers (#257, #558): the menu open (if any), the
highlighted row, its rows, the open submenu's name and rows, and where on
screen the shell drew them (`rects`).

# Examples

```rust
use crawrad_actions::menu::open_menu_json;

let json = open_menu_json(None, None, &[], None, serde_json::json!({}));
assert_eq!(json["open"], false);
```
