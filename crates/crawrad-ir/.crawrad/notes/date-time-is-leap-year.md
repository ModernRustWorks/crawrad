---
id: date-time-is-leap-year
kind: doc
target_file: src/date_time.rs
line_hint: 84
author: 
created: 
title: "is_leap_year"
tags: []
signature: "pub fn is_leap_year (year : i32) -> bool"
---

Returns true if `year` is a leap year in the Gregorian calendar.

# Examples

```rust
use crawrad_ir::date_time::is_leap_year;

assert!(is_leap_year(2024));
assert!(!is_leap_year(2023));
assert!(!is_leap_year(1900));
assert!(is_leap_year(2000));
```
