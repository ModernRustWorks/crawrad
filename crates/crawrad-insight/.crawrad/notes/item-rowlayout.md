---
id: item-rowlayout
kind: doc
target_file: src/item.rs
line_hint: 130
author: 
created: 
title: "RowLayout"
tags: []
signature: "pub struct RowLayout"
---

Visual layout measurements and strikeout ranges for a row.

A row laid out in its width: the signature cut to the room left, and
where the strike over a deprecated label runs, `(from, to)` (#730).

# Examples

```rust
use crawrad_insight::item::RowLayout;

let layout = RowLayout { signature: "".into(), strike: (0.0, 0.0) };
assert_eq!(layout.strike, (0.0, 0.0));
```
