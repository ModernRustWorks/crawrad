---
id: date-time-datetimekind-mask
kind: doc
target_file: src/date_time.rs
line_hint: 51
author: 
created: 
title: "DateTimeKind::mask"
tags: []
signature: "pub fn mask (& self) -> & 'static str"
---

The input mask used for entry and display.

# Examples

```rust
use crawrad_ir::date_time::{DateTimeKind, DATE_MASK, TIME_MASK};

assert_eq!(DateTimeKind::Date.mask(), DATE_MASK);
assert_eq!(DateTimeKind::Time.mask(), TIME_MASK);
```
