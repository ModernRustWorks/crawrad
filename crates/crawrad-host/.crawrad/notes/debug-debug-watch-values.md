---
id: debug-debug-watch-values
kind: doc
target_file: src/debug.rs
line_hint: 160
author: 
created: 
title: "Debug::watch_values"
tags: []
signature: "pub fn watch_values (& self) -> Option < & [ShownVariable] >"
---

Returns the watch values for the current frame if stopped.

The watches' values in the frame the panel shows; `None` while they
are on their way or nothing is stopped.

# Examples

```rust
use crawrad_host::Debug;

let debug = Debug::default();
assert!(debug.watch_values().is_none());
```
