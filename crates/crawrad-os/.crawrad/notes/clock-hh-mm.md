---
id: clock-hh-mm
kind: doc
target_file: src/clock.rs
line_hint: 8
author: 
created: 
title: "hh_mm"
tags: []
signature: "pub fn hh_mm (hour : u32 , minute : u32) -> String"
---

Formats hour and minute into a 24-hour zero-padded `"HH:MM"` string.

# Examples

```rust
use crawrad_os::clock::hh_mm;

assert_eq!(hh_mm(9, 5), "09:05");
```
