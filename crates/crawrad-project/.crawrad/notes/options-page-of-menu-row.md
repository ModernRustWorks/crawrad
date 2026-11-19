---
id: options-page-of-menu-row
kind: doc
target_file: src/options.rs
line_hint: 56
author: 
created: 
title: "page_of_menu_row"
tags: []
signature: "pub fn page_of_menu_row (label : & str) -> Option < Category >"
---

The page a row of the gear's or the tabs' `...` menu opens the Options on.

# Examples

```rust
use crawrad_project::options::{Category, page_of_menu_row};

assert_eq!(page_of_menu_row("Keyboard Shortcuts"), Some(Category::Keyboard));
assert_eq!(page_of_menu_row("Unknown"), None);
```
