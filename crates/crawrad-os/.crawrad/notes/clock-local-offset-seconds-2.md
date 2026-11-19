---
id: clock-local-offset-seconds-2
kind: doc
target_file: src/clock.rs
line_hint: 136
author: 
created: 
title: "local_offset_seconds"
tags: []
signature: "pub fn local_offset_seconds () -> i64"
---

Returns current local time offset from UTC in seconds.

Returns the current local time offset from UTC in seconds (east of UTC is positive).

On non-Windows platforms, returns 0.

# Examples

```rust
use crawrad_os::clock::local_offset_seconds;

let offset = local_offset_seconds();
assert_eq!(offset % 60, 0);
```
