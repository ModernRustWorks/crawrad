---
id: palette-row-class
kind: doc
target_file: src/palette.rs
line_hint: 71
author: 
created: 
title: "Row::class"
tags: []
signature: "pub fn class (& self) -> Option < & 'static str >"
---

The class that this row arms, or `None` when it is just a label.

# Examples

```rust
use crawrad_designer::palette::Row;

let row = Row::Item { class: "Button", label: "Button", blocked: None };
assert_eq!(row.class(), Some("Button"));
```
