---
id: palette-row-hint
kind: doc
target_file: src/palette.rs
line_hint: 80
author: 
created: 
title: "Row::hint"
tags: []
signature: "pub fn hint (& self) -> Option < String >"
---

The tooltip of a greyed Toolbox row, worded as a greyed menu row's.

# Examples

```rust
use crawrad_designer::palette::Row;

let row = Row::Item { class: "SqliteProvider", label: "SQLite", blocked: Some("no disk") };
assert_eq!(row.hint().as_deref(), Some("SQLite needs a desktop project: no disk"));
assert_eq!(row.class(), None);
```
