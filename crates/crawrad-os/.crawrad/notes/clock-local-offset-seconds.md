---
id: clock-local-offset-seconds
kind: doc
target_file: src/clock.rs
line_hint: 109
author: 
created: 
title: "local_offset_seconds"
tags: []
signature: "pub fn local_offset_seconds () -> i64"
---

Returns current local time offset from UTC in seconds.

Returns the current local time offset from UTC in seconds (east of UTC is positive).

On Windows, queries `GetLocalTime` and `GetSystemTime` and rounds the difference to the nearest minute.
On other platforms, returns 0.

# Examples

```rust
use crawrad_os::clock::local_offset_seconds;

let offset = local_offset_seconds();
assert_eq!(offset % 60, 0);
```
