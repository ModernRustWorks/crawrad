---
id: debug-debug-toggle-variable
kind: doc
target_file: src/debug.rs
line_hint: 250
author: 
created: 
title: "Debug::toggle_variable"
tags: []
signature: "pub fn toggle_variable (& mut self , reference : i64) -> bool"
---

Opens or closes a variable by reference in the debug panel.

Opens or closes a variable (▸/▾); its children are asked of the
session the first time. Returns whether it is open now.

# Examples

```rust
use crawrad_host::Debug;

let mut debug = Debug::default();
assert!(!debug.toggle_variable(1));
```
