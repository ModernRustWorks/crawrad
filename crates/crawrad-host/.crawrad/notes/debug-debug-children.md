---
id: debug-debug-children
kind: doc
target_file: src/debug.rs
line_hint: 263
author: 
created: 
title: "Debug::children"
tags: []
signature: "pub fn children (& self , reference : i64) -> Option < Option < & [ShownVariable] > >"
---

Returns the child variables of an opened variable reference.

`None` when the variable is closed; `Some(None)` while its children
are on their way.

# Examples

```rust
use crawrad_host::Debug;

let debug = Debug::default();
assert!(debug.children(1).is_none());
```
