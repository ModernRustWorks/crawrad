---
id: clock-civil-from-days
kind: doc
target_file: src/clock.rs
line_hint: 85
author: 
created: 
title: "civil_from_days"
tags: []
signature: "pub fn civil_from_days (z : i64) -> (i64 , u32 , u32)"
---

The calendar date of a day count since 1970-01-01 (Howard Hinnant's algorithm).

# Examples

```rust
use crawrad_os::clock::civil_from_days;

assert_eq!(civil_from_days(0), (1970, 1, 1));
```
