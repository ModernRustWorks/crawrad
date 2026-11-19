---
id: menu-row-rule
kind: doc
target_file: src/menu.rs
line_hint: 146
author: 
created: 
title: "Row::rule"
tags: []
signature: "pub fn rule () -> Self"
---

The hairline between two blocks.

# Examples

```rust
use crawrad_actions::Row;

let row = Row::rule();
assert!(row.separator);
```
