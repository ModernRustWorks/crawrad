---
id: history-now
kind: doc
target_file: src/history.rs
line_hint: 198
author: 
created: 
title: "now"
tags: []
signature: "pub fn now () -> u64"
---

Returns current system time in seconds since the Unix epoch.

Returns the current system time in seconds since the Unix epoch, or 0 on clock error.

# Examples

```rust
use crawrad_host::history::now;

assert!(now() > 0);
```
