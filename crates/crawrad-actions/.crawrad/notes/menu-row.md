---
id: menu-row
kind: doc
target_file: src/menu.rs
line_hint: 108
author: 
created: 
title: "Row"
tags: []
signature: "pub struct Row"
---

One row of an open menu.

No `Eq`: [`Args`] carries a `Value`, which carries an `f64`. It never could
have `Eq`, and an argument that is a coordinate is exactly the case that
put `Args` in the catalogue in the first place.

# Examples

```rust
use crawrad_actions::Row;

let rule = Row::rule();
assert!(rule.separator);
```
