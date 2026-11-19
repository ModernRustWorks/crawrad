---
id: debug-debug-location
kind: doc
target_file: src/debug.rs
line_hint: 289
author: 
created: 
title: "Debug::location"
tags: []
signature: "pub fn location (& self) -> Option < (& Path , u32) >"
---

Where the program stands: the file and one-based line of the top frame.

# Examples

```rust
use crawrad_host::Debug;

let debug = Debug::default();
assert_eq!(debug.location(), None);
```
