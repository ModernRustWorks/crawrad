---
id: date-time-format-iso-date
kind: doc
target_file: src/date_time.rs
line_hint: 188
author: 
created: 
title: "format_iso_date"
tags: []
signature: "pub fn format_iso_date (year : i32 , month : u32 , day : u32) -> String"
---

Formats `(year, month, day)` as canonical ISO `YYYY-MM-DD`.

# Examples

```rust
use crawrad_ir::date_time::format_iso_date;

assert_eq!(format_iso_date(2024, 2, 29), "2024-02-29");
```
