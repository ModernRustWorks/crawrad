---
id: explorer-rows-icon-rowicon
kind: doc
target_file: src/explorer_rows/icon.rs
line_hint: 41
author: 
created: 
title: "RowIcon"
tags: []
signature: "pub type RowIcon = & 'static str"
---

One drawing of the Codicons font — the character, and nothing else.

A bare string and not a struct carrying the drawing's name beside it, which
is what `Chrome` does. The name earns a field THERE because that table is
compared against the egui shell's, and two tables agreeing on a code point
while disagreeing on what it means would pass a comparison of code points
alone. Here there is one table and one reader, so the name lives in the doc
of each constant and the constant's own name says the rest.

# Examples

```rust
use crawrad_project::explorer_rows::icon::RowIcon;

let icon: RowIcon = "\u{ea83}";
assert!(!icon.is_empty());
```
