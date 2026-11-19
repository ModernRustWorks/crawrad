---
id: registry-builtin
kind: doc
target_file: src/lib.rs
line_hint: 673
author: 
created: 
title: "Registry::builtin"
tags: []
signature: "pub fn builtin () -> Self"
---

The built-in component set of CrawRAD.

# Examples

```rust
use crawrad_registry::Registry;

let reg = Registry::builtin();
assert!(reg.get("Button").is_some());
```
