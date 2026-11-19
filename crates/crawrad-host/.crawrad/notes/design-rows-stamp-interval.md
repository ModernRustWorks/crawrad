---
id: design-rows-stamp-interval
kind: doc
target_file: src/design_rows.rs
line_hint: 21
author: 
created: 
title: "STAMP_INTERVAL"
tags: []
signature: "pub const STAMP_INTERVAL: Duration = Duration::from_secs(1)"
---

How often the database files' modification times are looked at again.

# Examples

```rust
use crawrad_host::design_rows::STAMP_INTERVAL;

assert_eq!(STAMP_INTERVAL.as_secs(), 1);
```
