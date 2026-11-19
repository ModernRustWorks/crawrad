---
id: design-rows-limit-of
kind: doc
target_file: src/design_rows.rs
line_hint: 87
author: 
created: 
title: "limit_of"
tags: []
signature: "pub fn limit_of(value: Option<&str>) -> Option<usize>"
---

How many rows the Designer shows for a `design_rows` value: `None` is Off.

# Examples

```rust
use crawrad_ir::design_rows::limit_of;

assert_eq!(limit_of(Some("Off")), None);
assert_eq!(limit_of(Some("20")), Some(20));
```
