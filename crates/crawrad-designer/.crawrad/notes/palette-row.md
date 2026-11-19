---
id: palette-row
kind: doc
target_file: src/palette.rs
line_hint: 52
author: 
created: 
title: "Row"
tags: []
signature: "pub enum Row"
---

A Toolbox row.

The section label is a **row**, and not a header outside the list:
`PortalList` numbers everything it draws, and the click action returns by index.

# Examples

```rust
use crawrad_designer::palette::Row;

let row = Row::Empty;
assert_eq!(row.text(), "no component matches");
```
