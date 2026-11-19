---
id: date-time-default-time
kind: doc
target_file: src/date_time.rs
line_hint: 20
author: 
created: 
title: "DEFAULT_TIME"
tags: []
signature: "pub const DEFAULT_TIME : & str = \"00:00:00\""
---

The default designer time value (canonical ISO `HH:MM:SS`).

# Examples

```rust
use crawrad_ir::date_time::DEFAULT_TIME;

assert_eq!(DEFAULT_TIME, "00:00:00");
```
