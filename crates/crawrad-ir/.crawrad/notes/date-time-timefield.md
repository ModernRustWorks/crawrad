---
id: date-time-timefield
kind: doc
target_file: src/date_time.rs
line_hint: 76
author: 
created: 
title: "TimeField"
tags: []
signature: "pub enum TimeField"
---

Identifies which time field the caret currently points to.

# Examples

```rust
use crawrad_ir::date_time::{time_field_at, TimeField};

assert_eq!(time_field_at(0), TimeField::Hour);
assert_eq!(time_field_at(3), TimeField::Minute);
assert_eq!(time_field_at(6), TimeField::Second);
```
