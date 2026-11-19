---
id: debug-debug-output
kind: doc
target_file: src/debug.rs
line_hint: 302
author: 
created: 
title: "Debug::output"
tags: []
signature: "pub fn output (& self) -> & [String]"
---

What the program printed in this session.

# Examples

```rust
use crawrad_host::Debug;

let debug = Debug::default();
assert!(debug.output().is_empty());
```
