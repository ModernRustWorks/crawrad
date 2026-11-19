---
id: menu-menu-all
kind: doc
target_file: src/menu.rs
line_hint: 65
author: 
created: 
title: "Menu::ALL"
tags: []
signature: "pub const ALL : & 'static [Self] = & [Self :: File , Self :: Edit , Self :: View , Self :: Go , Self :: Refactor , Self :: Project , Self :: Run , Self :: Component , Self :: Tools , Self :: Help ,]"
---

The top bar menus from left to right in display order.

The bar, left to right — the same order as the egui shell
(`crates/crawrad-ide/src/main.rs:4819-4995`).

# Examples

```rust
use crawrad_actions::menu::Menu;

assert_eq!(Menu::ALL.len(), 10);
```
