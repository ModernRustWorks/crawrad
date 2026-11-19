---
id: menu-caption
kind: doc
target_file: src/menu.rs
line_hint: 12
author: 
created: 
title: "caption"
tags: []
signature: "pub fn caption (text : & str , checked : bool , shortcut : & str) -> String"
---

Formats an item caption with check mark, accelerator strip, and shortcut.

The caption an item shows: a check mark when `checked`, and the
`shortcut` text after it when there is one. The `&` of the accelerator
is not shown (`&&` is one `&`).

# Examples

```rust
use crawrad_runtime_makepad::menu::caption;

assert_eq!(caption("&Save", false, "Ctrl+S"), "    Save    Ctrl+S");
```
