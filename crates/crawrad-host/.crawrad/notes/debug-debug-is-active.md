---
id: debug-debug-is-active
kind: doc
target_file: src/debug.rs
line_hint: 272
author: 
created: 
title: "Debug::is_active"
tags: []
signature: "pub fn is_active (& self) -> bool"
---

A session exists and has not ended.

# Examples

```rust
use crawrad_host::Debug;

let debug = Debug::default();
assert!(!debug.is_active());
```
