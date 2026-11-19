---
id: preferred-shortcut
kind: doc
target_file: src/lib.rs
line_hint: 5990
author: 
created: 
title: "preferred_shortcut"
tags: []
signature: "pub fn preferred_shortcut (id : & str) -> Option < & 'static str >"
---

The preferred chord that the interface DISPLAYS — the first in the list.

A single one, because a menu has one shortcut column, not three. The first is the
shortest to read, and the others are the same key written as the keyboard
delivers it. See [`PREFERRED_SHORTCUTS`].

# Examples

```rust
use crawrad_actions::preferred_shortcut;

assert!(preferred_shortcut("view.ui_zoom_in").is_some());
```
