---
id: date-time-format-shown-date
kind: doc
target_file: src/date_time.rs
line_hint: 198
author: 
created: 
title: "format_shown_date"
tags: []
signature: "pub fn format_shown_date (year : i32 , month : u32 , day : u32) -> String"
---

Formats `(year, month, day)` as display date `dd/mm/yyyy`.

# Examples

```rust
use crawrad_ir::date_time::format_shown_date;

assert_eq!(format_shown_date(2024, 2, 29), "29/02/2024");
```
