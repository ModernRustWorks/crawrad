---
id: components-main-menu-main-menu
kind: doc
target_file: src/components/main_menu.rs
line_hint: 5
author: 
created: 
title: "main_menu"
tags: []
signature: "pub fn main_menu () -> ComponentDesc"
---

`MainMenu`: main window menu (menu bar at the top).

Its `Items` (#455) are `MenuItem` children: the top-level ones are the
bar's entries, and an item's own items are its drop-down.

# Examples

```rust
use crawrad_registry::main_menu;

let desc = main_menu();
assert_eq!(desc.class, "MainMenu");
```
