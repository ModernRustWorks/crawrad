---
id: menu-rows-height
kind: doc
target_file: src/menu.rs
line_hint: 313
author: 
created: 
title: "rows_height"
tags: []
signature: "pub fn rows_height (rows : & [Row] , row : f64 , rule : f64) -> f64"
---

The height of `rows` stacked: a rule is `rule` tall, any other row `row`.

# Examples

```rust
use crawrad_actions::menu::rows_height;
use crawrad_actions::Row;

let rows = vec![Row::rule()];
assert_eq!(rows_height(&rows, 24.0, 4.0), 4.0);
```
