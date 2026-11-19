---
id: palette-row-text
kind: doc
target_file: src/palette.rs
line_hint: 79
author: 
created: 
title: "Row::text"
tags: []
signature: "pub fn text (& self) -> & 'static str"
---

The text that the row shows.

# Examples

```rust
use crawrad_designer::palette::Row;

let row = Row::Section("Standard");
assert_eq!(row.text(), "Standard");
```
