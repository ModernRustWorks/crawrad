---
id: components-popup-menu-popup-menu
kind: doc
target_file: src/components/popup_menu.rs
line_hint: 5
author: 
created: 
title: "popup_menu"
tags: []
signature: "pub fn popup_menu () -> ComponentDesc"
---

`PopupMenu`: context menu triggered on demand (e.g. right click).

Its `Items` are `MenuItem` children (#456); a control shows it on a
secondary click when its `PopupMenu` names it, and `OnPopup` runs first.

# Examples

```rust
use crawrad_registry::popup_menu;

let desc = popup_menu();
assert_eq!(desc.class, "PopupMenu");
```
