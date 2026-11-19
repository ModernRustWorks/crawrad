---
id: props-props-popup-menu
kind: doc
target_file: src/props.rs
line_hint: 493
author: 
created: 
title: "Props::popup_menu"
tags: []
signature: "pub fn popup_menu () -> PropDesc"
---

The context menu a secondary click shows (`PopupMenu`, #456).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::popup_menu();
assert_eq!(p.key, "popup_menu");
```
