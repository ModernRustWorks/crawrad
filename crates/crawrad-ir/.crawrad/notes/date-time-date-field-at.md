---
id: date-time-date-field-at
kind: doc
target_file: src/date_time.rs
line_hint: 269
author: 
created: 
title: "date_field_at"
tags: []
signature: "pub fn date_field_at (caret : usize) -> DateField"
---

Returns which date field contains the caret index in `dd/mm/yyyy`.

# Examples

```rust
use crawrad_ir::date_time::{date_field_at, DateField};

assert_eq!(date_field_at(1), DateField::Day);
assert_eq!(date_field_at(4), DateField::Month);
assert_eq!(date_field_at(8), DateField::Year);
```
