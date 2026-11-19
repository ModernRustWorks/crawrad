---
id: date-time-parse-shown-date
kind: doc
target_file: src/date_time.rs
line_hint: 150
author: 
created: 
title: "parse_shown_date"
tags: []
signature: "pub fn parse_shown_date (shown : & str) -> Option < (i32 , u32 , u32) >"
---

Parses a display date in `dd/mm/yyyy` format into `(year, month, day)`.

# Examples

```rust
use crawrad_ir::date_time::parse_shown_date;

assert_eq!(parse_shown_date("29/02/2024"), Some((2024, 2, 29)));
assert_eq!(parse_shown_date("31/02/2024"), None);
```
