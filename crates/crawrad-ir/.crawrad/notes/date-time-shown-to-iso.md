---
id: date-time-shown-to-iso
kind: doc
target_file: src/date_time.rs
line_hint: 232
author: 
created: 
title: "shown_to_iso"
tags: []
signature: "pub fn shown_to_iso (kind : DateTimeKind , shown : & str) -> Option < String >"
---

Converts a display string to canonical ISO if complete and valid.

Returns `None` if the input is incomplete or contains invalid dates/times.

# Examples

```rust
use crawrad_ir::date_time::{shown_to_iso, DateTimeKind};

assert_eq!(shown_to_iso(DateTimeKind::Date, "29/02/2024"), Some("2024-02-29".to_string()));
assert_eq!(shown_to_iso(DateTimeKind::Date, "31/02/2024"), None);
```
