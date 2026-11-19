---
id: date-time-time-mask
kind: doc
target_file: src/date_time.rs
line_hint: 14
author: 
created: 
title: "TIME_MASK"
tags: []
signature: "pub const TIME_MASK : & str = \"00:00:00;1;_\""
---

The display and entry mask for times (`HH:MM:SS`).

# Examples

```rust
use crawrad_ir::date_time::TIME_MASK;

assert_eq!(TIME_MASK, "00:00:00;1;_");
```
