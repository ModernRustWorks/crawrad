---
id: debug-debug-remove-watch
kind: doc
target_file: src/debug.rs
line_hint: 176
author: 
created: 
title: "Debug::remove_watch"
tags: []
signature: "pub fn remove_watch (& mut self , index : usize) -> Option < String >"
---

Stops watching the expression at `index`. Returns it, if there was one.

# Examples

```rust
use crawrad_host::Debug;

let mut debug = Debug::default();
debug.add_watch("x");
assert_eq!(debug.remove_watch(0), Some("x".to_string()));
```
