---
id: menu-panel-height
kind: doc
target_file: src/menu.rs
line_hint: 320
author: 
created: 
title: "panel_height"
tags: []
signature: "pub fn panel_height (rows : & [Row] , row : f64 , rule : f64 , padding : f64) -> f64"
---

The total height of a menu panel holding the given rows.

The height of a menu panel holding `rows`: never less than one row, plus
the padding at both ends — the panel fits exactly, or the list scrolls.

# Examples

```rust
use crawrad_actions::menu::panel_height;
use crawrad_actions::Row;

let rows = vec![Row::rule()];
assert_eq!(panel_height(&rows, 24.0, 4.0, 5.0), 34.0);
```
