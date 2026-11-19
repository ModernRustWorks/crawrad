---
id: clock-local-hh-mm
kind: doc
target_file: src/clock.rs
line_hint: 13
author: 
created: 
title: "local_hh_mm"
tags: []
signature: "pub fn local_hh_mm () -> String"
---

Returns the current local time formatted as `"HH:MM"`.

On Windows, queries the operating system local time using the Win32 `GetLocalTime` API.
On other platforms, falls back to UTC time derived from `std::time::SystemTime`.

# Examples

```rust
use crawrad_os::clock::local_hh_mm;

let time = local_hh_mm();
assert_eq!(time.len(), 5);
```
