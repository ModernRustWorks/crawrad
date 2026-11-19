---
id: date-time-step-time-field
kind: doc
target_file: src/date_time.rs
line_hint: 334
author: 
created: 
title: "step_time_field"
tags: []
signature: "pub fn step_time_field (mut hour : u32 , mut min : u32 , mut sec : u32 , field : TimeField , up : bool ,) -> (u32 , u32 , u32)"
---

Steps a time field up or down, wrapping within its range.

# Examples

```rust
use crawrad_ir::date_time::{step_time_field, TimeField};

assert_eq!(step_time_field(23, 59, 59, TimeField::Hour, true), (0, 59, 59));
assert_eq!(step_time_field(0, 0, 0, TimeField::Minute, false), (0, 59, 0));
```
