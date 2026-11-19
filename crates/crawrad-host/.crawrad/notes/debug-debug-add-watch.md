---
id: debug-debug-add-watch
kind: doc
target_file: src/debug.rs
line_hint: 165
author: 
created: 
title: "Debug::add_watch"
tags: []
signature: "pub fn add_watch (& mut self , expression : & str) -> bool"
---

Adds an expression to watch, returning whether it was added.

Watches `expression` (trimmed; an empty or repeated one is refused).
Returns whether it was added.

# Examples

```rust
use crawrad_host::Debug;

let mut debug = Debug::default();
assert!(debug.add_watch("x + 1"));
assert!(!debug.add_watch("x + 1"));
```
