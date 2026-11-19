---
id: date-time-parse-iso-date
kind: doc
target_file: src/date_time.rs
line_hint: 118
author: 
created: 
title: "parse_iso_date"
tags: []
signature: "pub fn parse_iso_date (iso : & str) -> Option < (i32 , u32 , u32) >"
---

Parses a date string in ISO format (`YYYY-MM-DD`) into `(year, month, day)`.

# Examples

```rust
use crawrad_ir::date_time::parse_iso_date;

assert_eq!(parse_iso_date("2024-02-29"), Some((2024, 2, 29)));
assert_eq!(parse_iso_date("invalid"), None);
```
