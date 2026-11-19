---
id: menu-menu-title
kind: doc
target_file: src/menu.rs
line_hint: 79
author: 
created: 
title: "Menu::title"
tags: []
signature: "pub fn title (self) -> & 'static str"
---

The title on the bar — and the `Action::group` this menu draws.

One string for both jobs on purpose: a title that did not match the
group would open an empty menu, and nothing would say so.

# Examples

```rust
use crawrad_actions::menu::Menu;

assert_eq!(Menu::Edit.title(), "Edit");
```
