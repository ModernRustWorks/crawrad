---
id: date-time-date-mask
kind: doc
target_file: src/date_time.rs
line_hint: 11
author: 
created: 
title: "DATE_MASK"
tags: []
signature: "pub const DATE_MASK : & str = \"00/00/0000;1;_\""
---

The display and entry mask for dates (`dd/mm/yyyy`).

# Examples

```rust
use crawrad_ir::date_time::DATE_MASK;

assert_eq!(DATE_MASK, "00/00/0000;1;_");
```
