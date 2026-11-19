---
id: components-all-builtin
kind: doc
target_file: src/components/mod.rs
line_hint: 116
author: 
created: 
title: "all_builtin"
tags: []
signature: "pub fn all_builtin () -> Vec < ComponentDesc >"
---

Returns the list of all built-in component descriptors of CrawRAD.

# Examples

```rust
use crawrad_registry::all_builtin;

let descs = all_builtin();
assert!(!descs.is_empty());
```
