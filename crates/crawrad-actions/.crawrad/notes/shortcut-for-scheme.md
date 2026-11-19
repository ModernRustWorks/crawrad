---
id: shortcut-for-scheme
kind: doc
target_file: src/lib.rs
line_hint: 1437
author: 
created: 
title: "shortcut_for_scheme"
tags: []
signature: "pub fn shortcut_for_scheme (id : & str , scheme : KeymapScheme) -> Option < & 'static str >"
---

Returns the display shortcut of an action for a given keymap scheme.

# Examples

```rust
use crawrad_actions::{shortcut_for_scheme, KeymapScheme};

assert_eq!(shortcut_for_scheme("file.save", KeymapScheme::Modern), Some("Ctrl+S"));
```
