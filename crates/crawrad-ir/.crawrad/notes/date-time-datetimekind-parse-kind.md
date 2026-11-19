---
id: date-time-datetimekind-parse-kind
kind: doc
target_file: src/date_time.rs
line_hint: 34
author: 
created: 
title: "DateTimeKind::parse_kind"
tags: []
signature: "pub fn parse_kind (s : & str) -> Self"
---

Parses a registry string identifier (`dtkDate` or `dtkTime`).

# Examples

```rust
use crawrad_ir::date_time::DateTimeKind;

assert_eq!(DateTimeKind::parse_kind("dtkDate"), DateTimeKind::Date);
assert_eq!(DateTimeKind::parse_kind("dtkTime"), DateTimeKind::Time);
```
