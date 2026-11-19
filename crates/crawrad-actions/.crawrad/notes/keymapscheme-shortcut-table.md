---
id: keymapscheme-shortcut-table
kind: doc
target_file: src/lib.rs
line_hint: 1080
author: 
created: 
title: "KeymapScheme::shortcut_table"
tags: []
signature: "pub fn shortcut_table (self) -> Vec < (& 'static str , & 'static str) >"
---

The shortcuts the Options keyboard page lists for this scheme.

# Examples

```rust
use crawrad_actions::KeymapScheme;

let table = KeymapScheme::VsCode.shortcut_table();
assert!(!table.is_empty());
```
