---
id: menu-menu
kind: doc
target_file: src/menu.rs
line_hint: 49
author: 
created: 
title: "Menu"
tags: []
signature: "pub enum Menu"
---

One menu of the top bar.

The variants ARE the catalogue groups the bar draws, in the order it shows
them. `Align`, `Size`, `Order` and `In form` are NOT here: they are designer
groups and belong to the canvas.

`Component` joined on 18/09/2026, with package installation — it is the classic RAD
own menu, and it sits where a classic RAD IDE puts it: after `Run`, before `Help`.

# Examples

```rust
use crawrad_actions::menu::Menu;

assert_eq!(Menu::File.title(), "File");
```
