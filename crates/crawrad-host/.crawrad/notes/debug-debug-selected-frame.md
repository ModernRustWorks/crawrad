---
id: debug-debug-selected-frame
kind: doc
target_file: src/debug.rs
line_hint: 209
author: 
created: 
title: "Debug::selected_frame"
tags: []
signature: "pub fn selected_frame (& self) -> usize"
---

The frame the panel shows.

# Examples

```rust
use crawrad_host::Debug;

let debug = Debug::default();
assert_eq!(debug.selected_frame(), 0);
```
