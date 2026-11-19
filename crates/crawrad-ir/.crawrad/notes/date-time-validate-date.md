---
id: date-time-validate-date
kind: doc
target_file: src/date_time.rs
line_hint: 105
author: 
created: 
title: "validate_date"
tags: []
signature: "pub fn validate_date (year : i32 , month : u32 , day : u32) -> bool"
---

Validates that `year`, `month`, and `day` represent a valid Gregorian date.

# Examples

```rust
use crawrad_ir::date_time::validate_date;

assert!(validate_date(2024, 2, 29));
assert!(!validate_date(2023, 2, 29));
assert!(!validate_date(2024, 4, 31));
```
