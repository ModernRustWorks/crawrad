---
id: menu-entry
kind: doc
target_file: src/menu.rs
line_hint: 23
author: 
created: 
title: "entry"
tags: []
signature: "pub fn entry (text : & str , checked : bool , shortcut : & str) -> String"
---

Formats a menu bar entry caption without leading check padding.

The caption of an entry of the MainMenu's bar: the same, without the
check mark's room when it is not checked.

# Examples

```rust
use crawrad_runtime_makepad::menu::entry;

assert_eq!(entry("&File", false, ""), "File");
```
