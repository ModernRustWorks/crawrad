---
id: date-time-datetimekind
kind: doc
target_file: src/date_time.rs
line_hint: 23
author: 
created: 
title: "DateTimeKind"
tags: []
signature: "pub enum DateTimeKind"
---

The entry and display mode of a `DateTimePicker`.

# Examples

```rust
use crawrad_ir::date_time::DateTimeKind;

assert_eq!(DateTimeKind::default(), DateTimeKind::Date);
assert_eq!(DateTimeKind::Time.as_str(), "dtkTime");
```
