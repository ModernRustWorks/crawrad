---
id: date-time-step-iso
kind: doc
target_file: src/date_time.rs
line_hint: 368
author: 
created: 
title: "step_iso"
tags: []
signature: "pub fn step_iso (kind : DateTimeKind , iso : & str , caret : usize , up : bool , min : & str , max : & str ,) -> Option < String >"
---

Steps the field under the caret in `iso`.

Steps the field under the caret in `iso`, respecting min and max limits.
Returns the new ISO string if the input was valid and could be stepped.

# Examples

```rust
use crawrad_ir::date_time::{step_iso, DateTimeKind};

let stepped = step_iso(DateTimeKind::Date, "2024-02-28", 0, true, "", "");
assert_eq!(stepped, Some("2024-02-29".to_string()));
```
