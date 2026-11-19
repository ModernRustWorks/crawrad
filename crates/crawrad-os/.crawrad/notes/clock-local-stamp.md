---
id: clock-local-stamp
kind: doc
target_file: src/clock.rs
line_hint: 42
author: 
created: 
title: "local_stamp"
tags: []
signature: "pub fn local_stamp () -> String"
---

Returns the local date and time as `"YYYY-MM-DD HH:MM"`.

The local date and time as `"YYYY-MM-DD HH:MM"` (#478: when a conversation
was last used). Outside Windows, UTC.

# Examples

```rust
use crawrad_os::clock::local_stamp;

let stamp = local_stamp();
assert_eq!(stamp.len(), 16);
```
