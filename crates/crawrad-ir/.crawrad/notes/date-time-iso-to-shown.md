---
id: date-time-iso-to-shown
kind: doc
target_file: src/date_time.rs
line_hint: 208
author: 
created: 
title: "iso_to_shown"
tags: []
signature: "pub fn iso_to_shown (kind : DateTimeKind , iso : & str) -> String"
---

Converts a canonical ISO string to the masked display string.

An empty `iso` is no value and shows no text; an invalid one shows what
the mask makes of it.

# Examples

```rust
use crawrad_ir::date_time::{iso_to_shown, DateTimeKind};

assert_eq!(iso_to_shown(DateTimeKind::Date, "2024-02-29"), "29/02/2024");
assert_eq!(iso_to_shown(DateTimeKind::Time, "14:30:00"), "14:30:00");
assert_eq!(iso_to_shown(DateTimeKind::Date, ""), "");
```
