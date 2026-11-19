---
id: date-time-datefield
kind: doc
target_file: src/date_time.rs
line_hint: 68
author: 
created: 
title: "DateField"
tags: []
signature: "pub enum DateField"
---

Identifies which date field the caret currently points to.

# Examples

```rust
use crawrad_ir::date_time::{date_field_at, DateField};

assert_eq!(date_field_at(0), DateField::Day);
assert_eq!(date_field_at(3), DateField::Month);
assert_eq!(date_field_at(6), DateField::Year);
```
