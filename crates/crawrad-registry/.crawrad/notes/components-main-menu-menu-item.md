---
id: components-main-menu-menu-item
kind: doc
target_file: src/components/main_menu.rs
line_hint: 23
author: 
created: 
title: "menu_item"
tags: []
signature: "pub fn menu_item () -> ComponentDesc"
---

An individual item in a menu tree.

One item of a menu (#455): its caption, shortcut, check mark and the
`Action` it runs. Its own `MenuItem` children make it a sub-menu, the
classic RAD IDE's `Items` tree.

# Examples

```rust
use crawrad_registry::menu_item;

let desc = menu_item();
assert_eq!(desc.class, "MenuItem");
```
