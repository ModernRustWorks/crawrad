---
id: recover-when
kind: doc
target_file: src/recover.rs
line_hint: 228
author: 
created: 
title: "when"
tags: []
signature: "pub fn when (then : SystemTime , now : SystemTime , offset : i64) -> String"
---

Formats a timestamp the way the recovery list shows it.

When a time was, the way the recovery list shows it: `HH:MM` today,
`yesterday`, else the date. `offset` is the local zone's offset from UTC.

# Examples

```rust
use std::time::SystemTime;
use crawrad_project::recover::when;

let text = when(SystemTime::UNIX_EPOCH, SystemTime::UNIX_EPOCH, 0);
assert_eq!(text, "00:00");
```
