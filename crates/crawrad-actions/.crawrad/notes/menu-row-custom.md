---
id: menu-row-custom
kind: doc
target_file: src/menu.rs
line_hint: 168
author: 
created: 
title: "Row::custom"
tags: []
signature: "pub fn custom (label : impl Into < Cow < 'static , str > > , shortcut : Option < & 'static str >) -> Self"
---

A custom uncatalogued row (e.g. from Manage popup or extensions).

# Examples

```rust
use crawrad_actions::Row;

let row = Row::custom("Custom Action", Some("Ctrl+Shift+X"));
assert_eq!(row.label, "Custom Action");
```
