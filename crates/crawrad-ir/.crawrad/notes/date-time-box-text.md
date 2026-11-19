---
id: date-time-box-text
kind: doc
target_file: src/date_time.rs
line_hint: 246
author: 
created: 
title: "box_text"
tags: []
signature: "pub fn box_text (kind : DateTimeKind , typed : & str , value : & str) -> Option < String >"
---

Determines the entry box text, or `None` to leave it unchanged.

Determines what the entry box text must be set to, or `None` to leave it alone (#21).

Returns `None` when `typed` does not parse to a valid ISO value (the user is
still typing, e.g. `12/0_/____` or has an invalid complete date like `31/02/2024`)
or when it parses to exactly `value`.
Otherwise returns `Some(iso_to_shown(kind, value))` (for example when the program
changed the value programmatically, or when the value was clamped by min/max).

# Examples

```rust
use crawrad_ir::date_time::{box_text, DateTimeKind};

assert_eq!(box_text(DateTimeKind::Date, "29/02/2024", "2024-02-29"), None);
assert_eq!(box_text(DateTimeKind::Date, "01/01/2000", "2024-02-29"), Some("29/02/2024".to_string()));
```
