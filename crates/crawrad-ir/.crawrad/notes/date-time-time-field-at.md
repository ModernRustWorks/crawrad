---
id: date-time-time-field-at
kind: doc
target_file: src/date_time.rs
line_hint: 280
author: 
created: 
title: "time_field_at"
tags: []
signature: "pub fn time_field_at (caret : usize) -> TimeField"
---

Returns which time field contains the caret index in `HH:MM:SS`.

# Examples

```rust
use crawrad_ir::date_time::{time_field_at, TimeField};

assert_eq!(time_field_at(1), TimeField::Hour);
assert_eq!(time_field_at(4), TimeField::Minute);
assert_eq!(time_field_at(7), TimeField::Second);
```
