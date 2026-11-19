---
id: debug-debug-shown-locals
kind: doc
target_file: src/debug.rs
line_hint: 240
author: 
created: 
title: "Debug::shown_locals"
tags: []
signature: "pub fn shown_locals (& self) -> Option < & [ShownVariable] >"
---

The locals of the frame the panel shows; `None` while they are on
their way.

# Examples

```rust
use crawrad_host::Debug;

let debug = Debug::default();
assert!(debug.shown_locals().is_none());
```
