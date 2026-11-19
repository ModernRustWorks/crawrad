---
id: clock-days-from-civil
kind: doc
target_file: src/clock.rs
line_hint: 98
author: 
created: 
title: "days_from_civil"
tags: []
signature: "pub fn days_from_civil (y : i64 , m : u32 , d : u32) -> i64"
---

The day count since 1970-01-01 of a calendar date.

The day count since 1970-01-01 of a calendar date (the inverse of `civil_from_days`).

# Examples

```rust
use crawrad_os::clock::days_from_civil;

assert_eq!(days_from_civil(1970, 1, 1), 0);
```
