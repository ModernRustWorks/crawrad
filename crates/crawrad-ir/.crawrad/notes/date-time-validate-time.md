---
id: date-time-validate-time
kind: doc
target_file: src/date_time.rs
line_hint: 113
author: 
created: 
title: "validate_time"
tags: []
signature: "pub fn validate_time (hour : u32 , min : u32 , sec : u32) -> bool"
---

Validates that `hour`, `min`, and `sec` represent a valid 24-hour time.

# Examples

```rust
use crawrad_ir::date_time::validate_time;

assert!(validate_time(23, 59, 59));
assert!(!validate_time(24, 0, 0));
assert!(!validate_time(12, 60, 0));
```
