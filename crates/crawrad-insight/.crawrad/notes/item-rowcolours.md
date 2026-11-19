---
id: item-rowcolours
kind: doc
target_file: src/item.rs
line_hint: 88
author: 
created: 
title: "RowColours"
tags: []
signature: "pub struct RowColours"
---

Color configuration for rendering a completion list row.

A row's colours, `[r, g, b, a]`: background, background under the
mouse, text. The selected row in the accent blue of a classic RAD IDE.

# Examples

```rust
use crawrad_insight::item::RowColours;

let colours = RowColours::of(true);
assert_eq!(colours.text, [1.0, 1.0, 1.0, 1.0]);
```
