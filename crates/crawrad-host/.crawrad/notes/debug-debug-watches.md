---
id: debug-debug-watches
kind: doc
target_file: src/debug.rs
line_hint: 155
author: 
created: 
title: "Debug::watches"
tags: []
signature: "pub fn watches (& self) -> & [String]"
---

The watched expressions, in order.

# Examples

```rust
use crawrad_host::Debug;

let debug = Debug::default();
assert!(debug.watches().is_empty());
```
