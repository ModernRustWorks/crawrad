---
id: debug-debug-current-stop
kind: doc
target_file: src/debug.rs
line_hint: 284
author: 
created: 
title: "Debug::current_stop"
tags: []
signature: "pub fn current_stop (& self) -> Option < & Stop >"
---

The last stop, while the program is stopped.

# Examples

```rust
use crawrad_host::Debug;

let debug = Debug::default();
assert!(debug.current_stop().is_none());
```
