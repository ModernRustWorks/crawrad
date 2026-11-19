---
id: date-time-parse-iso-time
kind: doc
target_file: src/date_time.rs
line_hint: 134
author: 
created: 
title: "parse_iso_time"
tags: []
signature: "pub fn parse_iso_time (iso : & str) -> Option < (u32 , u32 , u32) >"
---

Parses a time string in ISO format (`HH:MM:SS`) into `(hour, minute, second)`.

# Examples

```rust
use crawrad_ir::date_time::parse_iso_time;

assert_eq!(parse_iso_time("14:30:00"), Some((14, 30, 0)));
assert_eq!(parse_iso_time("99:99:99"), None);
```
