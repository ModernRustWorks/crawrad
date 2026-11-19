---
id: date-time-parse-shown-time
kind: doc
target_file: src/date_time.rs
line_hint: 169
author: 
created: 
title: "parse_shown_time"
tags: []
signature: "pub fn parse_shown_time (shown : & str) -> Option < (u32 , u32 , u32) >"
---

Parses a display time in `HH:MM:SS` format into `(hour, minute, second)`.

# Examples

```rust
use crawrad_ir::date_time::parse_shown_time;

assert_eq!(parse_shown_time("14:30:00"), Some((14, 30, 0)));
assert_eq!(parse_shown_time("invalid"), None);
```
