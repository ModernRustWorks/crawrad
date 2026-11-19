---
id: value-datetime
kind: doc
target_file: src/value.rs
line_hint: 408
author: 
created: 
title: "DateTime"
tags: []
signature: "pub struct DateTime"
---

Date and time in the `.crawform` document.

Date and time in the `.crawform` document: year, month, day, hour, minute, second.

# Examples

```rust
use crawrad_ir::DateTime;

let dt = DateTime::epoch();
assert_eq!(dt.year, 1970);
assert_eq!(dt.month, 1);
```
