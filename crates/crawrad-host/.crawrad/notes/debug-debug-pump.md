---
id: debug-debug-pump
kind: doc
target_file: src/debug.rs
line_hint: 364
author: 
created: 
title: "Debug::pump"
tags: []
signature: "pub fn pump (& mut self) -> Vec < DebugEvent >"
---

Applies what the session reported since the last call.

# Examples

```rust
use crawrad_host::Debug;

let mut debug = Debug::default();
assert!(debug.pump().is_empty());
```
