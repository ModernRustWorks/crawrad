---
id: date-time-format-shown-time
kind: doc
target_file: src/date_time.rs
line_hint: 203
author: 
created: 
title: "format_shown_time"
tags: []
signature: "pub fn format_shown_time (hour : u32 , min : u32 , sec : u32) -> String"
---

Formats `(hour, minute, second)` as display time `HH:MM:SS`.

# Examples

```rust
use crawrad_ir::date_time::format_shown_time;

assert_eq!(format_shown_time(9, 5, 1), "09:05:01");
```
