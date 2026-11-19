---
id: date-time-datetimekind-as-str
kind: doc
target_file: src/date_time.rs
line_hint: 43
author: 
created: 
title: "DateTimeKind::as_str"
tags: []
signature: "pub fn as_str (& self) -> & 'static str"
---

The string identifier used in property streams.

# Examples

```rust
use crawrad_ir::date_time::DateTimeKind;

assert_eq!(DateTimeKind::Date.as_str(), "dtkDate");
assert_eq!(DateTimeKind::Time.as_str(), "dtkTime");
```
