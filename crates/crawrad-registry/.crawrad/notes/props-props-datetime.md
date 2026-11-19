---
id: props-props-datetime
kind: doc
target_file: src/props.rs
line_hint: 532
author: 
created: 
title: "Props::datetime"
tags: []
signature: "pub fn datetime (default : crawrad_ir :: DateTime) -> PropDesc"
---

Date and time (`DateTime`).

# Examples

```rust
use crawrad_ir::DateTime;
use crawrad_registry::Props;

let p = Props::datetime(DateTime::new(2026, 1, 1, 12, 0, 0));
assert_eq!(p.key, "datetime");
```
