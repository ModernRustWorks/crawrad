---
id: date-time-clamp-iso
kind: doc
target_file: src/date_time.rs
line_hint: 255
author: 
created: 
title: "clamp_iso"
tags: []
signature: "pub fn clamp_iso (iso : & str , min : & str , max : & str) -> String"
---

Clamps an ISO date or time string between optional `min` and `max` bounds.

# Examples

```rust
use crawrad_ir::date_time::clamp_iso;

assert_eq!(clamp_iso("2024-05-10", "2024-06-01", "2024-12-31"), "2024-06-01");
assert_eq!(clamp_iso("2025-01-01", "2024-06-01", "2024-12-31"), "2024-12-31");
assert_eq!(clamp_iso("2024-07-01", "2024-06-01", "2024-12-31"), "2024-07-01");
```
