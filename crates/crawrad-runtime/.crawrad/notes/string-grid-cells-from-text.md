---
id: string-grid-cells-from-text
kind: doc
target_file: src/string_grid.rs
line_hint: 13
author: 
created: 
title: "Cells::from_text"
tags: []
signature: "pub fn from_text (text : & str) -> Self"
---

Parses cell contents from TSV formatted text.

The design-time text: a line per row, a tab between columns (what a
spreadsheet copy gives). Empty pieces stay unset; CRLF is accepted.

# Examples

```rust
use crawrad_runtime::string_grid::Cells;

let cells = Cells::from_text("A\tB\nC\tD");
assert_eq!(cells.get(1, 0), "B");
assert_eq!(cells.get(0, 1), "C");
```
