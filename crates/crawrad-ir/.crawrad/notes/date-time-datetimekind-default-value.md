---
id: date-time-datetimekind-default-value
kind: doc
target_file: src/date_time.rs
line_hint: 59
author: 
created: 
title: "DateTimeKind::default_value"
tags: []
signature: "pub fn default_value (& self) -> & 'static str"
---

The default designer value for this kind.

# Examples

```rust
use crawrad_ir::date_time::{DateTimeKind, DEFAULT_DATE, DEFAULT_TIME};

assert_eq!(DateTimeKind::Date.default_value(), DEFAULT_DATE);
assert_eq!(DateTimeKind::Time.default_value(), DEFAULT_TIME);
```
