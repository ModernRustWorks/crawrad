---
id: date-time-days-in-month
kind: doc
target_file: src/date_time.rs
line_hint: 89
author: 
created: 
title: "days_in_month"
tags: []
signature: "pub fn days_in_month (year : i32 , month : u32) -> u32"
---

Returns the number of days in the given month (1..=12), or 0 if out of range.

# Examples

```rust
use crawrad_ir::date_time::days_in_month;

assert_eq!(days_in_month(2024, 2), 29);
assert_eq!(days_in_month(2023, 2), 28);
assert_eq!(days_in_month(2024, 4), 30);
assert_eq!(days_in_month(2024, 13), 0);
```
