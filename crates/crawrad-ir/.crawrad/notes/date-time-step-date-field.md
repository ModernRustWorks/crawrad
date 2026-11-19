---
id: date-time-step-date-field
kind: doc
target_file: src/date_time.rs
line_hint: 291
author: 
created: 
title: "step_date_field"
tags: []
signature: "pub fn step_date_field (mut year : i32 , mut month : u32 , mut day : u32 , field : DateField , up : bool ,) -> (i32 , u32 , u32)"
---

Steps a date field up or down within its bounds.

Steps a date field up or down, wrapping within its natural bounds
and clamping days to the target month's capacity.

# Examples

```rust
use crawrad_ir::date_time::{step_date_field, DateField};

assert_eq!(step_date_field(2024, 2, 28, DateField::Day, true), (2024, 2, 29));
assert_eq!(step_date_field(2024, 2, 29, DateField::Day, true), (2024, 2, 1));
```
