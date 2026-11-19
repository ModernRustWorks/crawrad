---
id: date-time-format-iso-time
kind: doc
target_file: src/date_time.rs
line_hint: 193
author: 
created: 
title: "format_iso_time"
tags: []
signature: "pub fn format_iso_time (hour : u32 , min : u32 , sec : u32) -> String"
---

Formats `(hour, minute, second)` as canonical ISO `HH:MM:SS`.

# Examples

```rust
use crawrad_ir::date_time::format_iso_time;

assert_eq!(format_iso_time(9, 5, 1), "09:05:01");
```
