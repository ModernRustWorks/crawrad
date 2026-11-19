---
id: registry-all
kind: doc
target_file: src/lib.rs
line_hint: 690
author: 
created: 
title: "Registry::all"
tags: []
signature: "pub fn all (& self) -> impl Iterator < Item = & ComponentDesc >"
---

All descriptors, in class order.

# Examples

```rust
use crawrad_registry::Registry;

let reg = Registry::builtin();
assert!(reg.all().count() > 0);
```
