---
id: props-props-min-date
kind: doc
target_file: src/props.rs
line_hint: 2175
author: 
created: 
title: "Props::min_date"
tags: []
signature: "pub fn min_date (default : & str) -> PropDesc"
---

Minimum permitted date or time in ISO format (`MinDate`, #21).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::min_date("1900-01-01");
assert_eq!(p.key, "min_date");
```
