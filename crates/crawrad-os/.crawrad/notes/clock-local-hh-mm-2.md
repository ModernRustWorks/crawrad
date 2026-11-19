---
id: clock-local-hh-mm-2
kind: doc
target_file: src/clock.rs
line_hint: 28
author: 
created: 
title: "local_hh_mm"
tags: []
signature: "pub fn local_hh_mm () -> String"
---

Returns the current local time formatted as `"HH:MM"`.

On non-Windows platforms, falls back to UTC time derived from `std::time::SystemTime`.

# Examples

```rust
use crawrad_os::clock::local_hh_mm;

let time = local_hh_mm();
assert_eq!(time.len(), 5);
```
