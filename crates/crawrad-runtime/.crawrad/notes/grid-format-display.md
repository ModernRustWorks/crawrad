---
id: grid-format-display
kind: doc
target_file: src/grid.rs
line_hint: 698
author: 
created: 
title: "format_display"
tags: []
signature: "pub fn format_display (text : & str , mask : & str) -> String"
---

A cell's text through a column's `DisplayFormat` mask (#936).

`#` and `0` are digits (`0` always shown), `,` groups thousands, `.` starts the
decimals; text around the digits is kept. Text that is not a number, or an
empty mask, comes back unchanged.

# Examples

```rust
use crawrad_runtime::grid::format_display;

assert_eq!(format_display("1250", "#,##0.00"), "1,250.00");
assert_eq!(format_display("5", "000"), "005");
assert_eq!(format_display("Recife", "#,##0.00"), "Recife");
```
